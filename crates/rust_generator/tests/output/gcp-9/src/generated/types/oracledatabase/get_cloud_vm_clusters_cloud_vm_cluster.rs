#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCloudVmClustersCloudVmCluster {
    /// CIDR range of the backup subnet.
    #[builder(into)]
    #[serde(rename = "backupSubnetCidr")]
    pub r#backup_subnet_cidr: Box<String>,
    /// Network settings. CIDR to use for cluster IP allocation.
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
    /// The ID of the VM Cluster to create. This value is restricted
    /// to (^a-z?$) and must be a maximum of 63
    /// characters in length. The value must start with a letter and end with
    /// a letter or a number.
    #[builder(into)]
    #[serde(rename = "cloudVmClusterId")]
    pub r#cloud_vm_cluster_id: Box<String>,
    /// The date and time that the VM cluster was created.
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
    /// The name of the Exadata Infrastructure resource on which VM cluster
    /// resource is created, in the following format:
    /// projects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}
    #[builder(into)]
    #[serde(rename = "exadataInfrastructure")]
    pub r#exadata_infrastructure: Box<String>,
    /// GCP location where Oracle Exadata is hosted. It is same as GCP Oracle zone
    /// of Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "gcpOracleZone")]
    pub r#gcp_oracle_zone: Box<String>,
    /// Labels or tags associated with the VM Cluster. 
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
    /// Identifier. The name of the VM Cluster resource with the format:
    /// projects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the VPC network.
    /// Format: projects/{project}/global/networks/{network}
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// The project to which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// Various properties and settings associated with Exadata VM cluster.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Vec<super::super::types::oracledatabase::GetCloudVmClustersCloudVmClusterProperty>>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
}
