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

#define PNG_PUSH_SAVE_BUFFER_IF_FULL \
if (png_rust_get_push_length(png_ptr->rust_ptr) + 4 > png_rust_get_buffer_size(png_ptr->rust_ptr)) \
   { png_push_save_buffer(png_ptr); return; }
#define PNG_PUSH_SAVE_BUFFER_IF_LT(N) \
if (png_rust_get_buffer_size(png_ptr->rust_ptr) < N) \
   { png_push_save_buffer(png_ptr); return; }

void PNGAPI
png_process_data(png_structrp png_ptr, png_inforp info_ptr,
    png_bytep buffer, size_t buffer_size)
{
   if (png_ptr == NULL || info_ptr == NULL)
      return;

   png_push_restore_buffer(png_ptr, buffer, buffer_size);

   while (png_rust_get_buffer_size(png_ptr->rust_ptr))
   {
      png_process_some_data(png_ptr, info_ptr);
   }
}

size_t PNGAPI
png_process_data_pause(png_structrp png_ptr, int save)
{
   if (png_ptr != NULL)
   {
      /* It's easiest for the caller if we do the save; then the caller doesn't
       * have to supply the same data again:
       */
      if (save != 0)
         png_push_save_buffer(png_ptr);
      else
      {
         /* This includes any pending saved bytes: */
         size_t remaining = png_rust_get_buffer_size(png_ptr->rust_ptr);
         png_rust_set_buffer_size(png_ptr->rust_ptr, 0);

         /* So subtract the saved buffer size, unless all the data
          * is actually 'saved', in which case we just return 0
          */
         if (png_rust_get_save_buffer_size(png_ptr->rust_ptr) < remaining)
            return remaining - png_rust_get_save_buffer_size(png_ptr->rust_ptr);
      }
   }

   return 0;
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

/* What we do with the incoming data depends on what we were previously
 * doing before we ran out of data...
 */
void /* PRIVATE */
png_process_some_data(png_structrp png_ptr, png_inforp info_ptr)
{
   if (png_ptr == NULL)
      return;

   switch (png_rust_get_process_mode(png_ptr->rust_ptr))
   {
      case PNG_READ_SIG_MODE:
      {
         png_push_read_sig(png_ptr, info_ptr);
         break;
      }

      case PNG_READ_CHUNK_MODE:
      {
         png_push_read_chunk(png_ptr, info_ptr);
         break;
      }

      case PNG_READ_IDAT_MODE:
      {
         png_push_read_IDAT(png_ptr->rust_ptr);
         break;
      }

      default:
      {
         png_rust_set_buffer_size(png_ptr->rust_ptr, 0);
         break;
      }
   }
}

/* Read any remaining signature bytes from the stream and compare them with
 * the correct PNG signature.  It is possible that this routine is called
 * with bytes already read from the signature, either because they have been
 * checked by the calling application, or because of multiple calls to this
 * routine.
 */
void /* PRIVATE */
png_push_read_sig(png_structrp png_ptr, png_inforp info_ptr)
{
   size_t num_checked = png_rust_get_sig_bytes(png_ptr->rust_ptr); /* SAFE, does not exceed 8 */
   size_t num_to_check = 8 - num_checked;

   if (png_rust_get_buffer_size(png_ptr->rust_ptr) < num_to_check)
   {
      num_to_check = png_rust_get_buffer_size(png_ptr->rust_ptr);
   }

   png_push_fill_buffer(png_ptr, &(info_ptr->signature[num_checked]),
       num_to_check);
   png_rust_set_sig_bytes(png_ptr->rust_ptr, (png_byte)(png_rust_get_sig_bytes(png_ptr->rust_ptr) + num_to_check));

   if (png_sig_cmp(info_ptr->signature, num_checked, num_to_check))
   {
      if (num_checked < 4 &&
          png_sig_cmp(info_ptr->signature, num_checked, num_to_check - 4))
         png_error(png_ptr, "Not a PNG file");

      else
         png_error(png_ptr, "PNG file corrupted by ASCII conversion");
   }
   else
   {
      if (png_rust_get_sig_bytes(png_ptr->rust_ptr) >= 8)
      {
         png_rust_set_process_mode(png_ptr->rust_ptr, PNG_READ_CHUNK_MODE);
      }
   }
}

void /* PRIVATE */
png_push_read_chunk(png_structrp png_ptr, png_inforp info_ptr)
{
   png_uint_32 chunk_name;
#ifdef PNG_HANDLE_AS_UNKNOWN_SUPPORTED
   int keep; /* unknown handling method */
#endif

   /* First we make sure we have enough data for the 4-byte chunk name
    * and the 4-byte chunk length before proceeding with decoding the
    * chunk data.  To fully decode each of these chunks, we also make
    * sure we have enough data in the buffer for the 4-byte CRC at the
    * end of every chunk (except IDAT, which is handled separately).
    */
   if ( ! png_rust_has_mode(png_ptr->rust_ptr, PNG_HAVE_CHUNK_HEADER) )
   {
      png_byte chunk_length[4];
      png_byte chunk_tag[4];

      PNG_PUSH_SAVE_BUFFER_IF_LT(8)
      png_push_fill_buffer(png_ptr, chunk_length, 4);
      png_rust_set_push_length(png_ptr->rust_ptr, png_get_uint_31(png_ptr, chunk_length));
      png_reset_crc(png_ptr);
      png_crc_read(png_ptr, chunk_tag, 4);
      png_rust_set_chunk_name(png_ptr->rust_ptr, PNG_CHUNK_FROM_STRING(chunk_tag));
      png_check_chunk_name(png_ptr, png_rust_get_chunk_name(png_ptr->rust_ptr));
      png_check_chunk_length(png_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
      png_rust_add_mode(png_ptr->rust_ptr, PNG_HAVE_CHUNK_HEADER);
   }

   chunk_name = png_rust_get_chunk_name(png_ptr->rust_ptr);

   if (chunk_name == png_IDAT)
   {
      if (png_rust_has_mode(png_ptr->rust_ptr, PNG_AFTER_IDAT))
         png_rust_add_mode(png_ptr->rust_ptr, PNG_HAVE_CHUNK_AFTER_IDAT);

      /* If we reach an IDAT chunk, this means we have read all of the
       * header chunks, and we can start reading the image (or if this
       * is called after the image has been read - we have an error).
       */
      if ( ! png_rust_has_mode(png_ptr->rust_ptr, PNG_HAVE_IHDR) )
         png_error(png_ptr, "Missing IHDR before IDAT");

      else if (png_rust_is_color_type(png_ptr->rust_ptr, PNG_COLOR_TYPE_PALETTE) &&
               ! png_rust_has_mode(png_ptr->rust_ptr, PNG_HAVE_PLTE) )
         png_error(png_ptr, "Missing PLTE before IDAT");

      png_rust_set_process_mode(png_ptr->rust_ptr, PNG_READ_IDAT_MODE);

      if (png_rust_has_mode(png_ptr->rust_ptr, PNG_HAVE_IDAT) &&
          ! png_rust_has_mode(png_ptr->rust_ptr, PNG_HAVE_CHUNK_AFTER_IDAT) &&
          png_rust_get_push_length(png_ptr->rust_ptr) == 0)
         return;

      png_rust_add_mode(png_ptr->rust_ptr, PNG_HAVE_IDAT);

      if (png_rust_has_mode(png_ptr->rust_ptr, PNG_AFTER_IDAT))
         png_benign_error(png_ptr, "Too many IDATs found");
   }

   if (chunk_name == png_IHDR)
   {
      if (png_rust_get_push_length(png_ptr->rust_ptr) != 13)
         png_error(png_ptr, "Invalid IHDR length");

      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_IHDR(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

   else if (chunk_name == png_IEND)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_IEND(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));

      png_rust_set_process_mode(png_ptr->rust_ptr, PNG_READ_DONE_MODE);
      png_push_have_end(png_ptr, info_ptr);
   }

#ifdef PNG_HANDLE_AS_UNKNOWN_SUPPORTED
   else if ((keep = png_chunk_unknown_handling(png_ptr, chunk_name)) != 0)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_unknown(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr), keep);

      if (chunk_name == png_PLTE)
         png_rust_add_mode(png_ptr->rust_ptr, PNG_HAVE_PLTE);
   }
#endif

   else if (chunk_name == png_PLTE)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_PLTE(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

   else if (chunk_name == png_IDAT)
   {
      png_rust_set_idat_size(png_ptr->rust_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
      png_rust_set_process_mode(png_ptr->rust_ptr, PNG_READ_IDAT_MODE);
      png_push_have_info(png_ptr, info_ptr);
      png_ptr->zstream.avail_out =
          (uInt) PNG_ROWBYTES(png_rust_get_pixel_depth(png_ptr->rust_ptr),
          png_rust_get_iwidth(png_ptr->rust_ptr)) + 1;
      png_ptr->zstream.next_out = png_rust_get_row_buf(png_ptr->rust_ptr);
      return;
   }

#ifdef PNG_READ_gAMA_SUPPORTED
   else if (png_rust_get_chunk_name(png_ptr->rust_ptr) == png_gAMA)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_gAMA(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_sBIT_SUPPORTED
   else if (png_rust_get_chunk_name(png_ptr->rust_ptr) == png_sBIT)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_sBIT(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_cHRM_SUPPORTED
   else if (png_rust_get_chunk_name(png_ptr->rust_ptr) == png_cHRM)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_cHRM(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_sRGB_SUPPORTED
   else if (chunk_name == png_sRGB)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_sRGB(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_iCCP_SUPPORTED
   else if (png_rust_get_chunk_name(png_ptr->rust_ptr) == png_iCCP)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_iCCP(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_sPLT_SUPPORTED
   else if (chunk_name == png_sPLT)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_sPLT(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_tRNS_SUPPORTED
   else if (chunk_name == png_tRNS)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_tRNS(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_bKGD_SUPPORTED
   else if (chunk_name == png_bKGD)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_bKGD(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_hIST_SUPPORTED
   else if (chunk_name == png_hIST)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_hIST(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_pHYs_SUPPORTED
   else if (chunk_name == png_pHYs)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_pHYs(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_oFFs_SUPPORTED
   else if (chunk_name == png_oFFs)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_oFFs(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }
#endif

#ifdef PNG_READ_pCAL_SUPPORTED
   else if (chunk_name == png_pCAL)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_pCAL(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_sCAL_SUPPORTED
   else if (chunk_name == png_sCAL)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_sCAL(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_tIME_SUPPORTED
   else if (chunk_name == png_tIME)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_tIME(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_tEXt_SUPPORTED
   else if (chunk_name == png_tEXt)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_tEXt(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_zTXt_SUPPORTED
   else if (chunk_name == png_zTXt)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_zTXt(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }

#endif
#ifdef PNG_READ_iTXt_SUPPORTED
   else if (chunk_name == png_iTXt)
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_iTXt(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
   }
#endif

   else
   {
      PNG_PUSH_SAVE_BUFFER_IF_FULL
      png_handle_unknown(png_ptr, info_ptr, png_rust_get_push_length(png_ptr->rust_ptr),
          PNG_HANDLE_CHUNK_AS_DEFAULT);
   }

   png_rust_remove_mode(png_ptr->rust_ptr, PNG_HAVE_CHUNK_HEADER);
}

void PNGCBAPI
png_push_fill_buffer(png_structp png_ptr, png_bytep buffer, size_t length)
{
   png_bytep ptr;

   if (png_ptr == NULL)
      return;

   ptr = buffer;
   if (png_rust_get_save_buffer_size(png_ptr->rust_ptr) != 0)
   {
      size_t save_size;

      if (length < png_rust_get_save_buffer_size(png_ptr->rust_ptr))
         save_size = length;

      else
         save_size = png_rust_get_save_buffer_size(png_ptr->rust_ptr);

      memcpy(ptr, png_rust_get_save_buffer_ptr(png_ptr->rust_ptr), save_size);
      length -= save_size;
      ptr += save_size;
      png_rust_sub_buffer_size(png_ptr->rust_ptr, save_size);
      png_rust_sub_save_buffer_size(png_ptr->rust_ptr, save_size);
      png_rust_add_save_buffer_ptr(png_ptr->rust_ptr, save_size);
   }
   if (length != 0 && png_rust_get_current_buffer_size(png_ptr->rust_ptr) != 0)
   {
      size_t save_size;

      if (length < png_rust_get_current_buffer_size(png_ptr->rust_ptr))
         save_size = length;

      else
         save_size = png_rust_get_current_buffer_size(png_ptr->rust_ptr);

      memcpy(ptr, png_rust_get_current_buffer_ptr(png_ptr->rust_ptr), save_size);
      png_rust_sub_buffer_size(png_ptr->rust_ptr, save_size);
      png_rust_sub_current_buffer_size(png_ptr->rust_ptr, save_size);
      png_rust_add_current_buffer_ptr(png_ptr->rust_ptr, save_size);
   }
}

void /* PRIVATE */
png_push_save_buffer(png_structrp png_ptr)
{
   if (png_rust_get_save_buffer_size(png_ptr->rust_ptr) != 0)
   {
      if (png_rust_get_save_buffer_ptr(png_ptr->rust_ptr) != png_rust_get_save_buffer(png_ptr->rust_ptr))
      {
         size_t i, istop;
         png_bytep sp;
         png_bytep dp;

         istop = png_rust_get_save_buffer_size(png_ptr->rust_ptr);
         for (i = 0, sp = png_rust_get_save_buffer_ptr(png_ptr->rust_ptr), dp = png_rust_get_save_buffer(png_ptr->rust_ptr);
             i < istop; i++, sp++, dp++)
         {
            *dp = *sp;
         }
      }
   }
   if (png_rust_get_save_buffer_size(png_ptr->rust_ptr) + png_rust_get_current_buffer_size(png_ptr->rust_ptr) >
       png_rust_get_save_buffer_max(png_ptr->rust_ptr))
   {
      size_t new_max;
      png_bytep old_buffer;

      if (png_rust_get_save_buffer_size(png_ptr->rust_ptr) > PNG_SIZE_MAX -
          (png_rust_get_current_buffer_size(png_ptr->rust_ptr) + 256))
      {
         png_error(png_ptr, "Potential overflow of save_buffer");
      }

      new_max = png_rust_get_save_buffer_size(png_ptr->rust_ptr) + png_rust_get_current_buffer_size(png_ptr->rust_ptr) + 256;
      old_buffer = png_rust_get_save_buffer(png_ptr->rust_ptr);
      png_rust_set_save_buffer(png_ptr->rust_ptr, (png_bytep)png_malloc_warn(png_ptr,
          (size_t)new_max));

      if (png_rust_get_save_buffer(png_ptr->rust_ptr) == NULL)
      {
         png_free(png_ptr, old_buffer);
         png_error(png_ptr, "Insufficient memory for save_buffer");
      }

      if (old_buffer)
         memcpy(png_rust_get_save_buffer(png_ptr->rust_ptr), old_buffer, png_rust_get_save_buffer_size(png_ptr->rust_ptr));
      else if (png_rust_get_save_buffer_size(png_ptr->rust_ptr))
         png_error(png_ptr, "save_buffer error");
      png_free(png_ptr, old_buffer);
      png_rust_set_save_buffer_max(png_ptr->rust_ptr, new_max);
   }
   if (png_rust_get_current_buffer_size(png_ptr->rust_ptr))
   {
      memcpy(png_rust_get_save_buffer(png_ptr->rust_ptr) + png_rust_get_save_buffer_size(png_ptr->rust_ptr),
         png_rust_get_current_buffer_ptr(png_ptr->rust_ptr), png_rust_get_current_buffer_size(png_ptr->rust_ptr));
      png_rust_add_save_buffer_size(png_ptr->rust_ptr, png_rust_get_current_buffer_size(png_ptr->rust_ptr));
      png_rust_set_current_buffer_size(png_ptr->rust_ptr, 0);
   }
   png_rust_set_save_buffer_ptr(png_ptr->rust_ptr, png_rust_get_save_buffer(png_ptr->rust_ptr));
   png_rust_set_buffer_size(png_ptr->rust_ptr, 0);
}

void /* PRIVATE */
png_push_restore_buffer(png_structrp png_ptr, png_bytep buffer,
    size_t buffer_length)
{
   png_rust_set_current_buffer(png_ptr->rust_ptr, buffer);
   png_rust_set_current_buffer_size(png_ptr->rust_ptr, buffer_length);
   png_rust_set_buffer_size(png_ptr->rust_ptr, buffer_length + png_rust_get_save_buffer_size(png_ptr->rust_ptr));
   png_rust_set_current_buffer_ptr(png_ptr->rust_ptr, png_rust_get_current_buffer(png_ptr->rust_ptr));
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
   if (png_ptr == NULL)
      return;

   png_rust_set_info_fn(png_ptr->rust_ptr, info_fn);
   png_rust_set_row_fn(png_ptr->rust_ptr, row_fn);
   png_rust_set_end_fn(png_ptr->rust_ptr, end_fn);

   png_set_read_fn(png_ptr, progressive_ptr, png_push_fill_buffer);
}

png_voidp PNGAPI
png_get_progressive_ptr(png_const_structrp png_ptr)
{
   if (png_ptr == NULL)
      return (NULL);

   return png_rust_get_io_ptr(png_ptr->rust_ptr);
}
#endif /* PROGRESSIVE_READ */
