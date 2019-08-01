/* pngpread.c - read a png file in push mode
 *
 * Copyright (c) 2018 Cosmin Truta
 * Copyright (c) 1998-2002,2004,2006-2018 Glenn Randers-Pehrson
 * Copyright (c) 1996-1997 Andreas Dilger
 * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */

#include "pngpriv.h"

#ifdef PNG_PROGRESSIVE_READ_SUPPORTED

/* Push model modes */
#define PNG_READ_SIG_MODE   0
#define PNG_READ_CHUNK_MODE 1
#define PNG_READ_IDAT_MODE  2
#define PNG_READ_tEXt_MODE  4
#define PNG_READ_zTXt_MODE  5
#define PNG_READ_DONE_MODE  6
#define PNG_READ_iTXt_MODE  7
#define PNG_ERROR_MODE      8

void PNGAPI
png_process_data(png_structrp png_ptr, png_inforp info_ptr,
    png_bytep buffer, size_t buffer_size)
{
   if (png_ptr == NULL || info_ptr == NULL)
      return;

   png_rust_process_data(png_ptr->rust_ptr, info_ptr->rust_ptr, buffer, buffer_size);
}

size_t PNGAPI
png_process_data_pause(png_structrp png_ptr, int save)
{
   if (png_ptr == NULL)
   {
      return 0;
   }

   return png_rust_process_data_pause(png_ptr->rust_ptr, save);
}

png_uint_32 PNGAPI
png_process_data_skip(png_structrp png_ptr)
{
/* TODO: Deprecate and remove this API.
 * Somewhere the implementation of this seems to have been lost,
 * or abandoned.  It was only to support some internal back-door access
 * to png_struct) in libpng-1.4.x.
 */
   png_app_warning(png_ptr,
"png_process_data_skip is not implemented in any current version of libpng");
   return 0;
}



void /* PRIVATE */
png_process_IDAT_data(png_structrp png_ptr, png_bytep buffer,
    size_t buffer_length)
{
   /* The caller checks for a non-zero buffer length. */
   if (!(buffer_length > 0) || buffer == NULL)
      png_error(png_ptr, "No IDAT data (internal error)");

   /* This routine must process all the data it has been given
    * before returning, calling the row callback as required to
    * handle the uncompressed results.
    */
   png_ptr->zstream.next_in = buffer;
   /* TODO: WARNING: TRUNCATION ERROR: DANGER WILL ROBINSON: */
   png_ptr->zstream.avail_in = (uInt)buffer_length;

   /* Keep going until the decompressed data is all processed
    * or the stream marked as finished.
    */
   while (png_ptr->zstream.avail_in > 0 &&
          ! png_rust_has_flags(png_ptr->rust_ptr, PNG_FLAG_ZSTREAM_ENDED))
   {
      int ret;

      /* We have data for zlib, but we must check that zlib
       * has someplace to put the results.  It doesn't matter
       * if we don't expect any results -- it may be the input
       * data is just the LZ end code.
       */
      if (!(png_ptr->zstream.avail_out > 0))
      {
         /* TODO: WARNING: TRUNCATION ERROR: DANGER WILL ROBINSON: */
         png_ptr->zstream.avail_out = (uInt)(PNG_ROWBYTES(png_rust_get_pixel_depth(png_ptr->rust_ptr),
             png_rust_get_iwidth(png_ptr->rust_ptr)) + 1);

         png_ptr->zstream.next_out = png_rust_get_row_buf(png_ptr->rust_ptr);
      }

      /* Using Z_SYNC_FLUSH here means that an unterminated
       * LZ stream (a stream with a missing end code) can still
       * be handled, otherwise (Z_NO_FLUSH) a future zlib
       * implementation might defer output and therefore
       * change the current behavior (see comments in inflate.c
       * for why this doesn't happen at present with zlib 1.2.5).
       */
      ret = PNG_INFLATE(png_ptr, Z_SYNC_FLUSH);

      /* Check for any failure before proceeding. */
      if (ret != Z_OK && ret != Z_STREAM_END)
      {
         /* Terminate the decompression. */
         png_rust_add_flags(png_ptr->rust_ptr, PNG_FLAG_ZSTREAM_ENDED);
         png_rust_set_zowner(png_ptr->rust_ptr, 0);

         /* This may be a truncated stream (missing or
          * damaged end code).  Treat that as a warning.
          */
         if (png_rust_get_row_number(png_ptr->rust_ptr) >= png_rust_get_num_rows(png_ptr->rust_ptr) ||
             ! png_rust_pass_is_valid(png_ptr->rust_ptr) )
            png_warning(png_ptr, "Truncated compressed data in IDAT");

         else
         {
            if (ret == Z_DATA_ERROR)
               png_benign_error(png_ptr, "IDAT: ADLER32 checksum mismatch");
            else
               png_error(png_ptr, "Decompression error in IDAT");
         }

         /* Skip the check on unprocessed input */
         return;
      }

      /* Did inflate output any data? */
      if (png_ptr->zstream.next_out != png_rust_get_row_buf(png_ptr->rust_ptr))
      {
         /* Is this unexpected data after the last row?
          * If it is, artificially terminate the LZ output
          * here.
          */
         if (png_rust_get_row_number(png_ptr->rust_ptr) >= png_rust_get_num_rows(png_ptr->rust_ptr) ||
             ! png_rust_pass_is_valid(png_ptr->rust_ptr))
         {
            /* Extra data. */
            png_warning(png_ptr, "Extra compressed data in IDAT");
            png_rust_add_flags(png_ptr->rust_ptr, PNG_FLAG_ZSTREAM_ENDED);
            png_rust_set_zowner(png_ptr->rust_ptr, 0);

            /* Do no more processing; skip the unprocessed
             * input check below.
             */
            return;
         }

         /* Do we have a complete row? */
         if (png_ptr->zstream.avail_out == 0)
            png_push_process_row(png_ptr);
      }

      /* And check for the end of the stream. */
      if (ret == Z_STREAM_END)
         png_rust_add_flags(png_ptr->rust_ptr, PNG_FLAG_ZSTREAM_ENDED);
   }

   /* All the data should have been processed, if anything
    * is left at this point we have bytes of IDAT data
    * after the zlib end code.
    */
   if (png_ptr->zstream.avail_in > 0)
      png_warning(png_ptr, "Extra compression data in IDAT");
}

void /* PRIVATE */
png_push_process_row(png_structrp png_ptr)
{
   /* 1.5.6: row_info moved out of png_struct to a local here. */
   png_row_info row_info;

   row_info.width = png_rust_get_iwidth(png_ptr->rust_ptr); /* NOTE: width of current interlaced row */
   row_info.color_type = png_rust_get_color_type(png_ptr->rust_ptr);
   row_info.bit_depth = png_rust_get_bit_depth(png_ptr->rust_ptr);
   row_info.channels = png_rust_get_channels(png_ptr->rust_ptr);
   row_info.pixel_depth = png_rust_get_pixel_depth(png_ptr->rust_ptr);
   row_info.rowbytes = PNG_ROWBYTES(row_info.pixel_depth, row_info.width);

   if (png_rust_get_row_buf(png_ptr->rust_ptr)[0] > PNG_FILTER_VALUE_NONE)
   {
      if (png_rust_get_row_buf(png_ptr->rust_ptr)[0] < PNG_FILTER_VALUE_LAST)
         png_read_filter_row(png_ptr, &row_info, png_rust_get_row_buf(png_ptr->rust_ptr) + 1,
            png_rust_get_prev_row(png_ptr->rust_ptr) + 1, png_rust_get_row_buf(png_ptr->rust_ptr)[0]);
      else
         png_error(png_ptr, "bad adaptive filter value");
   }

   /* libpng 1.5.6: the following line was copying png_ptr->rowbytes before
    * 1.5.6, while the buffer really is this big in current versions of libpng
    * it may not be in the future, so this was changed just to copy the
    * interlaced row count:
    */
   memcpy(png_rust_get_prev_row(png_ptr->rust_ptr), png_rust_get_row_buf(png_ptr->rust_ptr), row_info.rowbytes + 1);

#ifdef PNG_READ_TRANSFORMS_SUPPORTED
   if ( ! png_rust_empty_transformations(png_ptr->rust_ptr) )
      png_do_read_transformations(png_ptr, &row_info);
#endif

   /* The transformed pixel depth should match the depth now in row_info. */
   if (png_rust_get_transformed_pixel_depth(png_ptr->rust_ptr) == 0)
   {
      png_rust_set_transformed_pixel_depth(png_ptr->rust_ptr, row_info.pixel_depth);
      if (row_info.pixel_depth > png_rust_get_maximum_pixel_depth(png_ptr->rust_ptr))
         png_error(png_ptr, "progressive row overflow");
   }

   else if (png_rust_get_transformed_pixel_depth(png_ptr->rust_ptr) != row_info.pixel_depth)
      png_error(png_ptr, "internal progressive row size calculation error");


#ifdef PNG_READ_INTERLACING_SUPPORTED
   /* Expand interlaced rows to full size */
   if (png_rust_get_interlace(png_ptr->rust_ptr) != 0 &&
       png_rust_has_transformations(png_ptr->rust_ptr, PNG_INTERLACE))
   {
      if (png_rust_get_pass(png_ptr->rust_ptr) < 6)
         png_do_read_interlace(&row_info, png_rust_get_row_buf(png_ptr->rust_ptr) + 1, png_rust_get_pass(png_ptr->rust_ptr),
             png_rust_get_transformations(png_ptr->rust_ptr));

      switch (png_rust_get_pass(png_ptr->rust_ptr))
      {
         case 0:
         {
            int i;
            for (i = 0; i < 8 && png_rust_get_pass(png_ptr->rust_ptr) == 0; i++)
            {
               png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
               png_read_push_finish_row(png_ptr->rust_ptr); /* Updates png_ptr->pass */
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 2) /* Pass 1 might be empty */
            {
               for (i = 0; i < 4 && png_rust_get_pass(png_ptr->rust_ptr) == 2; i++)
               {
                  png_push_have_row(png_ptr, NULL);
                  png_read_push_finish_row(png_ptr->rust_ptr);
               }
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 4 && png_rust_get_height(png_ptr->rust_ptr) <= 4)
            {
               for (i = 0; i < 2 && png_rust_get_pass(png_ptr->rust_ptr) == 4; i++)
               {
                  png_push_have_row(png_ptr, NULL);
                  png_read_push_finish_row(png_ptr->rust_ptr);
               }
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 6 && png_rust_get_height(png_ptr->rust_ptr) <= 4)
            {
                png_push_have_row(png_ptr, NULL);
                png_read_push_finish_row(png_ptr->rust_ptr);
            }

            break;
         }

         case 1:
         {
            int i;
            for (i = 0; i < 8 && png_rust_get_pass(png_ptr->rust_ptr) == 1; i++)
            {
               png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 2) /* Skip top 4 generated rows */
            {
               for (i = 0; i < 4 && png_rust_get_pass(png_ptr->rust_ptr) == 2; i++)
               {
                  png_push_have_row(png_ptr, NULL);
                  png_read_push_finish_row(png_ptr->rust_ptr);
               }
            }

            break;
         }

         case 2:
         {
            int i;

            for (i = 0; i < 4 && png_rust_get_pass(png_ptr->rust_ptr) == 2; i++)
            {
               png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            for (i = 0; i < 4 && png_rust_get_pass(png_ptr->rust_ptr) == 2; i++)
            {
               png_push_have_row(png_ptr, NULL);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 4) /* Pass 3 might be empty */
            {
               for (i = 0; i < 2 && png_rust_get_pass(png_ptr->rust_ptr) == 4; i++)
               {
                  png_push_have_row(png_ptr, NULL);
                  png_read_push_finish_row(png_ptr->rust_ptr);
               }
            }

            break;
         }

         case 3:
         {
            int i;

            for (i = 0; i < 4 && png_rust_get_pass(png_ptr->rust_ptr) == 3; i++)
            {
               png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 4) /* Skip top two generated rows */
            {
               for (i = 0; i < 2 && png_rust_get_pass(png_ptr->rust_ptr) == 4; i++)
               {
                  png_push_have_row(png_ptr, NULL);
                  png_read_push_finish_row(png_ptr->rust_ptr);
               }
            }

            break;
         }

         case 4:
         {
            int i;

            for (i = 0; i < 2 && png_rust_get_pass(png_ptr->rust_ptr) == 4; i++)
            {
               png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            for (i = 0; i < 2 && png_rust_get_pass(png_ptr->rust_ptr) == 4; i++)
            {
               png_push_have_row(png_ptr, NULL);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 6) /* Pass 5 might be empty */
            {
               png_push_have_row(png_ptr, NULL);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            break;
         }

         case 5:
         {
            int i;

            for (i = 0; i < 2 && png_rust_get_pass(png_ptr->rust_ptr) == 5; i++)
            {
               png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            if (png_rust_get_pass(png_ptr->rust_ptr) == 6) /* Skip top generated row */
            {
               png_push_have_row(png_ptr, NULL);
               png_read_push_finish_row(png_ptr->rust_ptr);
            }

            break;
         }

         default:
         case 6:
         {
            png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
            png_read_push_finish_row(png_ptr->rust_ptr);

            if (png_rust_get_pass(png_ptr->rust_ptr) != 6)
               break;

            png_push_have_row(png_ptr, NULL);
            png_read_push_finish_row(png_ptr->rust_ptr);
         }
      }
   }
   else
#endif
   {
      png_push_have_row(png_ptr, png_rust_get_row_buf(png_ptr->rust_ptr) + 1);
      png_read_push_finish_row(png_ptr->rust_ptr);
   }
}

void /* PRIVATE */
png_push_have_info(png_structrp png_ptr, png_inforp info_ptr)
{
   if (png_rust_get_info_fn(png_ptr->rust_ptr) != NULL)
      (*(png_rust_get_info_fn(png_ptr->rust_ptr)))(png_ptr, info_ptr);
}

void /* PRIVATE */
png_push_have_end(png_structrp png_ptr, png_inforp info_ptr)
{
   if (png_rust_get_end_fn(png_ptr->rust_ptr) != NULL)
      (*(png_rust_get_end_fn(png_ptr->rust_ptr)))(png_ptr, info_ptr);
}

void /* PRIVATE */
png_push_have_row(png_structrp png_ptr, png_bytep row)
{
   if (png_rust_get_row_fn(png_ptr->rust_ptr) != NULL)
      (*(png_rust_get_row_fn(png_ptr->rust_ptr)))(png_ptr, row, png_rust_get_row_number(png_ptr->rust_ptr),
          (int)png_rust_get_pass(png_ptr->rust_ptr));
}

#ifdef PNG_READ_INTERLACING_SUPPORTED
void PNGAPI
png_progressive_combine_row(png_const_structrp png_ptr, png_bytep old_row,
    png_const_bytep new_row)
{
   if (png_ptr == NULL)
      return;

   /* new_row is a flag here - if it is NULL then the app callback was called
    * from an empty row (see the calls to png_struct::row_fn below), otherwise
    * it must be png_ptr->row_buf+1
    */
   if (new_row != NULL)
      png_combine_row(png_ptr, old_row, 1/*blocky display*/);
}
#endif /* READ_INTERLACING */

void PNGAPI
png_set_progressive_read_fn(png_structrp png_ptr, png_voidp progressive_ptr,
                            png_progressive_info_ptr info_fn, png_progressive_row_ptr row_fn,
                            png_progressive_end_ptr end_fn)
{
   png_rust_set_progressive_read_fn(png_ptr->rust_ptr, progressive_ptr,
                                    info_fn, row_fn, end_fn);
}

png_voidp PNGAPI
png_get_progressive_ptr(png_const_structrp png_ptr)
{
   if (png_ptr == NULL)
      return (NULL);

   return png_rust_get_io_ptr(png_ptr->rust_ptr);
}

void /* PRIVATE */
png_push_fill_buffer(png_structp png_ptr, png_bytep buffer, size_t length)
{
   png_rust_push_fill_buffer(png_ptr->rust_ptr, buffer, length);
}


png_voidp /* PRIVATE */
png_push_fill_buffer_func_ptr()
{
   return (void*)&png_push_fill_buffer;
}

void
png_c_set_zstream_avail_out(png_structp png_ptr, size_t value)
{
   png_ptr->zstream.avail_out = value;
}

void
png_c_set_zstream_next_out(png_structp png_ptr, void* value)
{
   png_ptr->zstream.next_out = value;
}


#endif /* PROGRESSIVE_READ */
