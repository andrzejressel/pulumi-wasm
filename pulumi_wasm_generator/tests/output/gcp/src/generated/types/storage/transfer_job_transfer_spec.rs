#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TransferJobTransferSpec {
    /// An AWS S3 data source. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "awsS3DataSource")]
    pub r#aws_s_3_data_source: Box<Option<super::super::types::storage::TransferJobTransferSpecAwsS3DataSource>>,
    /// An Azure Blob Storage data source. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "azureBlobStorageDataSource")]
    pub r#azure_blob_storage_data_source: Box<Option<super::super::types::storage::TransferJobTransferSpecAzureBlobStorageDataSource>>,
    /// A Google Cloud Storage data sink. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "gcsDataSink")]
    pub r#gcs_data_sink: Box<Option<super::super::types::storage::TransferJobTransferSpecGcsDataSink>>,
    /// A Google Cloud Storage data source. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "gcsDataSource")]
    pub r#gcs_data_source: Box<Option<super::super::types::storage::TransferJobTransferSpecGcsDataSource>>,
    /// An HDFS data source. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "hdfsDataSource")]
    pub r#hdfs_data_source: Box<Option<super::super::types::storage::TransferJobTransferSpecHdfsDataSource>>,
    /// A HTTP URL data source. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "httpDataSource")]
    pub r#http_data_source: Box<Option<super::super::types::storage::TransferJobTransferSpecHttpDataSource>>,
    /// Only objects that satisfy these object conditions are included in the set of data source and data sink objects. Object conditions based on objects' `last_modification_time` do not exclude objects in a data sink. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "objectConditions")]
    pub r#object_conditions: Box<Option<super::super::types::storage::TransferJobTransferSpecObjectConditions>>,
    /// A POSIX data sink. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "posixDataSink")]
    pub r#posix_data_sink: Box<Option<super::super::types::storage::TransferJobTransferSpecPosixDataSink>>,
    /// A POSIX filesystem data source. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "posixDataSource")]
    pub r#posix_data_source: Box<Option<super::super::types::storage::TransferJobTransferSpecPosixDataSource>>,
    /// Specifies the agent pool name associated with the posix data sink. When unspecified, the default name is used.
    #[builder(into, default)]
    #[serde(rename = "sinkAgentPoolName")]
    pub r#sink_agent_pool_name: Box<Option<String>>,
    /// Specifies the agent pool name associated with the posix data source. When unspecified, the default name is used.
    #[builder(into, default)]
    #[serde(rename = "sourceAgentPoolName")]
    pub r#source_agent_pool_name: Box<Option<String>>,
    /// Characteristics of how to treat files from datasource and sink during job. If the option `delete_objects_unique_in_sink` is true, object conditions based on objects' `last_modification_time` are ignored and do not exclude objects in a data source or a data sink. Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "transferOptions")]
    pub r#transfer_options: Box<Option<super::super::types::storage::TransferJobTransferSpecTransferOptions>>,
}
