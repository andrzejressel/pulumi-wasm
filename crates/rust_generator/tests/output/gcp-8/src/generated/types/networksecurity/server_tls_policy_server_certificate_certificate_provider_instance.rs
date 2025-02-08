#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerTlsPolicyServerCertificateCertificateProviderInstance {
    /// Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to "google_cloud_private_spiffe" to use Certificate Authority Service certificate provider instance.
    #[builder(into)]
    #[serde(rename = "pluginInstance")]
    pub r#plugin_instance: Box<String>,
}
