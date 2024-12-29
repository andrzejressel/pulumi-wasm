#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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
