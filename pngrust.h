/* pngrust.h - Binding to pngrust
 *
 * Copyright (c) 2019 Corentin Rossignon
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */
#ifndef PNGRUST_H
#define PNGRUST_H

#include <stdint.h>
#include <stdbool.h>

typedef struct PngInfoRust PngInfoRust;
typedef struct PngRust PngRust;

PngRust* png_rust_new();
PngRust* png_rust_set_png_ptr(PngRust* pngrust, void *png_ptr);

bool png_rust_pass_is_valid(PngRust* pngrust);
uint8_t png_rust_get_pass(PngRust* pngrust);
void png_rust_incr_pass(PngRust* pngrust);
void png_rust_decr_pass(PngRust* pngrust);

int32_t png_rust_get_interlace(PngRust* pngrust);
void    png_rust_set_interlace(PngRust* pngrust, int32_t value);

uint32_t png_rust_get_flags(PngRust* pngrust);
void png_rust_set_flags(PngRust* pngrust, uint32_t flags);
bool png_rust_has_flags(PngRust* pngrust, uint32_t flags);
bool png_rust_one_of_flags(PngRust* pngrust, uint32_t flags);
void png_rust_add_flags(PngRust* pngrust, uint32_t flags);
void png_rust_remove_flags(PngRust* pngrust, uint32_t flags);
void png_rust_reset_flags(PngRust* pngrust);

uint32_t png_rust_get_mode(PngRust* pngrust);
void png_rust_set_mode(PngRust* pngrust, uint32_t flags);
bool png_rust_has_mode(PngRust* pngrust, uint32_t flags);
void png_rust_add_mode(PngRust* pngrust, uint32_t flags);
void png_rust_remove_mode(PngRust* pngrust, uint32_t flags);
void png_rust_reset_mode(PngRust* pngrust);

uint32_t png_rust_get_transformations(PngRust* pngrust);
bool png_rust_has_transformations(PngRust* pngrust, uint32_t flags);
bool png_rust_one_of_transformations(PngRust* pngrust, uint32_t flags);
void png_rust_add_transformations(PngRust* pngrust, uint32_t flags);
void png_rust_remove_transformations(PngRust* pngrust, uint32_t flags);
bool png_rust_empty_transformations(PngRust* pngrust);

uint8_t png_rust_get_color_type(PngRust* pngrust);
void png_rust_set_color_type(PngRust* pngrust, uint8_t flags);
bool png_rust_has_color_type(PngRust* pngrust, uint8_t flags);
bool png_rust_is_color_type(PngRust* pngrust, uint8_t flags);

uint8_t png_rust_get_filter_type(PngRust* rust_ptr);
void png_rust_set_filter_type(PngRust* rust_ptr, uint8_t value);

uint8_t png_rust_get_channels(PngRust* pngrust);
void png_rust_set_channels(PngRust* pngrust, uint8_t channels);

uint16_t png_rust_get_num_trans(PngRust* pngrust);
void png_rust_set_num_trans(PngRust* pngrust, uint16_t num_trans);

uint8_t png_rust_get_pixel_depth(PngRust* pngrust);
void png_rust_set_pixel_depth(PngRust* pngrust, uint8_t pixel_depth);

uint8_t png_rust_get_bit_depth(PngRust* pngrust);
void png_rust_set_bit_depth(PngRust* pngrust, uint8_t bit_depth);

uint8_t png_rust_get_usr_bit_depth(PngRust* pngrust);
void png_rust_set_usr_bit_depth(PngRust* pngrust, uint8_t bit_depth);

uint32_t png_rust_get_do_filter(PngRust* pngrust);
void png_rust_set_do_filter(PngRust* pngrust, uint32_t flags);
bool png_rust_is_do_filter(PngRust* pngrust, uint32_t flags);



uint32_t png_rust_get_width(PngRust* pngrust);
uint32_t png_rust_get_height(PngRust* pngrust);
uint32_t png_rust_get_num_rows(PngRust* pngrust);
uint32_t png_rust_get_usr_width(PngRust* pngrust);
size_t   png_rust_get_rowbytes(PngRust* pngrust);
uint32_t png_rust_get_iwidth(PngRust* pngrust);
uint32_t png_rust_get_row_number(PngRust* pngrust);
uint32_t png_rust_get_chunk_name(PngRust* pngrust);
uint8_t *png_rust_get_prev_row(PngRust* pngrust);
uint8_t *png_rust_get_row_buf(PngRust* pngrust);
uint8_t *png_rust_get_try_row(PngRust* pngrust);
uint8_t *png_rust_get_tst_row(PngRust* pngrust);
size_t   png_rust_get_info_rowbytes(PngRust* pngrust);
uint32_t png_rust_get_idat_size(PngRust* pngrust);
uint32_t png_rust_get_crc(PngRust* pngrust);
png_color *png_rust_get_palette(PngRust* pngrust);
uint16_t png_rust_get_num_palette(PngRust* pngrust);
int32_t  png_rust_get_num_palette_max(PngRust* pngrust);
uint8_t  png_rust_get_usr_channels(PngRust* pngrust);
uint8_t  png_rust_get_sig_bytes(PngRust* pngrust);
uint8_t  png_rust_get_maximum_pixel_depth(PngRust* pngrust);
uint8_t  png_rust_get_transformed_pixel_depth(PngRust* pngrust);


void png_rust_set_width(PngRust* pngrust, uint32_t value);
void png_rust_set_height(PngRust* pngrust, uint32_t value);
void png_rust_set_num_rows(PngRust* pngrust, uint32_t value);
void png_rust_set_usr_width(PngRust* pngrust, uint32_t value);
void png_rust_set_rowbytes(PngRust* pngrust, size_t value);
void png_rust_set_iwidth(PngRust* pngrust, uint32_t value);
void png_rust_set_row_number(PngRust* pngrust, uint32_t value);
void png_rust_set_chunk_name(PngRust* pngrust, uint32_t value);
void png_rust_set_prev_row(PngRust* pngrust, uint8_t *value);
void png_rust_set_row_buf(PngRust* pngrust, uint8_t *value);
void png_rust_set_try_row(PngRust* pngrust, uint8_t *value);
void png_rust_set_tst_row(PngRust* pngrust, uint8_t *value);
void png_rust_set_info_rowbytes(PngRust* pngrust, size_t value);
void png_rust_set_idat_size(PngRust* pngrust, uint32_t value);
void png_rust_set_crc(PngRust* pngrust, uint32_t value);
void png_rust_set_palette(PngRust* pngrust, png_color *value);
void png_rust_set_num_palette(PngRust* pngrust, uint16_t value);
void png_rust_set_num_palette_max(PngRust* pngrust, int32_t value);
void png_rust_set_usr_channels(PngRust* pngrust, uint8_t value);
void png_rust_set_sig_bytes(PngRust* pngrust, uint8_t value);
void png_rust_set_maximum_pixel_depth(PngRust* pngrust, uint8_t value);
void png_rust_set_transformed_pixel_depth(PngRust* pngrust, uint8_t value);

void png_rust_sub_idat_size(PngRust* pngrust, uint32_t value);
void png_rust_incr_row_number(PngRust* pngrust);

void png_rust_handle_IEND(PngRust* pngrust, uint32_t length);
void png_rust_handle_IHDR(PngRust* pngrust, PngInfoRust* rust_info_ptr, uint32_t length);

png_progressive_info_ptr  png_rust_get_info_fn(PngRust* pngrust);
png_progressive_row_ptr   png_rust_get_row_fn(PngRust* pngrust);
png_progressive_end_ptr   png_rust_get_end_fn(PngRust* pngrust);
uint32_t   png_rust_get_push_length(PngRust* pngrust);
uint32_t   png_rust_get_skip_length(PngRust* pngrust);
size_t     png_rust_get_buffer_size(PngRust* pngrust);
size_t     png_rust_get_current_buffer_size(PngRust* pngrust);
int        png_rust_get_process_mode(PngRust* pngrust);
int        png_rust_get_cur_palette(PngRust* pngrust);
uint32_t   png_rust_get_zowner(PngRust* pngrust);
void      *png_rust_get_io_ptr(PngRust* pngrust);

png_bytep png_rust_get_read_buffer(PngRust* pngrust);
void png_rust_set_read_buffer(PngRust* pngrust, png_bytep ptr);

size_t png_rust_get_read_buffer_size(PngRust* pngrust);
void png_rust_set_read_buffer_size(PngRust* pngrust, size_t length);

uint32_t png_rust_get_user_chunk_cache_max(PngRust* pngrust);
void png_rust_set_user_chunk_cache_max(PngRust* pngrust, uint32_t value);

size_t png_rust_get_user_chunk_malloc_max(PngRust* pngrust);
void png_rust_set_user_chunk_malloc_max(PngRust* pngrust, size_t value);

uint32_t png_rust_get_user_width_max(PngRust* pngrust);
void png_rust_set_user_width_max(PngRust* pngrust, uint32_t value);

uint32_t png_rust_get_user_height_max(PngRust* pngrust);
void png_rust_set_user_height_max(PngRust* pngrust, uint32_t value);

uint8_t png_rust_get_mng_features_permitted(PngRust* pngrust);
void png_rust_set_mng_features_permitted(PngRust* pngrust, uint8_t value);

void png_rust_set_info_fn(PngRust* pngrust, png_progressive_info_ptr value);
void png_rust_set_row_fn(PngRust* pngrust, png_progressive_row_ptr value);
void png_rust_set_end_fn(PngRust* pngrust, png_progressive_end_ptr value);
void png_rust_set_push_length(PngRust* pngrust, uint32_t value);
void png_rust_set_skip_length(PngRust* pngrust, uint32_t value);
void png_rust_set_buffer_size(PngRust* pngrust, size_t value);
void png_rust_set_process_mode(PngRust* pngrust, int value);
void png_rust_set_cur_palette(PngRust* pngrust, int value);
void png_rust_set_zowner(PngRust* pngrust, uint32_t value);
void png_rust_set_io_ptr(PngRust* pngrust, void *value);

size_t png_rust_get_save_buffer_size(PngRust* pngrust);
uint32_t png_rust_decr_user_chunk_cache_max(PngRust* pngrust);

void png_c_set_strip_error_numbers(PngRust* pngrust, uint32_t ustrip_mode);
void png_rust_process_data(PngRust* pngrust, PngInfoRust* rust_ptr, void* buffer, size_t buffer_size);
size_t png_rust_process_data_pause(PngRust* pngrust, bool save);

int32_t png_rust_get_IHDR(PngRust* pngrust, PngInfoRust* rust_ptr,
                          uint32_t* width, uint32_t* height, int32_t* bit_depth,
                          int32_t* color_type, int32_t* interlace_type,
                          int32_t* compression_type, int32_t* filter_type);
/*
 *      PngInfo part
 */

PngInfoRust* png_info_rust_new();

uint32_t png_info_rust_get_width(PngInfoRust* rust_ptr);
void png_info_rust_set_width(PngInfoRust* rust_ptr, uint32_t value);
uint32_t png_info_rust_get_height(PngInfoRust* rust_ptr);
void png_info_rust_set_height(PngInfoRust* rust_ptr, uint32_t value);
uint32_t png_info_rust_get_valid(PngInfoRust* rust_ptr);
void png_info_rust_set_valid(PngInfoRust* rust_ptr, uint32_t value);
size_t png_info_rust_get_rowbytes(PngInfoRust* rust_ptr);
void png_info_rust_set_rowbytes(PngInfoRust* rust_ptr, size_t value);
void* png_info_rust_get_palette(PngInfoRust* rust_ptr);
void png_info_rust_set_palette(PngInfoRust* rust_ptr, void* value);
uint16_t png_info_rust_get_num_palette(PngInfoRust* rust_ptr);
void png_info_rust_set_num_palette(PngInfoRust* rust_ptr, uint16_t value);
uint16_t png_info_rust_get_num_trans(PngInfoRust* rust_ptr);
void png_info_rust_set_num_trans(PngInfoRust* rust_ptr, uint16_t value);
uint8_t png_info_rust_get_bit_depth(PngInfoRust* rust_ptr);
void png_info_rust_set_bit_depth(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_color_type(PngInfoRust* rust_ptr);
void png_info_rust_set_color_type(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_compression_type(PngInfoRust* rust_ptr);
void png_info_rust_set_compression_type(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_filter_type(PngInfoRust* rust_ptr);
void png_info_rust_set_filter_type(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_channels(PngInfoRust* rust_ptr);
void png_info_rust_set_channels(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_pixel_depth(PngInfoRust* rust_ptr);
void png_info_rust_set_pixel_depth(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_spare_byte(PngInfoRust* rust_ptr);
void png_info_rust_set_spare_byte(PngInfoRust* rust_ptr, uint8_t value);
png_charp png_info_rust_get_iccp_name(PngInfoRust* rust_ptr);
void png_info_rust_set_iccp_name(PngInfoRust* rust_ptr, png_charp value);
png_bytep png_info_rust_get_iccp_profile(PngInfoRust* rust_ptr);
void png_info_rust_set_iccp_profile(PngInfoRust* rust_ptr, png_bytep value);
uint32_t png_info_rust_get_iccp_proflen(PngInfoRust* rust_ptr);
void png_info_rust_set_iccp_proflen(PngInfoRust* rust_ptr, uint32_t value);
int32_t png_info_rust_get_num_text(PngInfoRust* rust_ptr);
void png_info_rust_set_num_text(PngInfoRust* rust_ptr, int32_t value);
int32_t png_info_rust_get_max_text(PngInfoRust* rust_ptr);
void png_info_rust_set_max_text(PngInfoRust* rust_ptr, int32_t value);
png_textp png_info_rust_get_text(PngInfoRust* rust_ptr);
void png_info_rust_set_text(PngInfoRust* rust_ptr, png_textp value);
png_bytep png_info_rust_get_trans_alpha(PngInfoRust* rust_ptr);
void png_info_rust_set_trans_alpha(PngInfoRust* rust_ptr, png_bytep value);
int32_t png_info_rust_get_x_offset(PngInfoRust* rust_ptr);
void png_info_rust_set_x_offset(PngInfoRust* rust_ptr, int32_t value);
int32_t png_info_rust_get_y_offset(PngInfoRust* rust_ptr);
void png_info_rust_set_y_offset(PngInfoRust* rust_ptr, int32_t value);
uint8_t png_info_rust_get_offset_unit_type(PngInfoRust* rust_ptr);
void png_info_rust_set_offset_unit_type(PngInfoRust* rust_ptr, uint8_t value);
uint32_t png_info_rust_get_x_pixels_per_unit(PngInfoRust* rust_ptr);
void png_info_rust_set_x_pixels_per_unit(PngInfoRust* rust_ptr, uint32_t value);
uint32_t png_info_rust_get_y_pixels_per_unit(PngInfoRust* rust_ptr);
void png_info_rust_set_y_pixels_per_unit(PngInfoRust* rust_ptr, uint32_t value);
int32_t png_info_rust_get_num_exif(PngInfoRust* rust_ptr);
void png_info_rust_set_num_exif(PngInfoRust* rust_ptr, int32_t value);
uint8_t* png_info_rust_get_exif(PngInfoRust* rust_ptr);
void png_info_rust_set_exif(PngInfoRust* rust_ptr, uint8_t* value);
uint8_t* png_info_rust_get_eXIf_buf(PngInfoRust* rust_ptr);
void png_info_rust_set_eXIf_buf(PngInfoRust* rust_ptr, uint8_t* value);
uint8_t png_info_rust_get_scal_unit(PngInfoRust* rust_ptr);
void png_info_rust_set_scal_unit(PngInfoRust* rust_ptr, uint8_t value);
void* png_info_rust_get_scal_s_width(PngInfoRust* rust_ptr);
void png_info_rust_set_scal_s_width(PngInfoRust* rust_ptr, void* value);
void* png_info_rust_get_scal_s_height(PngInfoRust* rust_ptr);
void png_info_rust_set_scal_s_height(PngInfoRust* rust_ptr, void* value);

uint8_t png_info_rust_get_interlace_type(PngInfoRust* rust_ptr);
void png_info_rust_set_interlace_type(PngInfoRust* rust_ptr, uint8_t value);
uint8_t png_info_rust_get_phys_unit_type(PngInfoRust* rust_ptr);
void png_info_rust_set_phys_unit_type(PngInfoRust* rust_ptr, uint8_t value);

void png_info_rust_add_valid(PngInfoRust* rust_ptr, uint32_t value);
void png_info_rust_remove_valid(PngInfoRust* rust_ptr, uint32_t value);

void png_info_rust_add_color_type(PngInfoRust* rust_ptr, uint8_t value);
bool png_info_rust_has_color_type(PngInfoRust* rust_ptr, uint8_t value);


png_color_16p png_info_rust_ptr_trans_color(PngInfoRust* rust_ptr);
png_color_16p png_info_rust_ptr_background(PngInfoRust* rust_ptr);
png_color_8p png_info_rust_ptr_sig_bit(PngInfoRust* rust_ptr);

void png_info_rust_set_background(PngInfoRust* rust_ptr, png_const_color_16p background);
void png_info_rust_set_sig_bit(PngInfoRust* rust_ptr, png_const_color_8p sig_bit);
void png_info_rust_set_trans_color(PngInfoRust* rust_ptr, png_const_color_16p trans_color);

void png_info_rust_incr_channels(PngInfoRust* rust_ptr);
void png_info_rust_incr_num_text(PngInfoRust* rust_ptr);

#endif /* PNGRUST_H */
