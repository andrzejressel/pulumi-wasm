#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistrationDnsSettings {
    /// Configuration for an arbitrary DNS provider.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customDns")]
    pub r#custom_dns: Box<Option<super::super::types::clouddomains::RegistrationDnsSettingsCustomDns>>,
    /// The list of glue records for this Registration. Commonly empty.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "glueRecords")]
    pub r#glue_records: Box<Option<Vec<super::super::types::clouddomains::RegistrationDnsSettingsGlueRecord>>>,
}
