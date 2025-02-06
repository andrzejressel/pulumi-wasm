#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceTemplateSchedulingNodeAffinity {
    /// The key for the node affinity label.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The operator. Can be `IN` for node-affinities
    /// or `NOT_IN` for anti-affinities.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Corresponds to the label values of a reservation resource.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
