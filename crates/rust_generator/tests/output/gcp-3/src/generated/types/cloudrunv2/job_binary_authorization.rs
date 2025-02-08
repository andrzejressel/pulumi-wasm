#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobBinaryAuthorization {
    /// If present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass
    #[builder(into, default)]
    #[serde(rename = "breakglassJustification")]
    pub r#breakglass_justification: Box<Option<String>>,
    /// The path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}
    #[builder(into, default)]
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
    /// If True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled.
    #[builder(into, default)]
    #[serde(rename = "useDefault")]
    pub r#use_default: Box<Option<bool>>,
}
