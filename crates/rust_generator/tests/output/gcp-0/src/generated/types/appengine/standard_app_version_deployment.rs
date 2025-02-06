#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardAppVersionDeployment {
    /// Manifest of the files stored in Google Cloud Storage that are included as part of this version.
    /// All files must be readable using the credentials supplied with this call.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "files")]
    pub r#files: Box<Option<Vec<super::super::types::appengine::StandardAppVersionDeploymentFile>>>,
    /// Zip File
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "zip")]
    pub r#zip: Box<Option<super::super::types::appengine::StandardAppVersionDeploymentZip>>,
}
