#[macro_use]
extern crate bitflags;

enum PngInterlace {
    ADAM7,
}

bitflags! {
    struct PngColor: u8 {
        const MASK_PALETTE = 0x1;
        const MASK_COLOR   = 0x2;
        const MASK_ALPHA   = 0x4;

        const TYPE_GRAY       = 0x0;
        const TYPE_PALETTE    = 0x2 | 0x1;
        const TYPE_RGB        = 0x2;
        const TYPE_RGB_ALPHA  = 0x2 | 0x4;
        const TYPE_GRAY_ALPHA = 0x4;
    }
}


bitflags! {
    struct PngFlags: u32 {
        const ZLIB_CUSTOM_STRATEGY  = 0x1;
        const ZSTREAM_INITIALIZED   = 0x2;
        /* 0x0004    unused */
        const ZSTREAM_ENDED         = 0x8;
        /* 0x0010    unused */
        /* 0x0020    unused */
        const ROW_INIT              = 0x40;
        const FILLER_AFTER          = 0x80;
        const CRC_ANCILLARY_USE     = 0x100;
        const CRC_ANCILLARY_NOWARN  = 0x200;
        const CRC_CRITICAL_USE      = 0x400;
        const CRC_CRITICAL_IGNORE   = 0x800;
        const ASSUME_sRGB           = 0x1000;
        const OPTIMIZE_ALPHA        = 0x2000;
        const DETECT_UNINITIALIZED  = 0x4000;
        /* const KEEP_UNKNOWN_CHUNKS  = 0x8000; */
        /* const KEEP_UNSAFE_CHUNKS   = 0x10000; */
        const LIBRARY_MISMATCH      = 0x20000;
        const STRIP_ERROR_NUMBERS   = 0x40000;
        const STRIP_ERROR_TEXT      = 0x80000;
        const BENIGN_ERRORS_WARN    = 0x100000;
        const APP_WARNINGS_WARN     = 0x200000;
        const APP_ERRORS_WARN       = 0x400000;
    }
}


bitflags! {
    struct PngMode: u32 {
        const HAVE_IHDR           = 0x01;
        const HAVE_PLTE           = 0x02;
        const HAVE_IDAT           = 0x04;
        const AFTER_IDAT          = 0x08;
        const HAVE_IEND           = 0x10;
        /* 0x20; (unused) */
        /* 0x40; (unused) */
        /* 0x80; (unused) */
        const HAVE_CHUNK_HEADER      = 0x100;
        const WROTE_tIME             = 0x200;
        const WROTE_INFO_BEFORE_PLTE = 0x400;
        const BACKGROUND_IS_GRAY     = 0x800;
        const HAVE_PNG_SIGNATURE     = 0x1000;
        const HAVE_CHUNK_AFTER_IDAT  = 0x2000; /* Have another chunk after IDAT */
        /* 0x4000; (unused) */
        const IS_READ_STRUCT         = 0x8000; /* Else is a write struct */
    }
}


bitflags! {
    struct PngTransformations: u32 {
        const BGR                = 0x0001;
        const INTERLACE          = 0x0002;
        const PACK               = 0x0004;
        const SHIFT              = 0x0008;
        const SWAP_BYTES         = 0x0010;
        const INVERT_MONO        = 0x0020;
        const QUANTIZE           = 0x0040;
        const COMPOSE            = 0x0080;    /* Was PNG_BACKGROUND */
        const BACKGROUND_EXPAND  = 0x0100;
        const EXPAND_16          = 0x0200;    /* Added to libpng 1.5.2 */
        const T_16_TO_8            = 0x0400;    /* Becomes 'chop' in 1.5.4 */
        const RGBA               = 0x0800;
        const EXPAND             = 0x1000;
        const GAMMA              = 0x2000;
        const GRAY_TO_RGB        = 0x4000;
        const FILLER             = 0x8000;
        const PACKSWAP           = 0x10000;
        const SWAP_ALPHA         = 0x20000;
        const STRIP_ALPHA        = 0x40000;
        const INVERT_ALPHA       = 0x80000;
        const USER_TRANSFORM     = 0x100000;
        const RGB_TO_GRAY_ERR    = 0x200000;
        const RGB_TO_GRAY_WARN   = 0x400000;
        const RGB_TO_GRAY        = 0x600000; /* two bits, RGB_TO_GRAY_ERR|WARN */
        const ENCODE_ALPHA       = 0x800000;
        const ADD_ALPHA          = 0x1000000;
        const EXPAND_tRNS        = 0x2000000;
        const SCALE_16_TO_8      = 0x4000000;
                       /* 0x8000000 unused */
                       /* 0x10000000 unused */
                       /* 0x20000000 unused */
                       /* 0x40000000 unused */
    }
}



pub struct Png {
    mode: PngMode,        /* tells us where we are in the PNG file */
    flags: PngFlags,      /* flags indicating various things to libpng */
    transformations: PngTransformations, /* which transformations to perform */

    pass: u8,        /* current interlace pass (0 - 6) */
    compression: u8, /* file compression type (always 0) */
    interlaced: Option<PngInterlace>,
    filter: u8,      /* file filter type (always 0) */

    num_trans: u16,       /* number of transparency values */
    do_filter: u8,        /* row filter flags (see PNG_FILTER_ in png.h ) */
    color_type: PngColor, /* color type of file */
    bit_depth: u8,        /* bit depth of file */
    usr_bit_depth: u8,    /* bit depth of users row: write only */
    pixel_depth: u8,      /* number of bits per pixel */
    channels: u8,         /* number of channels in file */
}

impl Drop for Png {
    fn drop(&mut self) {
        // Do something "interesting" in the destructor so we know when it gets
        // called
        println!("Destroying rust obj");
    }
}

#[no_mangle]
pub extern fn png_rust_new() -> *mut Png
{
    let obj = Box::new(Png {
        mode: PngMode::empty(),
        flags: PngFlags::empty(),
        transformations: PngTransformations::empty(),
        pass : 0,
        compression : 0,
        interlaced: None,
        filter : 0,
        num_trans: 0,
        do_filter: 0,
        color_type: PngColor::MASK_PALETTE,
        bit_depth: 0,
        usr_bit_depth: 0,
        pixel_depth: 0,
        channels: 0,
    });
    Box::into_raw(obj)
}

#[no_mangle]
pub extern fn png_rust_free(state: *mut Png)
{
}

#[no_mangle]
pub extern fn png_rust_pass_is_valid(this: *const Png) -> bool
{
    unsafe {
        (*this).pass <= 6
    }
}

#[no_mangle]
pub extern fn png_rust_get_pass(this: *const Png) -> u8
{
    unsafe {
        (*this).pass
    }
}

#[no_mangle]
pub extern fn png_rust_incr_pass(this: *mut Png)
{
    unsafe {
        (*this).pass += 1;
    }
}

#[no_mangle]
pub extern fn png_rust_decr_pass(this: *mut Png)
{
    unsafe {
        (*this).pass -= 1;
    }
}

#[no_mangle]
pub extern fn png_rust_get_interlace(this: *const Png) -> i32
{
    let interlace;
    unsafe {
        interlace = &(*this).interlaced;
    }

    match interlace {
        Some(interlace) => {
            match interlace {
                PngInterlace::ADAM7 => 1,
            }
        },
        None => 0,
    }
}

#[no_mangle]
pub extern fn png_rust_set_interlace(this: *mut Png, value: i32)
{
    let interlace = match value {
        0 => None,
        1 => Some(PngInterlace::ADAM7),
        _ => {
            println!("Invalid interlace value ({}), use none instead", value);
            None
        }
    };

    unsafe {
        (*this).interlaced = interlace;
    }
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern fn png_rust_get_flags(this: *const Png) -> u32
{
    unsafe {
        (*this).flags.bits()
    }
}

#[no_mangle]
pub extern fn png_rust_has_flags(this: *const Png, flags: u32) -> bool
{
    let flags = PngFlags::from_bits_truncate(flags);
    unsafe {
        (*this).flags.contains(flags)
    }
}

#[no_mangle]
pub extern fn png_rust_one_of_flags(this: *const Png, flags: u32) -> bool
{
    let flags = PngFlags::from_bits_truncate(flags);
    unsafe {
        (*this).flags.intersects(flags)
    }
}

#[no_mangle]
pub extern fn png_rust_add_flags(this: *mut Png, new_flags: u32)
{
    let flags;
    unsafe {
        flags = &mut (*this).flags;
    }

    let new_flags = PngFlags::from_bits_truncate(new_flags);
    flags.insert(new_flags);
}

#[no_mangle]
pub extern fn png_rust_remove_flags(this: *mut Png, new_flags: u32)
{
    let flags;
    unsafe {
        flags = &mut (*this).flags;
    }

    let new_flags = PngFlags::from_bits_truncate(new_flags);
    flags.remove(new_flags);
}

#[no_mangle]
pub extern fn png_rust_reset_flags(this: *mut Png)
{
    unsafe {
        (*this).flags = PngFlags::empty();
    }
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern fn png_rust_get_mode(this: *const Png) -> u32
{
    unsafe {
        (*this).mode.bits()
    }
}

#[no_mangle]
pub extern fn png_rust_set_mode(this: *mut Png, new_mode: u32)
{
    let new_mode = PngMode::from_bits_truncate(new_mode);
    unsafe {
        (*this).mode = new_mode;
    }
}

#[no_mangle]
pub extern fn png_rust_has_mode(this: *const Png, mode: u32) -> bool
{
    let mode = PngMode::from_bits_truncate(mode);
    unsafe {
        (*this).mode.contains(mode)
    }
}

#[no_mangle]
pub extern fn png_rust_add_mode(this: *mut Png, new_mode: u32)
{
    let mode;
    unsafe {
        mode = &mut (*this).mode;
    }

    let new_mode = PngMode::from_bits_truncate(new_mode);
    mode.insert(new_mode);
}

#[no_mangle]
pub extern fn png_rust_remove_mode(this: *mut Png, new_mode: u32)
{
    let mode;
    unsafe {
        mode = &mut (*this).mode;
    }

    let new_mode = PngMode::from_bits_truncate(new_mode);
    mode.remove(new_mode);
}

#[no_mangle]
pub extern fn png_rust_reset_mode(this: *mut Png)
{
    unsafe {
        (*this).mode = PngMode::empty();
    }
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern fn png_rust_get_transformations(this: *const Png) -> u32
{
    unsafe {
        (*this).transformations.bits()
    }
}

#[no_mangle]
pub extern fn png_rust_has_transformations(this: *const Png, transformations: u32) -> bool
{
    let transformations = PngTransformations::from_bits_truncate(transformations);
    unsafe {
        (*this).transformations.contains(transformations)
    }
}

#[no_mangle]
pub extern fn png_rust_one_of_transformations(this: *const Png, transformations: u32) -> bool
{
    let transformations = PngTransformations::from_bits_truncate(transformations);
    unsafe {
        (*this).transformations.intersects(transformations)
    }
}

#[no_mangle]
pub extern fn png_rust_empty_transformations(this: *const Png) -> bool
{
    unsafe {
        (*this).transformations.is_empty()
    }
}

#[no_mangle]
pub extern fn png_rust_add_transformations(this: *mut Png, new_transformations: u32)
{
    let transformations;
    unsafe {
        transformations = &mut (*this).transformations;
    }

    let new_transformations = PngTransformations::from_bits_truncate(new_transformations);
    transformations.insert(new_transformations);
}

#[no_mangle]
pub extern fn png_rust_remove_transformations(this: *mut Png, new_transformations: u32)
{
    let transformations;
    unsafe {
        transformations = &mut (*this).transformations;
    }

    let new_transformations = PngTransformations::from_bits_truncate(new_transformations);
    transformations.remove(new_transformations);
}

#[no_mangle]
pub extern fn png_rust_reset_transformations(this: *mut Png)
{
    unsafe {
        (*this).transformations = PngTransformations::empty();
    }
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern fn png_rust_get_color_type(this: *const Png) -> u8
{
    unsafe {
        (*this).color_type.bits()
    }
}

#[no_mangle]
pub extern fn png_rust_set_color_type(this: *mut Png, color_type: u8)
{
    let color_type = PngColor::from_bits_truncate(color_type);
    unsafe {
        (*this).color_type = color_type;
    }
}

#[no_mangle]
pub extern fn png_rust_has_color_type(this: *const Png, color_type: u8) -> bool
{
    let color_type = PngColor::from_bits_truncate(color_type);
    unsafe {
        (*this).color_type.contains(color_type)
    }
}

#[no_mangle]
pub extern fn png_rust_is_color_type(this: *const Png, color_type: u8) -> bool
{
    let color_type = PngColor::from_bits_truncate(color_type);
    unsafe {
        (*this).color_type.bits == color_type.bits()
    }
}


#[no_mangle]
pub extern fn png_c_set_strip_error_numbers(this: *mut Png, strip_mode: u32)
{
    let flags;
    unsafe {
        flags = &mut (*this).flags;
    }

    let mut strip_mode_without_errors = PngFlags::from_bits_truncate(strip_mode);
    strip_mode_without_errors.remove(PngFlags::STRIP_ERROR_NUMBERS | PngFlags::STRIP_ERROR_TEXT);
    flags.remove(strip_mode_without_errors);
}
