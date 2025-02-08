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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SyncArgs,
    ) -> SyncResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let incoming_traffic_policy_binding = args
            .incoming_traffic_policy
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/sync:Sync".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "incomingTrafficPolicy".into(),
                    value: &incoming_traffic_policy_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SyncResult {
            incoming_traffic_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("incomingTrafficPolicy"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            registered_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registeredServers"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
