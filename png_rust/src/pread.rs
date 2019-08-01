use crate::Png;
use crate::png_info::PngInfo;
use crate::PngTransformations;
use crate::PngMode;
use crate::PngInterlace;
use crate::PngColor;
use crate::PngFlags;
use crate::PngChunkType;
use crate::PngHandleChunk;
use crate::PngPushMode;
use crate::CPtr;
use std::slice;
use std::mem;
use std::cmp;
use std::collections::VecDeque;

extern crate num_traits;
use num_traits::FromPrimitive;
use std::ffi::CString;

extern {
    fn png_benign_error(ng_ptr: CPtr, err_msg: *const i8);
    fn png_error(err_msg: *const i8);

    fn png_chunk_unknown_handling(png_ptr: CPtr, chunk_name: PngChunkType) -> PngHandleChunk;
    fn png_handle_unknown(png_ptr: CPtr, info_ptr: CPtr, push_length: u32, keep: PngHandleChunk);
    fn png_handle_PLTE(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_gAMA(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_sBIT(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_cHRM(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_sRGB(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_iCCP(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_sPLT(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_tRNS(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_bKGD(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_hIST(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_pHYs(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_oFFs(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_pCAL(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_sCAL(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_tIME(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_tEXt(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_zTXt(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);
    fn png_handle_iTXt(png_ptr: CPtr, info_ptr: CPtr, push_length: u32);

    fn png_push_have_end(png_ptr: CPtr, png_info_ptr: CPtr);
    fn png_push_have_info(png_ptr: CPtr, png_info_ptr: CPtr);

    fn png_process_IDAT_data(png_ptr: CPtr, save_buffer_ptr: CPtr, min_size: usize);

    fn png_crc_read(png_ptr: CPtr, data: *mut u32, size: u32);
    fn png_reset_crc(png_ptr: CPtr);
    fn png_crc_finish(png_ptr: CPtr, skip: u32);
    fn png_calculate_crc(png_ptr: CPtr, save_buffer_ptr: CPtr, min_size: usize);

    fn png_push_fill_buffer_func_ptr() -> CPtr;
    fn png_set_read_fn(png_ptr: CPtr, io_ptr: CPtr, read_data_fn: CPtr);

    fn png_c_set_zstream_avail_out(png_ptr: CPtr, rowbytes: usize);
    fn png_c_set_zstream_next_out(png_ptr: CPtr, row_buf: CPtr);
}



#[no_mangle]
pub unsafe extern fn png_rust_set_progressive_read_fn(this: *mut Png,
                                                   progressive_ptr: CPtr,
                                                   info_fn: CPtr, row_fn: CPtr, end_fn: CPtr)
{
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            png_ptr.set_progressive_read_fn(progressive_ptr, info_fn, row_fn, end_fn);
        },
    }
}


#[no_mangle]
pub unsafe extern fn png_rust_push_fill_buffer(this: *mut Png, buffer: CPtr, length: usize)
{
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            let slice = slice::from_raw_parts_mut(buffer as *mut u8, length);
            png_ptr.push_fill_buffer(slice, 0);
        },
    }
}


#[no_mangle]
pub unsafe extern fn png_read_push_finish_row(this: *mut Png)
{
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            png_ptr.read_push_finish_row()
        }
    }
}


#[no_mangle]
pub unsafe extern fn png_rust_process_data_pause(this: *mut Png, save: bool) -> usize
{
    match this.as_mut() {
        None => {
            0
        },
        Some(png_ptr) => {
            png_ptr.process_data_pause(save)
        }
    }
}


#[no_mangle]
pub unsafe extern fn png_rust_process_data(this: *mut Png, png_info: *mut PngInfo, buffer: CPtr, length: usize)
{
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            let slice = slice::from_raw_parts_mut(buffer as *mut u8, length);
            png_ptr.process_data(&mut png_info.as_mut(), slice);
        }
    };
}


impl Png {
    fn push_save_buffer_if_full(&mut self) -> bool
    {
        if self.push_length + 4 > self.buffer_size as u32
        {
            self.push_save_buffer();
            return true;
        }
        false
    }

    fn process_data(&mut self, info_ptr: &mut Option<&mut PngInfo>, buffer: &[u8]) -> Result<(), &'static str>
    {
        self.push_restore_buffer(buffer);

        while self.buffer_size > 0
        {
            self.process_some_data(info_ptr.as_mut().map_or(None, |x| Some(x)));
        }

        Ok(())
    }


    fn process_data_pause(&mut self, save: bool) -> usize
    {
        /* It's easiest for the caller if we do the save; then the caller doesn't
         * have to supply the same data again:
         */
        if save
        {
            self.push_save_buffer();
        }
        else
        {
            /* This includes any pending saved bytes: */
            let remaining = self.buffer_size;
            self.buffer_size = 0;

            /* So subtract the saved buffer size, unless all the data
             * is actually 'saved', in which case we just return 0
             */
            if self.save_buffer.len() < remaining
            {
                return remaining - self.save_buffer.len();
            }
        }

        0
    }


    /* What we do with the incoming data depends on what we were previously
     * doing before we ran out of data...
     */
    fn process_some_data(&mut self, info_ptr: Option<&mut PngInfo>) -> Result<(), &'static str>
    {
        match self.process_mode {
            PngPushMode::ReadSig => {
                match info_ptr {
                    Some(info_ptr) => self.push_read_sig(info_ptr),
                    None => Err("PngInfo is mandatory for ReadSig mode"),
                }
            },
            PngPushMode::ReadChunk => {
                match info_ptr {
                    Some(info_ptr) => self.push_read_chunk(info_ptr),
                    None => Err("PngInfo is mandatory for ReadChunk mode"),
                }
            },
            PngPushMode::ReadIDAT => {
                self.push_read_IDAT()
            },
            _ => {
                self.buffer_size = 0;
                Ok(())
            }
        }
    }

    /* Read any remaining signature bytes from the stream and compare them with
     * the correct PNG signature.  It is possible that this routine is called
     * with bytes already read from the signature, either because they have been
     * checked by the calling application, or because of multiple calls to this
     * routine.
     */
    pub fn push_read_sig(&mut self, info_ptr: &mut PngInfo) -> Result<(), &'static str>
    {
        assert!(self.sig_bytes <= 8);
        self.sig_bytes = self.push_fill_buffer(&mut info_ptr.signature, self.sig_bytes);

        match Png::sig_cmp(&info_ptr.signature, self.sig_bytes) {
            /* At least one byte doesn't match */
            Some(first_unmatch) => {
                if first_unmatch >= 4 {
                    return Err("Not a PNG file");
                } else {
                    return Err("PNG file corrupted by ASCII conversion");
                }
            },
            /* Everything is valid but need to verify for completeness */
            None => {
                if self.sig_bytes >= 8 {
                    self.process_mode = PngPushMode::ReadChunk;
                }
            },
        };

        Ok(())
    }


    fn push_read_chunk(&mut self, info_ptr: &mut PngInfo) -> Result<(), &'static str>
    {
        /* First we make sure we have enough data for the 4-byte chunk name
         * and the 4-byte chunk length before proceeding with decoding the
         * chunk data.  To fully decode each of these chunks, we also make
         * sure we have enough data in the buffer for the 4-byte CRC at the
         * end of every chunk (except IDAT, which is handled separately).
         */
        if ! self.mode.contains(PngMode::HAVE_CHUNK_HEADER)
        {
            if self.buffer_size < 8
            {
                self.push_save_buffer();
                return Ok(());
            }

            self.push_length = u32::from_be(self.push_get_integer());

            let mut chunk_tag = 0 as u32;
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

            // TODO proper check with return result
            //png_check_chunk_name(png_ptr, png_rust_get_chunk_name(png_ptr->rust_ptr));
            //png_check_chunk_length(png_ptr, png_rust_get_push_length(png_ptr->rust_ptr));
            self.mode.insert(PngMode::HAVE_CHUNK_HEADER);
        }

        let chunk_name = self.chunk_name;
        let keep: PngHandleChunk = unsafe {png_chunk_unknown_handling(self.png_ptr, chunk_name)};

        if chunk_name == PngChunkType::IDAT
        {
            if self.mode.contains(PngMode::AFTER_IDAT)
            {
                self.mode.insert(PngMode::HAVE_CHUNK_AFTER_IDAT);
            }

            /* If we reach an IDAT chunk, this means we have read all of the
             * header chunks, and we can start reading the image (or if this
             * is called after the image has been read - we have an error).
             */
            if ! self.mode.contains(PngMode::HAVE_IHDR)
            {
                return Err("Missing IHDR before IDAT");
            }
            else if self.color_type == PngColor::TYPE_PALETTE &&
                ! self.mode.contains(PngMode::HAVE_PLTE)
            {
                return Err("Missing PLTE before IDAT");
            }

            self.process_mode = PngPushMode::ReadIDAT;

            if self.mode.contains(PngMode::HAVE_IDAT) &&
                ! self.mode.contains(PngMode::HAVE_CHUNK_AFTER_IDAT) &&
                self.push_length == 0
            {
                return Ok(());
            }

            self.mode.insert(PngMode::HAVE_IDAT);

            if self.mode.contains(PngMode::AFTER_IDAT)
            {
                let c_str = CString::new("Too many IDATs found").unwrap();
                unsafe {png_benign_error(self.png_ptr, c_str.as_ptr() as *const i8);}
            }
        }

        if chunk_name == PngChunkType::IHDR
        {
            if self.push_length != 13
            {
                return Err("Invalid IHDR length");
            }

            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            self.handle_IHDR(info_ptr, self.push_length);

        } else if chunk_name == PngChunkType::IEND {

            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            self.handle_IEND(self.push_length);

            self.process_mode = PngPushMode::ReadDONE;
            unsafe {png_push_have_end(self.png_ptr, info_ptr.png_info);}

        } else if keep != PngHandleChunk::AsDefault {

            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_unknown(self.png_ptr, info_ptr.png_info, self.push_length, keep);}

            if chunk_name == PngChunkType::PLTE
            {
                self.mode.insert(PngMode::HAVE_PLTE);
            }
        }

        else if chunk_name == PngChunkType::PLTE
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_PLTE(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::IDAT
        {
            self.idat_size = self.push_length;
            self.process_mode = PngPushMode::ReadIDAT;
            unsafe {
                png_push_have_info(self.png_ptr, info_ptr.png_info);
                png_c_set_zstream_avail_out(self.png_ptr, Png::compute_rowbytes(self.pixel_depth, self.iwidth as usize) + 1 as usize);
                png_c_set_zstream_next_out(self.png_ptr, self.row_buf);
            }
            return Ok(());
        }

        else if self.chunk_name == PngChunkType::gAMA
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_gAMA(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if self.chunk_name == PngChunkType::sBIT
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_sBIT(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if self.chunk_name == PngChunkType::cHRM
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_cHRM(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::sRGB
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_sRGB(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if self.chunk_name == PngChunkType::iCCP
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_iCCP(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::sPLT
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_sPLT(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::tRNS
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_tRNS(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::bKGD
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_bKGD(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::hIST
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_hIST(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::pHYs
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_pHYs(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::oFFs
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_oFFs(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::pCAL
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_pCAL(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::sCAL
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_sCAL(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::tIME
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_tIME(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::tEXt
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_tEXt(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::zTXt
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_zTXt(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else if chunk_name == PngChunkType::iTXt
        {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_iTXt(self.png_ptr, info_ptr.png_info, self.push_length);}
        }

        else {
            if self.push_save_buffer_if_full()
            {
                return Ok(());
            }
            unsafe {png_handle_unknown(self.png_ptr, info_ptr.png_info, self.push_length,
                               PngHandleChunk::AsDefault);}
        }

        self.mode.remove(PngMode::HAVE_CHUNK_HEADER);

        Ok(())
    }

    fn push_fill_buffer(&mut self, buffer: &mut [u8], start_index: usize) -> usize
    {
        let mut current_index = start_index;

        let mut drain_buffer = |deque_buffer: &mut VecDeque<u8>| -> usize {
            if ! deque_buffer.is_empty()
            {
                let save_size = cmp::min(buffer.len() - current_index, deque_buffer.len());

                for (i, x) in deque_buffer.drain(..save_size).enumerate() {
                    buffer[current_index + i] = x;
                }

                current_index += save_size;
                return save_size;
            }
            0
        };

        // First empty save_buffer then current buffer
        self.buffer_size -= drain_buffer(&mut self.save_buffer);
        self.buffer_size -= drain_buffer(&mut self.current_buffer);
        assert!(self.buffer_size == (self.current_buffer.len() + self.save_buffer.len()));

        return current_index;
    }


    fn push_get_integer(&mut self) -> u32
    {
        let mut buffer_int = 0 as u32;
        let slice = unsafe { slice::from_raw_parts_mut(&mut buffer_int as *mut u32 as *mut u8, mem::size_of_val(&buffer_int))} ;
        self.push_fill_buffer(slice, 0);
        buffer_int
    }


    fn push_read_IDAT(&mut self) -> Result<(), &'static str>
    {
        if ! self.mode.contains(PngMode::HAVE_CHUNK_HEADER)
        {
            /* TODO: this code can be commoned up with the same code in push_read */
            if self.buffer_size < 8
            {
                self.push_save_buffer();
                return Ok(());
            }

            self.push_length = u32::from_be(self.push_get_integer());

            let mut chunk_tag = 0 as u32;
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
        if self.idat_size != 0 && ! self.save_buffer.is_empty()
        {
            let min_size = cmp::min(self.save_buffer.len(), self.idat_size as usize);

            // TODO remove memory allocation when calculate_crc became rust code
            let mut data: Vec<_> = self.save_buffer.drain(..min_size).collect();

            unsafe {
                png_calculate_crc(self.png_ptr, data.as_mut_ptr() as CPtr, data.len());
                png_process_IDAT_data(self.png_ptr, data.as_mut_ptr() as CPtr, data.len());
            }

            self.idat_size -= min_size as u32;
            self.buffer_size -= min_size;
        }

        if self.idat_size != 0 && ! self.current_buffer.is_empty()
        {
            let min_size = cmp::min(self.current_buffer.len(), self.idat_size as usize);

            // TODO remove memory allocation when calculate_crc became rust code
            let mut data: Vec<_> = self.current_buffer.drain(..min_size).collect();

            unsafe {
                png_calculate_crc(self.png_ptr, data.as_mut_ptr() as CPtr, data.len());
                png_process_IDAT_data(self.png_ptr, data.as_mut_ptr() as CPtr, data.len());
            }

            self.idat_size -= min_size as u32;
            self.buffer_size -= min_size;
        }

        if self.idat_size == 0
        {
            if self.buffer_size < 4
            {
                self.push_save_buffer();
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
            PngInterlace::None => (),
            PngInterlace::ADAM7 => {
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


    fn push_save_buffer(&mut self)
    {
        // Move current buffer to save one
        if ! self.current_buffer.is_empty()
        {
            self.save_buffer.extend(&self.current_buffer);
            self.current_buffer.clear();
        }
        self.buffer_size = 0;//self.current_buffer.len() + self.save_buffer.len();
    }

    // Ok, they gave us a CPtr and size (we don't want copy nor allocate)
    fn push_restore_buffer(&mut self, buffer: &[u8])
    {
        self.current_buffer.clear();
        self.current_buffer.extend(buffer.into_iter());
        self.buffer_size = self.current_buffer.len() + self.save_buffer.len();
    }

    fn set_progressive_read_fn(&mut self, progressive_ptr: CPtr,
                               info_fn: CPtr, row_fn: CPtr, end_fn: CPtr)
    {
        self.info_fn = info_fn;
        self.row_fn  = row_fn;
        self.end_fn  = end_fn;

        unsafe {png_set_read_fn(self.png_ptr, progressive_ptr, png_push_fill_buffer_func_ptr());}
    }
}
