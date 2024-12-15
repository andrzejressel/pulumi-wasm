#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum MyEnum {
    #[serde(rename = "VALUE1")]
    Value1,
    Value2,
}
