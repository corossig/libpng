use crate::Png;
use crate::PngColor;
use crate::PngInterlace;
use crate::PngCompressionType;
use crate::PngFilterType;
use crate::png_info::PngInfo;

#[no_mangle]
pub unsafe extern fn png_rust_set_IHDR(this: *mut Png, png_info: *mut PngInfo,
                                       width: u32, height: u32, bit_depth: u8,
                                       color_type: u8, interlace_type: u8, compression_type: u8,
                                       filter_type: u8)
{
    match this.as_mut() {
        None => {
            return;
        },
        Some(png_ptr) => {
            png_ptr.set_IHDR(png_info.as_mut(), width, height, bit_depth,
                             PngColor::from_bits_truncate(color_type),
                             PngInterlace::from_u8(interlace_type),
                             PngCompressionType::from_u8(compression_type),
                             PngFilterType::from_u8(filter_type));
        },
    };
}


impl Png {
    pub fn set_IHDR(&mut self, png_info: Option<&mut PngInfo>,
                    width: u32, height: u32, bit_depth: u8,
                    color_type: PngColor, interlace_type: PngInterlace,
                    compression_type: PngCompressionType,
                    filter_type: PngFilterType) -> Result<(), &'static str>
    {
        //png_debug1(1, "in %s storage function", "IHDR");

        // Only work if PngInfo is present
        let mut png_info = match png_info {
            None => {
                return Ok(());
            },
            Some(png_info) => png_info,
        };

        png_info.width      = width;
        png_info.height     = height;
        png_info.bit_depth  = bit_depth;
        png_info.color_type = color_type;
        png_info.compression_type = compression_type;
        png_info.filter_type      = filter_type;
        png_info.interlace_type   = interlace_type;

        self.check_IHDR(png_info.width,
                        png_info.height,
                        png_info.bit_depth,
                        png_info.color_type,
                        png_info.compression_type,
                        png_info.filter_type)?;

        if png_info.color_type == PngColor::TYPE_PALETTE
        {
            png_info.channels = 1;
        } else if png_info.color_type.intersects(PngColor::MASK_COLOR) {
            png_info.channels = 3;
        } else {
            png_info.channels = 1;
        }

        if png_info.color_type.intersects(PngColor::MASK_ALPHA)
        {
            png_info.channels += 1;
        }

        png_info.pixel_depth = png_info.channels * png_info.bit_depth;

        png_info.rowbytes = Png::compute_rowbytes(png_info.pixel_depth, width as usize);

        Ok(())
    }
}
