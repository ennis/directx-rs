#[enum_flags]
pub enum CommitFlags {
    DEFAULT = 0,
    OVERWRITE = 1,
    ONLY_IF_CURRENT = 2,
    DANGEROUSLY_COMMIT_MERELY_TO_DISK_CACHE = 4,
    CONSOLIDATE = 8,
}
