#[auto_enum(u32, checked)]
pub enum BitmapCreateCacheOption {
    NoCache = 0x0,
    CacheOnDemand = 0x1,
    CacheOnLoad = 0x2,
}
