#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum RubberTreeVariety {
    /// A burgundy rubber tree.
    Burgundy,
    /// A ruby rubber tree.
    Ruby,
    /// A tineke rubber tree.
    Tineke,
}
