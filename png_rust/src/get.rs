use crate::Png;
use crate::png_info::PngInfo;

#[no_mangle]
pub unsafe extern fn png_rust_get_IHDR(this: *const Png, png_info: *const PngInfo,
                                       width: *mut u32, height: *mut u32, bit_depth: *mut i32,
                                       color_type: *mut i32, interlace_type: *mut i32,
                                       compression_type: *mut i32, filter_type: *mut i32) -> i32
{
    let png_info = match png_info.as_ref() {
        None => {
            return 0;
        },
        Some(png_info) => png_info,
    };

    match this.as_ref() {
        None => {
            return 0;
        },
        Some(png_ptr) => {
            match width.as_mut() {
                None => {},
                Some(width) => { *width = png_info.width; }
            }
            match height.as_mut() {
                None => {},
                Some(height) => { *height = png_info.height; }
            }
            match bit_depth.as_mut() {
                None => {},
                Some(bit_depth) => { *bit_depth = png_info.bit_depth as i32; }
            }
            match color_type.as_mut() {
                None => {},
                Some(color_type) => { *color_type = png_info.color_type.bits() as i32; }
            }
            match compression_type.as_mut() {
                None => {},
                Some(compression_type) => { *compression_type = png_info.compression_type as i32; }
            }
            match filter_type.as_mut() {
                None => {},
                Some(filter_type) => { *filter_type = png_info.filter_type as i32; }
            }
            match interlace_type.as_mut() {
                None => {},
                Some(interlace_type) => { *interlace_type = png_info.interlace_type as i32; }
            }

            /* This is redundant if we can be sure that the info_ptr values were all
             * assigned in png_set_IHDR().  We do the check anyhow in case an
             * application has ignored our advice not to mess with the members
             * of info_ptr directly.
             */
            let result = png_ptr.check_IHDR(png_info.width, png_info.height, png_info.bit_depth,
                                            png_info.color_type,
                                            png_info.compression_type, png_info.filter_type);

            match result {
                Ok(()) => 1,
                /* TODO: call png_error */
                Err(_str) => 0,
            }
        },
    }
}
