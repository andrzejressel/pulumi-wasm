#[derive(Debug, PartialEq, Clone)]
pub enum IntegerEnum {
    Value1,
    Value2,
}

impl serde::Serialize for IntegerEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            IntegerEnum::Value1 => 1,
            IntegerEnum::Value2 => 2,
        };
        serializer.serialize_i64(value)
    }
}

impl<'de> serde::Deserialize<'de> for IntegerEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = i64::deserialize(deserializer)?;
        match f {
            1 => Ok(IntegerEnum::Value1),
            2 => Ok(IntegerEnum::Value2),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}
