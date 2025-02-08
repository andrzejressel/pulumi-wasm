#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserHierarchyStructureHierarchyStructure {
    /// Details of level five. See below.
    #[builder(into)]
    #[serde(rename = "levelFives")]
    pub r#level_fives: Box<Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelFife>>,
    /// Details of level four. See below.
    #[builder(into)]
    #[serde(rename = "levelFours")]
    pub r#level_fours: Box<Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelFour>>,
    /// Details of level one. See below.
    #[builder(into)]
    #[serde(rename = "levelOnes")]
    pub r#level_ones: Box<Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelOne>>,
    /// Details of level three. See below.
    #[builder(into)]
    #[serde(rename = "levelThrees")]
    pub r#level_threes: Box<Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelThree>>,
    /// Details of level two. See below.
    #[builder(into)]
    #[serde(rename = "levelTwos")]
    pub r#level_twos: Box<Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelTwo>>,
}
