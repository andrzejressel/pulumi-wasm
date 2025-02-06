#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetRun {
    /// Required. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
