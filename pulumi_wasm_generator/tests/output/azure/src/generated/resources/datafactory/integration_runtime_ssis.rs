/// Manages a Data Factory Azure-SSIS Integration Runtime.
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
pub mod integration_runtime_ssis {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSsisArgs {
        /// A `catalog_info` block as defined below.
        #[builder(into, default)]
        pub catalog_info: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisCatalogInfo>,
        >,
        /// One `copy_compute_scale` block as defined below.
        #[builder(into, default)]
        pub copy_compute_scale: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCopyComputeScale,
            >,
        >,
        /// The name of a Data Factory Credential that the SSIS integration will use to access data sources. For example, `azure.datafactory.CredentialUserManagedIdentity`
        ///
        /// > **NOTE** If `credential_name` is omitted, the integration runtime will use the Data Factory assigned identity.
        #[builder(into, default)]
        pub credential_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `custom_setup_script` block as defined below.
        #[builder(into, default)]
        pub custom_setup_script: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCustomSetupScript,
            >,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure-SSIS Integration Runtime edition. Valid values are `Standard` and `Enterprise`. Defaults to `Standard`.
        #[builder(into, default)]
        pub edition: pulumi_wasm_rust::Output<Option<String>>,
        /// An `express_custom_setup` block as defined below.
        #[builder(into, default)]
        pub express_custom_setup: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetup,
            >,
        >,
        /// A `express_vnet_integration` block as defined below.
        #[builder(into, default)]
        pub express_vnet_integration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressVnetIntegration,
            >,
        >,
        /// The type of the license that is used. Valid values are `LicenseIncluded` and `BasePrice`. Defaults to `LicenseIncluded`.
        #[builder(into, default)]
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the maximum parallel executions per node. Defaults to `1`. Max is `1`.
        #[builder(into, default)]
        pub max_parallel_executions_per_node: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Azure-SSIS Integration Runtime. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The size of the nodes on which the Azure-SSIS Integration Runtime runs. Valid values are: `Standard_D2_v3`, `Standard_D4_v3`, `Standard_D8_v3`, `Standard_D16_v3`, `Standard_D32_v3`, `Standard_D64_v3`, `Standard_E2_v3`, `Standard_E4_v3`, `Standard_E8_v3`, `Standard_E16_v3`, `Standard_E32_v3`, `Standard_E64_v3`, `Standard_D1_v2`, `Standard_D2_v2`, `Standard_D3_v2`, `Standard_D4_v2`, `Standard_A4_v2` and `Standard_A8_v2`
        #[builder(into)]
        pub node_size: pulumi_wasm_rust::Output<String>,
        /// Number of nodes for the Azure-SSIS Integration Runtime. Max is `10`. Defaults to `1`.
        #[builder(into, default)]
        pub number_of_nodes: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `package_store` block as defined below.
        #[builder(into, default)]
        pub package_stores: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datafactory::IntegrationRuntimeSsisPackageStore>,
            >,
        >,
        /// One `pipeline_external_compute_scale` block as defined below.
        #[builder(into, default)]
        pub pipeline_external_compute_scale: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisPipelineExternalComputeScale,
            >,
        >,
        /// A `proxy` block as defined below.
        #[builder(into, default)]
        pub proxy: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisProxy>,
        >,
        /// A `vnet_integration` block as defined below.
        #[builder(into, default)]
        pub vnet_integration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisVnetIntegration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSsisResult {
        /// A `catalog_info` block as defined below.
        pub catalog_info: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisCatalogInfo>,
        >,
        /// One `copy_compute_scale` block as defined below.
        pub copy_compute_scale: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCopyComputeScale,
            >,
        >,
        /// The name of a Data Factory Credential that the SSIS integration will use to access data sources. For example, `azure.datafactory.CredentialUserManagedIdentity`
        ///
        /// > **NOTE** If `credential_name` is omitted, the integration runtime will use the Data Factory assigned identity.
        pub credential_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `custom_setup_script` block as defined below.
        pub custom_setup_script: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisCustomSetupScript,
            >,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Integration runtime description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure-SSIS Integration Runtime edition. Valid values are `Standard` and `Enterprise`. Defaults to `Standard`.
        pub edition: pulumi_wasm_rust::Output<Option<String>>,
        /// An `express_custom_setup` block as defined below.
        pub express_custom_setup: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetup,
            >,
        >,
        /// A `express_vnet_integration` block as defined below.
        pub express_vnet_integration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisExpressVnetIntegration,
            >,
        >,
        /// The type of the license that is used. Valid values are `LicenseIncluded` and `BasePrice`. Defaults to `LicenseIncluded`.
        pub license_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Defines the maximum parallel executions per node. Defaults to `1`. Max is `1`.
        pub max_parallel_executions_per_node: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Azure-SSIS Integration Runtime. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The size of the nodes on which the Azure-SSIS Integration Runtime runs. Valid values are: `Standard_D2_v3`, `Standard_D4_v3`, `Standard_D8_v3`, `Standard_D16_v3`, `Standard_D32_v3`, `Standard_D64_v3`, `Standard_E2_v3`, `Standard_E4_v3`, `Standard_E8_v3`, `Standard_E16_v3`, `Standard_E32_v3`, `Standard_E64_v3`, `Standard_D1_v2`, `Standard_D2_v2`, `Standard_D3_v2`, `Standard_D4_v2`, `Standard_A4_v2` and `Standard_A8_v2`
        pub node_size: pulumi_wasm_rust::Output<String>,
        /// Number of nodes for the Azure-SSIS Integration Runtime. Max is `10`. Defaults to `1`.
        pub number_of_nodes: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `package_store` block as defined below.
        pub package_stores: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datafactory::IntegrationRuntimeSsisPackageStore>,
            >,
        >,
        /// One `pipeline_external_compute_scale` block as defined below.
        pub pipeline_external_compute_scale: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::IntegrationRuntimeSsisPipelineExternalComputeScale,
            >,
        >,
        /// A `proxy` block as defined below.
        pub proxy: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::IntegrationRuntimeSsisProxy>,
        >,
        /// A `vnet_integration` block as defined below.
        pub vnet_integration: pulumi_wasm_rust::Output<
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
        name: &str,
        args: IntegrationRuntimeSsisArgs,
    ) -> IntegrationRuntimeSsisResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_info_binding = args.catalog_info.get_inner();
        let copy_compute_scale_binding = args.copy_compute_scale.get_inner();
        let credential_name_binding = args.credential_name.get_inner();
        let custom_setup_script_binding = args.custom_setup_script.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let edition_binding = args.edition.get_inner();
        let express_custom_setup_binding = args.express_custom_setup.get_inner();
        let express_vnet_integration_binding = args.express_vnet_integration.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let location_binding = args.location.get_inner();
        let max_parallel_executions_per_node_binding = args
            .max_parallel_executions_per_node
            .get_inner();
        let name_binding = args.name.get_inner();
        let node_size_binding = args.node_size.get_inner();
        let number_of_nodes_binding = args.number_of_nodes.get_inner();
        let package_stores_binding = args.package_stores.get_inner();
        let pipeline_external_compute_scale_binding = args
            .pipeline_external_compute_scale
            .get_inner();
        let proxy_binding = args.proxy.get_inner();
        let vnet_integration_binding = args.vnet_integration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/integrationRuntimeSsis:IntegrationRuntimeSsis"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogInfo".into(),
                    value: &catalog_info_binding,
                },
                register_interface::ObjectField {
                    name: "copyComputeScale".into(),
                    value: &copy_compute_scale_binding,
                },
                register_interface::ObjectField {
                    name: "credentialName".into(),
                    value: &credential_name_binding,
                },
                register_interface::ObjectField {
                    name: "customSetupScript".into(),
                    value: &custom_setup_script_binding,
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
                    name: "edition".into(),
                    value: &edition_binding,
                },
                register_interface::ObjectField {
                    name: "expressCustomSetup".into(),
                    value: &express_custom_setup_binding,
                },
                register_interface::ObjectField {
                    name: "expressVnetIntegration".into(),
                    value: &express_vnet_integration_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maxParallelExecutionsPerNode".into(),
                    value: &max_parallel_executions_per_node_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeSize".into(),
                    value: &node_size_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfNodes".into(),
                    value: &number_of_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "packageStores".into(),
                    value: &package_stores_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineExternalComputeScale".into(),
                    value: &pipeline_external_compute_scale_binding,
                },
                register_interface::ObjectField {
                    name: "proxy".into(),
                    value: &proxy_binding,
                },
                register_interface::ObjectField {
                    name: "vnetIntegration".into(),
                    value: &vnet_integration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogInfo".into(),
                },
                register_interface::ResultField {
                    name: "copyComputeScale".into(),
                },
                register_interface::ResultField {
                    name: "credentialName".into(),
                },
                register_interface::ResultField {
                    name: "customSetupScript".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "edition".into(),
                },
                register_interface::ResultField {
                    name: "expressCustomSetup".into(),
                },
                register_interface::ResultField {
                    name: "expressVnetIntegration".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxParallelExecutionsPerNode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeSize".into(),
                },
                register_interface::ResultField {
                    name: "numberOfNodes".into(),
                },
                register_interface::ResultField {
                    name: "packageStores".into(),
                },
                register_interface::ResultField {
                    name: "pipelineExternalComputeScale".into(),
                },
                register_interface::ResultField {
                    name: "proxy".into(),
                },
                register_interface::ResultField {
                    name: "vnetIntegration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationRuntimeSsisResult {
            catalog_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogInfo").unwrap(),
            ),
            copy_compute_scale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyComputeScale").unwrap(),
            ),
            credential_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialName").unwrap(),
            ),
            custom_setup_script: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSetupScript").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edition").unwrap(),
            ),
            express_custom_setup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressCustomSetup").unwrap(),
            ),
            express_vnet_integration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressVnetIntegration").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_parallel_executions_per_node: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxParallelExecutionsPerNode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeSize").unwrap(),
            ),
            number_of_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfNodes").unwrap(),
            ),
            package_stores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageStores").unwrap(),
            ),
            pipeline_external_compute_scale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineExternalComputeScale").unwrap(),
            ),
            proxy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxy").unwrap(),
            ),
            vnet_integration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vnetIntegration").unwrap(),
            ),
        }
    }
}