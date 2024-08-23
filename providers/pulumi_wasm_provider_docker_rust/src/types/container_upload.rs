#[derive(serde::Serialize)]
pub struct ContainerUpload {
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentBase64")]
    pub r#content_base_64: Box<Option<String>>,
    #[serde(rename = "executable")]
    pub r#executable: Box<Option<bool>>,
    #[serde(rename = "file")]
    pub r#file: Box<String>,
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    #[serde(rename = "sourceHash")]
    pub r#source_hash: Box<Option<String>>,
}
