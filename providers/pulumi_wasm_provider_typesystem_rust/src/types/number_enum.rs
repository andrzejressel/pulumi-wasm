#[derive(Debug, PartialEq, Clone)]
pub enum NumberEnum {
    Value1,
    Value2,
}

impl serde::Serialize for NumberEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            NumberEnum::Value1 => 1.0,
            NumberEnum::Value2 => 2.0,
        };
        serializer.serialize_f64(value)
    }
}

impl<'de> serde::Deserialize<'de> for NumberEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = f64::deserialize(deserializer)?;
        match f {
            1.0 => Ok(NumberEnum::Value1),
            2.0 => Ok(NumberEnum::Value2),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}
