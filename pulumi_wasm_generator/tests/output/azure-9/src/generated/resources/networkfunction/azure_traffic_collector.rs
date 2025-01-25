/// Manages a Network Function Azure Traffic Collector.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West US
///   exampleAzureTrafficCollector:
///     type: azure:networkfunction:AzureTrafficCollector
///     name: example
///     properties:
///       name: example-nfatc
///       resourceGroupName: ${example.name}
///       location: West US
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Network Function Azure Traffic Collector can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:networkfunction/azureTrafficCollector:AzureTrafficCollector example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.NetworkFunction/azureTrafficCollectors/azureTrafficCollector1
/// ```
///
pub mod azure_traffic_collector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AzureTrafficCollectorArgs {
        /// Specifies the Azure Region where the Network Function Azure Traffic Collector should exist. Changing this forces a new Network Function Azure Traffic Collector to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Network Function Azure Traffic Collector. Changing this forces a new Network Function Azure Traffic Collector to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Network Function Azure Traffic Collector should exist. Changing this forces a new Network Function Azure Traffic Collector to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Network Function Azure Traffic Collector.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AzureTrafficCollectorResult {
        /// The list of Resource IDs of collector policies.
        pub collector_policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the Azure Region where the Network Function Azure Traffic Collector should exist. Changing this forces a new Network Function Azure Traffic Collector to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Function Azure Traffic Collector. Changing this forces a new Network Function Azure Traffic Collector to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Network Function Azure Traffic Collector should exist. Changing this forces a new Network Function Azure Traffic Collector to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Network Function Azure Traffic Collector.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Resource ID of virtual hub.
        pub virtual_hub_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AzureTrafficCollectorArgs,
    ) -> AzureTrafficCollectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:networkfunction/azureTrafficCollector:AzureTrafficCollector"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "collectorPolicyIds".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubIds".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AzureTrafficCollectorResult {
            collector_policy_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectorPolicyIds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_hub_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubIds").unwrap(),
            ),
        }
    }
}
