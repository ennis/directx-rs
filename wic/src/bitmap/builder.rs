use crate::bitmap_source::BitmapSource;
use crate::imaging_factory::ImagingFactory;

pub mod empty;
pub mod memory;
pub mod source;

pub struct BitmapBuilder<'a> {
    factory: &'a ImagingFactory,
}

impl<'a> BitmapBuilder<'a> {
    pub(super) fn new(factory: &'a ImagingFactory) -> Self {
        BitmapBuilder { factory }
    }

    pub fn empty(self) -> empty::EmptyBuilder<'a> {
        empty::EmptyBuilder::new(self.factory)
    }

    pub fn from_source(self, source: &'a BitmapSource) -> source::SourceBuilder<'a> {
        source::SourceBuilder::new(self.factory, source)
    }

    pub fn from_memory(self) -> memory::MemoryBuilder<'a> {
        memory::MemoryBuilder::new(self.factory)
    }
}
