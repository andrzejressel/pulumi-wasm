#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserHierarchyGroupHierarchyPath {
    /// A block that defines the details of level five. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelFives")]
    pub r#level_fives: Box<Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelFife>>>,
    /// A block that defines the details of level four. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelFours")]
    pub r#level_fours: Box<Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelFour>>>,
    /// A block that defines the details of level one. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelOnes")]
    pub r#level_ones: Box<Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelOne>>>,
    /// A block that defines the details of level three. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelThrees")]
    pub r#level_threes: Box<Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelThree>>>,
    /// A block that defines the details of level two. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelTwos")]
    pub r#level_twos: Box<Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelTwo>>>,
}
