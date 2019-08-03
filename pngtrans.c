/* pngtrans.c - transforms the data in a row (used by both readers and writers)
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

#if defined(PNG_READ_BGR_SUPPORTED) || defined(PNG_WRITE_BGR_SUPPORTED)
/* Turn on BGR-to-RGB mapping */
void PNGAPI
png_set_bgr(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_bgr(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_SWAP_SUPPORTED) || defined(PNG_WRITE_SWAP_SUPPORTED)
/* Turn on 16-bit byte swapping */
void PNGAPI
png_set_swap(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_swap(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_PACK_SUPPORTED) || defined(PNG_WRITE_PACK_SUPPORTED)
/* Turn on pixel packing */
void PNGAPI
png_set_packing(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_packing(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_PACKSWAP_SUPPORTED)||defined(PNG_WRITE_PACKSWAP_SUPPORTED)
/* Turn on packed pixel swapping */
void PNGAPI
png_set_packswap(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_pack_swap(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_SHIFT_SUPPORTED) || defined(PNG_WRITE_SHIFT_SUPPORTED)
void PNGAPI
png_set_shift(png_structrp png_ptr, png_const_color_8p true_bits)
{
   png_debug(1, "in png_set_shift");

   if (png_ptr == NULL)
      return;

   png_rust_add_transformations(png_ptr->rust_ptr, PNG_SHIFT);
   png_ptr->shift = *true_bits;
}
#endif

#if defined(PNG_READ_INTERLACING_SUPPORTED) || \
    defined(PNG_WRITE_INTERLACING_SUPPORTED)
int PNGAPI
png_set_interlace_handling(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return 1;
   return png_c_set_interlace_handling(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_FILLER_SUPPORTED) || defined(PNG_WRITE_FILLER_SUPPORTED)
/* Add a filler byte on read, or remove a filler or alpha byte on write.
 * The filler type has changed in v0.95 to allow future 2-byte fillers
 * for 48-bit input data, as well as to avoid problems with some compilers
 * that don't like bytes as parameters.
 */
void PNGAPI
png_set_filler(png_structrp png_ptr, png_uint_32 filler, int filler_loc)
{
   png_c_set_filler(png_ptr->rust_ptr, filler, filler_loc);
}

/* Added to libpng-1.2.7 */
void PNGAPI
png_set_add_alpha(png_structrp png_ptr, png_uint_32 filler, int filler_loc)
{
   png_debug(1, "in png_set_add_alpha");

   if (png_ptr == NULL)
      return;

   png_set_filler(png_ptr, filler, filler_loc);
   /* The above may fail to do anything. */
   if (png_rust_has_transformations(png_ptr->rust_ptr, PNG_FILLER))
      png_rust_add_transformations(png_ptr->rust_ptr, PNG_ADD_ALPHA);
}

#endif

#if defined(PNG_READ_SWAP_ALPHA_SUPPORTED) || \
    defined(PNG_WRITE_SWAP_ALPHA_SUPPORTED)
void PNGAPI
png_set_swap_alpha(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_swap_alpha(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_INVERT_ALPHA_SUPPORTED) || \
    defined(PNG_WRITE_INVERT_ALPHA_SUPPORTED)
void PNGAPI
png_set_invert_alpha(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_invert_alpha(png_ptr->rust_ptr);
}
#endif

#if defined(PNG_READ_INVERT_SUPPORTED) || defined(PNG_WRITE_INVERT_SUPPORTED)
void PNGAPI
png_set_invert_mono(png_structrp png_ptr)
{
   if (png_ptr == NULL)
      return;
   png_c_set_invert_mono(png_ptr->rust_ptr);
}


#if defined(PNG_READ_USER_TRANSFORM_SUPPORTED) || \
    defined(PNG_WRITE_USER_TRANSFORM_SUPPORTED)
#ifdef PNG_USER_TRANSFORM_PTR_SUPPORTED
void PNGAPI
png_set_user_transform_info(png_structrp png_ptr, png_voidp
   user_transform_ptr, int user_transform_depth, int user_transform_channels)
{
   png_debug(1, "in png_set_user_transform_info");

   if (png_ptr == NULL)
      return;

#ifdef PNG_READ_USER_TRANSFORM_SUPPORTED
   if (png_rust_has_mode(png_ptr->rust_ptr, PNG_IS_READ_STRUCT) &&
       png_rust_has_flags(png_ptr->rust_ptr, PNG_FLAG_ROW_INIT))
   {
      png_app_error(png_ptr,
          "info change after png_start_read_image or png_read_update_info");
      return;
   }
#endif

   png_ptr->user_transform_ptr = user_transform_ptr;
   png_ptr->user_transform_depth = (png_byte)user_transform_depth;
   png_ptr->user_transform_channels = (png_byte)user_transform_channels;
}
#endif

/* This function returns a pointer to the user_transform_ptr associated with
 * the user transform functions.  The application should free any memory
 * associated with this pointer before png_write_destroy and png_read_destroy
 * are called.
 */
#ifdef PNG_USER_TRANSFORM_PTR_SUPPORTED
png_voidp PNGAPI
png_get_user_transform_ptr(png_const_structrp png_ptr)
{
   if (png_ptr == NULL)
      return (NULL);

   return png_ptr->user_transform_ptr;
}
#endif

#ifdef PNG_USER_TRANSFORM_INFO_SUPPORTED
png_uint_32 PNGAPI
png_get_current_row_number(png_const_structrp png_ptr)
{
   /* See the comments in png.h - this is the sub-image row when reading an
    * interlaced image.
    */
   if (png_ptr != NULL)
      return png_rust_get_row_number(png_ptr->rust_ptr);

   return PNG_UINT_32_MAX; /* help the app not to fail silently */
}

png_byte PNGAPI
png_get_current_pass_number(png_const_structrp png_ptr)
{
   if (png_ptr != NULL)
      return png_rust_get_pass(png_ptr->rust_ptr);
   return 8; /* invalid */
}
#endif /* USER_TRANSFORM_INFO */
#endif /* READ_USER_TRANSFORM || WRITE_USER_TRANSFORM */
#endif /* READ || WRITE */
