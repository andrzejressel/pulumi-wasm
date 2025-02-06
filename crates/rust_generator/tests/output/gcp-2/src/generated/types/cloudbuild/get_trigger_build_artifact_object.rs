#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerBuildArtifactObject {
    /// The Cloud Build location for the trigger.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Path globs used to match files in the build's workspace.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<String>>,
    /// Output only. Stores timing information for pushing all artifact objects.
    #[builder(into)]
    #[serde(rename = "timings")]
    pub r#timings: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactObjectTiming>>,
}
