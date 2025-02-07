#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEntitlementRequesterJustificationConfig {
    /// The justification is not mandatory but can be provided in any of the supported formats.
    #[builder(into)]
    #[serde(rename = "notMandatories")]
    pub r#not_mandatories: Box<Vec<super::super::types::privilegedaccessmanager::GetEntitlementRequesterJustificationConfigNotMandatory>>,
    /// The requester has to provide a justification in the form of free flowing text.
    #[builder(into)]
    #[serde(rename = "unstructureds")]
    pub r#unstructureds: Box<Vec<super::super::types::privilegedaccessmanager::GetEntitlementRequesterJustificationConfigUnstructured>>,
}
