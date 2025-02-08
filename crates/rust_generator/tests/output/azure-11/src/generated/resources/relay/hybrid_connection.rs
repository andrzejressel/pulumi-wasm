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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HybridConnectionArgs,
    ) -> HybridConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let relay_namespace_name_binding = args
            .relay_namespace_name
            .get_output(context)
            .get_inner();
        let requires_client_authorization_binding = args
            .requires_client_authorization
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let user_metadata_binding = args.user_metadata.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:relay/hybridConnection:HybridConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "relayNamespaceName".into(),
                    value: &relay_namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "requiresClientAuthorization".into(),
                    value: &requires_client_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "userMetadata".into(),
                    value: &user_metadata_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HybridConnectionResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            relay_namespace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("relayNamespaceName"),
            ),
            requires_client_authorization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requiresClientAuthorization"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            user_metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userMetadata"),
            ),
        }
    }
}
