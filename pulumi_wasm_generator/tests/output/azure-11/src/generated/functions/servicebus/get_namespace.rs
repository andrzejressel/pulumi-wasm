pub mod get_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// Specifies the name of the ServiceBus Namespace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the ServiceBus Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// The capacity of the ServiceBus Namespace.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// The URL to access the ServiceBus Namespace.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The location of the Resource Group in which the ServiceBus Namespace exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The messaging partitions of the ServiceBus Namespace.
        pub premium_messaging_partitions: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Tier used for the ServiceBus Namespace.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNamespaceArgs) -> GetNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getNamespace:getNamespace".into(),
            version: super::super::super::get_version(),
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
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "premiumMessagingPartitions".into(),
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
        GetNamespaceResult {
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            default_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryConnectionString").unwrap(),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryKey").unwrap(),
            ),
            default_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryConnectionString").unwrap(),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryKey").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            premium_messaging_partitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("premiumMessagingPartitions").unwrap(),
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
