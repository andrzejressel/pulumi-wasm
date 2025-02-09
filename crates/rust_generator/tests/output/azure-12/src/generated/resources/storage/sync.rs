/// Manages a Storage Sync.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSync:
///     type: azure:storage:Sync
///     name: example
///     properties:
///       name: example-storage-sync
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Storage Syncs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/sync:Sync example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StorageSync/storageSyncServices/sync1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sync {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SyncArgs {
        /// Incoming traffic policy. Possible values are `AllowAllTraffic` and `AllowVirtualNetworksOnly`. Defaults to `AllowAllTraffic`.
        #[builder(into, default)]
        pub incoming_traffic_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Storage Sync. Changing this forces a new Storage Sync to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Storage Sync.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SyncResult {
        /// Incoming traffic policy. Possible values are `AllowAllTraffic` and `AllowVirtualNetworksOnly`. Defaults to `AllowAllTraffic`.
        pub incoming_traffic_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Storage Sync. Changing this forces a new Storage Sync to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of registered servers owned by this Storage Sync.
        pub registered_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the Resource Group where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Storage Sync.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SyncArgs,
    ) -> SyncResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let incoming_traffic_policy_binding = args
            .incoming_traffic_policy
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/sync:Sync".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "incomingTrafficPolicy".into(),
                    value: incoming_traffic_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
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
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SyncResult {
            incoming_traffic_policy: o.get_field("incomingTrafficPolicy"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            registered_servers: o.get_field("registeredServers"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
