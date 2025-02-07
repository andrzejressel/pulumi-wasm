#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobBinaryAuthorization {
    /// If present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass
    #[builder(into)]
    #[serde(rename = "breakglassJustification")]
    pub r#breakglass_justification: Box<String>,
    /// The path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Box<String>,
    /// If True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled.
    #[builder(into)]
    #[serde(rename = "useDefault")]
    pub r#use_default: Box<bool>,
}
