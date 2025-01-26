/// Manages a Subnet Service Endpoint Storage Policy.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("GRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplestorageacct")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnetServiceEndpointStoragePolicy = subnet_service_endpoint_storage_policy::create(
///         "exampleSubnetServiceEndpointStoragePolicy",
///         SubnetServiceEndpointStoragePolicyArgs::builder()
///             .definitions(
///                 vec![
///                     SubnetServiceEndpointStoragePolicyDefinition::builder()
///                     .description("definition1").name("name1")
///                     .service("Microsoft.Storage").serviceResources(vec!["${example.id}",
///                     "${exampleAccount.id}",]).build_struct(),
///                     SubnetServiceEndpointStoragePolicyDefinition::builder()
///                     .description("definition2").name("name2").service("Global")
///                     .serviceResources(vec!["/services/Azure", "/services/Azure/Batch",
///                     "/services/Azure/DataFactory", "/services/Azure/MachineLearning",
///                     "/services/Azure/ManagedInstance", "/services/Azure/WebPI",])
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-policy")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Subnet Service Endpoint Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/subnetServiceEndpointStoragePolicy:SubnetServiceEndpointStoragePolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/serviceEndpointPolicies/policy1
/// ```
///
pub mod subnet_service_endpoint_storage_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetServiceEndpointStoragePolicyArgs {
        /// A `definition` block as defined below
        #[builder(into, default)]
        pub definitions: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::SubnetServiceEndpointStoragePolicyDefinition,
                >,
            >,
        >,
        /// The Azure Region where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Subnet Service Endpoint Storage Policy. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Subnet Service Endpoint Storage Policy.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SubnetServiceEndpointStoragePolicyResult {
        /// A `definition` block as defined below
        pub definitions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::network::SubnetServiceEndpointStoragePolicyDefinition,
                >,
            >,
        >,
        /// The Azure Region where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Subnet Service Endpoint Storage Policy. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Subnet Service Endpoint Storage Policy.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubnetServiceEndpointStoragePolicyArgs,
    ) -> SubnetServiceEndpointStoragePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definitions_binding = args.definitions.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/subnetServiceEndpointStoragePolicy:SubnetServiceEndpointStoragePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definitions".into(),
                    value: &definitions_binding,
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
                    name: "definitions".into(),
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetServiceEndpointStoragePolicyResult {
            definitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definitions").unwrap(),
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
        }
    }
}
