#[deny(missing_docs)]

/// A struct
pub struct Struct {
    /// A field
    #[cfg(feature = "feature")]
    pub field: u32,
    #[cfg(not(feature = "feature"))]
    pub field: u64,
}
