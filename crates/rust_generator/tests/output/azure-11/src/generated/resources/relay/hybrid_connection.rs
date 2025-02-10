/// Manages an Azure Relay Hybrid Connection.
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
///   exampleNamespace:
///     type: azure:relay:Namespace
///     name: example
///     properties:
///       name: example-relay
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Standard
///       tags:
///         source: managed
///   exampleHybridConnection:
///     type: azure:relay:HybridConnection
///     name: example
///     properties:
///       name: acctestrnhc-%d
///       resourceGroupName: ${example.name}
///       relayNamespaceName: ${exampleNamespace.name}
///       requiresClientAuthorization: false
///       userMetadata: testmetadata
/// ```
///
/// ## Import
///
/// Relay Hybrid Connection's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:relay/hybridConnection:HybridConnection relay1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Relay/namespaces/relay1/hybridConnections/hconn1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hybrid_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HybridConnectionArgs {
        /// Specifies the name of the Azure Relay Hybrid Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Azure Relay in which to create the Azure Relay Hybrid Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub relay_namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify if client authorization is needed for this hybrid connection. Changing this forces a new resource to be created. Defaults to `true`.
        #[builder(into, default)]
        pub requires_client_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the Azure Relay Hybrid Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The usermetadata is a placeholder to store user-defined string data for the hybrid connection endpoint. For example, it can be used to store descriptive data, such as a list of teams and their contact information. Also, user-defined configuration settings can be stored.
        #[builder(into, default)]
        pub user_metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HybridConnectionResult {
        /// Specifies the name of the Azure Relay Hybrid Connection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Azure Relay in which to create the Azure Relay Hybrid Connection. Changing this forces a new resource to be created.
        pub relay_namespace_name: pulumi_gestalt_rust::Output<String>,
        /// Specify if client authorization is needed for this hybrid connection. Changing this forces a new resource to be created. Defaults to `true`.
        pub requires_client_authorization: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Azure Relay Hybrid Connection. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The usermetadata is a placeholder to store user-defined string data for the hybrid connection endpoint. For example, it can be used to store descriptive data, such as a list of teams and their contact information. Also, user-defined configuration settings can be stored.
        pub user_metadata: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridConnectionArgs,
    ) -> HybridConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let relay_namespace_name_binding = args.relay_namespace_name.get_output(context);
        let requires_client_authorization_binding = args
            .requires_client_authorization
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let user_metadata_binding = args.user_metadata.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:relay/hybridConnection:HybridConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "relayNamespaceName".into(),
                    value: relay_namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requiresClientAuthorization".into(),
                    value: requires_client_authorization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userMetadata".into(),
                    value: user_metadata_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HybridConnectionResult {
            name: o.get_field("name"),
            relay_namespace_name: o.get_field("relayNamespaceName"),
            requires_client_authorization: o.get_field("requiresClientAuthorization"),
            resource_group_name: o.get_field("resourceGroupName"),
            user_metadata: o.get_field("userMetadata"),
        }
    }
}
