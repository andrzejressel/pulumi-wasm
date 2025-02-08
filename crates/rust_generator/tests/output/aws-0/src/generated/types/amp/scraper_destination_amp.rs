#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScraperDestinationAmp {
    /// The Amazon Resource Name (ARN) of the prometheus workspace.
    #[builder(into)]
    #[serde(rename = "workspaceArn")]
    pub r#workspace_arn: Box<String>,
}
