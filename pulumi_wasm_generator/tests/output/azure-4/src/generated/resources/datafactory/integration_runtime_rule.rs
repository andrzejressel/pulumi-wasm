/// Manages a Data Factory Azure Integration Runtime.
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
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleIntegrationRuntimeRule = integration_runtime_rule::create(
///         "exampleIntegrationRuntimeRule",
///         IntegrationRuntimeRuleArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .location("${example.location}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Azure Integration Runtimes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/integrationRuntimeRule:IntegrationRuntimeRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/integrationruntimes/example
/// ```
///
pub mod integration_runtime_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeRuleArgs {
        /// Cluster will not be recycled and it will be used in next data flow activity run until TTL (time to live) is reached if this is set as `false`. Defaults to `true`.
        #[builder(into, default)]
        pub cleanup_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Compute type of the cluster which will execute data flow job. Valid values are `General`, `ComputeOptimized` and `MemoryOptimized`. Defaults to `General`.
        #[builder(into, default)]
        pub compute_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Core count of the cluster which will execute data flow job. Valid values are `8`, `16`, `32`, `48`, `80`, `144` and `272`. Defaults to `8`.
        #[builder(into, default)]
        pub core_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Use `AutoResolve` to create an auto-resolve integration runtime. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Managed Integration Runtime. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Time to live (in minutes) setting of the cluster which will execute data flow job. Defaults to `0`.
        #[builder(into, default)]
        pub time_to_live_min: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Is Integration Runtime compute provisioned within Managed Virtual Network? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub virtual_network_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeRuleResult {
        /// Cluster will not be recycled and it will be used in next data flow activity run until TTL (time to live) is reached if this is set as `false`. Defaults to `true`.
        pub cleanup_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Compute type of the cluster which will execute data flow job. Valid values are `General`, `ComputeOptimized` and `MemoryOptimized`. Defaults to `General`.
        pub compute_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Core count of the cluster which will execute data flow job. Valid values are `8`, `16`, `32`, `48`, `80`, `144` and `272`. Defaults to `8`.
        pub core_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Integration runtime description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Use `AutoResolve` to create an auto-resolve integration runtime. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Managed Integration Runtime. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Time to live (in minutes) setting of the cluster which will execute data flow job. Defaults to `0`.
        pub time_to_live_min: pulumi_wasm_rust::Output<Option<i32>>,
        /// Is Integration Runtime compute provisioned within Managed Virtual Network? Changing this forces a new resource to be created.
        pub virtual_network_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IntegrationRuntimeRuleArgs,
    ) -> IntegrationRuntimeRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cleanup_enabled_binding = args
            .cleanup_enabled
            .get_output(context)
            .get_inner();
        let compute_type_binding = args.compute_type.get_output(context).get_inner();
        let core_count_binding = args.core_count.get_output(context).get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let time_to_live_min_binding = args
            .time_to_live_min
            .get_output(context)
            .get_inner();
        let virtual_network_enabled_binding = args
            .virtual_network_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/integrationRuntimeRule:IntegrationRuntimeRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cleanupEnabled".into(),
                    value: &cleanup_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "computeType".into(),
                    value: &compute_type_binding,
                },
                register_interface::ObjectField {
                    name: "coreCount".into(),
                    value: &core_count_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "timeToLiveMin".into(),
                    value: &time_to_live_min_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkEnabled".into(),
                    value: &virtual_network_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cleanupEnabled".into(),
                },
                register_interface::ResultField {
                    name: "computeType".into(),
                },
                register_interface::ResultField {
                    name: "coreCount".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "timeToLiveMin".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationRuntimeRuleResult {
            cleanup_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cleanupEnabled").unwrap(),
            ),
            compute_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeType").unwrap(),
            ),
            core_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreCount").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            time_to_live_min: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeToLiveMin").unwrap(),
            ),
            virtual_network_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkEnabled").unwrap(),
            ),
        }
    }
}
