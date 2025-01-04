/// Manages an Azure Maps Account.
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
///     type: azure:maps:Account
///     name: example
///     properties:
///       name: example-maps-account
///       resourceGroupName: ${example.name}
///       skuName: S1
///       localAuthenticationEnabled: true
///       tags:
///         environment: Test
/// ```
///
/// ## Import
///
/// A Maps Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maps/account:Account example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Maps/accounts/my-maps-account
/// ```
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// A `cors` block as defined below
        #[builder(into, default)]
        pub cors: pulumi_wasm_rust::Output<
            Option<super::super::types::maps::AccountCors>,
        >,
        /// One or more `data_store` blocks as defined below.
        #[builder(into, default)]
        pub data_stores: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::maps::AccountDataStore>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::maps::AccountIdentity>,
        >,
        /// Is local authentication enabled for this Azure Maps Account? When `false`, all authentication to the Azure Maps data-plane REST API is disabled, except Azure AD authentication. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Location in which the Azure Maps Account should be provisioned. Changing this forces a new resource to be created. Defaults to `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Azure Maps Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Azure Maps Account should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Azure Maps Account. Possible values are `S0`, `S1` and `G2`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Gen1 SKUs (`S0` and `S1`) are deprecated and can no longer be used for new deployments, which should instead use a Gen2 SKU (`G2`) - more information can be found [in the Azure documentation](https://learn.microsoft.com/azure/azure-maps/how-to-manage-pricing-tier).
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Azure Maps Account.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// A `cors` block as defined below
        pub cors: pulumi_wasm_rust::Output<
            Option<super::super::types::maps::AccountCors>,
        >,
        /// One or more `data_store` blocks as defined below.
        pub data_stores: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::maps::AccountDataStore>>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::maps::AccountIdentity>,
        >,
        /// Is local authentication enabled for this Azure Maps Account? When `false`, all authentication to the Azure Maps data-plane REST API is disabled, except Azure AD authentication. Defaults to `true`.
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Location in which the Azure Maps Account should be provisioned. Changing this forces a new resource to be created. Defaults to `global`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Azure Maps Account. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary key used to authenticate and authorize access to the Maps REST APIs.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Azure Maps Account should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary key used to authenticate and authorize access to the Maps REST APIs.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Azure Maps Account. Possible values are `S0`, `S1` and `G2`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Gen1 SKUs (`S0` and `S1`) are deprecated and can no longer be used for new deployments, which should instead use a Gen2 SKU (`G2`) - more information can be found [in the Azure documentation](https://learn.microsoft.com/azure/azure-maps/how-to-manage-pricing-tier).
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Azure Maps Account.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique identifier for the Maps Account.
        pub x_ms_client_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cors_binding = args.cors.get_inner();
        let data_stores_binding = args.data_stores.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:maps/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cors".into(),
                    value: &cors_binding,
                },
                register_interface::ObjectField {
                    name: "dataStores".into(),
                    value: &data_stores_binding,
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
                    name: "cors".into(),
                },
                register_interface::ResultField {
                    name: "dataStores".into(),
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
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "xMsClientId".into(),
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
            cors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cors").unwrap(),
            ),
            data_stores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataStores").unwrap(),
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
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            x_ms_client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xMsClientId").unwrap(),
            ),
        }
    }
}
