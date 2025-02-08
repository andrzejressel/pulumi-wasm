/// Manages a Automation DSC Node Configuration.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dsc_node_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DscNodeConfigurationArgs {
        /// The name of the automation account in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The PowerShell DSC Node Configuration (mof content).
        #[builder(into)]
        pub content_embedded: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the DSC Node Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DscNodeConfigurationResult {
        /// The name of the automation account in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        pub configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The PowerShell DSC Node Configuration (mof content).
        pub content_embedded: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the DSC Node Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the DSC Node Configuration is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DscNodeConfigurationArgs,
    ) -> DscNodeConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context)
            .get_inner();
        let content_embedded_binding = args
            .content_embedded
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/dscNodeConfiguration:DscNodeConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DscNodeConfigurationResult {
            automation_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            configuration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationName"),
            ),
            content_embedded: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentEmbedded"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
