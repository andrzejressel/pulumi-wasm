/// Manages a Data Factory Managed Private Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_kind("BlobStorage")
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .managed_virtual_network_enabled(true)
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleManagedPrivateEndpoint = managed_private_endpoint::create(
///         "exampleManagedPrivateEndpoint",
///         ManagedPrivateEndpointArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .subresource_name("blob")
///             .target_resource_id("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Managed Private Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/managedPrivateEndpoint:ManagedPrivateEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/managedVirtualNetworks/default/managedPrivateEndpoints/endpoint1
/// ```
///
pub mod managed_private_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPrivateEndpointArgs {
        /// The ID of the Data Factory on which to create the Managed Private Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Fully qualified domain names. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Possible values are listed in [documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-overview#dns-configuration).
        #[builder(into, default)]
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the name which should be used for this Managed Private Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the sub resource name which the Data Factory Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subresource_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Private Link Enabled Remote Resource which this Data Factory Private Endpoint should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedPrivateEndpointResult {
        /// The ID of the Data Factory on which to create the Managed Private Endpoint. Changing this forces a new resource to be created.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Fully qualified domain names. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Possible values are listed in [documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-overview#dns-configuration).
        pub fqdns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the name which should be used for this Managed Private Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the sub resource name which the Data Factory Private Endpoint is able to connect to. Changing this forces a new resource to be created.
        pub subresource_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Private Link Enabled Remote Resource which this Data Factory Private Endpoint should be connected to. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedPrivateEndpointArgs,
    ) -> ManagedPrivateEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let fqdns_binding = args.fqdns.get_inner();
        let name_binding = args.name.get_inner();
        let subresource_name_binding = args.subresource_name.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/managedPrivateEndpoint:ManagedPrivateEndpoint"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "fqdns".into(),
                    value: &fqdns_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subresourceName".into(),
                    value: &subresource_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "fqdns".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subresourceName".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedPrivateEndpointResult {
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            fqdns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdns").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subresource_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subresourceName").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
        }
    }
}