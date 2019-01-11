use crate::bitmap::Bitmap;
use crate::descriptions::{pixel_format::PixelFormatDescription, PixelFormat};
use crate::imaging_factory::ImagingFactory;
use crate::palette::Palette;

use com_wrapper::ComWrapper;
use dcommon::{Error, Status};
use math2d::Recti;
use math2d::Sizeu;
use winapi::um::wincodec::IWICBitmapSource;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(debug)]
pub struct BitmapSource {
    ptr: ComPtr<IWICBitmapSource>,
}

impl BitmapSource {
    pub fn size(&self) -> Result<Sizeu, Error> {
        unsafe {
            let (mut width, mut height) = (0, 0);
            let hr = self.ptr.GetSize(&mut width, &mut height);
            Error::map(hr, (width, height).into())
        }
    }

    pub fn pixel_format(&self) -> Result<PixelFormat, Error> {
        unsafe {
            let mut guid = std::mem::zeroed();
            let hr = self.ptr.GetPixelFormat(&mut guid);
            Error::map(hr, PixelFormat { guid })
        }
    }

    pub fn pixel_format_desc(&self) -> Option<&'static PixelFormatDescription> {
        self.pixel_format().ok().and_then(|f| f.description())
    }

    /// Gets the reported DPI of the image
    pub fn resolution(&self) -> Result<(f64, f64), Error> {
        unsafe {
            let mut dpi_x = 0.0;
            let mut dpi_y = 0.0;
            let hr = self.ptr.GetResolution(&mut dpi_x, &mut dpi_y);
            Error::map(hr, (dpi_x, dpi_y))
        }
    }

    pub fn copy_palette(&self, palette: &mut Palette) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.CopyPalette(palette.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn copy_pixels(
        &self,
        source_rect: impl Into<Recti>,
        buffer: &mut [u8],
        stride: u32,
    ) -> Result<Status, Error> {
        let rect = source_rect.into();
        let height = rect.height() as isize;
        let width = rect.width() as isize;
        let format = self.pixel_format_desc().expect(
            "Pixel Format must be determinable to use this function. If you would \
             like to copy pixels using an unsupported pixel format, you must use \
             the unsafe `unchecked_copy_pixels` instead.",
        );

        assert!(buffer.len() < std::u32::MAX as usize);
        assert!(stride as isize * height <= buffer.len() as isize);
        assert!(stride as isize >= (width * format.bits_per_pixel as isize + 7) / 8);

        unsafe { self.unchecked_copy_pixels(rect, buffer, stride) }
    }

    pub unsafe fn unchecked_copy_pixels(
        &self,
        source_rect: impl Into<Recti>,
        buffer: &mut [u8],
        stride: u32,
    ) -> Result<Status, Error> {
        let rect = source_rect.into();
        let data = buffer.as_mut_ptr();
        let len = buffer.len() as u32;
        let hr = self.ptr.CopyPixels(&rect.into(), stride, len, data);
        Error::map_status(hr)
    }

    pub fn clone_to_bitmap(&self, factory: &ImagingFactory) -> Result<Bitmap, Error> {
        Bitmap::create(factory).from_source(self).build()
    }
}
