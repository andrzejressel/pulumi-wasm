#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomActionTypeInputArtifactDetails {
    /// The maximum number of artifacts allowed for the action type. Min: 0, Max: 5
    #[builder(into)]
    #[serde(rename = "maximumCount")]
    pub r#maximum_count: Box<i32>,
    /// The minimum number of artifacts allowed for the action type. Min: 0, Max: 5
    #[builder(into)]
    #[serde(rename = "minimumCount")]
    pub r#minimum_count: Box<i32>,
}
