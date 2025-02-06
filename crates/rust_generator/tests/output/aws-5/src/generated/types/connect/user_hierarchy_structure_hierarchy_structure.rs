#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserHierarchyStructureHierarchyStructure {
    /// A block that defines the details of level five. The level block is documented below.
    /// 
    /// Each level block supports the following arguments:
    #[builder(into, default)]
    #[serde(rename = "levelFive")]
    pub r#level_five: Box<Option<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelFive>>,
    /// A block that defines the details of level four. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelFour")]
    pub r#level_four: Box<Option<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelFour>>,
    /// A block that defines the details of level one. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelOne")]
    pub r#level_one: Box<Option<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelOne>>,
    /// A block that defines the details of level three. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelThree")]
    pub r#level_three: Box<Option<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelThree>>,
    /// A block that defines the details of level two. The level block is documented below.
    #[builder(into, default)]
    #[serde(rename = "levelTwo")]
    pub r#level_two: Box<Option<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelTwo>>,
}
