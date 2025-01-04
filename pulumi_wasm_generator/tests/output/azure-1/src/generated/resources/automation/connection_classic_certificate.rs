/// Manages an Automation Connection with type `AzureClassicCertificate`.
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
///   exampleConnectionClassicCertificate:
///     type: azure:automation:ConnectionClassicCertificate
///     name: example
///     properties:
///       name: connection-example
///       resourceGroupName: ${exampleResourceGroup.name}
///       automationAccountName: ${exampleAccount.name}
///       certificateAssetName: cert1
///       subscriptionName: subs1
///       subscriptionId: ${example.subscriptionId}
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
/// $ pulumi import azure:automation/connectionClassicCertificate:ConnectionClassicCertificate conn1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/connections/conn1
/// ```
///
pub mod connection_classic_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionClassicCertificateArgs {
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the certificate asset.
        #[builder(into)]
        pub certificate_asset_name: pulumi_wasm_rust::Output<String>,
        /// A description for this Connection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Connection is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The id of subscription.
        #[builder(into)]
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The name of subscription.
        #[builder(into)]
        pub subscription_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionClassicCertificateResult {
        /// The name of the automation account in which the Connection is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the certificate asset.
        pub certificate_asset_name: pulumi_wasm_rust::Output<String>,
        /// A description for this Connection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Connection is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The id of subscription.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The name of subscription.
        pub subscription_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConnectionClassicCertificateArgs,
    ) -> ConnectionClassicCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let certificate_asset_name_binding = args.certificate_asset_name.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subscription_id_binding = args.subscription_id.get_inner();
        let subscription_name_binding = args.subscription_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/connectionClassicCertificate:ConnectionClassicCertificate"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "certificateAssetName".into(),
                    value: &certificate_asset_name_binding,
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
                    name: "subscriptionName".into(),
                    value: &subscription_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "certificateAssetName".into(),
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
                    name: "subscriptionName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionClassicCertificateResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            certificate_asset_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAssetName").unwrap(),
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
            subscription_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionName").unwrap(),
            ),
        }
    }
}
