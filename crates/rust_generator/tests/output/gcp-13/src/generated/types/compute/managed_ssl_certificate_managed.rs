#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedSslCertificateManaged {
    /// Domains for which a managed SSL certificate will be valid.  Currently,
    /// there can be up to 100 domains in this list.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Vec<String>>,
}
