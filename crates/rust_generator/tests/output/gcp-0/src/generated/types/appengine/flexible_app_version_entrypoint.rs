#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionEntrypoint {
    /// The format should be a shell command that can be fed to bash -c.
    #[builder(into)]
    #[serde(rename = "shell")]
    pub r#shell: Box<String>,
}
