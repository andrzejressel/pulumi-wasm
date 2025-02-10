#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_shared_image_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageVersionArgs {
        /// The name of the Shared Image Gallery in which the Shared Image exists.
        #[builder(into)]
        pub gallery_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Shared Image in which this Version exists.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Image Version.
        ///
        /// > **Note:** You may specify `latest` to obtain the latest version or `recent` to obtain the most recently updated version.
        ///
        /// > **Note:** In 3.0, `latest` may return an image version with `exclude_from_latest` set to `true`. Starting from 4.0 onwards `latest` will not return image versions with `exlude_from_latest` set to `true`.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Sort available versions taking SemVer versioning scheme into account. Defaults to `false`.
        #[builder(into, default)]
        pub sort_versions_by_semver: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags assigned to the Shared Image.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSharedImageVersionResult {
        /// Is this Image Version excluded from the `latest` filter?
        pub exclude_from_latest: pulumi_gestalt_rust::Output<bool>,
        pub gallery_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// The supported Azure location where the Shared Image Gallery exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Managed Image which was the source of this Shared Image Version.
        pub managed_image_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which this Image Version exists.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The size of the OS disk snapshot (in Gigabytes) which was the source of this Shared Image Version.
        pub os_disk_image_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the OS disk snapshot which was the source of this Shared Image Version.
        pub os_disk_snapshot_id: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub sort_versions_by_semver: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags assigned to the Shared Image.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as documented below.
        pub target_regions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSharedImageVersionTargetRegion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSharedImageVersionArgs,
    ) -> GetSharedImageVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let gallery_name_binding = args.gallery_name.get_output(context);
        let image_name_binding = args.image_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sort_versions_by_semver_binding = args
            .sort_versions_by_semver
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getSharedImageVersion:getSharedImageVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryName".into(),
                    value: gallery_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageName".into(),
                    value: image_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sortVersionsBySemver".into(),
                    value: sort_versions_by_semver_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSharedImageVersionResult {
            exclude_from_latest: o.get_field("excludeFromLatest"),
            gallery_name: o.get_field("galleryName"),
            id: o.get_field("id"),
            image_name: o.get_field("imageName"),
            location: o.get_field("location"),
            managed_image_id: o.get_field("managedImageId"),
            name: o.get_field("name"),
            os_disk_image_size_gb: o.get_field("osDiskImageSizeGb"),
            os_disk_snapshot_id: o.get_field("osDiskSnapshotId"),
            resource_group_name: o.get_field("resourceGroupName"),
            sort_versions_by_semver: o.get_field("sortVersionsBySemver"),
            tags: o.get_field("tags"),
            target_regions: o.get_field("targetRegions"),
        }
    }
}
