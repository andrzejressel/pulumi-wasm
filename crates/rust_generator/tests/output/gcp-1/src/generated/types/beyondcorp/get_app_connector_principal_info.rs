#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAppConnectorPrincipalInfo {
    /// ServiceAccount represents a GCP service account.
    #[builder(into)]
    #[serde(rename = "serviceAccounts")]
    pub r#service_accounts: Box<Vec<super::super::types::beyondcorp::GetAppConnectorPrincipalInfoServiceAccount>>,
}
