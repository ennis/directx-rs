use crate::bitmap_lock::BitmapLock;
use crate::bitmap_source::BitmapSource;
use crate::enums::BitmapLockFlags;
use crate::imaging_factory::ImagingFactory;
use crate::palette::Palette;

use com_wrapper::ComWrapper;
use dcommon::{Error, Status};
use math2d::Recti;
use winapi::um::wincodec::IWICBitmap;
use wio::com::ComPtr;

pub use self::builder::BitmapBuilder;

pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Bitmap {
    ptr: ComPtr<IWICBitmap>,
}

impl Bitmap {
    pub fn create(factory: &ImagingFactory) -> BitmapBuilder {
        BitmapBuilder::new(factory)
    }

    pub fn lock(
        &self,
        rect: impl Into<Recti>,
        flags: BitmapLockFlags,
    ) -> Result<BitmapLock, Error> {
        unsafe {
            let rect = rect.into();
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.Lock(&rect.into(), flags.0, &mut ptr);
            Error::map_if(hr, || BitmapLock::from_raw(ptr))
        }
    }

    pub fn set_palette(&mut self, palette: Palette) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.SetPalette(palette.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn set_resolution(&mut self, dpi_x: f64, dpi_y: f64) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.SetResolution(dpi_x, dpi_y);
            Error::map_status(hr)
        }
    }

    pub fn try_clone(&self, factory: &ImagingFactory) -> Result<Self, Error> {
        Bitmap::create(factory)
            .from_source(self)
            .build()
    }
}

impl std::ops::Deref for Bitmap {
    type Target = BitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { dcommon::helpers::deref_com_wrapper(self) }
    }
}
