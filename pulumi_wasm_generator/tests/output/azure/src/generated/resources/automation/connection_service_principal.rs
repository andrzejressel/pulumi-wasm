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
pub mod connection_service_principal {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionServicePrincipalArgs {
        /// The (Client) ID of the Service Principal.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the Service Principal Certificate.
        #[builder(into)]
        pub certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// A description for this Connection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The subscription GUID.
        #[builder(into)]
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Tenant the Service Principal is assigned in.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionServicePrincipalResult {
        /// The (Client) ID of the Service Principal.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the Service Principal Certificate.
        pub certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// A description for this Connection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Connection is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The subscription GUID.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Tenant the Service Principal is assigned in.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConnectionServicePrincipalArgs,
    ) -> ConnectionServicePrincipalResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let certificate_thumbprint_binding = args.certificate_thumbprint.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subscription_id_binding = args.subscription_id.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/connectionServicePrincipal:ConnectionServicePrincipal"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "certificateThumbprint".into(),
                    value: &certificate_thumbprint_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "certificateThumbprint".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionId".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionServicePrincipalResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            certificate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateThumbprint").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionId").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}