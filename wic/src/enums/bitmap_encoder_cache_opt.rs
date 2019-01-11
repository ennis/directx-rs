#[auto_enum(u32, checked)]
pub enum BitmapEncoderCacheOptions {
    CacheInMemory,
    CacheTempFile,
    NoCache,
}