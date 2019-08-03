use crate::Png;
use crate::png_info::PngInfo;
use crate::PngMode;
use crate::PngColor;
use crate::PngInterlace;
use crate::PngCompressionType;
use crate::PngFilterType;
use crate::CPtr;
use std::ffi::CString;

extern {
    //fn png_chunk_benign_error(ng_ptr: CPtr, err_msg: *const i8);
    fn png_chunk_error(ng_ptr: CPtr, err_msg: *const i8);

    fn png_crc_read(png_ptr: CPtr, data: *mut u32, size: u32);
    fn png_crc_finish(png_ptr: CPtr, skip: u32);
}

#[no_mangle]
pub unsafe extern fn png_rust_handle_IHDR(this: *mut Png, info_ptr: *mut PngInfo, length: u32)
{
    let info_ptr = match info_ptr.as_mut() {
        None => {
            return;
        },
        Some(info_ptr) => info_ptr,
    };

    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            png_ptr.handle_IHDR(info_ptr, length);
        }
    };
}


#[allow(clippy::cast_ptr_alignment)]
impl Png {

    pub fn compute_rowbytes(pixel_depth: u8, width: usize) -> usize
    {
        if pixel_depth >= 8 {
            width * (pixel_depth as usize >> 3)
        } else{
            ((width * pixel_depth as usize) + 7) >> 3
        }
    }

    pub fn handle_IHDR(&mut self, info_ptr: &mut PngInfo, length: u32)
    {
     /*   int bit_depth, color_type, compression_type, filter_type;
        int interlace_type;
*/
        //png_debug(1, "in png_handle_IHDR");

        if self.mode.contains(PngMode::HAVE_IHDR)
        {
            let c_str = CString::new("out of place").unwrap();
            unsafe {png_chunk_error(self.png_ptr, c_str.as_ptr() as *const i8);}
        }

        /* Check the length */
        if length != 13
        {
            let c_str = CString::new("invalid").unwrap();
            unsafe {png_chunk_error(self.png_ptr, c_str.as_ptr() as *const i8);}
        }

        self.mode.insert(PngMode::HAVE_IHDR);

        let mut buf = [0 as u8; 13];
        unsafe {
            png_crc_read(self.png_ptr, buf.as_mut_ptr() as *mut u32, buf.len() as u32);
            png_crc_finish(self.png_ptr, 0);
        }

        let mut width_array  = [0 as u8; 4];
        let mut height_array = [0 as u8; 4];
        width_array.copy_from_slice(&buf[0..4]);
        height_array.copy_from_slice(&buf[4..8]);

        let width  = unsafe { std::mem::transmute::<[u8;4], u32>(width_array)  }.to_be();
        let height = unsafe { std::mem::transmute::<[u8;4], u32>(height_array) }.to_be();
        let bit_depth = buf[8];
        let color_type = PngColor::from_bits_truncate(buf[9]);
        let compression_type = PngCompressionType::from_u8(buf[10]);
        let filter_type = PngFilterType::from_u8(buf[11]);
        let interlace_type = PngInterlace::from_u8(buf[12]);

        /* Set internal variables */
        self.width = width;
        self.height = height;
        self.bit_depth = bit_depth;
        self.interlaced = interlace_type;
        self.color_type = color_type;
        //png_ptr->filter_type = (png_byte)filter_type;
        //png_ptr->compression_type = (png_byte)compression_type;

        /* Find number of channels */
        self.channels = match self.color_type {
            PngColor::TYPE_RGB => 3,
            PngColor::TYPE_GRAY_ALPHA => 2,
            PngColor::TYPE_RGB_ALPHA => 4,
            /* invalid or pallette, in case of invalid png_set_IHDR calls png_error */
            _ => 1,
        };

        /* Set up other useful info */
        self.pixel_depth = self.bit_depth * self.channels;
        self.rowbytes = Png::compute_rowbytes(self.pixel_depth, self.width as usize);
        //png_debug1(3, "bit_depth = %d", png_rust_get_bit_depth(png_ptr->rust_ptr));
        //png_debug1(3, "channels = %d", png_rust_get_channels(png_ptr->rust_ptr));
        //png_debug1(3, "rowbytes = %lu", (unsigned long)png_rust_get_rowbytes(png_ptr->rust_ptr));
        self.set_IHDR(Some(info_ptr),
                      width, height, bit_depth,
                      color_type, interlace_type,
                      compression_type, filter_type);
    }
}
