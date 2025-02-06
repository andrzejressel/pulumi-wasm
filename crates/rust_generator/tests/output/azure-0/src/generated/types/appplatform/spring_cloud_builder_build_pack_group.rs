#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudBuilderBuildPackGroup {
    /// Specifies a list of the build pack's ID.
    #[builder(into, default)]
    #[serde(rename = "buildPackIds")]
    pub r#build_pack_ids: Box<Option<Vec<String>>>,
    /// The name which should be used for this build pack group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
