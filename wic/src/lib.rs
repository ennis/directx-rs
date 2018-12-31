#[macro_use]
extern crate derive_com_wrapper;

#[macro_use]
extern crate auto_enum;

pub use crate::{
    bitmap::Bitmap, bitmap_clipper::BitmapClipper, bitmap_decoder::BitmapDecoder,
    bitmap_lock::BitmapLock, bitmap_source::BitmapSource, imaging_factory::ImagingFactory,
    palette::Palette, stream::Stream, metadata_query_reader::MetadataQueryReader,
};

pub use dcommon::GUID;

pub mod bitmap;
pub mod bitmap_clipper;
pub mod bitmap_decoder;
pub mod bitmap_lock;
pub mod bitmap_source;
pub mod descriptions;
pub mod enums;
pub mod imaging_factory;
pub mod palette;
pub mod metadata_query_reader;
pub mod stream;
