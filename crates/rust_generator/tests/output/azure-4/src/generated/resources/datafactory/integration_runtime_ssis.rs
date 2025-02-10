/// Manages a Data Factory Azure-SSIS Integration Runtime.
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
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleIntegrationRuntimeSsis = integration_runtime_ssis::create(
///         "exampleIntegrationRuntimeSsis",
///         IntegrationRuntimeSsisArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .location("${example.location}")
///             .name("example")
///             .node_size("Standard_D8_v3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Azure-SSIS Integration Runtimes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/integrationRuntimeSsis:IntegrationRuntimeSsis example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/integrationruntimes/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_runtime_ssis {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSsisArgs {
        /// A `catalog_info` block as defined below.
        #[builder(into, default)]
        pub catalog_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisCatalogInfo>,
        >,
        /// One `copy_compute_scale` block as defined below.
        #[builder(into, default)]
        pub copy_compute_scale: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCopyComputeScale,
            >,
        >,
        /// The name of a Data Factory Credential that the SSIS integration will use to access data sources. For example, `azure.datafactory.CredentialUserManagedIdentity`
        ///
        /// > **NOTE** If `credential_name` is omitted, the integration runtime will use the Data Factory assigned identity.
        #[builder(into, default)]
        pub credential_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `custom_setup_script` block as defined below.
        #[builder(into, default)]
        pub custom_setup_script: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCustomSetupScript,
            >,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure-SSIS Integration Runtime edition. Valid values are `Standard` and `Enterprise`. Defaults to `Standard`.
        #[builder(into, default)]
        pub edition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `express_custom_setup` block as defined below.
        #[builder(into, default)]
        pub express_custom_setup: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetup,
            >,
        >,
        /// A `express_vnet_integration` block as defined below.
        #[builder(into, default)]
        pub express_vnet_integration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressVnetIntegration,
            >,
        >,
        /// The type of the license that is used. Valid values are `LicenseIncluded` and `BasePrice`. Defaults to `LicenseIncluded`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the maximum parallel executions per node. Defaults to `1`. Max is `1`.
        #[builder(into, default)]
        pub max_parallel_executions_per_node: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Specifies the name of the Azure-SSIS Integration Runtime. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The size of the nodes on which the Azure-SSIS Integration Runtime runs. Valid values are: `Standard_D2_v3`, `Standard_D4_v3`, `Standard_D8_v3`, `Standard_D16_v3`, `Standard_D32_v3`, `Standard_D64_v3`, `Standard_E2_v3`, `Standard_E4_v3`, `Standard_E8_v3`, `Standard_E16_v3`, `Standard_E32_v3`, `Standard_E64_v3`, `Standard_D1_v2`, `Standard_D2_v2`, `Standard_D3_v2`, `Standard_D4_v2`, `Standard_A4_v2` and `Standard_A8_v2`
        #[builder(into)]
        pub node_size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of nodes for the Azure-SSIS Integration Runtime. Max is `10`. Defaults to `1`.
        #[builder(into, default)]
        pub number_of_nodes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `package_store` block as defined below.
        #[builder(into, default)]
        pub package_stores: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::datafactory::IntegrationRuntimeSsisPackageStore>,
            >,
        >,
        /// One `pipeline_external_compute_scale` block as defined below.
        #[builder(into, default)]
        pub pipeline_external_compute_scale: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisPipelineExternalComputeScale,
            >,
        >,
        /// A `proxy` block as defined below.
        #[builder(into, default)]
        pub proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisProxy>,
        >,
        /// A `vnet_integration` block as defined below.
        #[builder(into, default)]
        pub vnet_integration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisVnetIntegration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSsisResult {
        /// A `catalog_info` block as defined below.
        pub catalog_info: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisCatalogInfo>,
        >,
        /// One `copy_compute_scale` block as defined below.
        pub copy_compute_scale: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCopyComputeScale,
            >,
        >,
        /// The name of a Data Factory Credential that the SSIS integration will use to access data sources. For example, `azure.datafactory.CredentialUserManagedIdentity`
        ///
        /// > **NOTE** If `credential_name` is omitted, the integration runtime will use the Data Factory assigned identity.
        pub credential_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `custom_setup_script` block as defined below.
        pub custom_setup_script: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCustomSetupScript,
            >,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// Integration runtime description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure-SSIS Integration Runtime edition. Valid values are `Standard` and `Enterprise`. Defaults to `Standard`.
        pub edition: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `express_custom_setup` block as defined below.
        pub express_custom_setup: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetup,
            >,
        >,
        /// A `express_vnet_integration` block as defined below.
        pub express_vnet_integration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressVnetIntegration,
            >,
        >,
        /// The type of the license that is used. Valid values are `LicenseIncluded` and `BasePrice`. Defaults to `LicenseIncluded`.
        pub license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Defines the maximum parallel executions per node. Defaults to `1`. Max is `1`.
        pub max_parallel_executions_per_node: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Azure-SSIS Integration Runtime. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The size of the nodes on which the Azure-SSIS Integration Runtime runs. Valid values are: `Standard_D2_v3`, `Standard_D4_v3`, `Standard_D8_v3`, `Standard_D16_v3`, `Standard_D32_v3`, `Standard_D64_v3`, `Standard_E2_v3`, `Standard_E4_v3`, `Standard_E8_v3`, `Standard_E16_v3`, `Standard_E32_v3`, `Standard_E64_v3`, `Standard_D1_v2`, `Standard_D2_v2`, `Standard_D3_v2`, `Standard_D4_v2`, `Standard_A4_v2` and `Standard_A8_v2`
        pub node_size: pulumi_gestalt_rust::Output<String>,
        /// Number of nodes for the Azure-SSIS Integration Runtime. Max is `10`. Defaults to `1`.
        pub number_of_nodes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `package_store` block as defined below.
        pub package_stores: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::datafactory::IntegrationRuntimeSsisPackageStore>,
            >,
        >,
        /// One `pipeline_external_compute_scale` block as defined below.
        pub pipeline_external_compute_scale: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisPipelineExternalComputeScale,
            >,
        >,
        /// A `proxy` block as defined below.
        pub proxy: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisProxy>,
        >,
        /// A `vnet_integration` block as defined below.
        pub vnet_integration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisVnetIntegration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationRuntimeSsisArgs,
    ) -> IntegrationRuntimeSsisResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_info_binding = args.catalog_info.get_output(context);
        let copy_compute_scale_binding = args.copy_compute_scale.get_output(context);
        let credential_name_binding = args.credential_name.get_output(context);
        let custom_setup_script_binding = args.custom_setup_script.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let edition_binding = args.edition.get_output(context);
        let express_custom_setup_binding = args.express_custom_setup.get_output(context);
        let express_vnet_integration_binding = args
            .express_vnet_integration
            .get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let max_parallel_executions_per_node_binding = args
            .max_parallel_executions_per_node
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let node_size_binding = args.node_size.get_output(context);
        let number_of_nodes_binding = args.number_of_nodes.get_output(context);
        let package_stores_binding = args.package_stores.get_output(context);
        let pipeline_external_compute_scale_binding = args
            .pipeline_external_compute_scale
            .get_output(context);
        let proxy_binding = args.proxy.get_output(context);
        let vnet_integration_binding = args.vnet_integration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/integrationRuntimeSsis:IntegrationRuntimeSsis"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogInfo".into(),
                    value: catalog_info_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyComputeScale".into(),
                    value: copy_compute_scale_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentialName".into(),
                    value: credential_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSetupScript".into(),
                    value: custom_setup_script_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edition".into(),
                    value: edition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressCustomSetup".into(),
                    value: express_custom_setup_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressVnetIntegration".into(),
                    value: express_vnet_integration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseType".into(),
                    value: license_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxParallelExecutionsPerNode".into(),
                    value: max_parallel_executions_per_node_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeSize".into(),
                    value: node_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberOfNodes".into(),
                    value: number_of_nodes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageStores".into(),
                    value: package_stores_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineExternalComputeScale".into(),
                    value: pipeline_external_compute_scale_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxy".into(),
                    value: proxy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vnetIntegration".into(),
                    value: vnet_integration_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationRuntimeSsisResult {
            catalog_info: o.get_field("catalogInfo"),
            copy_compute_scale: o.get_field("copyComputeScale"),
            credential_name: o.get_field("credentialName"),
            custom_setup_script: o.get_field("customSetupScript"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            edition: o.get_field("edition"),
            express_custom_setup: o.get_field("expressCustomSetup"),
            express_vnet_integration: o.get_field("expressVnetIntegration"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            max_parallel_executions_per_node: o
                .get_field("maxParallelExecutionsPerNode"),
            name: o.get_field("name"),
            node_size: o.get_field("nodeSize"),
            number_of_nodes: o.get_field("numberOfNodes"),
            package_stores: o.get_field("packageStores"),
            pipeline_external_compute_scale: o.get_field("pipelineExternalComputeScale"),
            proxy: o.get_field("proxy"),
            vnet_integration: o.get_field("vnetIntegration"),
        }
    }
}
