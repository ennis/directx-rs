use self::BitsPerChannel::*;
use self::Category as C;
use self::StorageType::*;

use dxgi::enums::Format::*;
use winapi::shared::guiddef::{IsEqualGUID, GUID};
use winapi::um::wincodec::*;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PixelFormat {
    pub guid: GUID,
}

impl PixelFormat {
    pub fn description(&self) -> Option<&'static PixelFormatDescription> {
        NATIVE_PIXEL_FORMATS
            .iter()
            .find(|(guid, _)| IsEqualGUID(guid, &self.guid))
            .map(|(_, desc)| desc)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PixelFormatDescription {
    pub label: &'static str,
    pub description: &'static str,
    pub channel_count: u16,
    pub bits_per_channel: BitsPerChannel,
    pub bits_per_pixel: u16,
    pub storage_type: StorageType,
    pub category: Category,
    pub dxgi_format: dxgi::enums::Format,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BitsPerChannel {
    Uniform(u16),
    PerChannel(u16, u16, u16, u16),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StorageType {
    Unsigned,
    Float,
    Fixed,
}

#[enum_flags]
pub enum Category {
    RGB,
    BGR,
    GRAYSCALE,
    ALPHA,
    PACKED,
    INDEXED,
    PREMULTIPLIED,
    CMYK,
    N_CHANNEL,

    NONE = 0,
}

pub static NATIVE_PIXEL_FORMATS: &[(GUID, PixelFormatDescription)] = &[
    // Unknown/Don't Care
    (
        GUID_WICPixelFormatDontCare,
        define_desc(
            "GUID_WICPixelFormatDontCare",
            "Format Unknown/Don't Care",
            0,
            0,
            0,
            Unsigned,
            C::NONE,
            Unknown,
        ),
    ),
    // Indexed Pixel Formats
    (
        GUID_WICPixelFormat1bppIndexed,
        define_desc(
            "GUID_WICPixelFormat1bppIndexed",
            "Palette Indexed 1bpp",
            1,
            1,
            1,
            Unsigned,
            C::INDEXED,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat2bppIndexed,
        define_desc(
            "GUID_WICPixelFormat2bppIndexed",
            "Palette Indexed 2bpp",
            1,
            2,
            2,
            Unsigned,
            C::INDEXED,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat4bppIndexed,
        define_desc(
            "GUID_WICPixelFormat4bppIndexed",
            "Palette Indexed 4bpp",
            1,
            4,
            4,
            Unsigned,
            C::INDEXED,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat8bppIndexed,
        define_desc(
            "GUID_WICPixelFormat8bppIndexed",
            "Palette Indexed 8bpp",
            1,
            8,
            8,
            Unsigned,
            C::INDEXED,
            Unknown,
        ),
    ),
    // Packed Bit Pixel Formats
    (
        GUID_WICPixelFormat16bppBGR555,
        define_desc(
            "GUID_WICPixelFormat16bppBGR555",
            "Packed 16bpp BGR 5,5,5",
            3,
            5,
            16,
            Unsigned,
            C(C::PACKED.0 | C::BGR.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat16bppBGR565,
        define_desc_p(
            "GUID_WICPixelFormat16bppBGR565",
            "Packed 16bpp BGR 5,6,5",
            3,
            5,
            6,
            5,
            0,
            16,
            Unsigned,
            C(C::PACKED.0 | C::BGR.0),
            B5G6R5Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat16bppBGRA5551,
        define_desc_p(
            "GUID_WICPixelFormat16bppBGRA5551",
            "Packed 16bpp BGRA 5,5,5,1",
            4,
            5,
            5,
            5,
            1,
            16,
            Unsigned,
            C(C::PACKED.0 | C::BGR.0 | C::ALPHA.0),
            B5G5R5A1Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat32bppBGR101010,
        define_desc(
            "GUID_WICPixelFormat32bppBGR101010",
            "Packed 32bpp BGR 10,10,10",
            3,
            10,
            32,
            Unsigned,
            C(C::PACKED.0 | C::BGR.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat32bppRGBA1010102,
        define_desc_p(
            "GUID_WICPixelFormat32bppRGBA1010102",
            "Packed 32bpp RGBA 10,10,10,2",
            3,
            10,
            10,
            10,
            2,
            32,
            Unsigned,
            C(C::PACKED.0 | C::RGB.0 | C::ALPHA.0),
            R10G10B10A2Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat32bppRGBA1010102XR,
        define_desc_p(
            "GUID_WICPixelFormat32bppRGBA1010102XR",
            "Packed 32bpp RGBA 10,10,10,2 XR",
            3,
            10,
            10,
            10,
            2,
            32,
            Unsigned,
            C(C::PACKED.0 | C::RGB.0 | C::ALPHA.0),
            Unknown,
        ),
    ),
    // Grayscale Pixel Formats
    (
        GUID_WICPixelFormatBlackWhite,
        define_desc(
            "GUID_WICPixelFormatBlackWhite",
            "Black and White 1bpp",
            1,
            1,
            1,
            Unsigned,
            C::GRAYSCALE,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat2bppGray,
        define_desc(
            "GUID_WICPixelFormat2bppGray",
            "Grayscale 2bpp",
            1,
            2,
            2,
            Unsigned,
            C::GRAYSCALE,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat4bppGray,
        define_desc(
            "GUID_WICPixelFormat4bppGray",
            "Grayscale 4bpp",
            1,
            4,
            4,
            Unsigned,
            C::GRAYSCALE,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat8bppGray,
        define_desc(
            "GUID_WICPixelFormat8bppGray",
            "Grayscale 8bpp",
            1,
            8,
            8,
            Unsigned,
            C::GRAYSCALE,
            R8Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat16bppGray,
        define_desc(
            "GUID_WICPixelFormat16bppGray",
            "Grayscale 16bpp",
            1,
            16,
            16,
            Unsigned,
            C::GRAYSCALE,
            R16Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat16bppGrayFixedPoint,
        define_desc(
            "GUID_WICPixelFormat16bppGrayFixedPoint",
            "Grayscale 16bpp Fixed-Point",
            1,
            16,
            16,
            Fixed,
            C::GRAYSCALE,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat16bppGrayHalf,
        define_desc(
            "GUID_WICPixelFormat16bppGrayHalf",
            "Grayscale 16bpp Half-Float",
            1,
            16,
            16,
            Float,
            C::GRAYSCALE,
            R16Float,
        ),
    ),
    (
        GUID_WICPixelFormat32bppGrayFloat,
        define_desc(
            "GUID_WICPixelFormat32bppGrayFloat",
            "Grayscale 32bpp Float",
            1,
            32,
            32,
            Float,
            C::GRAYSCALE,
            R32Float,
        ),
    ),
    (
        GUID_WICPixelFormat32bppGrayFixedPoint,
        define_desc(
            "GUID_WICPixelFormat32bppGrayFixedPoint",
            "Grayscale 32bpp Fixed-Point",
            1,
            32,
            32,
            Fixed,
            C::GRAYSCALE,
            Unknown,
        ),
    ),
    // Standard RGB/BGR Pixel Formats
    (
        GUID_WICPixelFormat24bppRGB,
        define_desc(
            "GUID_WICPixelFormat24bppRGB",
            "RGB 24bpp",
            3,
            8,
            24,
            Unsigned,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat24bppBGR,
        define_desc(
            "GUID_WICPixelFormat24bppBGR",
            "BGR 24bpp",
            3,
            8,
            24,
            Unsigned,
            C::BGR,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat32bppBGR,
        define_desc(
            "GUID_WICPixelFormat32bppBGR",
            "BGR 32bpp",
            3,
            8,
            32,
            Unsigned,
            C::BGR,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat32bppRGBA,
        define_desc(
            "GUID_WICPixelFormat32bppRGBA",
            "RGBA 32bpp",
            4,
            8,
            32,
            Unsigned,
            C(C::RGB.0 | C::ALPHA.0),
            R8G8B8A8Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat32bppBGRA,
        define_desc(
            "GUID_WICPixelFormat32bppBGRA",
            "BGRA 32bpp",
            4,
            8,
            32,
            Unsigned,
            C(C::BGR.0 | C::ALPHA.0),
            B8G8R8A8Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat32bppRGBE,
        define_desc(
            "GUID_WICPixelFormat32bppRGBE",
            "RGB Packed Half-Float (Shared Exponent) 32bpp",
            4,
            8,
            32,
            Float,
            C(C::RGB.0 | C::PACKED.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat32bppPRGBA,
        define_desc(
            "GUID_WICPixelFormat32bppPRGBA",
            "RGBA 32bpp Premultiplied Alpha",
            4,
            8,
            32,
            Unsigned,
            C(C::RGB.0 | C::ALPHA.0 | C::PREMULTIPLIED.0),
            R8G8B8A8Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat32bppPBGRA,
        define_desc(
            "GUID_WICPixelFormat32bppPBGRA",
            "BGRA 32bpp Premultiplied Alpha",
            4,
            8,
            32,
            Unsigned,
            C(C::BGR.0 | C::ALPHA.0 | C::PREMULTIPLIED.0),
            B8G8R8A8Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat48bppRGB,
        define_desc(
            "GUID_WICPixelFormat48bppRGB",
            "RGB 48bpp",
            3,
            16,
            48,
            Unsigned,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat48bppBGR,
        define_desc(
            "GUID_WICPixelFormat48bppBGR",
            "BGR 48bpp",
            3,
            16,
            48,
            Unsigned,
            C::BGR,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat48bppRGBFixedPoint,
        define_desc(
            "GUID_WICPixelFormat48bppRGBFixedPoint",
            "RGB 48bpp Fixed-Point",
            3,
            16,
            48,
            Fixed,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat48bppBGRFixedPoint,
        define_desc(
            "GUID_WICPixelFormat48bppBGRFixedPoint",
            "BGR 48bpp Fixed-Point",
            3,
            16,
            48,
            Fixed,
            C::BGR,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat48bppRGBHalf,
        define_desc(
            "GUID_WICPixelFormat48bppRGBHalf",
            "RGB 48bpp Half-Float",
            3,
            16,
            48,
            Float,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppRGBA,
        define_desc(
            "GUID_WICPixelFormat64bppRGBA",
            "RGBA 64bpp",
            4,
            16,
            64,
            Unsigned,
            C(C::RGB.0 | C::ALPHA.0),
            R16G16B16A16Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat64bppBGRA,
        define_desc(
            "GUID_WICPixelFormat64bppBGRA",
            "BGRA 64bpp",
            4,
            16,
            64,
            Unsigned,
            C(C::BGR.0 | C::ALPHA.0),
            R16G16B16A16Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat64bppPRGBA,
        define_desc(
            "GUID_WICPixelFormat64bppPRGBA",
            "RGBA 64bpp Premultiplied Alpha",
            4,
            16,
            64,
            Unsigned,
            C(C::RGB.0 | C::ALPHA.0 | C::PREMULTIPLIED.0),
            R16G16B16A16Unorm,
        ),
    ),
    (
        GUID_WICPixelFormat64bppPBGRA,
        define_desc(
            "GUID_WICPixelFormat64bppPBGRA",
            "BGRA 64bpp",
            4,
            16,
            64,
            Unsigned,
            C(C::BGR.0 | C::ALPHA.0 | C::PREMULTIPLIED.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppRGBFixedPoint,
        define_desc(
            "GUID_WICPixelFormat64bppRGBFixedPoint",
            "RGB 64bpp Fixed-Point",
            3,
            16,
            64,
            Fixed,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppRGBAFixedPoint,
        define_desc(
            "GUID_WICPixelFormat64bppRGBAFixedPoint",
            "RGBA 64bpp Fixed-Point",
            4,
            16,
            64,
            Fixed,
            C(C::RGB.0 | C::ALPHA.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppBGRAFixedPoint,
        define_desc(
            "GUID_WICPixelFormat64bppBGRAFixedPoint",
            "BGRA 64bpp Fixed-Point",
            4,
            16,
            64,
            Fixed,
            C(C::BGR.0 | C::ALPHA.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppRGBHalf,
        define_desc(
            "GUID_WICPixelFormat64bppRGBHalf",
            "RGB 64bpp Half-Float",
            3,
            16,
            64,
            Float,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppRGBAHalf,
        define_desc(
            "GUID_WICPixelFormat64bppRGBAHalf",
            "RGBA 64bpp Half-Float",
            4,
            16,
            64,
            Float,
            C(C::RGB.0 | C::ALPHA.0),
            R16G16B16A16Float,
        ),
    ),
    (
        GUID_WICPixelFormat96bppRGBFixedPoint,
        define_desc(
            "GUID_WICPixelFormat96bppRGBFixedPoint",
            "RGB 96bpp Fixed-Point",
            3,
            32,
            96,
            Fixed,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat128bppRGBFloat,
        define_desc(
            "GUID_WICPixelFormat128bppRGBFloat",
            "RGB 128bpp Float",
            3,
            32,
            128,
            Float,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat128bppRGBAFloat,
        define_desc(
            "GUID_WICPixelFormat128bppRGBAFloat",
            "RGBA 128bpp Float",
            4,
            32,
            128,
            Float,
            C(C::RGB.0 | C::ALPHA.0),
            R32G32B32A32Float,
        ),
    ),
    (
        GUID_WICPixelFormat128bppPRGBAFloat,
        define_desc(
            "GUID_WICPixelFormat128bppPRGBAFloat",
            "RGBA 128bpp Float, Premultiplied Alpha",
            4,
            32,
            128,
            Float,
            C(C::RGB.0 | C::ALPHA.0 | C::PREMULTIPLIED.0),
            R32G32B32A32Float,
        ),
    ),
    (
        GUID_WICPixelFormat128bppRGBFixedPoint,
        define_desc(
            "GUID_WICPixelFormat128bppRGBFixedPoint",
            "RGB 128bpp Fixed-Point",
            3,
            32,
            128,
            Fixed,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat128bppRGBAFixedPoint,
        define_desc(
            "GUID_WICPixelFormat128bppRGBAFixedPoint",
            "RGBA 128bpp Fixed-Point",
            4,
            32,
            128,
            Fixed,
            C(C::RGB.0 | C::ALPHA.0),
            Unknown,
        ),
    ),
    // Windows 8 + Windows 7 Platform Update
    (
        GUID_WICPixelFormat32bppRGB,
        define_desc(
            "GUID_WICPixelFormat32bppRGB",
            "RGB 32bpp",
            3,
            8,
            32,
            Unsigned,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppRGB,
        define_desc(
            "GUID_WICPixelFormat64bppRGB",
            "RGB 64bpp",
            3,
            16,
            64,
            Unsigned,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat96bppRGBFloat,
        define_desc(
            "GUID_WICPixelFormat96bppRGBFloat",
            "RGB 64bpp Float",
            3,
            32,
            96,
            Float,
            C::RGB,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppPRGBAHalf,
        define_desc(
            "GUID_WICPixelFormat64bppPRGBAHalf",
            "RGBA 64bpp Float, Premultiplied Alpha",
            4,
            16,
            64,
            Float,
            C(C::RGB.0 | C::ALPHA.0 | C::PREMULTIPLIED.0),
            R16G16B16A16Float,
        ),
    ),
    // CMYK
    (
        GUID_WICPixelFormat32bppCMYK,
        define_desc(
            "GUID_WICPixelFormat32bppCMYK",
            "CMYK 32bpp",
            4,
            8,
            32,
            Unsigned,
            C::CMYK,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat64bppCMYK,
        define_desc(
            "GUID_WICPixelFormat64bppCMYK",
            "CMYK 64bpp",
            4,
            16,
            64,
            Unsigned,
            C::CMYK,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat40bppCMYKAlpha,
        define_desc(
            "GUID_WICPixelFormat40bppCMYKAlpha",
            "CMYKA 40bpp",
            5,
            8,
            40,
            Unsigned,
            C(C::CMYK.0 | C::ALPHA.0),
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat80bppCMYKAlpha,
        define_desc(
            "GUID_WICPixelFormat80bppCMYKAlpha",
            "CMYKA 80bpp",
            5,
            16,
            80,
            Unsigned,
            C(C::CMYK.0 | C::ALPHA.0),
            Unknown,
        ),
    ),
    // N-Channel
    (
        GUID_WICPixelFormat24bpp3Channels,
        define_desc(
            "GUID_WICPixelFormat24bpp3Channels",
            "3-Channel 24bpp",
            3,
            8,
            24,
            Unsigned,
            C::N_CHANNEL,
            Unknown,
        ),
    ),
    (
        GUID_WICPixelFormat48bpp3Channels,
        define_desc(
            "GUID_WICPixelFormat48bpp3Channels",
            "3-Channel 48bpp",
            3,
            16,
            48,
            Unsigned,
            C::N_CHANNEL,
            Unknown,
        ),
    ),
    // TODO: FINISH THESE FUCKERS WITH FRESH CONCERTA IN YOUR SYSTEM
    // https://docs.microsoft.com/en-us/windows/desktop/wic/-wic-codec-native-pixel-formats
];

const fn define_desc(
    label: &'static str,
    description: &'static str,
    channel_count: u16,
    bpc: u16,
    bits_per_pixel: u16,
    storage_type: StorageType,
    category: Category,
    dxgi_format: dxgi::enums::Format,
) -> PixelFormatDescription {
    PixelFormatDescription {
        label,
        description,
        channel_count,
        bits_per_channel: Uniform(bpc),
        bits_per_pixel,
        storage_type,
        dxgi_format,
        category,
    }
}

const fn define_desc_p(
    label: &'static str,
    description: &'static str,
    channel_count: u16,
    bpc0: u16,
    bpc1: u16,
    bpc2: u16,
    bpc3: u16,
    bits_per_pixel: u16,
    storage_type: StorageType,
    category: Category,
    dxgi_format: dxgi::enums::Format,
) -> PixelFormatDescription {
    PixelFormatDescription {
        label,
        description,
        channel_count,
        bits_per_channel: PerChannel(bpc0, bpc1, bpc2, bpc3),
        bits_per_pixel,
        storage_type,
        dxgi_format,
        category,
    }
}
