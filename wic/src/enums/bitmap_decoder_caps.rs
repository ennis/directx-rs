#[auto_enum::enum_flags]
pub enum BitmapDecoderCapabilities {
    SAME_ENCODER = 0x01,
    CAN_DECODE_ALL_IMAGES = 0x02,
    CAN_DECODE_SOME_IMAGES = 0x04,
    CAN_ENUMERATE_METADATA = 0x08,
    CAN_DECODE_THUMBNAIL = 0x10,

    NONE = 0,
}
