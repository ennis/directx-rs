use crate::bitmap_source::BitmapSource;
use crate::descriptions::{Color, ColorType};
use crate::enums::BitmapPaletteType;
use crate::imaging_factory::ImagingFactory;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::{Error, Status};
use winapi::um::wincodec::IWICPalette;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Palette {
    ptr: ComPtr<IWICPalette>,
}

impl Palette {
    pub fn create(factory: &ImagingFactory) -> Result<Palette, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreatePalette(&mut ptr);
            Error::map_if(hr, || Palette::from_raw(ptr))
        }
    }

    pub fn initialize_predefined(
        &mut self,
        palette: BitmapPaletteType,
        add_transparent: bool,
    ) -> Result<Status, Error> {
        unsafe {
            let palette = palette as u32;
            let add_transparent = add_transparent as i32;
            let hr = self.ptr.InitializePredefined(palette, add_transparent);
            Error::map_status(hr)
        }
    }

    pub fn initialize_custom(&mut self, colors: &[impl ColorType]) -> Result<Status, Error> {
        unsafe {
            let ptr = colors.as_ptr() as _;
            let len = colors.len() as u32;
            let hr = self.ptr.InitializeCustom(ptr, len);
            Error::map_status(hr)
        }
    }

    pub fn initialize_from_bitmap(
        &mut self,
        bitmap: &BitmapSource,
        count: u32,
        add_transparent: bool,
    ) -> Result<Status, Error> {
        unsafe {
            let bitmap = bitmap.get_raw();
            let atr = add_transparent as i32;
            let hr = self.ptr.InitializeFromBitmap(bitmap, count, atr);
            Error::map_status(hr)
        }
    }

    pub fn initialize_from_palette(&mut self, palette: &Palette) -> Result<Status, Error> {
        unsafe {
            let hr = self.ptr.InitializeFromPalette(palette.get_raw());
            Error::map_status(hr)
        }
    }

    pub fn palette_type(&self) -> Result<UncheckedEnum<BitmapPaletteType>, Error> {
        unsafe {
            let mut palette_type = 0;
            let hr = self.ptr.GetType(&mut palette_type);
            Error::map(hr, palette_type.into())
        }
    }

    pub fn color_count(&self) -> Result<u32, Error> {
        unsafe {
            let mut count = 0;
            let hr = self.ptr.GetColorCount(&mut count);
            Error::map(hr, count)
        }
    }

    pub fn colors(&self) -> Result<Vec<Color>, Error> {
        unsafe {
            let count = self.color_count()?;
            let mut actual = 0;
            let mut buf = vec![Color { value: 0 }; count as usize];
            let hr = self
                .ptr
                .GetColors(count, buf.as_mut_ptr() as _, &mut actual);
            if actual != count {
                return Err(Error::WIC_VALUEOUTOFRANGE);
            }

            Error::map(hr, buf)
        }
    }

    pub fn is_black_and_white(&self) -> Result<bool, Error> {
        unsafe {
            let mut is = 0;
            let hr = self.ptr.IsBlackWhite(&mut is);
            Error::map(hr, is != 0)
        }
    }

    pub fn is_grayscale(&self) -> Result<bool, Error> {
        unsafe {
            let mut is = 0;
            let hr = self.ptr.IsGrayscale(&mut is);
            Error::map(hr, is != 0)
        }
    }

    pub fn has_alpha(&self) -> Result<bool, Error> {
        unsafe {
            let mut is = 0;
            let hr = self.ptr.HasAlpha(&mut is);
            Error::map(hr, is != 0)
        }
    }

    pub fn try_clone(&self, factory: &ImagingFactory) -> Result<Palette, Error> {
        let mut new = Palette::create(factory)?;
        new.initialize_from_palette(self)?;
        Ok(new)
    }
}
