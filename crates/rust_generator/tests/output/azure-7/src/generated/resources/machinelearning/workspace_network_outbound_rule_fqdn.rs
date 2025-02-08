/// Manages an Azure Machine Learning Workspace FQDN Network Outbound Rule.
///
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
///   exampleInsights:
///     type: azure:appinsights:Insights
///     name: example
///     properties:
///       name: workspace-example-ai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationType: web
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: workspaceexamplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: workspacestorageaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: GRS
///   exampleWorkspace:
///     type: azure:machinelearning:Workspace
///     name: example
///     properties:
///       name: example-workspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       applicationInsightsId: ${exampleInsights.id}
///       keyVaultId: ${exampleKeyVault.id}
///       storageAccountId: ${exampleAccount.id}
///       managedNetwork:
///         isolationMode: AllowOnlyApprovedOutbound
///       identity:
///         type: SystemAssigned
///   exampleWorkspaceNetworkOutboundRuleFqdn:
///     type: azure:machinelearning:WorkspaceNetworkOutboundRuleFqdn
///     name: example
///     properties:
///       name: example-outboundrule
///       workspaceId: ${exampleWorkspace.id}
///       destinationFqdn: example.com
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Machine Learning Workspace FQDN Network Outbound Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:machinelearning/workspaceNetworkOutboundRuleFqdn:WorkspaceNetworkOutboundRuleFqdn example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.MachineLearningServices/workspaces/workspace1/outboundRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace_network_outbound_rule_fqdn {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceNetworkOutboundRuleFqdnArgs {
        /// Specifies the fully qualified domain name to allow for outbound traffic.
        #[builder(into)]
        pub destination_fqdn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Machine Learning Workspace FQDN Network Outbound Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Machine Learning Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceNetworkOutboundRuleFqdnResult {
        /// Specifies the fully qualified domain name to allow for outbound traffic.
        pub destination_fqdn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Machine Learning Workspace FQDN Network Outbound Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Machine Learning Workspace. Changing this forces a new resource to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkspaceNetworkOutboundRuleFqdnArgs,
    ) -> WorkspaceNetworkOutboundRuleFqdnResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_fqdn_binding = args
            .destination_fqdn
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:machinelearning/workspaceNetworkOutboundRuleFqdn:WorkspaceNetworkOutboundRuleFqdn"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationFqdn".into(),
                    value: &destination_fqdn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkspaceNetworkOutboundRuleFqdnResult {
            destination_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationFqdn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
