/// Manages a Automation DSC Node Configuration.
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
///             .location("${example.location}")
///             .name("account1")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleDscConfiguration = dsc_configuration::create(
///         "exampleDscConfiguration",
///         DscConfigurationArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .content_embedded("configuration test {}")
///             .location("${example.location}")
///             .name("test")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleDscNodeConfiguration = dsc_node_configuration::create(
///         "exampleDscNodeConfiguration",
///         DscNodeConfigurationArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .content_embedded(
///                 "instance of MSFT_FileDirectoryConfiguration as $MSFT_FileDirectoryConfiguration1ref\n{\n  ResourceID = \"[File]bla\";\n  Ensure = \"Present\";\n  Contents = \"bogus Content\";\n  DestinationPath = \"c:\\\\bogus.txt\";\n  ModuleName = \"PSDesiredStateConfiguration\";\n  SourceInfo = \"::3::9::file\";\n  ModuleVersion = \"1.0\";\n  ConfigurationName = \"bla\";\n};\ninstance of OMI_ConfigurationDocument\n{\n  Version=\"2.0.0\";\n  MinimumCompatibleVersion = \"1.0.0\";\n  CompatibleVersionAdditionalProperties= {\"Omi_BaseResource:ConfigurationName\"};\n  Author=\"bogusAuthor\";\n  GenerationDate=\"06/15/2018 14:06:24\";\n  GenerationHost=\"bogusComputer\";\n  Name=\"test\";\n};\n",
///             )
///             .name("test.localhost")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation DSC Node Configuration's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/dscNodeConfiguration:DscNodeConfiguration configuration1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/nodeConfigurations/configuration1
/// ```
///
pub mod dsc_node_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DscNodeConfigurationArgs {
        /// The name of the automation account in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The PowerShell DSC Node Configuration (mof content).
        #[builder(into)]
        pub content_embedded: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the DSC Node Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DscNodeConfigurationResult {
        /// The name of the automation account in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        pub configuration_name: pulumi_wasm_rust::Output<String>,
        /// The PowerShell DSC Node Configuration (mof content).
        pub content_embedded: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the DSC Node Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DscNodeConfigurationArgs,
    ) -> DscNodeConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let content_embedded_binding = args.content_embedded.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/dscNodeConfiguration:DscNodeConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "contentEmbedded".into(),
                    value: &content_embedded_binding,
                },
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
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "configurationName".into(),
                },
                register_interface::ResultField {
                    name: "contentEmbedded".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DscNodeConfigurationResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationName").unwrap(),
            ),
            content_embedded: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentEmbedded").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}