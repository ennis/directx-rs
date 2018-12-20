use crate::bitmap::Bitmap;
use crate::bitmap_source::BitmapSource;
use crate::enums::BitmapCreateCacheOption;
use crate::imaging_factory::ImagingFactory;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Recti;

pub struct SourceBuilder<'a> {
    factory: &'a ImagingFactory,

    source: &'a BitmapSource,
    cache: Option<BitmapCreateCacheOption>,
    rect: Option<Recti>,
}

impl<'a> SourceBuilder<'a> {
    pub(super) fn new(factory: &'a ImagingFactory, source: &'a BitmapSource) -> Self {
        SourceBuilder {
            factory,

            source: source,
            cache: None,
            rect: None,
        }
    }

    pub fn build(self) -> Result<Bitmap, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let factory = &*self.factory.get_raw();
            let hr = match (self.cache, self.rect) {
                (Some(_), Some(_)) => panic!("`rect` and `cache_option` cannot both be specified"),
                (None, Some(rect)) => factory.CreateBitmapFromSourceRect(
                    self.source.get_raw(),
                    rect.left as u32,
                    rect.top as u32,
                    rect.width() as u32,
                    rect.height() as u32,
                    &mut ptr,
                ),
                (cache, None) => factory.CreateBitmapFromSource(
                    self.source.get_raw(),
                    cache.unwrap_or(BitmapCreateCacheOption::NoCache) as u32,
                    &mut ptr,
                ),
            };
            Error::map_if(hr, || Bitmap::from_raw(ptr))
        }
    }

    pub fn with_cache_option(mut self, option: BitmapCreateCacheOption) -> Self {
        self.cache = Some(option);
        self
    }

    pub fn with_rect(mut self, rect: impl Into<Recti>) -> Self {
        let rect = rect.into();
        assert!(
            rect.left >= 0 && rect.top >= 0 && rect.width() > 0 && rect.height() > 0,
            "`rect` must be positive"
        );
        self.rect = Some(rect);
        self
    }
}
