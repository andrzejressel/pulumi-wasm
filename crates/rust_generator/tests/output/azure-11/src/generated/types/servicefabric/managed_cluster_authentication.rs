#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedClusterAuthentication {
    /// A `active_directory` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Box<Option<super::super::types::servicefabric::ManagedClusterAuthenticationActiveDirectory>>,
    /// One or more `certificate` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "certificates")]
    pub r#certificates: Box<Option<Vec<super::super::types::servicefabric::ManagedClusterAuthenticationCertificate>>>,
}
