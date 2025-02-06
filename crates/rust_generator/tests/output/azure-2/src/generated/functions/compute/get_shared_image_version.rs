pub mod get_shared_image_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageVersionArgs {
        /// The name of the Shared Image Gallery in which the Shared Image exists.
        #[builder(into)]
        pub gallery_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Shared Image in which this Version exists.
        #[builder(into)]
        pub image_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Image Version.
        ///
        /// > **Note:** You may specify `latest` to obtain the latest version or `recent` to obtain the most recently updated version.
        ///
        /// > **Note:** In 3.0, `latest` may return an image version with `exclude_from_latest` set to `true`. Starting from 4.0 onwards `latest` will not return image versions with `exlude_from_latest` set to `true`.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Sort available versions taking SemVer versioning scheme into account. Defaults to `false`.
        #[builder(into, default)]
        pub sort_versions_by_semver: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags assigned to the Shared Image.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSharedImageVersionArgs,
    ) -> GetSharedImageVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let gallery_name_binding = args.gallery_name.get_output(context).get_inner();
        let image_name_binding = args.image_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sort_versions_by_semver_binding = args
            .sort_versions_by_semver
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSharedImageVersion:getSharedImageVersion".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSharedImageVersionResult {
            exclude_from_latest: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludeFromLatest"),
            ),
            gallery_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("galleryName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageName"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_image_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedImageId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            os_disk_image_size_gb: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("osDiskImageSizeGb"),
            ),
            os_disk_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("osDiskSnapshotId"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sort_versions_by_semver: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sortVersionsBySemver"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            target_regions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetRegions"),
            ),
        }
    }
}
