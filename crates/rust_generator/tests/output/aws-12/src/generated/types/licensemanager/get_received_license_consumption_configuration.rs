#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetReceivedLicenseConsumptionConfiguration {
    /// Details about a borrow configuration. Detailed below
    #[builder(into)]
    #[serde(rename = "borrowConfigurations")]
    pub r#borrow_configurations: Box<Vec<super::super::types::licensemanager::GetReceivedLicenseConsumptionConfigurationBorrowConfiguration>>,
    /// Details about a provisional configuration. Detailed below
    #[builder(into)]
    #[serde(rename = "provisionalConfigurations")]
    pub r#provisional_configurations: Box<Vec<super::super::types::licensemanager::GetReceivedLicenseConsumptionConfigurationProvisionalConfiguration>>,
    #[builder(into)]
    #[serde(rename = "renewType")]
    pub r#renew_type: Box<String>,
}
