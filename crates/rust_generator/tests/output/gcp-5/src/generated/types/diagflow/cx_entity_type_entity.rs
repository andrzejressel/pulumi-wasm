#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxEntityTypeEntity {
    /// A collection of value synonyms. For example, if the entity type is vegetable, and value is scallions, a synonym could be green onions.
    /// For KIND_LIST entity types: This collection must contain exactly one synonym equal to value.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "synonyms")]
    pub r#synonyms: Box<Option<Vec<String>>>,
    /// The primary value associated with this entity entry. For example, if the entity type is vegetable, the value could be scallions.
    /// For KIND_MAP entity types: A canonical value to be used in place of synonyms.
    /// For KIND_LIST entity types: A string that can contain references to other entity types (with or without aliases).
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
