#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalAdminClusterControlPlaneApiServerArg {
    /// The argument name as it appears on the API Server command line please make sure to remove the leading dashes.
    #[builder(into)]
    #[serde(rename = "argument")]
    pub r#argument: Box<String>,
    /// The value of the arg as it will be passed to the API Server command line.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
