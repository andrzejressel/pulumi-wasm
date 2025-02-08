#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSearchAllResourcesResult {
    /// The type of this resource.
    #[builder(into)]
    #[serde(rename = "assetType")]
    pub r#asset_type: Box<String>,
    /// The create timestamp of this resource, at which the resource was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The display name of this resource.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// The folder(s) that this resource belongs to, in the form of `folders/{FOLDER_NUMBER}`. This field is available when the resource belongs to one or more folders.
    #[builder(into)]
    #[serde(rename = "folders")]
    pub r#folders: Box<Vec<String>>,
    /// The Cloud KMS CryptoKey names or CryptoKeyVersion names. This field is available only when the resource's Protobuf contains it.
    #[builder(into)]
    #[serde(rename = "kmsKeys")]
    pub r#kms_keys: Box<Vec<String>>,
    /// Labels associated with this resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The full resource name of this resource.. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Network tags associated with this resource.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Box<Vec<String>>,
    /// The organization that this resource belongs to, in the form of `organizations/{ORGANIZATION_NUMBER}`. This field is available when the resource belongs to an organization.
    #[builder(into)]
    #[serde(rename = "organization")]
    pub r#organization: Box<String>,
    /// The type of this resource's immediate parent, if there is one.
    #[builder(into)]
    #[serde(rename = "parentAssetType")]
    pub r#parent_asset_type: Box<String>,
    /// The full resource name of this resource's parent, if it has one.
    #[builder(into)]
    #[serde(rename = "parentFullResourceName")]
    pub r#parent_full_resource_name: Box<String>,
    /// The project that this resource belongs to, in the form of `projects/{project_number}`.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// The state of this resource.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// The last update timestamp of this resource, at which the resource was last modified or deleted.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
}
