#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExtensionsInstanceRuntimeDataFatalError {
    /// The error message. This is set by the extension developer to give
    /// more detail on why the extension is unusable and must be re-installed
    /// or reconfigured.
    #[builder(into, default)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Box<Option<String>>,
}
