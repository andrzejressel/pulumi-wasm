/// Manages a Gallery Application Version.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestorage")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleBlob = blob::create(
///         "exampleBlob",
///         BlobArgs::builder()
///             .name("scripts")
///             .source_content("[scripts file content]")
///             .storage_account_name("${exampleAccount.name}")
///             .storage_container_name("${exampleContainer.name}")
///             .type_("Block")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("blob")
///             .name("example-container")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleGalleryApplication = gallery_application::create(
///         "exampleGalleryApplication",
///         GalleryApplicationArgs::builder()
///             .gallery_id("${exampleSharedImageGallery.id}")
///             .location("${example.location}")
///             .name("example-app")
///             .supported_os_type("Linux")
///             .build_struct(),
///     );
///     let exampleGalleryApplicationVersion = gallery_application_version::create(
///         "exampleGalleryApplicationVersion",
///         GalleryApplicationVersionArgs::builder()
///             .gallery_application_id("${exampleGalleryApplication.id}")
///             .location("${exampleGalleryApplication.location}")
///             .manage_action(
///                 GalleryApplicationVersionManageAction::builder()
///                     .install("[install command]")
///                     .remove("[remove command]")
///                     .build_struct(),
///             )
///             .name("0.0.1")
///             .source(
///                 GalleryApplicationVersionSource::builder()
///                     .mediaLink("${exampleBlob.id}")
///                     .build_struct(),
///             )
///             .target_regions(
///                 vec![
///                     GalleryApplicationVersionTargetRegion::builder()
///                     .name("${exampleGalleryApplication.location}")
///                     .regionalReplicaCount(1).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleSharedImageGallery = shared_image_gallery::create(
///         "exampleSharedImageGallery",
///         SharedImageGalleryArgs::builder()
///             .location("${example.location}")
///             .name("examplegallery")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Gallery Application Versions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/galleryApplicationVersion:GalleryApplicationVersion example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/galleries/gallery1/applications/galleryApplication1/versions/galleryApplicationVersion1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gallery_application_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GalleryApplicationVersionArgs {
        /// Specifies the name of the config file on the VM. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub config_file: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Gallery Application reports health. Defaults to `false`.
        #[builder(into, default)]
        pub enable_health_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The end of life date in RFC3339 format of the Gallery Application Version.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Gallery Application Version be excluded from the `latest` filter? If set to `true` this Gallery Application Version won't be returned for the `latest` version. Defaults to `false`.
        #[builder(into, default)]
        pub exclude_from_latest: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Gallery Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Gallery Application Version exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `manage_action` block as defined below.
        #[builder(into)]
        pub manage_action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::GalleryApplicationVersionManageAction,
        >,
        /// The version name of the Gallery Application Version, such as `1.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the package file on the VM. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub package_file: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `source` block as defined below.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::GalleryApplicationVersionSource,
        >,
        /// A mapping of tags to assign to the Gallery Application Version.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as defined below.
        #[builder(into)]
        pub target_regions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::GalleryApplicationVersionTargetRegion>,
        >,
    }
    #[allow(dead_code)]
    pub struct GalleryApplicationVersionResult {
        /// Specifies the name of the config file on the VM. Changing this forces a new resource to be created.
        pub config_file: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Gallery Application reports health. Defaults to `false`.
        pub enable_health_check: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The end of life date in RFC3339 format of the Gallery Application Version.
        pub end_of_life_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the Gallery Application Version be excluded from the `latest` filter? If set to `true` this Gallery Application Version won't be returned for the `latest` version. Defaults to `false`.
        pub exclude_from_latest: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Gallery Application. Changing this forces a new resource to be created.
        pub gallery_application_id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Gallery Application Version exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `manage_action` block as defined below.
        pub manage_action: pulumi_gestalt_rust::Output<
            super::super::types::compute::GalleryApplicationVersionManageAction,
        >,
        /// The version name of the Gallery Application Version, such as `1.0.0`. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the package file on the VM. Changing this forces a new resource to be created.
        pub package_file: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `source` block as defined below.
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::compute::GalleryApplicationVersionSource,
        >,
        /// A mapping of tags to assign to the Gallery Application Version.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as defined below.
        pub target_regions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::GalleryApplicationVersionTargetRegion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GalleryApplicationVersionArgs,
    ) -> GalleryApplicationVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_file_binding = args.config_file.get_output(context);
        let enable_health_check_binding = args.enable_health_check.get_output(context);
        let end_of_life_date_binding = args.end_of_life_date.get_output(context);
        let exclude_from_latest_binding = args.exclude_from_latest.get_output(context);
        let gallery_application_id_binding = args
            .gallery_application_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let manage_action_binding = args.manage_action.get_output(context);
        let name_binding = args.name.get_output(context);
        let package_file_binding = args.package_file.get_output(context);
        let source_binding = args.source.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_regions_binding = args.target_regions.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/galleryApplicationVersion:GalleryApplicationVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configFile".into(),
                    value: config_file_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHealthCheck".into(),
                    value: enable_health_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endOfLifeDate".into(),
                    value: end_of_life_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeFromLatest".into(),
                    value: exclude_from_latest_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "galleryApplicationId".into(),
                    value: gallery_application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manageAction".into(),
                    value: manage_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageFile".into(),
                    value: package_file_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRegions".into(),
                    value: target_regions_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GalleryApplicationVersionResult {
            config_file: o.get_field("configFile"),
            enable_health_check: o.get_field("enableHealthCheck"),
            end_of_life_date: o.get_field("endOfLifeDate"),
            exclude_from_latest: o.get_field("excludeFromLatest"),
            gallery_application_id: o.get_field("galleryApplicationId"),
            location: o.get_field("location"),
            manage_action: o.get_field("manageAction"),
            name: o.get_field("name"),
            package_file: o.get_field("packageFile"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
            target_regions: o.get_field("targetRegions"),
        }
    }
}
