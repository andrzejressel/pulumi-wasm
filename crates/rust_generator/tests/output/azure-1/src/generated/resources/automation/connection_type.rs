/// Manages anAutomation Connection Type.
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
///   exampleConnectionType:
///     type: azure:automation:ConnectionType
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${exampleResourceGroup.name}
///       automationAccountName: ${exampleAccount.name}
///       fields:
///         - name: example
///           type: string
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Automations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/connectionType:ConnectionType example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/connectionTypes/type1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionTypeArgs {
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `field` blocks as defined below. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub fields: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::automation::ConnectionTypeField>,
        >,
        /// Whether the connection type is global. Changing this forces a new Automation to be created.
        #[builder(into, default)]
        pub is_global: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Automation Connection Type. Changing this forces a new Automation to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionTypeResult {
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `field` blocks as defined below. Changing this forces a new Automation to be created.
        pub fields: pulumi_gestalt_rust::Output<
            Vec<super::super::types::automation::ConnectionTypeField>,
        >,
        /// Whether the connection type is global. Changing this forces a new Automation to be created.
        pub is_global: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Automation Connection Type. Changing this forces a new Automation to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConnectionTypeArgs,
    ) -> ConnectionTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding_1 = args
            .automation_account_name
            .get_output(context);
        let automation_account_name_binding = automation_account_name_binding_1
            .get_inner();
        let fields_binding_1 = args.fields.get_output(context);
        let fields_binding = fields_binding_1.get_inner();
        let is_global_binding_1 = args.is_global.get_output(context);
        let is_global_binding = is_global_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/connectionType:ConnectionType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "fields".into(),
                    value: &fields_binding,
                },
                register_interface::ObjectField {
                    name: "isGlobal".into(),
                    value: &is_global_binding,
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
        ConnectionTypeResult {
            automation_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fields"),
            ),
            is_global: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isGlobal"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
