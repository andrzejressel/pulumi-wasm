/// Manages a Storage Sync Group.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleSync = sync::create(
///         "exampleSync",
///         SyncArgs::builder()
///             .location("${example.location}")
///             .name("example-ss")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSyncGroup = sync_group::create(
///         "exampleSyncGroup",
///         SyncGroupArgs::builder()
///             .name("example-ss-group")
///             .storage_sync_id("${exampleSync.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Sync Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/syncGroup:SyncGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.StorageSync/storageSyncServices/sync1/syncGroups/group1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sync_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SyncGroupArgs {
        /// The name which should be used for this Storage Sync Group. Changing this forces a new Storage Sync Group to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Storage Sync where this Storage Sync Group is. Changing this forces a new Storage Sync Group to be created.
        #[builder(into)]
        pub storage_sync_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SyncGroupResult {
        /// The name which should be used for this Storage Sync Group. Changing this forces a new Storage Sync Group to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Storage Sync where this Storage Sync Group is. Changing this forces a new Storage Sync Group to be created.
        pub storage_sync_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SyncGroupArgs,
    ) -> SyncGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let storage_sync_id_binding = args
            .storage_sync_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/syncGroup:SyncGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageSyncId".into(),
                    value: &storage_sync_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SyncGroupResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            storage_sync_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageSyncId"),
            ),
        }
    }
}
