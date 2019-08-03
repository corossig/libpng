use crate::Png;
use crate::PngColor;
use crate::PngCompressionType;
use crate::PngFilterType;
use crate::PngMode;
use crate::PngMng;
use crate::CPtr;
use std::ffi::CString;

extern {
    fn png_warning(png_ptr: CPtr, err_msg: *const i8);
}

impl Png {
    pub fn warning(&self, msg: &'static str)
    {
        let c_str = CString::new(msg).unwrap();
        unsafe {png_warning(self.png_ptr, c_str.as_ptr() as *const i8);}
    }

    pub fn check_IHDR(&self, width: u32, height: u32, bit_depth: u8,
                      color_type: PngColor,
                      compression_type: PngCompressionType,
                      filter_type: PngFilterType) -> Result<(), &'static str>
    {
        let mut error = false;

        /* Check for width and height valid values */
        if width == 0
        {
            self.warning("Image width is zero in IHDR");
            error = true;
        }

        if width > std::i32::MAX as u32
        {
            self.warning("Invalid image width in IHDR");
            error = true;
        }

        /* TODO : WTF !!!! */
        let a = (width + 7) & (!7);
        let b = ((std::usize::MAX
                  - 48        /* big_row_buf hack */
                  - 1)        /* filter byte */
                 / 8)        /* 8-byte RGBA pixels */
            - 1;    /* extra max_pixel_depth pad */
        if a as usize > b
        {
            /* The size of the row must be within the limits of this architecture.
             * Because the read code can perform arbitrary transformations the
             * maximum size is checked here.  Because the code in png_read_start_row
             * adds extra space "for safety's sake" in several places a conservative
             * limit is used here.
             *
             * NOTE: it would be far better to check the size that is actually used,
             * but the effect in the real world is minor and the changes are more
             * extensive, therefore much more dangerous and much more difficult to
             * write in a way that avoids compiler warnings.
             */
            self.warning("Image width is too large for this architecture");
            error = true;
        }

        if width > self.user_width_max
        {
            self.warning("Image width exceeds user limit in IHDR");
            error = true;
        }

        if height == 0
        {
            self.warning("Image height is zero in IHDR");
            error = true;
        }

        if height > std::i32::MAX as u32
        {
            self.warning("Invalid image height in IHDR");
            error = true;
        }

        if height > self.user_height_max
        {
            self.warning("Image height exceeds user limit in IHDR");
            error = true;
        }

        /* Check other values */
        if bit_depth != 1 && bit_depth != 2 && bit_depth != 4 &&
            bit_depth != 8 && bit_depth != 16
        {
            self.warning("Invalid bit depth in IHDR");
            error = true;
        }

        let color_type_int = color_type.bits();
        if color_type_int == 1 || color_type_int == 5 || color_type_int > 6
        {
            self.warning("Invalid color type in IHDR");
            error = true;
        }

        if ((color_type == PngColor::MASK_PALETTE) && bit_depth > 8) ||
            ((color_type == PngColor::TYPE_RGB ||
              color_type == PngColor::TYPE_GRAY_ALPHA ||
              color_type == PngColor::TYPE_RGB_ALPHA) && bit_depth < 8)
        {
            self.warning("Invalid color type/bit depth combination in IHDR");
            error = true;
        }

        if compression_type != PngCompressionType::Base
        {
            self.warning("Unknown compression method in IHDR");
            error = true;
        }

        /* Accept filter_method 64 (intrapixel differencing) only if
         * 1. Libpng was compiled with PNG_MNG_FEATURES_SUPPORTED and
         * 2. Libpng did not read a PNG signature (this filter_method is only
         *    used in PNG datastreams that are embedded in MNG datastreams) and
         * 3. The application called png_permit_mng_features with a mask that
         *    included PNG_FLAG_MNG_FILTER_64 and
         * 4. The filter_method is 64 and
         * 5. The color_type is RGB or RGBA
         */
        if self.mode.contains(PngMode::HAVE_PNG_SIGNATURE) &&
            ! self.mng_features_permitted.is_empty()
        {
            self.warning("MNG features are not allowed in a PNG datastream");
        }

        if filter_type != PngFilterType::Base
        {
            if ! (self.mng_features_permitted.contains(PngMng::Filter64) &&
                  (filter_type == PngFilterType::Differencing) &&
                  ! self.mode.contains(PngMode::HAVE_PNG_SIGNATURE) &&
                  (color_type == PngColor::TYPE_RGB ||
                   color_type == PngColor::TYPE_RGB_ALPHA))
            {
                self.warning("Unknown filter method in IHDR");
                error = true;
            }

            if self.mode.contains(PngMode::HAVE_PNG_SIGNATURE)
            {
                self.warning("Invalid filter method in IHDR");
                error = true;
            }
        }

        if error
        {
            return Err("Invalid IHDR data");
        }

        Ok(())
    }

    pub fn sig_cmp(signature: &[u8], num_to_check: usize) -> Option<usize>
    {
        let png_signature = [137, 80, 78, 71, 13, 10, 26, 10];
        assert!(num_to_check <= png_signature.len());

        // If signature parameter isn't big enough
        if signature.len() < png_signature.len()
        {
            return Some(0);
        }

        // Look byte by byte and return on first different
        for i in 0..num_to_check
        {
            if signature[i] != png_signature[i]
            {
                return Some(i);
            }
        }

        None
    }
}
