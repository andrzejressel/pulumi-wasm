#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum MyEnum {
    #[serde(rename = "VALUE1")]
    Value1,
    Value2,
    #[serde(rename = "Plants'R'Us")]
    SpecialCharacters,
}
