pub mod get_shared_image_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageVersionArgs {
        /// The name of the Shared Image Gallery in which the Shared Image exists.
        #[builder(into)]
        pub gallery_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Shared Image in which this Version exists.
        #[builder(into)]
        pub image_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Image Version.
        ///
        /// > **Note:** You may specify `latest` to obtain the latest version or `recent` to obtain the most recently updated version.
        ///
        /// > **Note:** In 3.0, `latest` may return an image version with `exclude_from_latest` set to `true`. Starting from 4.0 onwards `latest` will not return image versions with `exlude_from_latest` set to `true`.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Sort available versions taking SemVer versioning scheme into account. Defaults to `false`.
        #[builder(into, default)]
        pub sort_versions_by_semver: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags assigned to the Shared Image.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSharedImageVersionResult {
        /// Is this Image Version excluded from the `latest` filter?
        pub exclude_from_latest: pulumi_wasm_rust::Output<bool>,
        pub gallery_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_name: pulumi_wasm_rust::Output<String>,
        /// The supported Azure location where the Shared Image Gallery exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Managed Image which was the source of this Shared Image Version.
        pub managed_image_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which this Image Version exists.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The size of the OS disk snapshot (in Gigabytes) which was the source of this Shared Image Version.
        pub os_disk_image_size_gb: pulumi_wasm_rust::Output<i32>,
        /// The ID of the OS disk snapshot which was the source of this Shared Image Version.
        pub os_disk_snapshot_id: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub sort_versions_by_semver: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags assigned to the Shared Image.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as documented below.
        pub target_regions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSharedImageVersionTargetRegion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSharedImageVersionArgs) -> GetSharedImageVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let gallery_name_binding = args.gallery_name.get_inner();
        let image_name_binding = args.image_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sort_versions_by_semver_binding = args.sort_versions_by_semver.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSharedImageVersion:getSharedImageVersion".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "galleryName".into(),
                    value: &gallery_name_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sortVersionsBySemver".into(),
                    value: &sort_versions_by_semver_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "excludeFromLatest".into(),
                },
                register_interface::ResultField {
                    name: "galleryName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedImageId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "osDiskImageSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "osDiskSnapshotId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sortVersionsBySemver".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetRegions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSharedImageVersionResult {
            exclude_from_latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeFromLatest").unwrap(),
            ),
            gallery_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedImageId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            os_disk_image_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osDiskImageSizeGb").unwrap(),
            ),
            os_disk_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osDiskSnapshotId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sort_versions_by_semver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sortVersionsBySemver").unwrap(),
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
