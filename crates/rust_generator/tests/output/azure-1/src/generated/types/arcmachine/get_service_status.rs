#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceStatus {
    /// A `extension_service` block as defined above.
    #[builder(into)]
    #[serde(rename = "extensionServices")]
    pub r#extension_services: Box<Vec<super::super::types::arcmachine::GetServiceStatusExtensionService>>,
    /// A `guest_configuration_service` block as defined above.
    #[builder(into)]
    #[serde(rename = "guestConfigurationServices")]
    pub r#guest_configuration_services: Box<Vec<super::super::types::arcmachine::GetServiceStatusGuestConfigurationService>>,
}
