/// Manages a Version of a Shared Image within a Shared Image Gallery.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:compute:SharedImageVersion
///     properties:
///       name: 0.0.1
///       galleryName: ${existingGetSharedImage.galleryName}
///       imageName: ${existingGetSharedImage.name}
///       resourceGroupName: ${existingGetSharedImage.resourceGroupName}
///       location: ${existingGetSharedImage.location}
///       managedImageId: ${existing.id}
///       targetRegions:
///         - name: ${existingGetSharedImage.location}
///           regionalReplicaCount: 5
///           storageAccountType: Standard_LRS
/// variables:
///   existing:
///     fn::invoke:
///       function: azure:compute:getImage
///       arguments:
///         name: search-api
///         resourceGroupName: packerimages
///   existingGetSharedImage:
///     fn::invoke:
///       function: azure:compute:getSharedImage
///       arguments:
///         name: existing-image
///         galleryName: existing_gallery
///         resourceGroupName: existing-resources
/// ```
///
/// ## Import
///
/// Shared Image Versions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/sharedImageVersion:SharedImageVersion version /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/galleries/gallery1/images/image1/versions/1.2.3
/// ```
///
pub mod shared_image_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedImageVersionArgs {
        /// URI of the Azure Storage Blob used to create the Image Version. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** You must specify exact one of `blob_uri`, `managed_image_id` and `os_disk_snapshot_id`.
        ///
        /// > **NOTE:** `blob_uri` and `storage_account_id` must be specified together
        #[builder(into, default)]
        pub blob_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether this Shared Image Version can be deleted from the Azure Regions this is replicated to. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub deletion_of_replicated_locations_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The end of life date in RFC3339 format of the Image Version.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should this Image Version be excluded from the `latest` filter? If set to `true` this Image Version won't be returned for the `latest` version. Defaults to `false`.
        #[builder(into, default)]
        pub exclude_from_latest: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Shared Image Gallery in which the Shared Image exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Shared Image within the Shared Image Gallery in which this Version should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Managed Image or Virtual Machine ID which should be used for this Shared Image Version. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The ID can be sourced from the `azure.compute.Image` data source or resource
        ///
        /// > **NOTE:** You must specify exact one of `blob_uri`, `managed_image_id` and `os_disk_snapshot_id`.
        #[builder(into, default)]
        pub managed_image_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version number for this Image Version, such as `1.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the OS disk snapshot which should be used for this Shared Image Version. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** You must specify exact one of `blob_uri`, `managed_image_id` and `os_disk_snapshot_id`.
        #[builder(into, default)]
        pub os_disk_snapshot_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mode to be used for replication. Possible values are `Full` and `Shallow`. Defaults to `Full`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub replication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Storage Account where the Blob exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `blob_uri` and `storage_account_id` must be specified together
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A collection of tags which should be applied to this resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as documented below.
        #[builder(into)]
        pub target_regions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::SharedImageVersionTargetRegion>,
        >,
    }
    #[allow(dead_code)]
    pub struct SharedImageVersionResult {
        /// URI of the Azure Storage Blob used to create the Image Version. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** You must specify exact one of `blob_uri`, `managed_image_id` and `os_disk_snapshot_id`.
        ///
        /// > **NOTE:** `blob_uri` and `storage_account_id` must be specified together
        pub blob_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether this Shared Image Version can be deleted from the Azure Regions this is replicated to. Defaults to `false`. Changing this forces a new resource to be created.
        pub deletion_of_replicated_locations_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The end of life date in RFC3339 format of the Image Version.
        pub end_of_life_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should this Image Version be excluded from the `latest` filter? If set to `true` this Image Version won't be returned for the `latest` version. Defaults to `false`.
        pub exclude_from_latest: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Shared Image Gallery in which the Shared Image exists. Changing this forces a new resource to be created.
        pub gallery_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Shared Image within the Shared Image Gallery in which this Version should be created. Changing this forces a new resource to be created.
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Managed Image or Virtual Machine ID which should be used for this Shared Image Version. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The ID can be sourced from the `azure.compute.Image` data source or resource
        ///
        /// > **NOTE:** You must specify exact one of `blob_uri`, `managed_image_id` and `os_disk_snapshot_id`.
        pub managed_image_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version number for this Image Version, such as `1.0.0`. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the OS disk snapshot which should be used for this Shared Image Version. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** You must specify exact one of `blob_uri`, `managed_image_id` and `os_disk_snapshot_id`.
        pub os_disk_snapshot_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Mode to be used for replication. Possible values are `Full` and `Shallow`. Defaults to `Full`. Changing this forces a new resource to be created.
        pub replication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Storage Account where the Blob exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `blob_uri` and `storage_account_id` must be specified together
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A collection of tags which should be applied to this resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more `target_region` blocks as documented below.
        pub target_regions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::SharedImageVersionTargetRegion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SharedImageVersionArgs,
    ) -> SharedImageVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let blob_uri_binding = args.blob_uri.get_output(context).get_inner();
        let deletion_of_replicated_locations_enabled_binding = args
            .deletion_of_replicated_locations_enabled
            .get_output(context)
            .get_inner();
        let end_of_life_date_binding = args
            .end_of_life_date
            .get_output(context)
            .get_inner();
        let exclude_from_latest_binding = args
            .exclude_from_latest
            .get_output(context)
            .get_inner();
        let gallery_name_binding = args.gallery_name.get_output(context).get_inner();
        let image_name_binding = args.image_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let managed_image_id_binding = args
            .managed_image_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let os_disk_snapshot_id_binding = args
            .os_disk_snapshot_id
            .get_output(context)
            .get_inner();
        let replication_mode_binding = args
            .replication_mode
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_regions_binding = args.target_regions.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/sharedImageVersion:SharedImageVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blobUri".into(),
                    value: &blob_uri_binding,
                },
                register_interface::ObjectField {
                    name: "deletionOfReplicatedLocationsEnabled".into(),
                    value: &deletion_of_replicated_locations_enabled_binding,
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
                    name: "galleryName".into(),
                    value: &gallery_name_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedImageId".into(),
                    value: &managed_image_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "osDiskSnapshotId".into(),
                    value: &os_disk_snapshot_id_binding,
                },
                register_interface::ObjectField {
                    name: "replicationMode".into(),
                    value: &replication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SharedImageVersionResult {
            blob_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blobUri"),
            ),
            deletion_of_replicated_locations_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionOfReplicatedLocationsEnabled"),
            ),
            end_of_life_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endOfLifeDate"),
            ),
            exclude_from_latest: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludeFromLatest"),
            ),
            gallery_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("galleryName"),
            ),
            image_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_image_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedImageId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            os_disk_snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osDiskSnapshotId"),
            ),
            replication_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationMode"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            target_regions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetRegions"),
            ),
        }
    }
}
