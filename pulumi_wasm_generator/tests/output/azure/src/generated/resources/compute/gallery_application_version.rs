/// Manages a Gallery Application Version.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod gallery_application_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GalleryApplicationVersionArgs {
        /// Specifies the name of the config file on the VM. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub config_file: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Gallery Application reports health. Defaults to `false`.
        #[builder(into, default)]
        pub enable_health_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// The end of life date in RFC3339 format of the Gallery Application Version.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Gallery Application Version be excluded from the `latest` filter? If set to `true` this Gallery Application Version won't be returned for the `latest` version. Defaults to `false`.
        #[builder(into, default)]
        pub exclude_from_latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Gallery Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_application_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Gallery Application Version exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `manage_action` block as defined below.
        #[builder(into)]
        pub manage_action: pulumi_wasm_rust::Output<
            super::super::types::compute::GalleryApplicationVersionManageAction,
        >,
        /// The version name of the Gallery Application Version, such as `1.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the package file on the VM. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub package_file: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source` block as defined below.
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<
            super::super::types::compute::GalleryApplicationVersionSource,
        >,
        /// A mapping of tags to assign to the Gallery Application Version.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as defined below.
        #[builder(into)]
        pub target_regions: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::GalleryApplicationVersionTargetRegion>,
        >,
    }
    #[allow(dead_code)]
    pub struct GalleryApplicationVersionResult {
        /// Specifies the name of the config file on the VM. Changing this forces a new resource to be created.
        pub config_file: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Gallery Application reports health. Defaults to `false`.
        pub enable_health_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// The end of life date in RFC3339 format of the Gallery Application Version.
        pub end_of_life_date: pulumi_wasm_rust::Output<Option<String>>,
        /// Should the Gallery Application Version be excluded from the `latest` filter? If set to `true` this Gallery Application Version won't be returned for the `latest` version. Defaults to `false`.
        pub exclude_from_latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Gallery Application. Changing this forces a new resource to be created.
        pub gallery_application_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Gallery Application Version exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `manage_action` block as defined below.
        pub manage_action: pulumi_wasm_rust::Output<
            super::super::types::compute::GalleryApplicationVersionManageAction,
        >,
        /// The version name of the Gallery Application Version, such as `1.0.0`. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the package file on the VM. Changing this forces a new resource to be created.
        pub package_file: pulumi_wasm_rust::Output<Option<String>>,
        /// A `source` block as defined below.
        pub source: pulumi_wasm_rust::Output<
            super::super::types::compute::GalleryApplicationVersionSource,
        >,
        /// A mapping of tags to assign to the Gallery Application Version.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as defined below.
        pub target_regions: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::GalleryApplicationVersionTargetRegion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GalleryApplicationVersionArgs,
    ) -> GalleryApplicationVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_file_binding = args.config_file.get_inner();
        let enable_health_check_binding = args.enable_health_check.get_inner();
        let end_of_life_date_binding = args.end_of_life_date.get_inner();
        let exclude_from_latest_binding = args.exclude_from_latest.get_inner();
        let gallery_application_id_binding = args.gallery_application_id.get_inner();
        let location_binding = args.location.get_inner();
        let manage_action_binding = args.manage_action.get_inner();
        let name_binding = args.name.get_inner();
        let package_file_binding = args.package_file.get_inner();
        let source_binding = args.source.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_regions_binding = args.target_regions.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/galleryApplicationVersion:GalleryApplicationVersion"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configFile".into(),
                    value: &config_file_binding,
                },
                register_interface::ObjectField {
                    name: "enableHealthCheck".into(),
                    value: &enable_health_check_binding,
                },
                register_interface::ObjectField {
                    name: "endOfLifeDate".into(),
                    value: &end_of_life_date_binding,
                },
                register_interface::ObjectField {
                    name: "excludeFromLatest".into(),
                    value: &exclude_from_latest_binding,
                },
                register_interface::ObjectField {
                    name: "galleryApplicationId".into(),
                    value: &gallery_application_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "manageAction".into(),
                    value: &manage_action_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "packageFile".into(),
                    value: &package_file_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetRegions".into(),
                    value: &target_regions_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configFile".into(),
                },
                register_interface::ResultField {
                    name: "enableHealthCheck".into(),
                },
                register_interface::ResultField {
                    name: "endOfLifeDate".into(),
                },
                register_interface::ResultField {
                    name: "excludeFromLatest".into(),
                },
                register_interface::ResultField {
                    name: "galleryApplicationId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "manageAction".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "packageFile".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetRegions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GalleryApplicationVersionResult {
            config_file: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configFile").unwrap(),
            ),
            enable_health_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableHealthCheck").unwrap(),
            ),
            end_of_life_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endOfLifeDate").unwrap(),
            ),
            exclude_from_latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeFromLatest").unwrap(),
            ),
            gallery_application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryApplicationId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            manage_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manageAction").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            package_file: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageFile").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRegions").unwrap(),
            ),
        }
    }
}