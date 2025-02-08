#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCloudExadataInfrastructuresCloudExadataInfrastructure {
    /// The ID of the Exadata Infrastructure to create. This value is restricted
    /// to (^a-z?$) and must be a maximum of 63
    /// characters in length. The value must start with a letter and end with
    /// a letter or a number.
    #[builder(into)]
    #[serde(rename = "cloudExadataInfrastructureId")]
    pub r#cloud_exadata_infrastructure_id: Box<String>,
    /// The date and time that the Exadata Infrastructure was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    #[builder(into)]
    #[serde(rename = "deletionProtection")]
    pub r#deletion_protection: Box<bool>,
    /// User friendly name for this resource.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Box<std::collections::HashMap<String, String>>,
    /// Entitlement ID of the private offer against which this infrastructure
    /// resource is provisioned.
    #[builder(into)]
    #[serde(rename = "entitlementId")]
    pub r#entitlement_id: Box<String>,
    /// GCP location where Oracle Exadata is hosted.
    #[builder(into)]
    #[serde(rename = "gcpOracleZone")]
    pub r#gcp_oracle_zone: Box<String>,
    /// Labels or tags associated with the resource. 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The location of the resource.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Identifier. The name of the Exadata Infrastructure resource with the following format:
    /// projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The project to which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// Various properties of Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty>>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
}
