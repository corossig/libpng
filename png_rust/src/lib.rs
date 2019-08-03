#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate enum_primitive_derive;
use num_traits::{FromPrimitive,ToPrimitive};

use std::collections::VecDeque;

mod read;
mod rutil;
mod trans;
mod set;
mod get;
mod pread;
mod png;
mod png_info;

const PNG_USER_CHUNK_CACHE_MAX: u32 = 1000 as u32;
const PNG_USER_CHUNK_MALLOC_MAX: usize = 8000000 as usize;
const PNG_USER_HEIGHT_MAX: u32 = 1000000 as u32;
const PNG_USER_WIDTH_MAX: u32 = 1000000 as u32;

type CPtr = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PngInterlace {
    None  = 0,
    ADAM7 = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PngCompressionType {
    Base,
    Default,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PngFilterType {
    Base         = 0,
    Differencing = 64,
}

/* For use in png_set_keep_unknown, added to version 1.2.6 */
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PngHandleChunk {
    AsDefault = 0,
    Never     = 1,
    IfSafe    = 2,
    Always    = 3,
}

impl PngInterlace {
    fn from_u8(value: u8) -> PngInterlace {
        match value {
            0 => PngInterlace::None,
            1 => PngInterlace::ADAM7,
            _ => {
                println!("Invalid interlace value ({}), use none instead", value);
                PngInterlace::None
            }
        }
    }
}

impl PngCompressionType {
    fn from_u8(value: u8) -> PngCompressionType {
        match value {
            0 => PngCompressionType::Base,
            _ => {
                println!("Invalid compression type value ({}), use base instead", value);
                PngCompressionType::Base
            }
        }
    }
}

impl PngFilterType {
    fn from_u8(value: u8) -> PngFilterType {
        match value {
            0  => PngFilterType::Base,
            64 => PngFilterType::Differencing,
            _ => {
                println!("Invalid filter type value ({}), use base instead", value);
                PngFilterType::Base
            }
        }
    }
}


bitflags! {
    pub struct PngColor: u8 {
        const MASK_PALETTE = 0x1;
        const MASK_COLOR   = 0x2;
        const MASK_ALPHA   = 0x4;

        const TYPE_GRAY       = 0x0;
        const TYPE_PALETTE    = 0x2 | 0x1;
        const TYPE_RGB        = 0x2;
        const TYPE_RGB_ALPHA  = 0x2 | 0x4;
        const TYPE_RGBA       = 0x2 | 0x4;
        const TYPE_GRAY_ALPHA = 0x4;
        const TYPE_GA         = 0x4;
    }
}


bitflags! {
    pub struct PngFlags: u32 {
        const ZLIB_CUSTOM_STRATEGY  = 0x1;
        const ZSTREAM_INITIALIZED   = 0x2;
        /* 0x0004    unused */
        const ZSTREAM_ENDED         = 0x8;
        /* 0x0010    unused */
        /* 0x0020    unused */
        const ROW_INIT              = 0x40;
        const FILLER_AFTER          = 0x80;
        const CRC_ANCILLARY_USE     = 0x1_00;
        const CRC_ANCILLARY_NOWARN  = 0x2_00;
        const CRC_CRITICAL_USE      = 0x4_00;
        const CRC_CRITICAL_IGNORE   = 0x8_00;
        const ASSUME_S_RGB          = 0x10_00;
        const OPTIMIZE_ALPHA        = 0x20_00;
        const DETECT_UNINITIALIZED  = 0x40_00;
        /* const KEEP_UNKNOWN_CHUNKS  = 0x80_00; */
        /* const KEEP_UNSAFE_CHUNKS   = 0x1_00_00; */
        const LIBRARY_MISMATCH      = 0x2_00_00;
        const STRIP_ERROR_NUMBERS   = 0x4_00_00;
        const STRIP_ERROR_TEXT      = 0x8_00_00;
        const BENIGN_ERRORS_WARN    = 0x10_00_00;
        const APP_WARNINGS_WARN     = 0x20_00_00;
        const APP_ERRORS_WARN       = 0x40_00_00;
    }
}


bitflags! {
    pub struct PngMode: u32 {
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
        const HAVE_PNG_SIGNATURE     = 0x10_00;
        const HAVE_CHUNK_AFTER_IDAT  = 0x20_00; /* Have another chunk after IDAT */
        /* 0x4000; (unused) */
        const IS_READ_STRUCT         = 0x80_00; /* Else is a write struct */
    }
}


bitflags! {
    pub struct PngTransformations: u32 {
        const BGR                = 0x1;
        const INTERLACE          = 0x2;
        const PACK               = 0x4;
        const SHIFT              = 0x8;
        const SWAP_BYTES         = 0x10;
        const INVERT_MONO        = 0x20;
        const QUANTIZE           = 0x40;
        const COMPOSE            = 0x80;    /* Was PNG_BACKGROUND */
        const BACKGROUND_EXPAND  = 0x1_00;
        const EXPAND_16          = 0x2_00;    /* Added to libpng 1.5.2 */
        const T_16_TO_8          = 0x4_00;    /* Becomes 'chop' in 1.5.4 */
        const RGBA               = 0x8_00;
        const EXPAND             = 0x10_00;
        const GAMMA              = 0x20_00;
        const GRAY_TO_RGB        = 0x40_00;
        const FILLER             = 0x80_00;
        const PACKSWAP           = 0x1_00_00;
        const SWAP_ALPHA         = 0x2_00_00;
        const STRIP_ALPHA        = 0x4_00_00;
        const INVERT_ALPHA       = 0x8_00_00;
        const USER_TRANSFORM     = 0x10_00_00;
        const RGB_TO_GRAY_ERR    = 0x20_00_00;
        const RGB_TO_GRAY_WARN   = 0x40_00_00;
        const RGB_TO_GRAY        = 0x60_00_00; /* two bits, RGB_TO_GRAY_ERR|WARN */
        const ENCODE_ALPHA       = 0x80_00_00;
        const ADD_ALPHA          = 0x1_00_00_00;
        const EXPAND_T_RNS       = 0x2_00_00_00;
        const SCALE_16_TO_8      = 0x4_00_00_00;
                       /* 0x8000000 unused */
                       /* 0x10000000 unused */
                       /* 0x20000000 unused */
                       /* 0x40000000 unused */
    }
}


bitflags! {
    pub struct PngFilter: u8 {
        const NONE            = 0x08;
        const SUB             = 0x10;
        const UP              = 0x20;
        const AVG             = 0x40;
        const PAETH           = 0x80;
        const FAST_FILTERS    = (0x08 | 0x10 | 0x20);
    }
}

bitflags! {
    pub struct PngInfoChunk: u32 {
        const gAMA = 0x0001;
        const sBIT = 0x0002;
        const cHRM = 0x0004;
        const PLTE = 0x0008;
        const tRNS = 0x0010;
        const bKGD = 0x0020;
        const hIST = 0x0040;
        const pHYs = 0x0080;
        const oFFs = 0x0100;
        const tIME = 0x0200;
        const pCAL = 0x0400;
        const sRGB = 0x0800;  /* GR-P, 0.96a */
        const iCCP = 0x1000;  /* ESR, 1.0.6 */
        const sPLT = 0x2000;  /* ESR, 1.0.6 */
        const sCAL = 0x4000;  /* ESR, 1.0.6 */
        const IDAT = 0x8000;  /* ESR, 1.0.6 */
        const eXIf = 0x10000; /* GR-P, 1.6.31 */
    }
}

bitflags! {
    struct PngMng: u8 {
        const EmptyPlte = 0x1;
        const Filter64  = 0x4;
    }
}

/* TODO : Transform it into regular enum when no more C code depends on it */
#[derive(Debug, PartialEq, Primitive, Clone, Copy)]
#[repr(i32)]
pub enum PngPushMode {
    ReadSig   = 0,
    ReadChunk = 1,
    ReadIDAT  = 2,
    ReadtEXt  = 4,
    ReadzTXt  = 5,
    ReadDONE  = 6,
    ReadiTXt  = 7,
    Error     = 8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Primitive, Clone, Copy)]
#[repr(u32)]
enum PngChunkType {
    NULL = 0,
    IDAT = 0x49_44_41_54,
    IEND = 0x49_45_4E_44,
    IHDR = 0x49_48_44_52,
    PLTE = 0x50_4C_54_45,
    bKGD = 0x62_4B_47_44,
    cHRM = 0x63_48_52_4D,
    eXIf = 0x65_58_49_66, /* registered July 2017 */
    fRAc = 0x66_52_41_63, /* registered, not defined */
    gAMA = 0x67_41_4D_41,
    gIFg = 0x67_49_46_67,
    gIFt = 0x67_49_46_74, /* deprecated */
    gIFx = 0x67_49_46_78,
    hIST = 0x68_49_53_54,
    iCCP = 0x69_43_43_50,
    iTXt = 0x69_54_58_74,
    oFFs = 0x6F_46_46_73,
    pCAL = 0x70_43_41_4C,
    pHYs = 0x70_48_59_73,
    sBIT = 0x73_42_49_54,
    sCAL = 0x73_43_41_4C,
    sPLT = 0x73_50_4C_54,
    sRGB = 0x73_52_47_42,
    sTER = 0x73_54_45_52,
    tEXt = 0x74_45_58_74,
    tIME = 0x74_49_4D_45,
    tRNS = 0x74_52_4E_53,
    vpAg = 0x76_70_41_67,
    zTXt = 0x7a_54_58_74,
}

pub struct Png {
    png_ptr: CPtr,        /* Pointer to C structure */

    mode: PngMode,        /* tells us where we are in the PNG file */
    flags: PngFlags,      /* flags indicating various things to libpng */
    transformations: PngTransformations, /* which transformations to perform */

    pass: u8,        /* current interlace pass (0 - 6) */
    //compression: u8, /* file compression type (always 0) */
    interlaced: PngInterlace,
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
    chunk_name: PngChunkType, /* PNG_CHUNK() id of current chunk */
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

    sig_bytes: usize,     /* magic bytes read/written from start of file */
    maximum_pixel_depth: u8,
                          /* pixel depth used for the row buffers */
    transformed_pixel_depth: u8,
                          /* pixel depth after read/write transforms */

    info_fn: CPtr,              /* called after header data fully read */
    row_fn: CPtr,               /* called after a prog. row is decoded */
    end_fn: CPtr,               /* called after image is complete */

    save_buffer: VecDeque<u8>,    /* buffer for previously read data */
    current_buffer: VecDeque<u8>, /* buffer for recently used data */

    /* New member added in libpng-1.2.30 */
    read_buffer: CPtr,          /* buffer for reading chunk data */
    read_buffer_size: usize,    /* current size of the buffer */

    push_length: u32,           /* size of current input chunk */
    skip_length: u32,           /* bytes to skip in input data */

    buffer_size: usize,         /* total amount of available input data */

    process_mode: PngPushMode,  /* what push library is currently doing */
    cur_palette: i32,           /* current push library palette index */
    zowner: u32,                /* ID (chunk type) of zstream owner, 0 if none */
    io_ptr: CPtr,               /* ptr to application struct for I/O functions */

    mng_features_permitted: PngMng,
    filter_type: PngFilterType, /* New member added in libpng-1.0.9, ifdef'ed out in 1.0.12, enabled in 1.2.0 */

    user_width_max: u32,
    user_height_max: u32,

    /* Added in libpng-1.4.0: Total number of sPLT, text, and unknown
     * chunks that can be stored (0 means unlimited).
     */
    user_chunk_cache_max: u32,

    /* Total memory that a zTXt, sPLT, iTXt, iCCP, or unknown chunk
     * can occupy when decompressed.  0 means unlimited.
     */
    user_chunk_malloc_max: usize,
    filler: u16,                /* filler bytes for pixel expansion */
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
        png_ptr: 0,
        mode: PngMode::empty(),
        flags: PngFlags::empty(),
        transformations: PngTransformations::empty(),
        pass : 0,
        //compression : 0,
        interlaced: PngInterlace::None,
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
        chunk_name: PngChunkType::NULL,
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
        save_buffer: VecDeque::new(),
        current_buffer: VecDeque::new(),
        read_buffer: 0,
        read_buffer_size: 0,
        push_length: 0,
        skip_length: 0,
        buffer_size: 0,
        process_mode: PngPushMode::ReadSig,
        cur_palette: 0,
        zowner: 0,
        io_ptr: 0,
        mng_features_permitted: PngMng::empty(),
        filter_type: PngFilterType::Base,
        user_width_max: PNG_USER_WIDTH_MAX,
        user_height_max: PNG_USER_HEIGHT_MAX,
        user_chunk_cache_max: PNG_USER_CHUNK_CACHE_MAX,
        user_chunk_malloc_max: PNG_USER_CHUNK_MALLOC_MAX,
        filler: 0,
    });
    Box::into_raw(obj)
}

#[no_mangle]
pub unsafe extern fn png_rust_free(this: *mut Png)
{
    Box::from_raw(this);
}

#[no_mangle]
pub unsafe extern fn png_rust_set_png_ptr(this: *mut Png, png_ptr: CPtr)
{
    this.as_mut().unwrap().png_ptr = png_ptr;
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
pub unsafe extern fn png_rust_get_interlace(this: *const Png) -> u8
{
    match &this.as_ref().unwrap().interlaced {
        PngInterlace::ADAM7 => 1,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern fn png_rust_set_interlace(this: *mut Png, value: u8)
{
    this.as_mut().unwrap().interlaced = PngInterlace::from_u8(value);
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
pub unsafe extern fn png_rust_get_process_mode(this: *const Png) -> i32
{
    this.as_ref().unwrap().process_mode.to_i32().unwrap()
}

#[no_mangle]
pub unsafe extern fn png_rust_set_process_mode(this: *mut Png, mode: i32)
{
    this.as_mut().unwrap().process_mode = match PngPushMode::from_i32(mode) {
        Some(mode) => mode,
        None => {
            println!("Got an error");
            /* TODO : Check if we need to add an Unknown value */
            PngPushMode::Error
        },
    }
}


////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_rust_get_chunk_name(this: *const Png) -> u32
{
    this.as_ref().unwrap().chunk_name.to_u32().unwrap()
}

#[no_mangle]
pub unsafe extern fn png_rust_set_chunk_name(this: *mut Png, name: u32)
{
    this.as_mut().unwrap().chunk_name = match PngChunkType::from_u32(name) {
        Some(name) => name,
        None => {
            /* TODO : Check if we need to add an Unknown value or error */
            PngChunkType::IEND
        },
    }
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

////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern fn png_c_set_strip_error_numbers(this: *mut Png, strip_mode: u32)
{
    let mut strip_mode_without_errors = PngFlags::from_bits_truncate(strip_mode);
    strip_mode_without_errors.remove(PngFlags::STRIP_ERROR_NUMBERS | PngFlags::STRIP_ERROR_TEXT);
    this.as_mut().unwrap().flags.remove(strip_mode_without_errors);
}

#[no_mangle]
pub unsafe extern fn png_rust_get_save_buffer_size(this: *const Png) -> usize
{
    this.as_ref().unwrap().save_buffer.len()
}

#[no_mangle]
pub unsafe extern fn png_rust_set_mng_features_permitted(this: *mut Png, value: u8)
{
    this.as_mut().unwrap().mng_features_permitted = PngMng::from_bits_truncate(value);
}

#[no_mangle]
pub unsafe extern fn png_rust_get_mng_features_permitted(this: *const Png) -> u8
{
    this.as_ref().unwrap().mng_features_permitted.bits()
}

#[no_mangle]
pub unsafe extern fn png_rust_get_filter_type(this: *const Png) -> u8
{
    this.as_ref().unwrap().filter_type as u8
}

#[no_mangle]
pub unsafe extern fn png_rust_set_filter_type(this: *mut Png, value: u8)
{
    this.as_mut().unwrap().filter_type = PngFilterType::from_u8(value);
}


#[no_mangle]
pub unsafe extern fn png_rust_decr_user_chunk_cache_max(this: *mut Png) -> u32
{
    this.as_mut().unwrap().user_chunk_cache_max -= 1;
    this.as_mut().unwrap().user_chunk_cache_max
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
get_set!(sig_bytes,      usize);
get_set!(maximum_pixel_depth,     u8);
get_set!(transformed_pixel_depth, u8);
get_set!(info_fn,        CPtr);
get_set!(row_fn,         CPtr);
get_set!(end_fn,         CPtr);
get_set!(push_length,    u32);
get_set!(skip_length,    u32);
get_set!(buffer_size,    usize);
get_set!(cur_palette,    i32);
get_set!(zowner,         u32);
get_set!(io_ptr,         CPtr);
get_set!(user_width_max,    u32);
get_set!(user_height_max,   u32);
get_set!(user_chunk_cache_max,  u32);
get_set!(user_chunk_malloc_max, usize);
get_set!(read_buffer,      CPtr);
get_set!(read_buffer_size, usize);
get_set!(filler, u16);
