/// Manages a Subnet Service Endpoint Storage Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subnet_service_endpoint_storage_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetServiceEndpointStoragePolicyArgs {
        /// A `definition` block as defined below
        #[builder(into, default)]
        pub definitions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::network::SubnetServiceEndpointStoragePolicyDefinition,
                >,
            >,
        >,
        /// The Azure Region where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Subnet Service Endpoint Storage Policy. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Subnet Service Endpoint Storage Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SubnetServiceEndpointStoragePolicyResult {
        /// A `definition` block as defined below
        pub definitions: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::network::SubnetServiceEndpointStoragePolicyDefinition,
                >,
            >,
        >,
        /// The Azure Region where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Subnet Service Endpoint Storage Policy. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Subnet Service Endpoint Storage Policy should exist. Changing this forces a new Subnet Service Endpoint Storage Policy to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Subnet Service Endpoint Storage Policy.
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
        args: SubnetServiceEndpointStoragePolicyArgs,
    ) -> SubnetServiceEndpointStoragePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let definitions_binding = args.definitions.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/subnetServiceEndpointStoragePolicy:SubnetServiceEndpointStoragePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definitions".into(),
                    value: definitions_binding.get_id(),
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
        SubnetServiceEndpointStoragePolicyResult {
            definitions: o.get_field("definitions"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
