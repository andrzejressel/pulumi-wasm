#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ContainerSize {
    FourInch,
    SixInch,
    EightInch,
}

impl pulumi_gestalt_rust::__private::serde::Serialize for ContainerSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            ContainerSize::FourInch => 4,
            ContainerSize::SixInch => 6,
            ContainerSize::EightInch => 8,
        };
        serializer.serialize_i64(value)
    }
}

impl<'de> pulumi_gestalt_rust::__private::serde::Deserialize<'de> for ContainerSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = i64::deserialize(deserializer)?;
        match f {
            4 => Ok(ContainerSize::FourInch),
            6 => Ok(ContainerSize::SixInch),
            8 => Ok(ContainerSize::EightInch),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}
