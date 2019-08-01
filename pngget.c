/* pngget.c - retrieval of values from info struct
 *
 * Copyright (c) 2018 Cosmin Truta
 * Copyright (c) 1998-2002,2004,2006-2018 Glenn Randers-Pehrson
 * Copyright (c) 1996-1997 Andreas Dilger
 * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 */

#include "pngpriv.h"

#if defined(PNG_READ_SUPPORTED) || defined(PNG_WRITE_SUPPORTED)

png_uint_32 PNGAPI
png_get_valid(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_uint_32 flag)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return(png_info_rust_get_valid(info_ptr->rust_ptr) & flag);

   return(0);
}

size_t PNGAPI
png_get_rowbytes(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return(png_info_rust_get_rowbytes(info_ptr->rust_ptr));

   return(0);
}

#ifdef PNG_INFO_IMAGE_SUPPORTED
png_bytepp PNGAPI
png_get_rows(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return(info_ptr->row_pointers);

   return(0);
}
#endif

#ifdef PNG_EASY_ACCESS_SUPPORTED
/* Easy access to info, added in libpng-0.99 */
png_uint_32 PNGAPI
png_get_image_width(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_width(info_ptr->rust_ptr);

   return (0);
}

png_uint_32 PNGAPI
png_get_image_height(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_height(info_ptr->rust_ptr);

   return (0);
}

png_byte PNGAPI
png_get_bit_depth(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_bit_depth(info_ptr->rust_ptr);

   return (0);
}

png_byte PNGAPI
png_get_color_type(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_color_type(info_ptr->rust_ptr);

   return (0);
}

png_byte PNGAPI
png_get_filter_type(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_filter_type(info_ptr->rust_ptr);

   return (0);
}

png_byte PNGAPI
png_get_interlace_type(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_interlace_type(info_ptr->rust_ptr);

   return (0);
}

png_byte PNGAPI
png_get_compression_type(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_info_rust_get_compression_type(info_ptr->rust_ptr);

   return (0);
}

png_uint_32 PNGAPI
png_get_x_pixels_per_meter(png_const_structrp png_ptr, png_const_inforp
   info_ptr)
{
#ifdef PNG_pHYs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0)
      {
         png_debug1(1, "in %s retrieval function",
             "png_get_x_pixels_per_meter");

         if (png_info_rust_get_phys_unit_type(info_ptr->rust_ptr) == PNG_RESOLUTION_METER)
            return (png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr));
      }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

png_uint_32 PNGAPI
png_get_y_pixels_per_meter(png_const_structrp png_ptr, png_const_inforp
    info_ptr)
{
#ifdef PNG_pHYs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0)
   {
      png_debug1(1, "in %s retrieval function",
          "png_get_y_pixels_per_meter");

      if (png_info_rust_get_phys_unit_type(info_ptr->rust_ptr) == PNG_RESOLUTION_METER)
         return (png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

png_uint_32 PNGAPI
png_get_pixels_per_meter(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
#ifdef PNG_pHYs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "png_get_pixels_per_meter");

      if (png_info_rust_get_phys_unit_type(info_ptr->rust_ptr) == PNG_RESOLUTION_METER &&
          png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr) == png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr))
         return (png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

#ifdef PNG_FLOATING_POINT_SUPPORTED
float PNGAPI
png_get_pixel_aspect_ratio(png_const_structrp png_ptr, png_const_inforp
   info_ptr)
{
#ifdef PNG_READ_pHYs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "png_get_aspect_ratio");

      if (png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr) != 0)
         return ((float)((float)png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr)
             /(float)png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr)));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return ((float)0.0);
}
#endif

#ifdef PNG_FIXED_POINT_SUPPORTED
png_fixed_point PNGAPI
png_get_pixel_aspect_ratio_fixed(png_const_structrp png_ptr,
    png_const_inforp info_ptr)
{
#ifdef PNG_READ_pHYs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0 &&
       png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr) > 0 && png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr) > 0 &&
       png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr) <= PNG_UINT_31_MAX &&
       png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr) <= PNG_UINT_31_MAX)
   {
      png_fixed_point res;

      png_debug1(1, "in %s retrieval function", "png_get_aspect_ratio_fixed");

      /* The following casts work because a PNG 4 byte integer only has a valid
       * range of 0..2^31-1; otherwise the cast might overflow.
       */
      if (png_muldiv(&res, (png_int_32)png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr), PNG_FP_1,
          (png_int_32)png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr)) != 0)
         return res;
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return 0;
}
#endif

png_int_32 PNGAPI
png_get_x_offset_microns(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
#ifdef PNG_oFFs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_oFFs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "png_get_x_offset_microns");

      if (png_info_rust_get_offset_unit_type(info_ptr->rust_ptr) == PNG_OFFSET_MICROMETER)
         return (png_info_rust_get_x_offset(info_ptr->rust_ptr));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

png_int_32 PNGAPI
png_get_y_offset_microns(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
#ifdef PNG_oFFs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_oFFs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "png_get_y_offset_microns");

      if (png_info_rust_get_offset_unit_type(info_ptr->rust_ptr) == PNG_OFFSET_MICROMETER)
         return (png_info_rust_get_y_offset(info_ptr->rust_ptr));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

png_int_32 PNGAPI
png_get_x_offset_pixels(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
#ifdef PNG_oFFs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_oFFs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "png_get_x_offset_pixels");

      if (png_info_rust_get_offset_unit_type(info_ptr->rust_ptr) == PNG_OFFSET_PIXEL)
         return (png_info_rust_get_x_offset(info_ptr->rust_ptr));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

png_int_32 PNGAPI
png_get_y_offset_pixels(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
#ifdef PNG_oFFs_SUPPORTED
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_oFFs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "png_get_y_offset_pixels");

      if (png_info_rust_get_offset_unit_type(info_ptr->rust_ptr) == PNG_OFFSET_PIXEL)
         return (png_info_rust_get_y_offset(info_ptr->rust_ptr));
   }
#else
   PNG_UNUSED(png_ptr)
   PNG_UNUSED(info_ptr)
#endif

   return (0);
}

#ifdef PNG_INCH_CONVERSIONS_SUPPORTED
static png_uint_32
ppi_from_ppm(png_uint_32 ppm)
{
#if 0
   /* The conversion is *(2.54/100), in binary (32 digits):
    * .00000110100000001001110101001001
    */
   png_uint_32 t1001, t1101;
   ppm >>= 1;                  /* .1 */
   t1001 = ppm + (ppm >> 3);   /* .1001 */
   t1101 = t1001 + (ppm >> 1); /* .1101 */
   ppm >>= 20;                 /* .000000000000000000001 */
   t1101 += t1101 >> 15;       /* .1101000000000001101 */
   t1001 >>= 11;               /* .000000000001001 */
   t1001 += t1001 >> 12;       /* .000000000001001000000001001 */
   ppm += t1001;               /* .000000000001001000001001001 */
   ppm += t1101;               /* .110100000001001110101001001 */
   return (ppm + 16) >> 5;/* .00000110100000001001110101001001 */
#else
   /* The argument is a PNG unsigned integer, so it is not permitted
    * to be bigger than 2^31.
    */
   png_fixed_point result;
   if (ppm <= PNG_UINT_31_MAX && png_muldiv(&result, (png_int_32)ppm, 127,
       5000) != 0)
      return (png_uint_32)result;

   /* Overflow. */
   return 0;
#endif
}

png_uint_32 PNGAPI
png_get_pixels_per_inch(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   return ppi_from_ppm(png_get_pixels_per_meter(png_ptr, info_ptr));
}

png_uint_32 PNGAPI
png_get_x_pixels_per_inch(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   return ppi_from_ppm(png_get_x_pixels_per_meter(png_ptr, info_ptr));
}

png_uint_32 PNGAPI
png_get_y_pixels_per_inch(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   return ppi_from_ppm(png_get_y_pixels_per_meter(png_ptr, info_ptr));
}

#ifdef PNG_FIXED_POINT_SUPPORTED
static png_fixed_point
png_fixed_inches_from_microns(png_const_structrp png_ptr, png_int_32 microns)
{
   /* Convert from meters * 1,000,000 to inches * 100,000, meters to
    * inches is simply *(100/2.54), so we want *(10/2.54) == 500/127.
    * Notice that this can overflow - a warning is output and 0 is
    * returned.
    */
   return png_muldiv_warn(png_ptr, microns, 500, 127);
}

png_fixed_point PNGAPI
png_get_x_offset_inches_fixed(png_const_structrp png_ptr,
    png_const_inforp info_ptr)
{
   return png_fixed_inches_from_microns(png_ptr,
       png_get_x_offset_microns(png_ptr, info_ptr));
}
#endif

#ifdef PNG_FIXED_POINT_SUPPORTED
png_fixed_point PNGAPI
png_get_y_offset_inches_fixed(png_const_structrp png_ptr,
    png_const_inforp info_ptr)
{
   return png_fixed_inches_from_microns(png_ptr,
       png_get_y_offset_microns(png_ptr, info_ptr));
}
#endif

#ifdef PNG_FLOATING_POINT_SUPPORTED
float PNGAPI
png_get_x_offset_inches(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   /* To avoid the overflow do the conversion directly in floating
    * point.
    */
   return (float)(png_get_x_offset_microns(png_ptr, info_ptr) * .00003937);
}
#endif

#ifdef PNG_FLOATING_POINT_SUPPORTED
float PNGAPI
png_get_y_offset_inches(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   /* To avoid the overflow do the conversion directly in floating
    * point.
    */
   return (float)(png_get_y_offset_microns(png_ptr, info_ptr) * .00003937);
}
#endif

#ifdef PNG_pHYs_SUPPORTED
png_uint_32 PNGAPI
png_get_pHYs_dpi(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_uint_32 *res_x, png_uint_32 *res_y, int *unit_type)
{
   png_uint_32 retval = 0;

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0)
   {
      png_debug1(1, "in %s retrieval function", "pHYs");

      if (res_x != NULL)
      {
         *res_x = png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr);
         retval |= PNG_INFO_pHYs;
      }

      if (res_y != NULL)
      {
         *res_y = png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr);
         retval |= PNG_INFO_pHYs;
      }

      if (unit_type != NULL)
      {
         *unit_type = (int)png_info_rust_get_phys_unit_type(info_ptr->rust_ptr);
         retval |= PNG_INFO_pHYs;

         if (*unit_type == 1)
         {
            if (res_x != NULL) *res_x = (png_uint_32)(*res_x * .0254 + .50);
            if (res_y != NULL) *res_y = (png_uint_32)(*res_y * .0254 + .50);
         }
      }
   }

   return (retval);
}
#endif /* pHYs */
#endif /* INCH_CONVERSIONS */

/* png_get_channels really belongs in here, too, but it's been around longer */

#endif /* EASY_ACCESS */


png_byte PNGAPI
png_get_channels(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return(png_info_rust_get_channels(info_ptr->rust_ptr));

   return (0);
}

#ifdef PNG_READ_SUPPORTED
png_const_bytep PNGAPI
png_get_signature(png_const_structrp png_ptr, png_const_inforp info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return(info_ptr->signature);

   return (NULL);
}
#endif

#ifdef PNG_bKGD_SUPPORTED
png_uint_32 PNGAPI
png_get_bKGD(png_const_structrp png_ptr, png_inforp info_ptr,
    png_color_16p *background)
{
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_bKGD) != 0 &&
       background != NULL)
   {
      png_debug1(1, "in %s retrieval function", "bKGD");

      *background = png_info_rust_ptr_background(info_ptr->rust_ptr);
      return (PNG_INFO_bKGD);
   }

   return (0);
}
#endif

#ifdef PNG_cHRM_SUPPORTED
/* The XYZ APIs were added in 1.5.5 to take advantage of the code added at the
 * same time to correct the rgb grayscale coefficient defaults obtained from the
 * cHRM chunk in 1.5.4
 */
#  ifdef PNG_FLOATING_POINT_SUPPORTED
png_uint_32 PNGAPI
png_get_cHRM(png_const_structrp png_ptr, png_const_inforp info_ptr,
    double *white_x, double *white_y, double *red_x, double *red_y,
    double *green_x, double *green_y, double *blue_x, double *blue_y)
{
   /* Quiet API change: this code used to only return the end points if a cHRM
    * chunk was present, but the end points can also come from iCCP or sRGB
    * chunks, so in 1.6.0 the png_get_ APIs return the end points regardless and
    * the png_set_ APIs merely check that set end points are mutually
    * consistent.
    */
   if (png_ptr != NULL && info_ptr != NULL &&
      (info_ptr->colorspace.flags & PNG_COLORSPACE_HAVE_ENDPOINTS) != 0)
   {
      png_debug1(1, "in %s retrieval function", "cHRM");

      if (white_x != NULL)
         *white_x = png_float(png_ptr,
             info_ptr->colorspace.end_points_xy.whitex, "cHRM white X");
      if (white_y != NULL)
         *white_y = png_float(png_ptr,
             info_ptr->colorspace.end_points_xy.whitey, "cHRM white Y");
      if (red_x != NULL)
         *red_x = png_float(png_ptr, info_ptr->colorspace.end_points_xy.redx,
             "cHRM red X");
      if (red_y != NULL)
         *red_y = png_float(png_ptr, info_ptr->colorspace.end_points_xy.redy,
             "cHRM red Y");
      if (green_x != NULL)
         *green_x = png_float(png_ptr,
             info_ptr->colorspace.end_points_xy.greenx, "cHRM green X");
      if (green_y != NULL)
         *green_y = png_float(png_ptr,
             info_ptr->colorspace.end_points_xy.greeny, "cHRM green Y");
      if (blue_x != NULL)
         *blue_x = png_float(png_ptr, info_ptr->colorspace.end_points_xy.bluex,
             "cHRM blue X");
      if (blue_y != NULL)
         *blue_y = png_float(png_ptr, info_ptr->colorspace.end_points_xy.bluey,
             "cHRM blue Y");
      return (PNG_INFO_cHRM);
   }

   return (0);
}

png_uint_32 PNGAPI
png_get_cHRM_XYZ(png_const_structrp png_ptr, png_const_inforp info_ptr,
    double *red_X, double *red_Y, double *red_Z, double *green_X,
    double *green_Y, double *green_Z, double *blue_X, double *blue_Y,
    double *blue_Z)
{
   if (png_ptr != NULL && info_ptr != NULL &&
       (info_ptr->colorspace.flags & PNG_COLORSPACE_HAVE_ENDPOINTS) != 0)
   {
      png_debug1(1, "in %s retrieval function", "cHRM_XYZ(float)");

      if (red_X != NULL)
         *red_X = png_float(png_ptr, info_ptr->colorspace.end_points_XYZ.red_X,
             "cHRM red X");
      if (red_Y != NULL)
         *red_Y = png_float(png_ptr, info_ptr->colorspace.end_points_XYZ.red_Y,
             "cHRM red Y");
      if (red_Z != NULL)
         *red_Z = png_float(png_ptr, info_ptr->colorspace.end_points_XYZ.red_Z,
             "cHRM red Z");
      if (green_X != NULL)
         *green_X = png_float(png_ptr,
             info_ptr->colorspace.end_points_XYZ.green_X, "cHRM green X");
      if (green_Y != NULL)
         *green_Y = png_float(png_ptr,
             info_ptr->colorspace.end_points_XYZ.green_Y, "cHRM green Y");
      if (green_Z != NULL)
         *green_Z = png_float(png_ptr,
             info_ptr->colorspace.end_points_XYZ.green_Z, "cHRM green Z");
      if (blue_X != NULL)
         *blue_X = png_float(png_ptr,
             info_ptr->colorspace.end_points_XYZ.blue_X, "cHRM blue X");
      if (blue_Y != NULL)
         *blue_Y = png_float(png_ptr,
             info_ptr->colorspace.end_points_XYZ.blue_Y, "cHRM blue Y");
      if (blue_Z != NULL)
         *blue_Z = png_float(png_ptr,
             info_ptr->colorspace.end_points_XYZ.blue_Z, "cHRM blue Z");
      return (PNG_INFO_cHRM);
   }

   return (0);
}
#  endif

#  ifdef PNG_FIXED_POINT_SUPPORTED
png_uint_32 PNGAPI
png_get_cHRM_XYZ_fixed(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_fixed_point *int_red_X, png_fixed_point *int_red_Y,
    png_fixed_point *int_red_Z, png_fixed_point *int_green_X,
    png_fixed_point *int_green_Y, png_fixed_point *int_green_Z,
    png_fixed_point *int_blue_X, png_fixed_point *int_blue_Y,
    png_fixed_point *int_blue_Z)
{
   if (png_ptr != NULL && info_ptr != NULL &&
      (info_ptr->colorspace.flags & PNG_COLORSPACE_HAVE_ENDPOINTS) != 0)
   {
      png_debug1(1, "in %s retrieval function", "cHRM_XYZ");

      if (int_red_X != NULL)
         *int_red_X = info_ptr->colorspace.end_points_XYZ.red_X;
      if (int_red_Y != NULL)
         *int_red_Y = info_ptr->colorspace.end_points_XYZ.red_Y;
      if (int_red_Z != NULL)
         *int_red_Z = info_ptr->colorspace.end_points_XYZ.red_Z;
      if (int_green_X != NULL)
         *int_green_X = info_ptr->colorspace.end_points_XYZ.green_X;
      if (int_green_Y != NULL)
         *int_green_Y = info_ptr->colorspace.end_points_XYZ.green_Y;
      if (int_green_Z != NULL)
         *int_green_Z = info_ptr->colorspace.end_points_XYZ.green_Z;
      if (int_blue_X != NULL)
         *int_blue_X = info_ptr->colorspace.end_points_XYZ.blue_X;
      if (int_blue_Y != NULL)
         *int_blue_Y = info_ptr->colorspace.end_points_XYZ.blue_Y;
      if (int_blue_Z != NULL)
         *int_blue_Z = info_ptr->colorspace.end_points_XYZ.blue_Z;
      return (PNG_INFO_cHRM);
   }

   return (0);
}

png_uint_32 PNGAPI
png_get_cHRM_fixed(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_fixed_point *white_x, png_fixed_point *white_y, png_fixed_point *red_x,
    png_fixed_point *red_y, png_fixed_point *green_x, png_fixed_point *green_y,
    png_fixed_point *blue_x, png_fixed_point *blue_y)
{
   png_debug1(1, "in %s retrieval function", "cHRM");

   if (png_ptr != NULL && info_ptr != NULL &&
      (info_ptr->colorspace.flags & PNG_COLORSPACE_HAVE_ENDPOINTS) != 0)
   {
      if (white_x != NULL)
         *white_x = info_ptr->colorspace.end_points_xy.whitex;
      if (white_y != NULL)
         *white_y = info_ptr->colorspace.end_points_xy.whitey;
      if (red_x != NULL)
         *red_x = info_ptr->colorspace.end_points_xy.redx;
      if (red_y != NULL)
         *red_y = info_ptr->colorspace.end_points_xy.redy;
      if (green_x != NULL)
         *green_x = info_ptr->colorspace.end_points_xy.greenx;
      if (green_y != NULL)
         *green_y = info_ptr->colorspace.end_points_xy.greeny;
      if (blue_x != NULL)
         *blue_x = info_ptr->colorspace.end_points_xy.bluex;
      if (blue_y != NULL)
         *blue_y = info_ptr->colorspace.end_points_xy.bluey;
      return (PNG_INFO_cHRM);
   }

   return (0);
}
#  endif
#endif

#ifdef PNG_gAMA_SUPPORTED
#  ifdef PNG_FIXED_POINT_SUPPORTED
png_uint_32 PNGAPI
png_get_gAMA_fixed(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_fixed_point *file_gamma)
{
   png_debug1(1, "in %s retrieval function", "gAMA");

   if (png_ptr != NULL && info_ptr != NULL &&
       (info_ptr->colorspace.flags & PNG_COLORSPACE_HAVE_GAMMA) != 0 &&
       file_gamma != NULL)
   {
      *file_gamma = info_ptr->colorspace.gamma;
      return (PNG_INFO_gAMA);
   }

   return (0);
}
#  endif

#  ifdef PNG_FLOATING_POINT_SUPPORTED
png_uint_32 PNGAPI
png_get_gAMA(png_const_structrp png_ptr, png_const_inforp info_ptr,
    double *file_gamma)
{
   png_debug1(1, "in %s retrieval function", "gAMA(float)");

   if (png_ptr != NULL && info_ptr != NULL &&
      (info_ptr->colorspace.flags & PNG_COLORSPACE_HAVE_GAMMA) != 0 &&
      file_gamma != NULL)
   {
      *file_gamma = png_float(png_ptr, info_ptr->colorspace.gamma,
          "png_get_gAMA");
      return (PNG_INFO_gAMA);
   }

   return (0);
}
#  endif
#endif

#ifdef PNG_sRGB_SUPPORTED
png_uint_32 PNGAPI
png_get_sRGB(png_const_structrp png_ptr, png_const_inforp info_ptr,
    int *file_srgb_intent)
{
   png_debug1(1, "in %s retrieval function", "sRGB");

   if (png_ptr != NULL && info_ptr != NULL &&
      (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_sRGB) != 0 && file_srgb_intent != NULL)
   {
      *file_srgb_intent = info_ptr->colorspace.rendering_intent;
      return (PNG_INFO_sRGB);
   }

   return (0);
}
#endif

#ifdef PNG_iCCP_SUPPORTED
png_uint_32 PNGAPI
png_get_iCCP(png_const_structrp png_ptr, png_inforp info_ptr,
    png_charpp name, int *compression_type,
    png_bytepp profile, png_uint_32 *proflen)
{
   png_debug1(1, "in %s retrieval function", "iCCP");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_iCCP) != 0 &&
       name != NULL && profile != NULL && proflen != NULL)
   {
      *name = png_info_rust_get_iccp_name(info_ptr->rust_ptr);
      *profile = png_info_rust_get_iccp_profile(info_ptr->rust_ptr);
      *proflen = png_get_uint_32(png_info_rust_get_iccp_profile(info_ptr->rust_ptr));
      /* This is somewhat irrelevant since the profile data returned has
       * actually been uncompressed.
       */
      if (compression_type != NULL)
         *compression_type = PNG_COMPRESSION_TYPE_BASE;
      return (PNG_INFO_iCCP);
   }

   return (0);

}
#endif

#ifdef PNG_sPLT_SUPPORTED
int PNGAPI
png_get_sPLT(png_const_structrp png_ptr, png_inforp info_ptr,
    png_sPLT_tpp spalettes)
{
   if (png_ptr != NULL && info_ptr != NULL && spalettes != NULL)
   {
      *spalettes = info_ptr->splt_palettes;
      return info_ptr->splt_palettes_num;
   }

   return (0);
}
#endif

#ifdef PNG_eXIf_SUPPORTED
png_uint_32 PNGAPI
png_get_eXIf(png_const_structrp png_ptr, png_inforp info_ptr,
    png_bytep *exif)
{
  png_warning(png_ptr, "png_get_eXIf does not work; use png_get_eXIf_1");
  PNG_UNUSED(info_ptr)
  PNG_UNUSED(exif)
  return 0;
}

png_uint_32 PNGAPI
png_get_eXIf_1(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_uint_32 *num_exif, png_bytep *exif)
{
   png_debug1(1, "in %s retrieval function", "eXIf");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_eXIf) != 0 && exif != NULL)
   {
      *num_exif = png_info_rust_get_num_exif(info_ptr->rust_ptr);
      *exif = png_info_rust_get_exif(info_ptr->rust_ptr);
      return (PNG_INFO_eXIf);
   }

   return (0);
}
#endif

#ifdef PNG_hIST_SUPPORTED
png_uint_32 PNGAPI
png_get_hIST(png_const_structrp png_ptr, png_inforp info_ptr,
    png_uint_16p *hist)
{
   png_debug1(1, "in %s retrieval function", "hIST");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_hIST) != 0 && hist != NULL)
   {
      *hist = info_ptr->hist;
      return (PNG_INFO_hIST);
   }

   return (0);
}
#endif

png_uint_32 PNGAPI
png_get_IHDR(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_uint_32 *width, png_uint_32 *height, int *bit_depth,
    int *color_type, int *interlace_type, int *compression_type,
    int *filter_type)
{
   png_debug1(1, "in %s retrieval function", "IHDR");

   if (png_ptr == NULL || info_ptr == NULL)
      return (0);

   return png_rust_get_IHDR(png_ptr->rust_ptr, info_ptr->rust_ptr,
                            width, height, bit_depth, color_type,
                            interlace_type, compression_type, filter_type);
}

#ifdef PNG_oFFs_SUPPORTED
png_uint_32 PNGAPI
png_get_oFFs(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_int_32 *offset_x, png_int_32 *offset_y, int *unit_type)
{
   png_debug1(1, "in %s retrieval function", "oFFs");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_oFFs) != 0 &&
       offset_x != NULL && offset_y != NULL && unit_type != NULL)
   {
      *offset_x = png_info_rust_get_x_offset(info_ptr->rust_ptr);
      *offset_y = png_info_rust_get_y_offset(info_ptr->rust_ptr);
      *unit_type = (int)png_info_rust_get_offset_unit_type(info_ptr->rust_ptr);
      return (PNG_INFO_oFFs);
   }

   return (0);
}
#endif

#ifdef PNG_pCAL_SUPPORTED
png_uint_32 PNGAPI
png_get_pCAL(png_const_structrp png_ptr, png_inforp info_ptr,
    png_charp *purpose, png_int_32 *X0, png_int_32 *X1, int *type, int *nparams,
    png_charp *units, png_charpp *params)
{
   png_debug1(1, "in %s retrieval function", "pCAL");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pCAL) != 0 &&
       purpose != NULL && X0 != NULL && X1 != NULL && type != NULL &&
       nparams != NULL && units != NULL && params != NULL)
   {
      *purpose = info_ptr->pcal_purpose;
      *X0 = info_ptr->pcal_X0;
      *X1 = info_ptr->pcal_X1;
      *type = (int)info_ptr->pcal_type;
      *nparams = (int)info_ptr->pcal_nparams;
      *units = info_ptr->pcal_units;
      *params = info_ptr->pcal_params;
      return (PNG_INFO_pCAL);
   }

   return (0);
}
#endif

#ifdef PNG_sCAL_SUPPORTED
#  ifdef PNG_FIXED_POINT_SUPPORTED
#    if defined(PNG_FLOATING_ARITHMETIC_SUPPORTED) || \
         defined(PNG_FLOATING_POINT_SUPPORTED)
png_uint_32 PNGAPI
png_get_sCAL_fixed(png_const_structrp png_ptr, png_const_inforp info_ptr,
    int *unit, png_fixed_point *width, png_fixed_point *height)
{
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_sCAL) != 0)
   {
      *unit = png_info_rust_get_scal_unit(info_ptr->rust_ptr);
      /*TODO: make this work without FP support; the API is currently eliminated
       * if neither floating point APIs nor internal floating point arithmetic
       * are enabled.
       */
      *width = png_fixed(png_ptr, atof(png_info_rust_get_scal_s_width(info_ptr->rust_ptr)), "sCAL width");
      *height = png_fixed(png_ptr, atof(png_info_rust_get_scal_s_height(info_ptr->rust_ptr)),
          "sCAL height");
      return (PNG_INFO_sCAL);
   }

   return(0);
}
#    endif /* FLOATING_ARITHMETIC */
#  endif /* FIXED_POINT */
#  ifdef PNG_FLOATING_POINT_SUPPORTED
png_uint_32 PNGAPI
png_get_sCAL(png_const_structrp png_ptr, png_const_inforp info_ptr,
    int *unit, double *width, double *height)
{
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_sCAL) != 0)
   {
      *unit = png_info_rust_get_scal_unit(info_ptr->rust_ptr);
      *width = atof(png_info_rust_get_scal_s_width(info_ptr->rust_ptr));
      *height = atof(png_info_rust_get_scal_s_height(info_ptr->rust_ptr));
      return (PNG_INFO_sCAL);
   }

   return(0);
}
#  endif /* FLOATING POINT */
png_uint_32 PNGAPI
png_get_sCAL_s(png_const_structrp png_ptr, png_const_inforp info_ptr,
    int *unit, png_charpp width, png_charpp height)
{
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_sCAL) != 0)
   {
      *unit = png_info_rust_get_scal_unit(info_ptr->rust_ptr);
      *width = png_info_rust_get_scal_s_width(info_ptr->rust_ptr);
      *height = png_info_rust_get_scal_s_height(info_ptr->rust_ptr);
      return (PNG_INFO_sCAL);
   }

   return(0);
}
#endif /* sCAL */

#ifdef PNG_pHYs_SUPPORTED
png_uint_32 PNGAPI
png_get_pHYs(png_const_structrp png_ptr, png_const_inforp info_ptr,
    png_uint_32 *res_x, png_uint_32 *res_y, int *unit_type)
{
   png_uint_32 retval = 0;

   png_debug1(1, "in %s retrieval function", "pHYs");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_pHYs) != 0)
   {
      if (res_x != NULL)
      {
         *res_x = png_info_rust_get_x_pixels_per_unit(info_ptr->rust_ptr);
         retval |= PNG_INFO_pHYs;
      }

      if (res_y != NULL)
      {
         *res_y = png_info_rust_get_y_pixels_per_unit(info_ptr->rust_ptr);
         retval |= PNG_INFO_pHYs;
      }

      if (unit_type != NULL)
      {
         *unit_type = (int)png_info_rust_get_phys_unit_type(info_ptr->rust_ptr);
         retval |= PNG_INFO_pHYs;
      }
   }

   return (retval);
}
#endif /* pHYs */

png_uint_32 PNGAPI
png_get_PLTE(png_const_structrp png_ptr, png_inforp info_ptr,
    png_colorp *palette, int *num_palette)
{
   png_debug1(1, "in %s retrieval function", "PLTE");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_PLTE) != 0 && palette != NULL)
   {
      *palette = png_info_rust_get_palette(info_ptr->rust_ptr);
      *num_palette = png_info_rust_get_num_palette(info_ptr->rust_ptr);
      png_debug1(3, "num_palette = %d", *num_palette);
      return (PNG_INFO_PLTE);
   }

   return (0);
}

#ifdef PNG_sBIT_SUPPORTED
png_uint_32 PNGAPI
png_get_sBIT(png_const_structrp png_ptr, png_inforp info_ptr,
    png_color_8p *sig_bit)
{
   png_debug1(1, "in %s retrieval function", "sBIT");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_sBIT) != 0 && sig_bit != NULL)
   {
      *sig_bit = png_info_rust_ptr_sig_bit(info_ptr->rust_ptr);
      return (PNG_INFO_sBIT);
   }

   return (0);
}
#endif

#ifdef PNG_TEXT_SUPPORTED
int PNGAPI
png_get_text(png_const_structrp png_ptr, png_inforp info_ptr,
    png_textp *text_ptr, int *num_text)
{
   if (png_ptr != NULL && info_ptr != NULL && png_info_rust_get_num_text(info_ptr->rust_ptr) > 0)
   {
      png_debug1(1, "in 0x%lx retrieval function",
         (unsigned long)png_rust_get_chunk_name(png_ptr->rust_ptr));

      if (text_ptr != NULL)
         *text_ptr = png_info_rust_get_text(info_ptr->rust_ptr);

      if (num_text != NULL)
         *num_text = png_info_rust_get_num_text(info_ptr->rust_ptr);

      return png_info_rust_get_num_text(info_ptr->rust_ptr);
   }

   if (num_text != NULL)
      *num_text = 0;

   return(0);
}
#endif

#ifdef PNG_tIME_SUPPORTED
png_uint_32 PNGAPI
png_get_tIME(png_const_structrp png_ptr, png_inforp info_ptr,
    png_timep *mod_time)
{
   png_debug1(1, "in %s retrieval function", "tIME");

   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_tIME) != 0 && mod_time != NULL)
   {
      *mod_time = &info_ptr->mod_time;
      return (PNG_INFO_tIME);
   }

   return (0);
}
#endif

#ifdef PNG_tRNS_SUPPORTED
png_uint_32 PNGAPI
png_get_tRNS(png_const_structrp png_ptr, png_inforp info_ptr,
    png_bytep *trans_alpha, int *num_trans, png_color_16p *trans_color)
{
   png_uint_32 retval = 0;
   if (png_ptr != NULL && info_ptr != NULL &&
       (png_info_rust_get_valid(info_ptr->rust_ptr) & PNG_INFO_tRNS) != 0)
   {
      png_debug1(1, "in %s retrieval function", "tRNS");

      if (png_info_rust_get_color_type(info_ptr->rust_ptr) == PNG_COLOR_TYPE_PALETTE)
      {
         if (trans_alpha != NULL)
         {
            *trans_alpha = png_info_rust_get_trans_alpha(info_ptr->rust_ptr);
            retval |= PNG_INFO_tRNS;
         }

         if (trans_color != NULL)
            *trans_color = png_info_rust_ptr_trans_color(info_ptr->rust_ptr);
      }

      else /* if (info_ptr->color_type != PNG_COLOR_TYPE_PALETTE) */
      {
         if (trans_color != NULL)
         {
            *trans_color = png_info_rust_ptr_trans_color(info_ptr->rust_ptr);
            retval |= PNG_INFO_tRNS;
         }

         if (trans_alpha != NULL)
            *trans_alpha = NULL;
      }

      if (num_trans != NULL)
      {
         *num_trans = png_info_rust_get_num_trans(info_ptr->rust_ptr);
         retval |= PNG_INFO_tRNS;
      }
   }

   return (retval);
}
#endif

#ifdef PNG_STORE_UNKNOWN_CHUNKS_SUPPORTED
int PNGAPI
png_get_unknown_chunks(png_const_structrp png_ptr, png_inforp info_ptr,
    png_unknown_chunkpp unknowns)
{
   if (png_ptr != NULL && info_ptr != NULL && unknowns != NULL)
   {
      *unknowns = info_ptr->unknown_chunks;
      return info_ptr->unknown_chunks_num;
   }

   return (0);
}
#endif

#ifdef PNG_READ_RGB_TO_GRAY_SUPPORTED
png_byte PNGAPI
png_get_rgb_to_gray_status (png_const_structrp png_ptr)
{
   return (png_byte)(png_ptr ? png_ptr->rgb_to_gray_status : 0);
}
#endif

#ifdef PNG_USER_CHUNKS_SUPPORTED
png_voidp PNGAPI
png_get_user_chunk_ptr(png_const_structrp png_ptr)
{
   return (png_ptr ? png_ptr->user_chunk_ptr : NULL);
}
#endif

size_t PNGAPI
png_get_compression_buffer_size(png_const_structrp png_ptr)
{
   if (png_ptr == NULL)
      return 0;

#ifdef PNG_WRITE_SUPPORTED
   if ( png_rust_has_mode(png_ptr->rust_ptr, PNG_IS_READ_STRUCT) )
#endif
   {
#ifdef PNG_SEQUENTIAL_READ_SUPPORTED
      return png_ptr->IDAT_read_size;
#else
      return PNG_IDAT_READ_SIZE;
#endif
   }

#ifdef PNG_WRITE_SUPPORTED
   else
      return png_ptr->zbuffer_size;
#endif
}

#ifdef PNG_SET_USER_LIMITS_SUPPORTED
/* These functions were added to libpng 1.2.6 and were enabled
 * by default in libpng-1.4.0 */
png_uint_32 PNGAPI
png_get_user_width_max (png_const_structrp png_ptr)
{
   return (png_ptr ? png_rust_get_user_width_max(png_ptr->rust_ptr) : 0);
}

png_uint_32 PNGAPI
png_get_user_height_max (png_const_structrp png_ptr)
{
   return (png_ptr ? png_rust_get_user_height_max(png_ptr->rust_ptr) : 0);
}

/* This function was added to libpng 1.4.0 */
png_uint_32 PNGAPI
png_get_chunk_cache_max (png_const_structrp png_ptr)
{
   return (png_ptr ? png_rust_get_user_chunk_cache_max(png_ptr->rust_ptr) : 0);
}

/* This function was added to libpng 1.4.1 */
png_alloc_size_t PNGAPI
png_get_chunk_malloc_max (png_const_structrp png_ptr)
{
   return (png_ptr ? png_rust_get_user_chunk_malloc_max(png_ptr->rust_ptr) : 0);
}
#endif /* SET_USER_LIMITS */

/* These functions were added to libpng 1.4.0 */
#ifdef PNG_IO_STATE_SUPPORTED
png_uint_32 PNGAPI
png_get_io_state (png_const_structrp png_ptr)
{
   return png_ptr->io_state;
}

png_uint_32 PNGAPI
png_get_io_chunk_type (png_const_structrp png_ptr)
{
   return png_rust_get_chunk_name(png_ptr->rust_ptr);
}
#endif /* IO_STATE */

#ifdef PNG_CHECK_FOR_INVALID_INDEX_SUPPORTED
#  ifdef PNG_GET_PALETTE_MAX_SUPPORTED
int PNGAPI
png_get_palette_max(png_const_structp png_ptr, png_const_infop info_ptr)
{
   if (png_ptr != NULL && info_ptr != NULL)
      return png_rust_get_num_palette_max(png_ptr->rust_ptr);

   return (-1);
}
#  endif
#endif

#endif /* READ || WRITE */
