/* pnginfo.h - header file for PNG reference library
 *
 * Copyright (c) 2018 Cosmin Truta
 * Copyright (c) 1998-2002,2004,2006-2013,2018 Glenn Randers-Pehrson
 * Copyright (c) 1996-1997 Andreas Dilger
 * Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */

 /* png_info is a structure that holds the information in a PNG file so
 * that the application can find out the characteristics of the image.
 * If you are reading the file, this structure will tell you what is
 * in the PNG file.  If you are writing the file, fill in the information
 * you want to put into the PNG file, using png_set_*() functions, then
 * call png_write_info().
 *
 * The names chosen should be very close to the PNG specification, so
 * consult that document for information about the meaning of each field.
 *
 * With libpng < 0.95, it was only possible to directly set and read the
 * the values in the png_info_struct, which meant that the contents and
 * order of the values had to remain fixed.  With libpng 0.95 and later,
 * however, there are now functions that abstract the contents of
 * png_info_struct from the application, so this makes it easier to use
 * libpng with dynamic libraries, and even makes it possible to use
 * libraries that don't have all of the libpng ancillary chunk-handing
 * functionality.  In libpng-1.5.0 this was moved into a separate private
 * file that is not visible to applications.
 *
 * The following members may have allocated storage attached that should be
 * cleaned up before the structure is discarded: palette, trans, text,
 * pcal_purpose, pcal_units, pcal_params, hist, iccp_name, iccp_profile,
 * splt_palettes, scal_unit, row_pointers, and unknowns.   By default, these
 * are automatically freed when the info structure is deallocated, if they were
 * allocated internally by libpng.  This behavior can be changed by means
 * of the png_data_freer() function.
 *
 * More allocation details: all the chunk-reading functions that
 * change these members go through the corresponding png_set_*
 * functions.  A function to clear these members is available: see
 * png_free_data().  The png_set_* functions do not depend on being
 * able to point info structure members to any of the storage they are
 * passed (they make their own copies), EXCEPT that the png_set_text
 * functions use the same storage passed to them in the text_ptr or
 * itxt_ptr structure argument, and the png_set_rows and png_set_unknowns
 * functions do not make their own copies.
 */
#ifndef PNGINFO_H
#define PNGINFO_H

#include "pngrust.h"

struct png_info_def
{
   /* This is never set during write */
   png_byte signature[8];   /* magic bytes read by libpng from start of file */

#if defined(PNG_COLORSPACE_SUPPORTED) || defined(PNG_GAMMA_SUPPORTED)
   /* png_colorspace only contains 'flags' if neither GAMMA or COLORSPACE are
    * defined.  When COLORSPACE is switched on all the colorspace-defining
    * chunks should be enabled, when GAMMA is switched on all the gamma-defining
    * chunks should be enabled.  If this is not done it becomes possible to read
    * inconsistent PNG files and assign a probably incorrect interpretation to
    * the information.  (In other words, by carefully choosing which chunks to
    * recognize the system configuration can select an interpretation for PNG
    * files containing ambiguous data and this will result in inconsistent
    * behavior between different libpng builds!)
    */
   png_colorspace colorspace;
#endif

#ifdef PNG_tIME_SUPPORTED
   /* The tIME chunk holds the last time the displayed image data was
    * modified.  See the png_time struct for the contents of this struct.
    */
   png_time mod_time;
#endif


#ifdef PNG_hIST_SUPPORTED
   /* The hIST chunk contains the relative frequency or importance of the
    * various palette entries, so that a viewer can intelligently select a
    * reduced-color palette, if required.  Data is an array of "num_palette"
    * values in the range [0,65535]. Data valid if (valid & PNG_INFO_hIST)
    * is non-zero.
    */
   png_uint_16p hist;
#endif

#ifdef PNG_pCAL_SUPPORTED
   /* The pCAL chunk describes a transformation between the stored pixel
    * values and original physical data values used to create the image.
    * The integer range [0, 2^bit_depth - 1] maps to the floating-point
    * range given by [pcal_X0, pcal_X1], and are further transformed by a
    * (possibly non-linear) transformation function given by "pcal_type"
    * and "pcal_params" into "pcal_units".  Please see the PNG_EQUATION_
    * defines below, and the PNG-Group's PNG extensions document for a
    * complete description of the transformations and how they should be
    * implemented, and for a description of the ASCII parameter strings.
    * Data values are valid if (valid & PNG_INFO_pCAL) non-zero.
    */
   png_charp pcal_purpose;  /* pCAL chunk description string */
   png_int_32 pcal_X0;      /* minimum value */
   png_int_32 pcal_X1;      /* maximum value */
   png_charp pcal_units;    /* Latin-1 string giving physical units */
   png_charpp pcal_params;  /* ASCII strings containing parameter values */
   png_byte pcal_type;      /* equation type (see PNG_EQUATION_ below) */
   png_byte pcal_nparams;   /* number of parameters given in pcal_params */
#endif

/* New members added in libpng-1.0.6 */
   png_uint_32 free_me;     /* flags items libpng is responsible for freeing */

#ifdef PNG_STORE_UNKNOWN_CHUNKS_SUPPORTED
   /* Storage for unknown chunks that the library doesn't recognize. */
   png_unknown_chunkp unknown_chunks;

   /* The type of this field is limited by the type of
    * png_struct::user_chunk_cache_max, else overflow can occur.
    */
   int                unknown_chunks_num;
#endif

#ifdef PNG_sPLT_SUPPORTED
   /* Data on sPLT chunks (there may be more than one). */
   png_sPLT_tp splt_palettes;
   int         splt_palettes_num; /* Match type returned by png_get API */
#endif

#ifdef PNG_INFO_IMAGE_SUPPORTED
   /* Memory has been allocated if (valid & PNG_ALLOCATED_INFO_ROWS)
      non-zero */
   /* Data valid if (valid & PNG_INFO_IDAT) non-zero */
   png_bytepp row_pointers;        /* the image bits */
#endif

   PngInfoRust* rust_ptr;
};
#endif /* PNGINFO_H */
