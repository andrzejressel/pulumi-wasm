#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceS3Import {
    /// The bucket name where your backup is stored
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
    /// This will not recreate the resource if the S3 object changes in some way.  It's only used to initialize the database.
    #[builder(into)]
    #[serde(rename = "sourceEngineVersion")]
    pub r#source_engine_version: Box<String>,
}
