#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetResourcesSearchAllResult {
    /// Additional searchable attributes of this resource. Informational only. The exact set of attributes is subject to change. For example: project id, DNS name etc.
    #[builder(into)]
    #[serde(rename = "additionalAttributes")]
    pub r#additional_attributes: Box<Vec<String>>,
    /// The type of this resource.
    #[builder(into)]
    #[serde(rename = "assetType")]
    pub r#asset_type: Box<String>,
    /// One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The display name of this resource.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// Labels associated with this resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The full resource name. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Network tags associated with this resource.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Box<Vec<String>>,
    /// The project that this resource belongs to, in the form of `projects/{project_number}`.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
}
