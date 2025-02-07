#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Diameter {
    sixinch,
    twelveinch,
}

impl pulumi_gestalt_rust::__private::serde::Serialize for Diameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Diameter::sixinch => 6.0,
            Diameter::twelveinch => 12.0,
        };
        serializer.serialize_f64(value)
    }
}

impl<'de> pulumi_gestalt_rust::__private::serde::Deserialize<'de> for Diameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = f64::deserialize(deserializer)?;
        match f {
            6.0 => Ok(Diameter::sixinch),
            12.0 => Ok(Diameter::twelveinch),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}
