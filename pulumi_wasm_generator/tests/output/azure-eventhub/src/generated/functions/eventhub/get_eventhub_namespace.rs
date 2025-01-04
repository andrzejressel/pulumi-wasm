pub mod get_eventhub_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEventhubNamespaceArgs {
        /// The name of the EventHub Namespace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the EventHub Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetEventhubNamespaceResult {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        pub auto_inflate_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Capacity / Throughput Units for a `Standard` SKU namespace.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The ID of the EventHub Dedicated Cluster where this Namespace exists.
        pub dedicated_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kafka_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure location where the EventHub Namespace exists
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled.
        pub maximum_throughput_units: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the EventHub Namespace.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEventhubNamespaceArgs) -> GetEventhubNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventhub/getEventhubNamespace:getEventhubNamespace".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoInflateEnabled".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedClusterId".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kafkaEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maximumThroughputUnits".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEventhubNamespaceResult {
            auto_inflate_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoInflateEnabled").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            dedicated_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedClusterId").unwrap(),
            ),
            default_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryConnectionString").unwrap(),
            ),
            default_primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryConnectionStringAlias").unwrap(),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryKey").unwrap(),
            ),
            default_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryConnectionString").unwrap(),
            ),
            default_secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryConnectionStringAlias").unwrap(),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryKey").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kafka_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maximum_throughput_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumThroughputUnits").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
