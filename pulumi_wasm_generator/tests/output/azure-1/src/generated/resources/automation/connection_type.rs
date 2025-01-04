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
pub mod connection_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionTypeArgs {
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// One or more `field` blocks as defined below. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub fields: pulumi_wasm_rust::Output<
            Vec<super::super::types::automation::ConnectionTypeField>,
        >,
        /// Whether the connection type is global. Changing this forces a new Automation to be created.
        #[builder(into, default)]
        pub is_global: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Automation Connection Type. Changing this forces a new Automation to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionTypeResult {
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// One or more `field` blocks as defined below. Changing this forces a new Automation to be created.
        pub fields: pulumi_wasm_rust::Output<
            Vec<super::super::types::automation::ConnectionTypeField>,
        >,
        /// Whether the connection type is global. Changing this forces a new Automation to be created.
        pub is_global: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Automation Connection Type. Changing this forces a new Automation to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionTypeArgs) -> ConnectionTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let fields_binding = args.fields.get_inner();
        let is_global_binding = args.is_global.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/connectionType:ConnectionType".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "fields".into(),
                },
                register_interface::ResultField {
                    name: "isGlobal".into(),
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
        ConnectionTypeResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            fields: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fields").unwrap(),
            ),
            is_global: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isGlobal").unwrap(),
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
