#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSharedImageVersionsImage {
    /// Is this Image Version excluded from the `latest` filter?
    #[builder(into)]
    #[serde(rename = "excludeFromLatest")]
    pub r#exclude_from_latest: Box<bool>,
    /// The ID of this Shared Image Version.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The supported Azure location where the Shared Image Gallery exists.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The ID of the Managed Image which was the source of this Shared Image Version.
    #[builder(into)]
    #[serde(rename = "managedImageId")]
    pub r#managed_image_id: Box<String>,
    /// The Azure Region in which this Image Version exists.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A mapping of tags assigned to the Shared Image.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
    /// One or more `target_region` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "targetRegions")]
    pub r#target_regions: Box<Vec<super::super::types::compute::GetSharedImageVersionsImageTargetRegion>>,
}
