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
pub mod sync {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SyncArgs {
        /// Incoming traffic policy. Possible values are `AllowAllTraffic` and `AllowVirtualNetworksOnly`. Defaults to `AllowAllTraffic`.
        #[builder(into, default)]
        pub incoming_traffic_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Storage Sync. Changing this forces a new Storage Sync to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Storage Sync.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SyncResult {
        /// Incoming traffic policy. Possible values are `AllowAllTraffic` and `AllowVirtualNetworksOnly`. Defaults to `AllowAllTraffic`.
        pub incoming_traffic_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Storage Sync. Changing this forces a new Storage Sync to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of registered servers owned by this Storage Sync.
        pub registered_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the Resource Group where the Storage Sync should exist. Changing this forces a new Storage Sync to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Storage Sync.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SyncArgs) -> SyncResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let incoming_traffic_policy_binding = args.incoming_traffic_policy.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/sync:Sync".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "incomingTrafficPolicy".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "registeredServers".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SyncResult {
            incoming_traffic_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("incomingTrafficPolicy").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            registered_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registeredServers").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
