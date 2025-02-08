#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataExchangeSharingEnvironmentConfig {
    /// Data Clean Room (DCR), used for privacy-safe and secured data sharing.
    #[builder(into, default)]
    #[serde(rename = "dcrExchangeConfig")]
    pub r#dcr_exchange_config: Box<Option<super::super::types::bigqueryanalyticshub::DataExchangeSharingEnvironmentConfigDcrExchangeConfig>>,
    /// Default Analytics Hub data exchange, used for secured data sharing.
    #[builder(into, default)]
    #[serde(rename = "defaultExchangeConfig")]
    pub r#default_exchange_config: Box<Option<super::super::types::bigqueryanalyticshub::DataExchangeSharingEnvironmentConfigDefaultExchangeConfig>>,
}
