use crate::Png;
use crate::PngTransformations;
use crate::PngMode;
use crate::PngFlags;
use crate::PngChunkType;
use crate::PngPushMode;
use crate::CPtr;
use std::slice;
use std::mem;
use std::cmp;

extern crate num_traits;
use num_traits::FromPrimitive;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern fn png_read_push_finish_row(this: *mut Png)
{
    this.as_mut().unwrap().read_push_finish_row()
}

#[no_mangle]
pub unsafe extern fn png_push_read_IDAT(this: *mut Png)
{
    match this.as_mut().unwrap().push_read_IDAT() {
        Ok(_) => (),
        Err(msg) => {
            let msg_str = CString::new(msg).unwrap();
            png_error(msg_str.as_ptr())
        },
    }
}

extern {
    fn png_error(err_msg: *const i8);

    fn png_push_save_buffer(png_ptr: CPtr);
    fn png_push_fill_buffer(png_ptr: CPtr, data: *mut u32, size: usize);

    fn png_process_IDAT_data(png_ptr: CPtr, save_buffer_ptr: CPtr, min_size: usize);

    fn png_crc_read(png_ptr: CPtr, data: *mut u32, size: u32);
    fn png_reset_crc(png_ptr: CPtr);
    fn png_crc_finish(png_ptr: CPtr, skip: u32);
    fn png_calculate_crc(png_ptr: CPtr, save_buffer_ptr: CPtr, min_size: usize);
}

impl Png {
    #![allow(non_snake_case)]
    fn push_read_IDAT(&mut self) -> Result<(), &'static str>
    {
        if ! self.mode.contains(PngMode::HAVE_CHUNK_HEADER)
        {
            let mut chunk_length = 0 as u32;
            let mut chunk_tag    = 0 as u32;

            /* TODO: this code can be commoned up with the same code in push_read */
            if self.buffer_size < 8
            {
                unsafe {
                    png_push_save_buffer(self.png_ptr);
                }
                return Ok(());
            }

            unsafe {
                png_push_fill_buffer(self.png_ptr, &mut chunk_length as *mut u32, mem::size_of_val(&chunk_length));
            }
            self.push_length = u32::from_be(chunk_length);

            unsafe {
                png_reset_crc(self.png_ptr);
                png_crc_read(self.png_ptr, &mut chunk_tag as *mut u32, mem::size_of_val(&chunk_tag) as u32);
            }
            self.chunk_name = match PngChunkType::from_u32(u32::from_be(chunk_tag)) {
                Some(chunk_name) => {
                    chunk_name
                },
                None => {
                    return Err("Invalid chunk type");
                },
            };

            self.mode.insert(PngMode::HAVE_CHUNK_HEADER);

            /* If not a IDAT, just return */
            if self.chunk_name != PngChunkType::IDAT
            {
                self.process_mode = PngPushMode::ReadChunk;

                if ! self.flags.contains(PngFlags::ZSTREAM_ENDED)
                {
                   return Err("Not enough compressed data");
                }

                return Ok(());
            }

            self.idat_size = self.push_length;
        }

        /* We want the smaller of 'idat_size' and '{save,current}_buffer_size', but they
         * are of different types and we don't know which variable has the fewest
         * bits.  Carefully select the smaller and cast it to the type of the
         * larger - this cannot overflow.
         */
        if self.idat_size != 0 && self.save_buffer_size != 0
        {
            let min_size = cmp::min(self.save_buffer_size, self.idat_size as usize);

            unsafe {
                png_calculate_crc(self.png_ptr, self.save_buffer_ptr, min_size);
                png_process_IDAT_data(self.png_ptr, self.save_buffer_ptr, min_size);
            }

            self.idat_size -= min_size as u32;
            self.buffer_size -= min_size;
            self.save_buffer_size -= min_size;
            self.save_buffer_ptr += min_size;
        }

        if self.idat_size != 0 && self.current_buffer_size != 0
        {
            let min_size = cmp::min(self.current_buffer_size, self.idat_size as usize);

            unsafe {
                png_calculate_crc(self.png_ptr, self.current_buffer_ptr, min_size);
                png_process_IDAT_data(self.png_ptr, self.current_buffer_ptr, min_size);
            }

            self.idat_size -= min_size as u32;
            self.buffer_size -= min_size;
            self.current_buffer_size -= min_size;
            self.current_buffer_ptr += min_size;
        }

        if self.idat_size == 0
        {
            if self.buffer_size < 4
            {
                unsafe {
                    png_push_save_buffer(self.png_ptr);
                }
                return Ok(());
            }
            
            unsafe {
                png_crc_finish(self.png_ptr, 0);
            }
            self.mode.remove(PngMode::HAVE_CHUNK_HEADER);
            self.mode.insert(PngMode::AFTER_IDAT);
            self.zowner = 0;
        }

        Ok(())
    }


    fn read_push_finish_row(&mut self)
    {
        /* Arrays to facilitate easy interlacing - use pass (0 - 6) as index */

        /* Start of interlace block */
        static PNG_PASS_START: &'static [u8] = &[0, 4, 0, 2, 0, 1, 0];

        /* Offset to next interlace block */
        static PNG_PASS_INC: &'static [u8] = &[8, 8, 4, 4, 2, 2, 1];

        /* Start of interlace block in the y direction */
        static PNG_PASS_YSTART: &'static [u8] = &[0, 0, 4, 0, 2, 0, 1];

        /* Offset to next interlace block in the y direction */
        static PNG_PASS_YINC: &'static [u8] = &[8, 8, 8, 4, 4, 2, 2];

        /* Height of interlace block.  This is not currently used - if you need it
        static png_pass_height: &'static [u8] = &[8, 8, 4, 4, 2, 2, 1];
         */

        self.row_number += 1;
        if self.row_number < self.num_rows
        {
            return;
        }

        match self.interlaced {
            None => (),
            Some(_) => {
                self.row_number = 0;

                unsafe {
                    let prev_row: &mut [u8] = slice::from_raw_parts_mut(self.prev_row as *mut u8, self.rowbytes + 1);
                    for i in prev_row.iter_mut() { *i = 0 }
                }

                loop {
                    self.pass += 1;
                    if  self.pass <= 7 &&
                        ((self.pass == 1 && self.width < 5) ||
                         (self.pass == 3 && self.width < 3) ||
                         (self.pass == 5 && self.width < 2))
                    {
                        self.pass += 1;
                    }

                    if self.pass >= 7
                    {
                        break;
                    }
                    let pass_idx = self.pass as usize;


                    let x_index = u32::from(PNG_PASS_INC[pass_idx] - PNG_PASS_START[pass_idx] - 1);
                    self.iwidth = (self.width + x_index) / u32::from(PNG_PASS_INC[pass_idx]);

                    if self.transformations.contains(PngTransformations::INTERLACE)
                    {
                        break;
                    }

                    let y_index = u32::from(PNG_PASS_YINC[pass_idx] - PNG_PASS_YSTART[pass_idx] - 1);
                    self.num_rows = (self.height + y_index) / u32::from(PNG_PASS_YINC[pass_idx]);

                    if self.iwidth != 0 && self.num_rows != 0
                    {
                        break;
                    }
                }
            }
        }
    }
}
