/// Manages a Synapse Self-hosted Integration Runtime.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: content
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       managedVirtualNetworkEnabled: true
///       identity:
///         type: SystemAssigned
///   exampleFirewallRule:
///     type: azure:synapse:FirewallRule
///     name: example
///     properties:
///       name: AllowAll
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 255.255.255.255
///   exampleIntegrationRuntimeSelfHosted:
///     type: azure:synapse:IntegrationRuntimeSelfHosted
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
/// ```
///
/// ## Import
///
/// Synapse Self-hosted Integration Runtimes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/integrationRuntimeSelfHosted:IntegrationRuntimeSelfHosted example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Synapse/workspaces/workspace1/integrationRuntimes/IntegrationRuntime1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_runtime_self_hosted {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSelfHostedArgs {
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Synapse Self-hosted Integration Runtime. Changing this forces a new Synapse Self-hosted Integration Runtime to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Synapse Workspace ID in which to associate the Integration Runtime with. Changing this forces a new Synapse Self-hosted Integration Runtime to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSelfHostedResult {
        /// The primary integration runtime authentication key.
        pub authorization_key_primary: pulumi_gestalt_rust::Output<String>,
        /// The secondary integration runtime authentication key.
        pub authorization_key_secondary: pulumi_gestalt_rust::Output<String>,
        /// Integration runtime description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Synapse Self-hosted Integration Runtime. Changing this forces a new Synapse Self-hosted Integration Runtime to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Synapse Workspace ID in which to associate the Integration Runtime with. Changing this forces a new Synapse Self-hosted Integration Runtime to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationRuntimeSelfHostedArgs,
    ) -> IntegrationRuntimeSelfHostedResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let synapse_workspace_id_binding = args.synapse_workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/integrationRuntimeSelfHosted:IntegrationRuntimeSelfHosted"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: synapse_workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationRuntimeSelfHostedResult {
            authorization_key_primary: o.get_field("authorizationKeyPrimary"),
            authorization_key_secondary: o.get_field("authorizationKeySecondary"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            synapse_workspace_id: o.get_field("synapseWorkspaceId"),
        }
    }
}
