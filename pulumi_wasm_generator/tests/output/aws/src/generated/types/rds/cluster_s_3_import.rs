#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterS3Import {
    /// Bucket name where your backup is stored
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// Can be blank, but is the path to your backup
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
    /// Role applied to load the data.
    #[builder(into)]
    #[serde(rename = "ingestionRole")]
    pub r#ingestion_role: Box<String>,
    /// Source engine for the backup
    #[builder(into)]
    #[serde(rename = "sourceEngine")]
    pub r#source_engine: Box<String>,
    /// Version of the source engine used to make the backup
    /// 
    /// This will not recreate the resource if the S3 object changes in some way. It's only used to initialize the database. This only works currently with the aurora engine. See AWS for currently supported engines and options. See [Aurora S3 Migration Docs](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Migrating.ExtMySQL.html#AuroraMySQL.Migrating.ExtMySQL.S3).
    #[builder(into)]
    #[serde(rename = "sourceEngineVersion")]
    pub r#source_engine_version: Box<String>,
}
