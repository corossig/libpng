use crate::Png;
use crate::PngTransformations;
use crate::PngInterlace;
use crate::PngMode;
use crate::PngColor;
use crate::PngFlags;
use crate::CPtr;
use std::slice;

/* This is used for the transformation routines, as some of them
 * change these values for the row.  It also should enable using
 * the routines for other purposes.
 */
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct PngRowInfo {
    width: u32,           /* width of row */
    rowbytes: usize,      /* number of bytes in row */
    color_type: PngColor, /* color type of row */
    bit_depth: u8,        /* bit depth of row */
    channels: u8,         /* number of channels (1, 2, 3, or 4) */
    pixel_depth: u8,      /* bits per pixel (depth * channels) */
}

/*******************************************************************************
 *
 *                               Png C API
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern fn png_c_set_filler(this: *mut Png, filler: u32, filler_loc: i32)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => { png_ptr.set_filler(filler as u16, filler_loc); },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_bgr(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => { png_ptr.transformations.insert(PngTransformations::BGR); },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_swap_alpha(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => {
            png_ptr.transformations.insert(PngTransformations::SWAP_ALPHA);
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_invert_alpha(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => {
            png_ptr.transformations.insert(PngTransformations::INVERT_ALPHA);
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_invert_mono(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => {
            png_ptr.transformations.insert(PngTransformations::INVERT_MONO);
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_swap(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => {
            if png_ptr.bit_depth == 16 {
                png_ptr.transformations.insert(PngTransformations::SWAP_BYTES);
            }
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_packing(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => {
            if png_ptr.bit_depth < 8 {
                png_ptr.transformations.insert(PngTransformations::PACK);
                png_ptr.usr_bit_depth = 8;
            }
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_pack_swap(this: *mut Png)
{
    match this.as_mut() {
        None => { return; },
        Some(png_ptr) => {
            if png_ptr.bit_depth < 8 {
                png_ptr.transformations.insert(PngTransformations::PACKSWAP);
            }
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_c_set_interlace_handling(this: *mut Png) -> i32
{
    match this.as_mut() {
        None => { return 1; },
        Some(png_ptr) => {
            if png_ptr.interlaced != PngInterlace::None {
                png_ptr.transformations.insert(PngTransformations::INTERLACE);
                return 7;
            }
            return 1;
        },
    }
}



/*******************************************************************************
 *
 *                               Png RUST
 *
 ******************************************************************************/


#[no_mangle]
pub unsafe extern fn png_do_bgr(this: *mut PngRowInfo, row: CPtr) {
    match this.as_ref() {
        None => {
            return;
        },
        Some(row_info) => {
            let slice = slice::from_raw_parts_mut(row as *mut u8, row_info.rowbytes);
            row_info.do_bgr(slice);
        },
    }
}


#[no_mangle]
pub unsafe extern fn png_do_packswap(this: *const PngRowInfo, row: CPtr) {
    match this.as_ref() {
        None => {
            return;
        },
        Some(row_info) => {
            let slice = slice::from_raw_parts_mut(row as *mut u8, row_info.rowbytes);
            row_info.do_packswap(slice);
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_do_invert(this: *const PngRowInfo, row: CPtr) {
    match this.as_ref() {
        None => { return; },
        Some(row_info) => {
            let slice = slice::from_raw_parts_mut(row as *mut u8, row_info.rowbytes);
            row_info.do_invert(slice);
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_do_strip_channel(this: *mut PngRowInfo, row: CPtr, at_start: i32) {
    match this.as_mut() {
        None => { return; },
        Some(row_info) => {
            let slice = slice::from_raw_parts_mut(row as *mut u8, row_info.rowbytes);
            row_info.do_strip_channel(slice, match at_start { 0 => false, _ => true });
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_do_check_palette_indexes(this: *mut Png, row_info: *const PngRowInfo) {
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            match row_info.as_ref() {
                None => { return; },
                Some(row_info) => { png_ptr.do_check_palette_indexes(row_info); },
            }
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_do_swap(this: *const PngRowInfo, row: CPtr) {
    match this.as_ref() {
        None => { return; },
        Some(row_info) => {
            let slice = slice::from_raw_parts_mut(row as *mut u8, row_info.rowbytes);
            row_info.do_swap(slice);
        },
    }
}

#[no_mangle]
pub unsafe extern fn png_rust_get_row_info(this: *const Png, row_info: *mut PngRowInfo) {
    match this.as_ref() {
        None => { return; },
        Some(png_ptr) => {
            match row_info.as_mut() {
                None => { return; },
                Some(row_info) => { *row_info = png_ptr.to_row_info(); },
            }
        },
    }
}

static onebppswaptable: &'static [u8] = &[
    0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0,
    0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0,
    0x08, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8,
    0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8,
    0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4,
    0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4,
    0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC,
    0x1C, 0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC,
    0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2,
    0x12, 0x92, 0x52, 0xD2, 0x32, 0xB2, 0x72, 0xF2,
    0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA,
    0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA,
    0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6,
    0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6,
    0x0E, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE,
    0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE,
    0x01, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61, 0xE1,
    0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1,
    0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9,
    0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9,
    0x05, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5,
    0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5,
    0x0D, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED,
    0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD,
    0x03, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3,
    0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3,
    0x0B, 0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB,
    0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB,
    0x07, 0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7,
    0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7,
    0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF,
    0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF
];

static twobppswaptable: &'static [u8] = &[
    0x00, 0x40, 0x80, 0xC0, 0x10, 0x50, 0x90, 0xD0,
    0x20, 0x60, 0xA0, 0xE0, 0x30, 0x70, 0xB0, 0xF0,
    0x04, 0x44, 0x84, 0xC4, 0x14, 0x54, 0x94, 0xD4,
    0x24, 0x64, 0xA4, 0xE4, 0x34, 0x74, 0xB4, 0xF4,
    0x08, 0x48, 0x88, 0xC8, 0x18, 0x58, 0x98, 0xD8,
    0x28, 0x68, 0xA8, 0xE8, 0x38, 0x78, 0xB8, 0xF8,
    0x0C, 0x4C, 0x8C, 0xCC, 0x1C, 0x5C, 0x9C, 0xDC,
    0x2C, 0x6C, 0xAC, 0xEC, 0x3C, 0x7C, 0xBC, 0xFC,
    0x01, 0x41, 0x81, 0xC1, 0x11, 0x51, 0x91, 0xD1,
    0x21, 0x61, 0xA1, 0xE1, 0x31, 0x71, 0xB1, 0xF1,
    0x05, 0x45, 0x85, 0xC5, 0x15, 0x55, 0x95, 0xD5,
    0x25, 0x65, 0xA5, 0xE5, 0x35, 0x75, 0xB5, 0xF5,
    0x09, 0x49, 0x89, 0xC9, 0x19, 0x59, 0x99, 0xD9,
    0x29, 0x69, 0xA9, 0xE9, 0x39, 0x79, 0xB9, 0xF9,
    0x0D, 0x4D, 0x8D, 0xCD, 0x1D, 0x5D, 0x9D, 0xDD,
    0x2D, 0x6D, 0xAD, 0xED, 0x3D, 0x7D, 0xBD, 0xFD,
    0x02, 0x42, 0x82, 0xC2, 0x12, 0x52, 0x92, 0xD2,
    0x22, 0x62, 0xA2, 0xE2, 0x32, 0x72, 0xB2, 0xF2,
    0x06, 0x46, 0x86, 0xC6, 0x16, 0x56, 0x96, 0xD6,
    0x26, 0x66, 0xA6, 0xE6, 0x36, 0x76, 0xB6, 0xF6,
    0x0A, 0x4A, 0x8A, 0xCA, 0x1A, 0x5A, 0x9A, 0xDA,
    0x2A, 0x6A, 0xAA, 0xEA, 0x3A, 0x7A, 0xBA, 0xFA,
    0x0E, 0x4E, 0x8E, 0xCE, 0x1E, 0x5E, 0x9E, 0xDE,
    0x2E, 0x6E, 0xAE, 0xEE, 0x3E, 0x7E, 0xBE, 0xFE,
    0x03, 0x43, 0x83, 0xC3, 0x13, 0x53, 0x93, 0xD3,
    0x23, 0x63, 0xA3, 0xE3, 0x33, 0x73, 0xB3, 0xF3,
    0x07, 0x47, 0x87, 0xC7, 0x17, 0x57, 0x97, 0xD7,
    0x27, 0x67, 0xA7, 0xE7, 0x37, 0x77, 0xB7, 0xF7,
    0x0B, 0x4B, 0x8B, 0xCB, 0x1B, 0x5B, 0x9B, 0xDB,
    0x2B, 0x6B, 0xAB, 0xEB, 0x3B, 0x7B, 0xBB, 0xFB,
    0x0F, 0x4F, 0x8F, 0xCF, 0x1F, 0x5F, 0x9F, 0xDF,
    0x2F, 0x6F, 0xAF, 0xEF, 0x3F, 0x7F, 0xBF, 0xFF
];

static fourbppswaptable: &'static [u8] = &[
    0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
    0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0,
    0x01, 0x11, 0x21, 0x31, 0x41, 0x51, 0x61, 0x71,
    0x81, 0x91, 0xA1, 0xB1, 0xC1, 0xD1, 0xE1, 0xF1,
    0x02, 0x12, 0x22, 0x32, 0x42, 0x52, 0x62, 0x72,
    0x82, 0x92, 0xA2, 0xB2, 0xC2, 0xD2, 0xE2, 0xF2,
    0x03, 0x13, 0x23, 0x33, 0x43, 0x53, 0x63, 0x73,
    0x83, 0x93, 0xA3, 0xB3, 0xC3, 0xD3, 0xE3, 0xF3,
    0x04, 0x14, 0x24, 0x34, 0x44, 0x54, 0x64, 0x74,
    0x84, 0x94, 0xA4, 0xB4, 0xC4, 0xD4, 0xE4, 0xF4,
    0x05, 0x15, 0x25, 0x35, 0x45, 0x55, 0x65, 0x75,
    0x85, 0x95, 0xA5, 0xB5, 0xC5, 0xD5, 0xE5, 0xF5,
    0x06, 0x16, 0x26, 0x36, 0x46, 0x56, 0x66, 0x76,
    0x86, 0x96, 0xA6, 0xB6, 0xC6, 0xD6, 0xE6, 0xF6,
    0x07, 0x17, 0x27, 0x37, 0x47, 0x57, 0x67, 0x77,
    0x87, 0x97, 0xA7, 0xB7, 0xC7, 0xD7, 0xE7, 0xF7,
    0x08, 0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78,
    0x88, 0x98, 0xA8, 0xB8, 0xC8, 0xD8, 0xE8, 0xF8,
    0x09, 0x19, 0x29, 0x39, 0x49, 0x59, 0x69, 0x79,
    0x89, 0x99, 0xA9, 0xB9, 0xC9, 0xD9, 0xE9, 0xF9,
    0x0A, 0x1A, 0x2A, 0x3A, 0x4A, 0x5A, 0x6A, 0x7A,
    0x8A, 0x9A, 0xAA, 0xBA, 0xCA, 0xDA, 0xEA, 0xFA,
    0x0B, 0x1B, 0x2B, 0x3B, 0x4B, 0x5B, 0x6B, 0x7B,
    0x8B, 0x9B, 0xAB, 0xBB, 0xCB, 0xDB, 0xEB, 0xFB,
    0x0C, 0x1C, 0x2C, 0x3C, 0x4C, 0x5C, 0x6C, 0x7C,
    0x8C, 0x9C, 0xAC, 0xBC, 0xCC, 0xDC, 0xEC, 0xFC,
    0x0D, 0x1D, 0x2D, 0x3D, 0x4D, 0x5D, 0x6D, 0x7D,
    0x8D, 0x9D, 0xAD, 0xBD, 0xCD, 0xDD, 0xED, 0xFD,
    0x0E, 0x1E, 0x2E, 0x3E, 0x4E, 0x5E, 0x6E, 0x7E,
    0x8E, 0x9E, 0xAE, 0xBE, 0xCE, 0xDE, 0xEE, 0xFE,
    0x0F, 0x1F, 0x2F, 0x3F, 0x4F, 0x5F, 0x6F, 0x7F,
    0x8F, 0x9F, 0xAF, 0xBF, 0xCF, 0xDF, 0xEF, 0xFF
];


impl Png {
    fn to_row_info(&self) -> PngRowInfo
    {
        PngRowInfo {
            width: self.iwidth, /* NOTE: width of current interlaced row */
            color_type: self.color_type,
            bit_depth: self.bit_depth,
            channels: self.channels,
            pixel_depth: self.pixel_depth,
            rowbytes: Png::compute_rowbytes(self.pixel_depth, self.iwidth as usize),
        }
    }

    /* Add a filler byte on read, or remove a filler or alpha byte on write.
     * The filler type has changed in v0.95 to allow future 2-byte fillers
     * for 48-bit input data, as well as to avoid problems with some compilers
     * that don't like bytes as parameters.
     */
    fn set_filler(&mut self, filler: u16, filler_loc: i32)
    {
        /* In libpng 1.6 it is possible to determine whether this is a read or write
         * operation and therefore to do more checking here for a valid call.
         */
        if self.mode.contains(PngMode::IS_READ_STRUCT)
        {
            /* On read png_set_filler is always valid, regardless of the base PNG
             * format, because other transformations can give a format where the
             * filler code can execute (basically an 8 or 16-bit component RGB or G
             * format.)
             *
             * NOTE: usr_channels is not used by the read code!  (This has led to
             * confusion in the past.)  The filler is only used in the read code.
             */
            self.filler = filler;
        }
        else /* write */
        {
            /* On write the usr_channels parameter must be set correctly at the
             * start to record the number of channels in the app-supplied data.
             */
            match self.color_type {
                PngColor::TYPE_RGB => {
                    self.usr_channels = 4;
                },
                PngColor::TYPE_GRAY => {
                    if self.bit_depth >= 8
                    {
                        self.usr_channels = 2;
                    }
                    else
                    {
                        /* There simply isn't any code in libpng to strip out bits
                         * from bytes when the components are less than a byte in
                         * size!
                         */
                        // TODO
                        //png_app_error(png_ptr,
                        //              "png_set_filler is invalid for"
                        //              " low bit depth gray output");
                        return;
                    }
                },
                _ => {
                    // TODO
                    //png_app_error(png_ptr,
                    //              "png_set_filler: inappropriate color type");
                    return;
                },
            };
        }

        /* Here on success - libpng supports the operation, set the transformation
         * and the flag to say where the filler channel is.
         */
        self.transformations.insert(PngTransformations::FILLER);

        if filler_loc == 1 /* AFTER */
        {
            self.flags.insert(PngFlags::FILLER_AFTER);
        }
        else
        {
            self.flags.remove(PngFlags::FILLER_AFTER);
        }
    }

    fn do_check_palette_indexes(&mut self, row_info: &PngRowInfo)
    {
        if self.num_palette == 0 || self.num_palette >= (1 << row_info.bit_depth)
        {
            /* num_palette can be 0 in MNG files */
            return;
        }

        // This only works because row_info.pixel_depth is multiple of 2
        let trailling_bits = (row_info.width % (8 / row_info.pixel_depth as u32)) * row_info.pixel_depth as u32;
        let padding = 8 - trailling_bits;
        let row_buf = unsafe { slice::from_raw_parts_mut(self.row_buf as *mut u8, self.rowbytes) };

        match row_info.bit_depth {
            1 => {
                /* in this case, all bytes must be 0 so we don't need
                 * to unpack the pixels except for the rightmost one.
                 */
                self.num_palette_max = 0;
                for byte_value in row_buf[..self.rowbytes - 1].iter() {
                    if *byte_value != 0
                    {
                        self.num_palette_max = 1;
                        break;
                    }
                }

                // Last byte is special (partial bits validity)
                if self.num_palette_max != 1 && row_buf[self.rowbytes - 1] >> padding != 0
                {
                    self.num_palette_max = 1;
                }
            },

            2 | 4 | 8 => {
                let mask = (1 << row_info.bit_depth) - 1;
                self.num_palette_max = 0;

                // For each byte (except the latest)
                for byte_value in row_buf[..self.rowbytes - 1].iter()  {

                    // For each channel value inside on byte
                    for j in num_iter::range_step(0, 8, row_info.bit_depth) {
                        // Channel value
                        let value = ((*byte_value >> j) & mask) as i32 ;

                        if value > self.num_palette_max {
                            self.num_palette_max = value;

                            // Can't be bigger than mask
                            if self.num_palette_max == mask as i32 {
                                break;
                            }
                        }
                    }
                }

                // Last byte is special
                if self.num_palette_max != mask as i32 {
                }
            },

            _ => {
                return;
            }
        };
    }
}

impl PngRowInfo {
    /* Invert monochrome grayscale data */
    fn do_invert(&self, row: &mut [u8])
    {
        assert!(self.rowbytes <= row.len());

        match self.color_type {
            PngColor::TYPE_GRAY => {
                /* This test removed from libpng version 1.0.13 and 1.2.0:
                 *   if (self->bit_depth == 1 &&
                 */
                for value in row[..self.rowbytes].iter_mut() {
                    *value = ! *value;
                }
            },
            PngColor::TYPE_GRAY_ALPHA => {
                match self.bit_depth {
                    8 => {
                        // Only invert gray, not alpha
                        for i in num_iter::range_step(0, self.rowbytes, 2) {
                            row[i] = ! row[i];
                        }
                    },
                    16 => {
                        // Consider element of 16bits
                        let row_16: &mut [u16] = unsafe { std::mem::transmute::<&mut [u8], &mut [u16]>(row) };
                        let nb_element = self.rowbytes/2;
                        for i in num_iter::range_step(0, nb_element, 2) {
                            row_16[i] = ! row_16[i];
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                };
            },
            _ => {
                // No gray, do nothing
            },
        };
    }

    /* Swaps byte order on 16-bit depth images */
    fn do_swap(&self, row: &mut [u8])
    {
        if self.bit_depth != 16
        {
            return;
        }

        // Consider element of 16bits
        let row_16: &mut [u16] = unsafe { std::mem::transmute::<&mut [u8], &mut [u16]>(row) };
        let nb_u16 = (self.width * self.channels as u32) as usize;
        assert!(nb_u16 <= row_16.len());

        for value in row_16[..nb_u16].iter_mut() {
            *value = (*value).swap_bytes();
        }
    }

    /* Swaps pixel packing order within bytes */
    fn do_packswap(&self, row: &mut [u8])
    {
        assert!(self.rowbytes <= row.len());

        let table = match self.bit_depth {
            1 => &onebppswaptable,
            2 => &twobppswaptable,
            4 => &fourbppswaptable,
            _ => {
                return;
            }
        };

        for element in row[..self.rowbytes].iter_mut() {
            *element = table[*element as usize];
        }
    }

    /* Swaps red and blue bytes within a pixel */
    fn do_bgr(&self, row: &mut [u8])
    {
        if ! self.color_type.contains(PngColor::MASK_COLOR)
        {
            return;
        }

        let nb_composant = match self.color_type {
            PngColor::TYPE_RGB => 3,
            PngColor::TYPE_RGB_ALPHA => 4,
            _ => {
                return;
            }
        };
        let nb_element = nb_composant * self.width as usize;
        assert!(nb_element * self.bit_depth as usize/8 <= row.len());

        match self.bit_depth {
            8 => {
                for i in num_iter::range_step(0, nb_element, nb_composant) {
                    row.swap(i, i+2);
                }
            },

            16 => {
                // Consider element of 16bits
                let row_16: &mut [u16] = unsafe { std::mem::transmute::<&mut [u8], &mut [u16]>(row) };

                for i in num_iter::range_step(0, nb_element, nb_composant) {
                    row_16.swap(i, i+2);
                }
            },
            _ => {
                return;
            }
        }
    }


    /* Remove a channel - this used to be 'png_do_strip_filler' but it used a
     * somewhat weird combination of flags to determine what to do.  All the calls
     * to png_do_strip_filler are changed in 1.5.2 to call this instead with the
     * correct arguments.
     *
     * The routine isn't general - the channel must be the channel at the start or
     * end (not in the middle) of each pixel.
     */
    fn do_strip_channel(&mut self, row: &mut [u8], remove_first: bool)
    {
        /* At the start sp will point to the first byte to copy and dp to where
         * it is copied to.  ep always points just beyond the end of the row, so
         * the loop simply copies (channels-1) channels until sp reaches ep.
         *
         * at_start:        0 -- convert AG, XG, ARGB, XRGB, AAGG, XXGG, etc.
         *            nonzero -- convert GA, GX, RGBA, RGBX, GGAA, RRGGBBXX, etc.
         */

        /* Fix the color type if it records an alpha channel */
        match self.channels {
            2 => {
                /* GA, GX, XG cases */
                if self.color_type == PngColor::TYPE_GRAY_ALPHA
                {
                    self.color_type = PngColor::TYPE_GRAY;
                }
            },
            4 => {
                /* RGBX, RRGGBBXX ... cases */
                if self.color_type == PngColor::TYPE_RGB_ALPHA
                {
                    self.color_type = PngColor::TYPE_RGB;
                }
            },
            _ => {
                /* The filler channel has gone already */
                return;
            }
        }

        match self.bit_depth {
            8 => {
                let nb_pixels = self.rowbytes / self.channels as usize;

                let mut source_idx = 0;
                let mut dest_idx   = match remove_first { true => 1, false => 0, };
                for _i in 0..nb_pixels {
                    for _j in 0..self.channels - 1 {
                        row[dest_idx] = row[source_idx];
                        source_idx += 1;
                        dest_idx   += 1;
                    }

                    // Ignore one channel per pixel
                    source_idx += 1;
                }

                // Update internal information
                self.channels -= 1;
                self.rowbytes = nb_pixels * self.channels as usize;
                self.pixel_depth = self.bit_depth * self.channels;
            },
            16 => {
                // Consider element of 16bits
                let row_16: &mut [u16] = unsafe { std::mem::transmute::<&mut [u8], &mut [u16]>(row) };
                let nb_pixels = self.rowbytes / (2 * self.channels as usize);

                let mut source_idx = 0;
                let mut dest_idx   = match remove_first { true => 1, false => 0, };
                for _i in 0..nb_pixels {
                    for _j in 0..self.channels - 1 {
                        row_16[dest_idx] = row_16[source_idx];
                        source_idx += 1;
                        dest_idx   += 1;
                    }

                    // Ignore one channel per pixel
                    source_idx += 1;
                }

                // Update internal information
                self.channels -= 1;
                self.rowbytes = 2 * nb_pixels * self.channels as usize;
                self.pixel_depth = self.bit_depth * self.channels;
            },
            _ => {
                assert!(false);
            }
        }
    }
}
