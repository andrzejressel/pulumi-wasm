#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableClientSideTimestamps {
    /// Shows how to enable client-side timestamps settings for the specified table. Valid values: `ENABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
