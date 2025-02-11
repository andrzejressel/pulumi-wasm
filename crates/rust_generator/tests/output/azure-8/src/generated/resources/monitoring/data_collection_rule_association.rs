/// Manages a Data Collection Rule Association.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let example1 = data_collection_rule_association::create(
///         "example1",
///         DataCollectionRuleAssociationArgs::builder()
///             .data_collection_rule_id("${exampleDataCollectionRule.id}")
///             .description("example")
///             .name("example1-dcra")
///             .target_resource_id("${exampleLinuxVirtualMachine.id}")
///             .build_struct(),
///     );
///     let example2 = data_collection_rule_association::create(
///         "example2",
///         DataCollectionRuleAssociationArgs::builder()
///             .data_collection_endpoint_id("${exampleDataCollectionEndpoint.id}")
///             .description("example")
///             .target_resource_id("${exampleLinuxVirtualMachine.id}")
///             .build_struct(),
///     );
///     let exampleDataCollectionEndpoint = data_collection_endpoint::create(
///         "exampleDataCollectionEndpoint",
///         DataCollectionEndpointArgs::builder()
///             .location("${example.location}")
///             .name("example-dce")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleDataCollectionRule = data_collection_rule::create(
///         "exampleDataCollectionRule",
///         DataCollectionRuleArgs::builder()
///             .data_flows(
///                 vec![
///                     DataCollectionRuleDataFlow::builder()
///                     .destinations(vec!["example-destination-metrics",])
///                     .streams(vec!["Microsoft-InsightsMetrics",]).build_struct(),
///                 ],
///             )
///             .destinations(
///                 DataCollectionRuleDestinations::builder()
///                     .azureMonitorMetrics(
///                         DataCollectionRuleDestinationsAzureMonitorMetrics::builder()
///                             .name("example-destination-metrics")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .name("example-dcr")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinuxVirtualMachine = linux_virtual_machine::create(
///         "exampleLinuxVirtualMachine",
///         LinuxVirtualMachineArgs::builder()
///             .admin_password("example-Password@7890")
///             .admin_username("adminuser")
///             .disable_password_authentication(false)
///             .location("${example.location}")
///             .name("machine")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_disk(
///                 LinuxVirtualMachineOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .size("Standard_B1ls")
///             .source_image_reference(
///                 LinuxVirtualMachineSourceImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("internal")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("virtualnetwork")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Collection Rules Association can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/dataCollectionRuleAssociation:DataCollectionRuleAssociation example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/vm1/providers/Microsoft.Insights/dataCollectionRuleAssociations/dca1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_collection_rule_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCollectionRuleAssociationArgs {
        /// The ID of the Data Collection Endpoint which will be associated to the target resource.
        #[builder(into, default)]
        pub data_collection_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Data Collection Rule which will be associated to the target resource.
        ///
        /// > **NOTE** Exactly one of `data_collection_endpoint_id` and `data_collection_rule_id` blocks must be specified.
        #[builder(into, default)]
        pub data_collection_rule_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the Data Collection Rule Association.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Data Collection Rule Association. Changing this forces a new Data Collection Rule Association to be created. Defaults to `configurationAccessEndpoint`.
        ///
        /// > **NOTE** `name` is required when `data_collection_rule_id` is specified. And when `data_collection_endpoint_id` is specified, the `name` is populated with `configurationAccessEndpoint`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Azure Resource which to associate to a Data Collection Rule or a Data Collection Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataCollectionRuleAssociationResult {
        /// The ID of the Data Collection Endpoint which will be associated to the target resource.
        pub data_collection_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Data Collection Rule which will be associated to the target resource.
        ///
        /// > **NOTE** Exactly one of `data_collection_endpoint_id` and `data_collection_rule_id` blocks must be specified.
        pub data_collection_rule_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the Data Collection Rule Association.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Data Collection Rule Association. Changing this forces a new Data Collection Rule Association to be created. Defaults to `configurationAccessEndpoint`.
        ///
        /// > **NOTE** `name` is required when `data_collection_rule_id` is specified. And when `data_collection_endpoint_id` is specified, the `name` is populated with `configurationAccessEndpoint`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure Resource which to associate to a Data Collection Rule or a Data Collection Endpoint. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataCollectionRuleAssociationArgs,
    ) -> DataCollectionRuleAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_collection_endpoint_id_binding = args
            .data_collection_endpoint_id
            .get_output(context);
        let data_collection_rule_id_binding = args
            .data_collection_rule_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let target_resource_id_binding = args.target_resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:monitoring/dataCollectionRuleAssociation:DataCollectionRuleAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCollectionEndpointId".into(),
                    value: &data_collection_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCollectionRuleId".into(),
                    value: &data_collection_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataCollectionRuleAssociationResult {
            data_collection_endpoint_id: o.get_field("dataCollectionEndpointId"),
            data_collection_rule_id: o.get_field("dataCollectionRuleId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            target_resource_id: o.get_field("targetResourceId"),
        }
    }
}
