use crate::bitmap::Bitmap;
use crate::descriptions::PixelFormat;
use crate::imaging_factory::ImagingFactory;

use com_wrapper::ComWrapper;
use dcommon::Error;
use math2d::Sizeu;

pub struct MemoryBuilder<'a> {
    factory: &'a ImagingFactory,

    size: Option<Sizeu>,
    format: Option<PixelFormat>,
    data: Option<(&'a [u8], u32)>,
}

impl<'a> MemoryBuilder<'a> {
    pub(super) fn new(factory: &'a ImagingFactory) -> Self {
        MemoryBuilder {
            factory,
            size: None,
            format: None,
            data: None,
        }
    }

    pub fn build(self) -> Result<Bitmap, Error> {
        let size = self.size.expect("`size` must be specified");
        let format = self.format.expect("`format` must be specified");
        let (data, stride) = self.data.expect("`data` must be specified");
        unsafe {
            let factory = &*self.factory.get_raw();

            let mut ptr = std::ptr::null_mut();
            let hr = factory.CreateBitmapFromMemory(
                size.width,
                size.height,
                &format.guid,
                stride,
                data.len() as u32,
                data.as_ptr(),
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

    pub fn with_data(self, data: &'a [u8], stride: u32) -> Self {
        let size = self.size.expect("Please specify `size` before `data`");
        let format = self.format.expect("Please specify `format` before `data`");
        let fdesc = format.description().expect(
            "Unknown pixel format. If you would like to use this pixel format, \
             use `with_data_unchecked` instead and validate that the buffer size \
             is correct manually.",
        );

        assert!(data.len() < std::u32::MAX as usize);
        assert!(data.len() >= stride as usize * size.height as usize);
        assert!(stride as usize >= (size.width as usize * fdesc.bits_per_pixel as usize + 7) / 8);

        unsafe { self.with_data_unchecked(data, stride) }
    }

    pub unsafe fn with_data_unchecked(mut self, data: &'a [u8], stride: u32) -> Self {
        self.data = Some((data, stride));
        self
    }
}
