pub mod get_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// Specifies the Name of the Notification Hub Namespace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Name of the Resource Group within which the Notification Hub exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// Is this Notification Hub Namespace enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region in which this Notification Hub Namespace exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the SKU to use for this Notification Hub Namespace. Possible values are `Free`, `Basic` or `Standard.`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Type of Namespace, such as `Messaging` or `NotificationHub`.
        pub namespace_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub servicebus_endpoint: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::super::types::notificationhub::GetNamespaceSku,
        >,
        /// A mapping of tags to assign to the resource.
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
            token: "azure:notificationhub/getNamespace:getNamespace".into(),
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
                    name: "enabled".into(),
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
                    name: "namespaceType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "servicebusEndpoint".into(),
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
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            servicebus_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicebusEndpoint").unwrap(),
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