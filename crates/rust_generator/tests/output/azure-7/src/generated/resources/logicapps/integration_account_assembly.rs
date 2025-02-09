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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account_assembly {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountAssemblyArgs {
        /// The name of the Logic App Integration Account Assembly.
        #[builder(into)]
        pub assembly_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the Logic App Integration Account Assembly. Defaults to `0.0.0.0`.
        #[builder(into, default)]
        pub assembly_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content of the Logic App Integration Account Assembly.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content link URI of the Logic App Integration Account Assembly.
        #[builder(into, default)]
        pub content_link_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metadata of the Logic App Integration Account Assembly.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Assembly Artifact. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Assembly Artifact should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountAssemblyResult {
        /// The name of the Logic App Integration Account Assembly.
        pub assembly_name: pulumi_gestalt_rust::Output<String>,
        /// The version of the Logic App Integration Account Assembly. Defaults to `0.0.0.0`.
        pub assembly_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content of the Logic App Integration Account Assembly.
        pub content: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content link URI of the Logic App Integration Account Assembly.
        pub content_link_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Assembly.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Assembly Artifact. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Assembly Artifact should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountAssemblyArgs,
    ) -> IntegrationAccountAssemblyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assembly_name_binding = args.assembly_name.get_output(context);
        let assembly_version_binding = args.assembly_version.get_output(context);
        let content_binding = args.content.get_output(context);
        let content_link_uri_binding = args.content_link_uri.get_output(context);
        let integration_account_name_binding = args
            .integration_account_name
            .get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountAssembly:IntegrationAccountAssembly"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assemblyName".into(),
                    value: assembly_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assemblyVersion".into(),
                    value: assembly_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentLinkUri".into(),
                    value: content_link_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationAccountName".into(),
                    value: integration_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationAccountAssemblyResult {
            assembly_name: o.get_field("assemblyName"),
            assembly_version: o.get_field("assemblyVersion"),
            content: o.get_field("content"),
            content_link_uri: o.get_field("contentLinkUri"),
            integration_account_name: o.get_field("integrationAccountName"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
