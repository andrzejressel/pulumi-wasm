#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OntapFileSystemEndpointManagement {
    /// The Domain Name Service (DNS) name for the file system. You can mount your file system using its DNS name.
    #[builder(into, default)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Box<Option<String>>,
    /// IP addresses of the file system endpoint.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
}
