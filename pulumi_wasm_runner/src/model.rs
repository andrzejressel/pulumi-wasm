#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct OutputId(pub(crate) String);

impl OutputId {
    pub(crate) const fn new(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for OutputId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
