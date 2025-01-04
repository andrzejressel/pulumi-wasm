/// Manages a Automation Account.
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
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///       tags:
///         environment: development
/// ```
///
/// ## Import
///
/// Automation Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/account:Account account1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1
/// ```
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        #[builder(into, default)]
        pub encryptions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::automation::AccountEncryption>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::AccountIdentity>,
        >,
        /// Whether requests using non-AAD authentication are blocked. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Automation Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed for the automation account. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which the Automation Account is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the account. Possible values are `Basic` and `Free`.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The Primary Access Key for the DSC Endpoint associated with this Automation Account.
        pub dsc_primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The Secondary Access Key for the DSC Endpoint associated with this Automation Account.
        pub dsc_secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The DSC Server Endpoint associated with this Automation Account.
        pub dsc_server_endpoint: pulumi_wasm_rust::Output<String>,
        pub encryptions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::automation::AccountEncryption>>,
        >,
        /// The URL of automation hybrid service which is used for hybrid worker on-boarding With this Automation Account.
        pub hybrid_service_url: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::automation::AccountIdentity>,
        >,
        /// Whether requests using non-AAD authentication are blocked. Defaults to `true`.
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Automation Account. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_endpoint_connections: pulumi_wasm_rust::Output<
            Vec<super::super::types::automation::AccountPrivateEndpointConnection>,
        >,
        /// Whether public network access is allowed for the automation account. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which the Automation Account is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the account. Possible values are `Basic` and `Free`.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryptions_binding = args.encryptions.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptions".into(),
                    value: &encryptions_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dscPrimaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "dscSecondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "dscServerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "encryptions".into(),
                },
                register_interface::ResultField {
                    name: "hybridServiceUrl".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateEndpointConnections".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            dsc_primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dscPrimaryAccessKey").unwrap(),
            ),
            dsc_secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dscSecondaryAccessKey").unwrap(),
            ),
            dsc_server_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dscServerEndpoint").unwrap(),
            ),
            encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptions").unwrap(),
            ),
            hybrid_service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hybridServiceUrl").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthenticationEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_endpoint_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpointConnections").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
