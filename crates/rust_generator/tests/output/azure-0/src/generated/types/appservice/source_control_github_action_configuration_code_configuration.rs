#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SourceControlGithubActionConfigurationCodeConfiguration {
    /// The value to use for the Runtime Stack in the workflow file content for code base apps. Possible values are `dotnetcore`, `spring`, `tomcat`, `node` and `python`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "runtimeStack")]
    pub r#runtime_stack: Box<String>,
    /// The value to use for the Runtime Version in the workflow file content for code base apps. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<String>,
}
