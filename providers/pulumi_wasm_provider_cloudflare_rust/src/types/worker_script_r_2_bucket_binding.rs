#[derive(serde::Serialize)]
pub struct WorkerScriptR2BucketBinding {
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
