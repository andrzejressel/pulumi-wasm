#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExtensionsInstanceRuntimeDataProcessingState {
    /// Details about the processing. e.g. This could include the type of
    /// processing in progress or it could list errors or failures.
    /// This information will be shown in the console on the detail page
    /// for the extension instance.
    #[builder(into, default)]
    #[serde(rename = "detailMessage")]
    pub r#detail_message: Box<Option<String>>,
    /// The processing state of the extension instance.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
