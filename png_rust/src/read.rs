use crate::Png;
use crate::PngMode;
use crate::CPtr;
use std::ffi::CString;

extern {
    fn png_chunk_benign_error(ng_ptr: CPtr, err_msg: *const i8);
    fn png_chunk_error(ng_ptr: CPtr, err_msg: *const i8);
    fn png_crc_finish(png_ptr: CPtr, skip: u32);
}

#[no_mangle]
pub unsafe extern fn png_rust_handle_IEND(this: *mut Png, length: u32)
{
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            png_ptr.handle_IEND(length);
        },
    }
}


impl Png {

    pub fn handle_IEND(&mut self, length: u32)
    {
        //png_debug(1, "in png_handle_IEND");

        if ! self.mode.contains(PngMode::HAVE_IHDR) ||
            ! self.mode.contains(PngMode::HAVE_IDAT)
        {
            let c_str = CString::new("out of place").unwrap();
            unsafe {png_chunk_error(self.png_ptr, c_str.as_ptr() as *const i8);}
        }

        self.mode.insert(PngMode::AFTER_IDAT | PngMode::HAVE_IEND);

        unsafe {png_crc_finish(self.png_ptr, length);}

        if length != 0
        {
            let c_str = CString::new("invalid").unwrap();
            unsafe {png_chunk_benign_error(self.png_ptr, c_str.as_ptr() as *const i8);}
        }
    }
}
