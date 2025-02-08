#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBucketObjectsBucketObject {
    /// [Content-Type](https://tools.ietf.org/html/rfc7231#section-3.1.1.5) of the object data.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    /// A url reference to download this object.
    #[builder(into)]
    #[serde(rename = "mediaLink")]
    pub r#media_link: Box<String>,
    /// The name of the object.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A url reference to this object.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
    /// The [StorageClass](https://cloud.google.com/storage/docs/storage-classes) of the bucket object.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<String>,
}
