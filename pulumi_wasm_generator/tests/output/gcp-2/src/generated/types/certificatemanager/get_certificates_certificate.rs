#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificatesCertificate {
    /// A human-readable description of the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Box<std::collections::HashMap<String, String>>,
    /// Set of label tags associated with the Certificate resource.
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The Certificate Manager location. If not specified, "global" is used.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Configuration and state of a Managed Certificate.
    /// Certificate Manager provisions and renews Managed Certificates
    /// automatically, for as long as it's authorized to do so.
    #[builder(into)]
    #[serde(rename = "manageds")]
    pub r#manageds: Box<Vec<super::super::types::certificatemanager::GetCertificatesCertificateManaged>>,
    /// A user-defined name of the certificate. Certificate names must be unique
    /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
    /// and all following characters must be a dash, underscore, letter or digit.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the project in which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
    /// The list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6)
    #[builder(into)]
    #[serde(rename = "sanDnsnames")]
    pub r#san_dnsnames: Box<Vec<String>>,
    /// The scope of the certificate.
    /// 
    /// DEFAULT: Certificates with default scope are served from core Google data centers.
    /// If unsure, choose this option.
    /// 
    /// EDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.
    /// See https://cloud.google.com/vpc/docs/edge-locations.
    /// 
    /// ALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).
    /// See https://cloud.google.com/compute/docs/regions-zones
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
}
