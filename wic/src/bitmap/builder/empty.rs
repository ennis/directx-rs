use crate::bitmap::Bitmap;
use crate::descriptions::PixelFormat;
use crate::enums::BitmapCreateCacheOption;
use crate::imaging_factory::ImagingFactory;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Sizeu;

/// Intermediate builder for an empty bitmap
pub struct EmptyBuilder<'a> {
    factory: &'a ImagingFactory,

    size: Option<Sizeu>,
    format: Option<PixelFormat>,
    cache: BitmapCreateCacheOption,
}

impl<'a> EmptyBuilder<'a> {
    pub(super) fn new(factory: &'a ImagingFactory) -> Self {
        EmptyBuilder {
            factory,
            size: None,
            format: None,
            cache: BitmapCreateCacheOption::NoCache,
        }
    }

    pub fn build(self) -> Result<Bitmap, Error> {
        let size = self.size.expect("`size` must be specified");
        let format = self.format.expect("`format` must be specified");
        unsafe {
            let factory = &*self.factory.get_raw();

            let mut ptr = std::ptr::null_mut();
            let hr = factory.CreateBitmap(
                size.width,
                size.height,
                &format.guid,
                self.cache as u32,
                &mut ptr,
            );

            Error::map_if(hr, || Bitmap::from_raw(ptr))
        }
    }

    pub fn with_size(mut self, size: impl Into<Sizeu>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn with_format(mut self, format: PixelFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn with_cache_option(mut self, option: BitmapCreateCacheOption) -> Self {
        self.cache = option;
        self
    }
}
