#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntitlementRequesterJustificationConfig {
    /// The justification is not mandatory but can be provided in any of the supported formats.
    #[builder(into, default)]
    #[serde(rename = "notMandatory")]
    pub r#not_mandatory: Box<Option<super::super::types::privilegedaccessmanager::EntitlementRequesterJustificationConfigNotMandatory>>,
    /// The requester has to provide a justification in the form of free flowing text.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "unstructured")]
    pub r#unstructured: Box<Option<super::super::types::privilegedaccessmanager::EntitlementRequesterJustificationConfigUnstructured>>,
}
