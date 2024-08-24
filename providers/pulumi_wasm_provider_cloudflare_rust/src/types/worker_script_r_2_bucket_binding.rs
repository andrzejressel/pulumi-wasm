#[derive(serde::Serialize)]
pub struct WorkerScriptR2BucketBinding {
    /// The name of the Bucket to bind to.
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
