#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionHandlerScript {
    /// Path to the script from the application root directory.
    #[builder(into)]
    #[serde(rename = "scriptPath")]
    pub r#script_path: Box<String>,
}
