#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolCertificate {
    /// The fully qualified ID of the certificate installed on the pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The location of the certificate store on the compute node into which the certificate is installed, either `CurrentUser` or `LocalMachine`.
    #[builder(into)]
    #[serde(rename = "storeLocation")]
    pub r#store_location: Box<String>,
    /// The name of the certificate store on the compute node into which the certificate is installed.
    #[builder(into)]
    #[serde(rename = "storeName")]
    pub r#store_name: Box<String>,
    /// Which user accounts on the compute node have access to the private data of the certificate.
    #[builder(into)]
    #[serde(rename = "visibilities")]
    pub r#visibilities: Box<Vec<String>>,
}
