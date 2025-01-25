/// Manages a Logic App Integration Account Assembly.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIntegrationAccount:
///     type: azure:logicapps:IntegrationAccount
///     name: example
///     properties:
///       name: example-ia
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleIntegrationAccountAssembly:
///     type: azure:logicapps:IntegrationAccountAssembly
///     name: example
///     properties:
///       name: example-assembly
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${exampleIntegrationAccount.name}
///       assemblyName: TestAssembly
///       content:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: testdata/log4net.dll
///           return: result
/// ```
///
/// ## Import
///
/// Logic App Integration Account Assemblies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountAssembly:IntegrationAccountAssembly example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/assemblies/assembly1
/// ```
///
pub mod integration_account_assembly {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountAssemblyArgs {
        /// The name of the Logic App Integration Account Assembly.
        #[builder(into)]
        pub assembly_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The version of the Logic App Integration Account Assembly. Defaults to `0.0.0.0`.
        #[builder(into, default)]
        pub assembly_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The content of the Logic App Integration Account Assembly.
        #[builder(into, default)]
        pub content: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The content link URI of the Logic App Integration Account Assembly.
        #[builder(into, default)]
        pub content_link_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The metadata of the Logic App Integration Account Assembly.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Assembly Artifact. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Assembly Artifact should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountAssemblyResult {
        /// The name of the Logic App Integration Account Assembly.
        pub assembly_name: pulumi_wasm_rust::Output<String>,
        /// The version of the Logic App Integration Account Assembly. Defaults to `0.0.0.0`.
        pub assembly_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The content of the Logic App Integration Account Assembly.
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        /// The content link URI of the Logic App Integration Account Assembly.
        pub content_link_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Assembly.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Assembly Artifact. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Assembly Artifact should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IntegrationAccountAssemblyArgs,
    ) -> IntegrationAccountAssemblyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assembly_name_binding = args.assembly_name.get_output(context).get_inner();
        let assembly_version_binding = args
            .assembly_version
            .get_output(context)
            .get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let content_link_uri_binding = args
            .content_link_uri
            .get_output(context)
            .get_inner();
        let integration_account_name_binding = args
            .integration_account_name
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountAssembly:IntegrationAccountAssembly"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assemblyName".into(),
                    value: &assembly_name_binding,
                },
                register_interface::ObjectField {
                    name: "assemblyVersion".into(),
                    value: &assembly_version_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "contentLinkUri".into(),
                    value: &content_link_uri_binding,
                },
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
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
                    name: "assemblyName".into(),
                },
                register_interface::ResultField {
                    name: "assemblyVersion".into(),
                },
                register_interface::ResultField {
                    name: "content".into(),
                },
                register_interface::ResultField {
                    name: "contentLinkUri".into(),
                },
                register_interface::ResultField {
                    name: "integrationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationAccountAssemblyResult {
            assembly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assemblyName").unwrap(),
            ),
            assembly_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assemblyVersion").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            content_link_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentLinkUri").unwrap(),
            ),
            integration_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationAccountName").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
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
