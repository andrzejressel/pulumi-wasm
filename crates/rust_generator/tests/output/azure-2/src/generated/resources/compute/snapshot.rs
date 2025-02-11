/// Manages a Disk Snapshot.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: snapshot-rg
///       location: West Europe
///   exampleManagedDisk:
///     type: azure:compute:ManagedDisk
///     name: example
///     properties:
///       name: managed-disk
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageAccountType: Standard_LRS
///       createOption: Empty
///       diskSizeGb: '10'
///   exampleSnapshot:
///     type: azure:compute:Snapshot
///     name: example
///     properties:
///       name: snapshot
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       createOption: Copy
///       sourceUri: ${exampleManagedDisk.id}
/// ```
///
/// ## Import
///
/// Snapshots can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/snapshot:Snapshot example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/snapshots/snapshot1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// Indicates how the snapshot is to be created. Possible values are `Copy` or `Import`.
        ///
        /// > **Note:** One of `source_uri`, `source_resource_id` or `storage_account_id` must be specified.
        #[builder(into)]
        pub create_option: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the Disk Access which should be used for this Snapshot. This is used in conjunction with setting `network_access_policy` to `AllowPrivate`.
        #[builder(into, default)]
        pub disk_access_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The size of the Snapshotted Disk in GB.
        #[builder(into, default)]
        pub disk_size_gb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A `encryption_settings` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_settings` forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SnapshotEncryptionSettings>,
        >,
        /// Specifies if the Snapshot is incremental. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub incremental_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Snapshot resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy for accessing the disk via network. Possible values are `AllowAll`, `AllowPrivate`, or `DenyAll`. Defaults to `AllowAll`.
        #[builder(into, default)]
        pub network_access_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy for controlling export on the disk. Possible values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the Snapshot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a reference to an existing snapshot, when `create_option` is `Copy`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_resource_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the URI to a Managed or Unmanaged Disk. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of an storage account. Used with `source_uri` to allow authorization during import of unmanaged blobs from a different subscription. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Indicates how the snapshot is to be created. Possible values are `Copy` or `Import`.
        ///
        /// > **Note:** One of `source_uri`, `source_resource_id` or `storage_account_id` must be specified.
        pub create_option: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Disk Access which should be used for this Snapshot. This is used in conjunction with setting `network_access_policy` to `AllowPrivate`.
        pub disk_access_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The size of the Snapshotted Disk in GB.
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// A `encryption_settings` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_settings` forces a new resource to be created.
        pub encryption_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SnapshotEncryptionSettings>,
        >,
        /// Specifies if the Snapshot is incremental. Changing this forces a new resource to be created.
        pub incremental_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Snapshot resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Policy for accessing the disk via network. Possible values are `AllowAll`, `AllowPrivate`, or `DenyAll`. Defaults to `AllowAll`.
        pub network_access_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Policy for controlling export on the disk. Possible values are `true` or `false`. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Snapshot. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a reference to an existing snapshot, when `create_option` is `Copy`. Changing this forces a new resource to be created.
        pub source_resource_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the URI to a Managed or Unmanaged Disk. Changing this forces a new resource to be created.
        pub source_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of an storage account. Used with `source_uri` to allow authorization during import of unmanaged blobs from a different subscription. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether Trusted Launch is enabled for the Snapshot.
        pub trusted_launch_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let create_option_binding = args.create_option.get_output(context);
        let disk_access_id_binding = args.disk_access_id.get_output(context);
        let disk_size_gb_binding = args.disk_size_gb.get_output(context);
        let encryption_settings_binding = args.encryption_settings.get_output(context);
        let incremental_enabled_binding = args.incremental_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_access_policy_binding = args
            .network_access_policy
            .get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let source_resource_id_binding = args.source_resource_id.get_output(context);
        let source_uri_binding = args.source_uri.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createOption".into(),
                    value: &create_option_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskAccessId".into(),
                    value: &disk_access_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskSizeGb".into(),
                    value: &disk_size_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionSettings".into(),
                    value: &encryption_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "incrementalEnabled".into(),
                    value: &incremental_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAccessPolicy".into(),
                    value: &network_access_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceResourceId".into(),
                    value: &source_resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceUri".into(),
                    value: &source_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotResult {
            create_option: o.get_field("createOption"),
            disk_access_id: o.get_field("diskAccessId"),
            disk_size_gb: o.get_field("diskSizeGb"),
            encryption_settings: o.get_field("encryptionSettings"),
            incremental_enabled: o.get_field("incrementalEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_access_policy: o.get_field("networkAccessPolicy"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_resource_id: o.get_field("sourceResourceId"),
            source_uri: o.get_field("sourceUri"),
            storage_account_id: o.get_field("storageAccountId"),
            tags: o.get_field("tags"),
            trusted_launch_enabled: o.get_field("trustedLaunchEnabled"),
        }
    }
}
