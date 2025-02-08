#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSourceDataSourceGcpResource {
    /// ComputeInstanceDataSourceProperties has a subset of Compute Instance properties that are useful at the Datasource level.
    #[builder(into)]
    #[serde(rename = "computeInstanceDataSourceProperties")]
    pub r#compute_instance_data_source_properties: Box<Vec<super::super::types::backupdisasterrecovery::GetDataSourceDataSourceGcpResourceComputeInstanceDataSourceProperty>>,
    /// Full resource pathname URL of the source Google Cloud resource.
    #[builder(into)]
    #[serde(rename = "gcpResourcename")]
    pub r#gcp_resourcename: Box<String>,
    /// Location of the resource: <region>/<zone>/"global"/"unspecified".
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The type of the Google Cloud resource. Use the Unified Resource Type,
    /// eg. compute.googleapis.com/Instance.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
