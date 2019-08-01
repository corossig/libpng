use crate::PngInterlace;
use crate::PngColor;
use crate::PngFilterType;
use crate::PngCompressionType;
use crate::PngInfoChunk;
use crate::CPtr;
use std::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PngColor16 {
    index: u8,    /* used for palette files */
    red: u16,   /* for use in red green blue files */
    green: u16,
    blue: u16,
    gray: u16,  /* for use in grayscale files */
}

impl fmt::Display for PngColor16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(i {}, r {}, g {}, b {}, gr {})", self.index, self.red, self.green, self.blue, self.gray)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PngColor8 {
    red: u8,   /* for use in red green blue files */
    green: u8,
    blue: u8,
    gray: u8,  /* for use in grayscale files */
    alpha: u8, /* for alpha channel files */
}


#[derive(Debug, PartialEq)]
#[repr(u8)]
enum PngResolution {
    Unknown = 0, /* pixels/unknown unit (aspect ratio) */
    Meter   = 1, /* pixels/meter */
    Last    = 2, /* Not a valid value */
}

#[allow(non_snake_case)]
pub struct PngInfo {
    pub png_info: CPtr,   /* Pointer to the C structure */

    /* This is never set during write */
    pub signature: [u8; 8], /* magic bytes read by libpng from start of file */

    /* The following are necessary for every PNG file */
    pub width: u32,       /* width of image in pixels (from IHDR) */
    pub height: u32,      /* height of image in pixels (from IHDR) */
    valid: PngInfoChunk, /* valid chunk data (see PNG_INFO_ below) */
    pub rowbytes: usize,  /* bytes needed to hold an untransformed row */
    palette: CPtr,    /* array of color values (valid & PNG_INFO_PLTE) */
    num_palette: u16, /* number of color entries in "palette" (PLTE) */
    num_trans: u16,   /* number of transparent palette color (tRNS) */
    pub bit_depth: u8,    /* 1, 2, 4, 8, or 16 bits/channel (from IHDR) */
    pub color_type: PngColor,   /* see PNG_COLOR_TYPE_ below (from IHDR) */

    /* The following three should have been named *_method not *_type */
    pub compression_type: PngCompressionType, /* must be PNG_COMPRESSION_TYPE_BASE (IHDR) */
    pub filter_type: PngFilterType,           /* must be PNG_FILTER_TYPE_BASE (from IHDR) */
    pub interlace_type: PngInterlace,         /* One of PNG_INTERLACE_NONE, PNG_INTERLACE_ADAM7 */

    /* The following are set by png_set_IHDR, called from the application on
     * write, but the are never actually used by the write code.
     */
    pub channels: u8,     /* number of data channels per pixel (1, 2, 3, 4) */
    pub pixel_depth: u8,  /* number of bits per pixel */
    spare_byte: u8,   /* to align the data, and for future use */

    /* This is never set during write */
    //signature: [i8; 8],   /* magic bytes read by libpng from start of file */

    /* The rest of the data is optional.  If you are reading, check the
     * valid field to see if the information in these are valid.  If you
     * are writing, set the valid field to those chunks you want written,
     * and initialize the appropriate fields below.
     */

    /* iCCP chunk data. */
    iccp_name: CPtr,     /* profile name */
    iccp_profile: CPtr,  /* International Color Consortium profile data */
    iccp_proflen: u32,   /* ICC profile data length */

    /* The tEXt, and zTXt chunks contain human-readable textual data in
     * uncompressed, compressed, and optionally compressed forms, respectively.
     * The data in "text" is an array of pointers to uncompressed,
     * null-terminated C strings. Each chunk has a keyword that describes the
     * textual data contained in that chunk.  Keywords are not required to be
     * unique, and the text string may be empty.  Any number of text chunks may
     * be in an image.
     */
    num_text: i32, /* number of comments read or comments to write */
    max_text: i32, /* current size of text array */
    text: CPtr,    /* array of comments read or comments to write */

    /* The sBIT chunk specifies the number of significant high-order bits
     * in the pixel data.  Values are in the range [1, bit_depth], and are
     * only specified for the channels in the pixel data.  The contents of
     * the low-order bits is not specified.  Data is valid if
     * (valid & PNG_INFO_sBIT) is non-zero.
     */
    sig_bit: PngColor8, /* significant bits in color channels */

    /* The tRNS chunk supplies transparency data for paletted images and
     * other image types that don't need a full alpha channel.  There are
     * "num_trans" transparency values for a paletted image, stored in the
     * same order as the palette colors, starting from index 0.  Values
     * for the data are in the range [0, 255], ranging from fully transparent
     * to fully opaque, respectively.  For non-paletted images, there is a
     * single color specified that should be treated as fully transparent.
     * Data is valid if (valid & PNG_INFO_tRNS) is non-zero.
     */
    trans_alpha: CPtr,       /* alpha values for paletted image */
    trans_color: PngColor16, /* transparent color for non-palette image */

    /* The bKGD chunk gives the suggested image background color if the
     * display program does not have its own background color and the image
     * is needs to composited onto a background before display.  The colors
     * in "background" are normally in the same color space/depth as the
     * pixel data.  Data is valid if (valid & PNG_INFO_bKGD) is non-zero.
     */
    background: PngColor16,

    /* The oFFs chunk gives the offset in "offset_unit_type" units rightwards
     * and downwards from the top-left corner of the display, page, or other
     * application-specific co-ordinate space.  See the PNG_OFFSET_ defines
     * below for the unit types.  Valid if (valid & PNG_INFO_oFFs) non-zero.
     */
    x_offset: i32, /* x offset on page */
    y_offset: i32, /* y offset on page */
    offset_unit_type: u8, /* offset units type */

    /* The pHYs chunk gives the physical pixel density of the image for
     * display or printing in "phys_unit_type" units (see PNG_RESOLUTION_
     * defines below).  Data is valid if (valid & PNG_INFO_pHYs) is non-zero.
     */
    x_pixels_per_unit: u32, /* horizontal pixel density */
    y_pixels_per_unit: u32, /* vertical pixel density */
    phys_unit_type: Option<PngResolution>, /* resolution type */

    num_exif: i32,  /* Added at libpng-1.6.31 */
    exif: CPtr,
    eXIf_buf: CPtr, /* Added at libpng-1.6.32 */

    /* The sCAL chunk describes the actual physical dimensions of the
     * subject matter of the graphic.  The chunk contains a unit specification
     * a byte value, and two ASCII strings representing floating-point
     * values.  The values are width and height corresponding to one pixel
     * in the image.  Data values are valid if (valid & PNG_INFO_sCAL) is
     * non-zero.
     */
    scal_unit: u8,         /* unit of physical scale */
    scal_s_width: CPtr,     /* string containing height */
    scal_s_height: CPtr,    /* string containing width */
}


#[no_mangle]
pub extern fn png_info_rust_new(png_info_ptr: CPtr) -> *mut PngInfo
{
    let obj = Box::new(PngInfo {
        png_info: png_info_ptr,
        signature: [0; 8],
        width: 0,
        height: 0,
        valid: PngInfoChunk::empty(),
        rowbytes: 0,
        palette: 0,
        num_palette: 0,
        num_trans: 0,
        bit_depth: 0,
        color_type: PngColor::empty(),
        compression_type: PngCompressionType::Base,
        filter_type: PngFilterType::Base,
        interlace_type: PngInterlace::None,
        channels: 0,
        pixel_depth: 0,
        spare_byte: 0,
        iccp_name: 0,
        iccp_profile: 0,
        iccp_proflen: 0,
        num_text: 0,
        max_text: 0,
        text: 0,
        sig_bit: PngColor8 {
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
            alpha: 0,
        },
        trans_alpha: 0,
        trans_color: PngColor16 {
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
        },
        background: PngColor16 {
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
        },
        x_offset: 0,
        y_offset: 0,
        offset_unit_type: 0,
        x_pixels_per_unit: 0,
        y_pixels_per_unit: 0,
        phys_unit_type: None,
        num_exif: 0,
        exif: 0,
        eXIf_buf: 0,
        scal_unit: 0,
        scal_s_width: 0,
        scal_s_height: 0,
    });
    Box::into_raw(obj)
}



macro_rules! get_set_info {
    ($field:ident, $type:ty) => (
        paste::item! {
            #[no_mangle]
            pub unsafe extern fn [<png_info_rust_get_ $field>](this: *const PngInfo) -> $type {
                this.as_ref().unwrap().$field
            }

            #[no_mangle]
            pub unsafe extern fn [<png_info_rust_set_ $field>](this: *mut PngInfo, value: $type) {
                this.as_mut().unwrap().$field = value;
            }
        }
    )
}

get_set_info!(width, u32);
get_set_info!(height, u32);
get_set_info!(rowbytes, usize);
get_set_info!(palette, CPtr);
get_set_info!(num_palette, u16);
get_set_info!(num_trans, u16);
get_set_info!(bit_depth, u8);
get_set_info!(channels, u8);
get_set_info!(pixel_depth, u8);
get_set_info!(spare_byte, u8);
get_set_info!(iccp_name, CPtr);
get_set_info!(iccp_profile, CPtr);
get_set_info!(iccp_proflen, u32);
get_set_info!(num_text, i32);
get_set_info!(max_text, i32);
get_set_info!(text, CPtr);
get_set_info!(trans_alpha, CPtr);
get_set_info!(x_offset, i32);
get_set_info!(y_offset, i32);
get_set_info!(offset_unit_type, u8);
get_set_info!(x_pixels_per_unit, u32);
get_set_info!(y_pixels_per_unit, u32);
get_set_info!(num_exif, i32);
get_set_info!(exif, CPtr);
get_set_info!(eXIf_buf, CPtr);
get_set_info!(scal_unit, u8);
get_set_info!(scal_s_width, CPtr);
get_set_info!(scal_s_height, CPtr);

////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_info_rust_get_color_type(this: *const PngInfo) -> u8
{
    this.as_ref().unwrap().color_type.bits()
}

#[no_mangle]
pub unsafe extern fn png_info_rust_has_color_type(this: *const PngInfo, color_type: u8) -> bool
{
    let color_type = PngColor::from_bits_truncate(color_type);
    this.as_ref().unwrap().color_type.contains(color_type)
}

#[no_mangle]
pub unsafe extern fn png_info_rust_set_color_type(this: *mut PngInfo, color_type: u8)
{
    let color_type = PngColor::from_bits_truncate(color_type);
    this.as_mut().unwrap().color_type = color_type;
}

#[no_mangle]
pub unsafe extern fn png_info_rust_add_color_type(this: *mut PngInfo, color_type: u8)
{
    let color_type = PngColor::from_bits_truncate(color_type);
    this.as_mut().unwrap().color_type.insert(color_type)
}

////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_info_rust_get_valid(this: *const PngInfo) -> u32
{
    this.as_ref().unwrap().valid.bits()
}

#[no_mangle]
pub unsafe extern fn png_info_rust_set_valid(this: *mut PngInfo, valid: u32)
{
    let valid = PngInfoChunk::from_bits_truncate(valid);
    this.as_mut().unwrap().valid = valid;
}

#[no_mangle]
pub unsafe extern fn png_info_rust_add_valid(this: *mut PngInfo, valid: u32)
{
    let valid = PngInfoChunk::from_bits_truncate(valid);
    this.as_mut().unwrap().valid.insert(valid)
}

#[no_mangle]
pub unsafe extern fn png_info_rust_remove_valid(this: *mut PngInfo, valid: u32)
{
    let valid = PngInfoChunk::from_bits_truncate(valid);
    this.as_mut().unwrap().valid.remove(valid)
}

////////////////////////////////////////////////////////////////////////


#[no_mangle]
pub unsafe extern fn png_info_rust_set_background(this: *mut PngInfo, background: *const PngColor16) {
    this.as_mut().unwrap().background = *background.as_ref().unwrap();
}

#[no_mangle]
pub unsafe extern fn png_info_rust_ptr_background(this: *mut PngInfo) -> *mut PngColor16 {
    &mut this.as_mut().unwrap().background
}

#[no_mangle]
pub unsafe extern fn png_info_rust_incr_num_text(this: *mut PngInfo) {
    this.as_mut().unwrap().num_text += 1;
}

#[no_mangle]
pub unsafe extern fn png_info_rust_incr_channels(this: *mut PngInfo) {
    this.as_mut().unwrap().channels += 1;
}

#[no_mangle]
pub unsafe extern fn png_info_rust_ptr_trans_color(this: *mut PngInfo) -> *mut PngColor16 {
    &mut this.as_mut().unwrap().trans_color
}

#[no_mangle]
pub unsafe extern fn png_info_rust_ptr_sig_bit(this: *mut PngInfo) -> *mut PngColor8 {
    &mut this.as_mut().unwrap().sig_bit
}

#[no_mangle]
pub unsafe extern fn png_info_rust_set_sig_bit(this: *mut PngInfo, sig_bit: *const PngColor8) {
    this.as_mut().unwrap().sig_bit = *sig_bit.as_ref().unwrap();
}

#[no_mangle]
pub unsafe extern fn png_info_rust_set_trans_color(this: *mut PngInfo, color: *const PngColor16) {
    this.as_mut().unwrap().trans_color = *color.as_ref().unwrap();
}


#[no_mangle]
pub unsafe extern fn png_info_rust_get_phys_unit_type(this: *const PngInfo) -> u32 {
    match &this.as_ref().unwrap().phys_unit_type {
        Some(resolution) => {
            match resolution {
                PngResolution::Unknown => 0,
                PngResolution::Meter => 1,
                PngResolution::Last => 2,
            }
        },
        None => 2,
    }
}

#[no_mangle]
pub unsafe extern fn png_info_rust_set_phys_unit_type(this: *mut PngInfo, value: u32) {
    this.as_mut().unwrap().phys_unit_type = match value {
        0 => Some(PngResolution::Unknown),
        1 => Some(PngResolution::Meter),
        2 => Some(PngResolution::Last),
        _ => {
            println!("Invalid phys_unit_type value ({}), use none instead", value);
            None
        }
    }
}

#[no_mangle]
pub unsafe extern fn png_info_rust_get_interlace_type(this: *const PngInfo) -> u8 {
    this.as_ref().unwrap().interlace_type as u8
}

#[no_mangle]
pub unsafe extern fn png_info_rust_get_compression_type(this: *const PngInfo) -> u8 {
    this.as_ref().unwrap().compression_type as u8
}

#[no_mangle]
pub unsafe extern fn png_info_rust_get_filter_type(this: *const PngInfo) -> u8 {
    this.as_ref().unwrap().filter_type as u8
}

impl PngInterlace {
    fn from_u32(value: u32) -> PngInterlace {
        match value {
            0 => PngInterlace::None,
            1 => PngInterlace::ADAM7,
            _ => {
                println!("Invalid interlace value ({}), use none instead", value);
                PngInterlace::None
            },
        }
    }
}

#[no_mangle]
pub unsafe extern fn png_info_rust_set_interlace_type(this: *mut PngInfo, value: u32) {
    this.as_mut().unwrap().interlace_type = PngInterlace::from_u32(value);
}
