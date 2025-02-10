/// Manages an Automation Connection with type `AzureServicePrincipal`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: resourceGroup-example
///       location: West Europe
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: account-example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       skuName: Basic
///   exampleConnectionServicePrincipal:
///     type: azure:automation:ConnectionServicePrincipal
///     name: example
///     properties:
///       name: connection-example
///       resourceGroupName: ${exampleResourceGroup.name}
///       automationAccountName: ${exampleAccount.name}
///       applicationId: 00000000-0000-0000-0000-000000000000
///       tenantId: ${example.tenantId}
///       subscriptionId: ${example.subscriptionId}
///       certificateThumbprint:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: automation_certificate_test.thumb
///           return: result
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Automation Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/connectionServicePrincipal:ConnectionServicePrincipal conn1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/connections/conn1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection_service_principal {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionServicePrincipalArgs {
        /// The (Client) ID of the Service Principal.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The thumbprint of the Service Principal Certificate.
        #[builder(into)]
        pub certificate_thumbprint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description for this Connection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The subscription GUID.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Tenant the Service Principal is assigned in.
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionServicePrincipalResult {
        /// The (Client) ID of the Service Principal.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint of the Service Principal Certificate.
        pub certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// A description for this Connection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Connection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Connection is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The subscription GUID.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Tenant the Service Principal is assigned in.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionServicePrincipalArgs,
    ) -> ConnectionServicePrincipalResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let certificate_thumbprint_binding = args
            .certificate_thumbprint
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/connectionServicePrincipal:ConnectionServicePrincipal"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: automation_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateThumbprint".into(),
                    value: certificate_thumbprint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: subscription_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionServicePrincipalResult {
            application_id: o.get_field("applicationId"),
            automation_account_name: o.get_field("automationAccountName"),
            certificate_thumbprint: o.get_field("certificateThumbprint"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            subscription_id: o.get_field("subscriptionId"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
