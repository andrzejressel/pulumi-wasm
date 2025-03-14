#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppConnectorPrincipalInfo {
    /// ServiceAccount represents a GCP service account.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<super::super::types::beyondcorp::AppConnectorPrincipalInfoServiceAccount>,
}
