#[macro_use]
extern crate bitflags;

enum PngInterlace {
    ADAM7,
}
type CPtr = usize;

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
        const ASSUME_S_RGB          = 0x1000;
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
        const WROTE_T_IME            = 0x200;
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
        const T_16_TO_8          = 0x0400;    /* Becomes 'chop' in 1.5.4 */
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
        const EXPAND_T_RNS       = 0x2000000;
        const SCALE_16_TO_8      = 0x4000000;
                       /* 0x8000000 unused */
                       /* 0x10000000 unused */
                       /* 0x20000000 unused */
                       /* 0x40000000 unused */
    }
}


bitflags! {
    struct PngFilter: u8 {
        const NONE            = 0x08;
        const SUB             = 0x10;
        const UP              = 0x20;
        const AVG             = 0x40;
        const PAETH           = 0x80;
        const FAST_FILTERS    = (0x08 | 0x10 | 0x20);
    }
}


pub struct Png {
    mode: PngMode,        /* tells us where we are in the PNG file */
    flags: PngFlags,      /* flags indicating various things to libpng */
    transformations: PngTransformations, /* which transformations to perform */

    pass: u8,        /* current interlace pass (0 - 6) */
    //compression: u8, /* file compression type (always 0) */
    interlaced: Option<PngInterlace>,
    //filter: u8,      /* file filter type (always 0) */

    num_trans: u16,       /* number of transparency values */
    do_filter: PngFilter,        /* row filter flags (see PNG_FILTER_ in png.h ) */
    color_type: PngColor, /* color type of file */
    bit_depth: u8,        /* bit depth of file */
    usr_bit_depth: u8,    /* bit depth of users row: write only */
    pixel_depth: u8,      /* number of bits per pixel */
    channels: u8,         /* number of channels in file */


    width: u32,           /* width of image in pixels */
    height: u32,          /* height of image in pixels */
    num_rows: u32,        /* number of rows in current pass */
    usr_width: u32,       /* width of row at start of write */
    rowbytes: usize,      /* size of row in bytes */
    iwidth: u32,          /* width of current interlaced row in pixels */
    row_number: u32,      /* current row in interlace pass */
    chunk_name: u32,      /* PNG_CHUNK() id of current chunk */
    prev_row: CPtr,       /* buffer to save previous (unfiltered) row.
                           * While reading this is a pointer into
                           * big_prev_row; while writing it is separately
                           * allocated if needed.
                           */
    row_buf: CPtr,        /* buffer to save current (unfiltered) row.
                           * While reading, this is a pointer into
                           * big_row_buf; while writing it is separately
                           * allocated.
                           */

    try_row: CPtr,        /* buffer to save trial row when filtering */
    tst_row: CPtr,        /* buffer to save best trial row when filtering */

    info_rowbytes: usize, /* Added in 1.5.4: cache of updated row bytes */

    idat_size: u32,     /* current IDAT size for read */
    crc: u32,           /* current chunk CRC value */
    palette: CPtr,      /* palette from the input file (array of RGB pixel) */
    num_palette: u16,   /* number of color entries in palette */

    num_palette_max: i32, /* maximum palette index found in IDAT */

    usr_channels: u8,     /* channels at start of write: write only */

    sig_bytes: u8,        /* magic bytes read/written from start of file */
    maximum_pixel_depth: u8,
                          /* pixel depth used for the row buffers */
    transformed_pixel_depth: u8,
                          /* pixel depth after read/write transforms */

    info_fn: CPtr,              /* called after header data fully read */
    row_fn: CPtr,               /* called after a prog. row is decoded */
    end_fn: CPtr,               /* called after image is complete */
    save_buffer_ptr: CPtr,      /* current location in save_buffer */
    save_buffer: CPtr,          /* buffer for previously read data */
    current_buffer_ptr: CPtr,   /* current location in current_buffer */
    current_buffer: CPtr,       /* buffer for recently used data */
    push_length: u32,           /* size of current input chunk */
    skip_length: u32,           /* bytes to skip in input data */
    save_buffer_size: usize,    /* amount of data now in save_buffer */
    save_buffer_max: usize,     /* total size of save_buffer */
    buffer_size: usize,         /* total amount of available input data */
    current_buffer_size: usize, /* amount of data now in current_buffer */
    process_mode: i32,          /* what push library is currently doing */
    cur_palette: i32,           /* current push library palette index */
    zowner: u32,                /* ID (chunk type) of zstream owner, 0 if none */
    io_ptr: CPtr,               /* ptr to application struct for I/O functions */
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
        //compression : 0,
        interlaced: None,
        //filter : 0,
        num_trans: 0,
        do_filter: PngFilter::empty(),
        color_type: PngColor::MASK_PALETTE,
        bit_depth: 0,
        usr_bit_depth: 0,
        pixel_depth: 0,
        channels: 0,
        width: 0,
        height: 0,
        num_rows: 0,
        usr_width: 0,
        rowbytes: 0,
        iwidth: 0,
        row_number: 0,
        chunk_name: 0,
        prev_row: 0,
        row_buf: 0,
        try_row: 0,
        tst_row: 0,
        info_rowbytes: 0,
        idat_size: 0,
        crc: 0,
        palette: 0,
        num_palette: 0,
        num_palette_max: 0,
        usr_channels: 0,
        sig_bytes: 0,
        maximum_pixel_depth: 0,
        transformed_pixel_depth: 0,
        info_fn: 0,
        row_fn: 0,
        end_fn: 0,
        save_buffer_ptr: 0,
        save_buffer: 0,
        current_buffer_ptr: 0,
        current_buffer: 0,
        push_length: 0,
        skip_length: 0,
        save_buffer_size: 0,
        save_buffer_max: 0,
        buffer_size: 0,
        current_buffer_size: 0,
        process_mode: 0,
        cur_palette: 0,
        zowner: 0,
        io_ptr: 0,
    });
    Box::into_raw(obj)
}

#[no_mangle]
pub unsafe extern fn png_rust_free(this: *mut Png)
{
    Box::from_raw(this);
}

#[no_mangle]
pub unsafe extern fn png_rust_pass_is_valid(this: *const Png) -> bool
{
    this.as_ref().unwrap().pass <= 6
}

#[no_mangle]
pub unsafe extern fn png_rust_incr_pass(this: *mut Png)
{
    this.as_mut().unwrap().pass += 1;
}

#[no_mangle]
pub unsafe extern fn png_rust_decr_pass(this: *mut Png)
{
    this.as_mut().unwrap().pass -= 1;
}

#[no_mangle]
pub unsafe extern fn png_rust_get_interlace(this: *const Png) -> i32
{
    match &this.as_ref().unwrap().interlaced {
        Some(interlace) => {
            match interlace {
                PngInterlace::ADAM7 => 1,
            }
        },
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern fn png_rust_set_interlace(this: *mut Png, value: i32)
{
    let interlace = match value {
        0 => None,
        1 => Some(PngInterlace::ADAM7),
        _ => {
            println!("Invalid interlace value ({}), use none instead", value);
            None
        }
    };

    this.as_mut().unwrap().interlaced = interlace;
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_get_flags(this: *const Png) -> u32
{
    this.as_ref().unwrap().flags.bits()
}

#[no_mangle]
pub unsafe extern fn png_rust_has_flags(this: *const Png, flags: u32) -> bool
{
    let flags = PngFlags::from_bits_truncate(flags);
    this.as_ref().unwrap().flags.contains(flags)
}

#[no_mangle]
pub unsafe extern fn png_rust_one_of_flags(this: *const Png, flags: u32) -> bool
{
    let flags = PngFlags::from_bits_truncate(flags);
    this.as_ref().unwrap().flags.intersects(flags)
}

#[no_mangle]
pub unsafe extern fn png_rust_add_flags(this: *mut Png, new_flags: u32)
{
    let new_flags = PngFlags::from_bits_truncate(new_flags);
    this.as_mut().unwrap().flags.insert(new_flags);
}

#[no_mangle]
pub unsafe extern fn png_rust_remove_flags(this: *mut Png, new_flags: u32)
{
    let new_flags = PngFlags::from_bits_truncate(new_flags);
    this.as_mut().unwrap().flags.remove(new_flags);
}

#[no_mangle]
pub unsafe extern fn png_rust_reset_flags(this: *mut Png)
{
    this.as_mut().unwrap().flags = PngFlags::empty();
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_get_mode(this: *const Png) -> u32
{
    this.as_ref().unwrap().mode.bits()
}

#[no_mangle]
pub unsafe extern fn png_rust_set_mode(this: *mut Png, new_mode: u32)
{
    let new_mode = PngMode::from_bits_truncate(new_mode);
    this.as_mut().unwrap().mode = new_mode;
}

#[no_mangle]
pub unsafe extern fn png_rust_has_mode(this: *const Png, mode: u32) -> bool
{
    let mode = PngMode::from_bits_truncate(mode);
    this.as_ref().unwrap().mode.contains(mode)
}

#[no_mangle]
pub unsafe extern fn png_rust_add_mode(this: *mut Png, new_mode: u32)
{
    let new_mode = PngMode::from_bits_truncate(new_mode);
    this.as_mut().unwrap().mode.insert(new_mode);
}

#[no_mangle]
pub unsafe extern fn png_rust_remove_mode(this: *mut Png, new_mode: u32)
{
    let new_mode = PngMode::from_bits_truncate(new_mode);
    this.as_mut().unwrap().mode.remove(new_mode);
}

#[no_mangle]
pub unsafe extern fn png_rust_reset_mode(this: *mut Png)
{
    this.as_mut().unwrap().mode = PngMode::empty();
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_get_transformations(this: *const Png) -> u32
{
    this.as_ref().unwrap().transformations.bits()
}

#[no_mangle]
pub unsafe extern fn png_rust_has_transformations(this: *const Png, transformations: u32) -> bool
{
    let transformations = PngTransformations::from_bits_truncate(transformations);
    this.as_ref().unwrap().transformations.contains(transformations)
}

#[no_mangle]
pub unsafe extern fn png_rust_one_of_transformations(this: *const Png, transformations: u32) -> bool
{
    let transformations = PngTransformations::from_bits_truncate(transformations);
    this.as_ref().unwrap().transformations.intersects(transformations)
}

#[no_mangle]
pub unsafe extern fn png_rust_empty_transformations(this: *const Png) -> bool
{
    this.as_ref().unwrap().transformations.is_empty()
}

#[no_mangle]
pub unsafe extern fn png_rust_add_transformations(this: *mut Png, new_transformations: u32)
{
    let new_transformations = PngTransformations::from_bits_truncate(new_transformations);
    this.as_mut().unwrap().transformations.insert(new_transformations);
}

#[no_mangle]
pub unsafe extern fn png_rust_remove_transformations(this: *mut Png, new_transformations: u32)
{
    let new_transformations = PngTransformations::from_bits_truncate(new_transformations);
    this.as_mut().unwrap().transformations.remove(new_transformations);
}

#[no_mangle]
pub unsafe extern fn png_rust_reset_transformations(this: *mut Png)
{
    this.as_mut().unwrap().transformations = PngTransformations::empty();
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_get_color_type(this: *const Png) -> u8
{
    this.as_ref().unwrap().color_type.bits()
}

#[no_mangle]
pub unsafe extern fn png_rust_set_color_type(this: *mut Png, color_type: u8)
{
    let color_type = PngColor::from_bits_truncate(color_type);
    this.as_mut().unwrap().color_type = color_type;
}

#[no_mangle]
pub unsafe extern fn png_rust_has_color_type(this: *const Png, color_type: u8) -> bool
{
    let color_type = PngColor::from_bits_truncate(color_type);
    this.as_ref().unwrap().color_type.contains(color_type)
}

#[no_mangle]
pub unsafe extern fn png_rust_is_color_type(this: *const Png, color_type: u8) -> bool
{
    let color_type = PngColor::from_bits_truncate(color_type);
    this.as_ref().unwrap().color_type.bits == color_type.bits()
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_get_do_filter(this: *const Png) -> u8
{
    this.as_ref().unwrap().do_filter.bits()
}

#[no_mangle]
pub unsafe extern fn png_rust_set_do_filter(this: *mut Png, filter: u8)
{
    let filter = PngFilter::from_bits_truncate(filter);
    this.as_mut().unwrap().do_filter = filter
}

#[no_mangle]
pub unsafe extern fn png_rust_is_do_filter(this: *const Png, filter: u8) -> bool
{
    let filter = PngFilter::from_bits_truncate(filter);
    this.as_ref().unwrap().do_filter.bits() == filter.bits()
}

////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_sub_idat_size(this: *mut Png, value: u32)
{
    this.as_mut().unwrap().idat_size -= value;
}

#[no_mangle]
pub unsafe extern fn png_rust_incr_row_number(this: *mut Png)
{
    this.as_mut().unwrap().row_number += 1;
}

#[no_mangle]
pub unsafe extern fn png_rust_add_current_buffer_ptr(this: *mut Png, value: usize)
{
    this.as_mut().unwrap().current_buffer_ptr += value;
}

#[no_mangle]
pub unsafe extern fn png_rust_add_save_buffer_size(this: *mut Png, value: usize)
{
    this.as_mut().unwrap().save_buffer_size += value;
}

#[no_mangle]
pub unsafe extern fn png_rust_add_save_buffer_ptr(this: *mut Png, value: usize)
{
    this.as_mut().unwrap().save_buffer_ptr += value;
}


#[no_mangle]
pub unsafe extern fn png_rust_sub_current_buffer_size(this: *mut Png, value: usize)
{
    this.as_mut().unwrap().current_buffer_size -= value;
}

#[no_mangle]
pub unsafe extern fn png_rust_sub_save_buffer_size(this: *mut Png, value: usize)
{
    this.as_mut().unwrap().save_buffer_size -= value;
}

#[no_mangle]
pub unsafe extern fn png_rust_sub_buffer_size(this: *mut Png, value: usize)
{
    this.as_mut().unwrap().buffer_size -= value;
}

////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_c_set_strip_error_numbers(this: *mut Png, strip_mode: u32)
{
    let mut strip_mode_without_errors = PngFlags::from_bits_truncate(strip_mode);
    strip_mode_without_errors.remove(PngFlags::STRIP_ERROR_NUMBERS | PngFlags::STRIP_ERROR_TEXT);
    this.as_mut().unwrap().flags.remove(strip_mode_without_errors);
}

macro_rules! get_set {
    ($field:ident, $type:ty) => (
        paste::item! {
            #[no_mangle]
            pub unsafe extern fn [<png_rust_get_ $field>](this: *const Png) -> $type {
                this.as_ref().unwrap().$field
            }

            #[no_mangle]
            pub unsafe extern fn [<png_rust_set_ $field>](this: *mut Png, value: $type) {
                this.as_mut().unwrap().$field = value;
            }
        }
    )
}

get_set!(pass,           u8);
get_set!(num_trans,      u16);
get_set!(bit_depth,      u8);
get_set!(usr_bit_depth,  u8);
get_set!(pixel_depth,    u8);
get_set!(channels,       u8);
get_set!(width,          u32);
get_set!(height,         u32);
get_set!(num_rows,       u32);
get_set!(usr_width,      u32);
get_set!(rowbytes,       usize);
get_set!(iwidth,         u32);
get_set!(row_number,     u32);
get_set!(chunk_name,     u32);
get_set!(prev_row,       CPtr);
get_set!(row_buf,        CPtr);
get_set!(try_row,        CPtr);
get_set!(tst_row,        CPtr);
get_set!(info_rowbytes,  usize);
get_set!(idat_size,      u32);
get_set!(crc,            u32);
get_set!(palette,        CPtr);
get_set!(num_palette,    u16);
get_set!(num_palette_max, i32);
get_set!(usr_channels,   u8);
get_set!(sig_bytes,      u8);
get_set!(maximum_pixel_depth,     u8);
get_set!(transformed_pixel_depth, u8);
get_set!(info_fn,        CPtr);
get_set!(row_fn,         CPtr);
get_set!(end_fn,         CPtr);
get_set!(save_buffer_ptr,         CPtr);
get_set!(save_buffer,    CPtr);
get_set!(current_buffer_ptr,      CPtr);
get_set!(current_buffer, CPtr);
get_set!(push_length,    u32);
get_set!(skip_length,    u32);
get_set!(save_buffer_size,        usize);
get_set!(save_buffer_max,         usize);
get_set!(buffer_size,             usize);
get_set!(current_buffer_size,     usize);
get_set!(process_mode,   i32);
get_set!(cur_palette,    i32);
get_set!(zowner,         u32);
get_set!(io_ptr,         CPtr);
