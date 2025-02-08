#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub enum ContainerBrightness {
    ZeroPointOne,
    One,
}

impl pulumi_gestalt_rust::__private::serde::Serialize for ContainerBrightness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            ContainerBrightness::ZeroPointOne => 0.1,
            ContainerBrightness::One => 1.0,
        };
        serializer.serialize_f64(value)
    }
}

impl<'de> pulumi_gestalt_rust::__private::serde::Deserialize<'de> for ContainerBrightness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = f64::deserialize(deserializer)?;
        match f {
            0.1 => Ok(ContainerBrightness::ZeroPointOne),
            1.0 => Ok(ContainerBrightness::One),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}
