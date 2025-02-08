#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SharedImageGallerySharing {
    /// A `community_gallery` block as defined below. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `community_gallery` must be set when `permission` is set to `Community`.
    #[builder(into, default)]
    #[serde(rename = "communityGallery")]
    pub r#community_gallery: Box<Option<super::super::types::compute::SharedImageGallerySharingCommunityGallery>>,
    /// The permission of the Shared Image Gallery when sharing. Possible values are `Community`, `Groups` and `Private`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This requires that the Preview Feature `Microsoft.Compute/CommunityGalleries` is enabled, see [the documentation](https://learn.microsoft.com/azure/virtual-machines/share-gallery-community?tabs=cli) for more information.
    #[builder(into)]
    #[serde(rename = "permission")]
    pub r#permission: Box<String>,
}
