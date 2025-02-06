#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageOutputResourceContainer {
    /// Set of URIs for created containers.
    #[builder(into, default)]
    #[serde(rename = "imageUris")]
    pub r#image_uris: Box<Option<Vec<String>>>,
    /// Region of the container image.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
}
