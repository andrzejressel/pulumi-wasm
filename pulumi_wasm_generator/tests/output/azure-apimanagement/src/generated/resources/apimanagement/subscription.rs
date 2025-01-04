/// Manages a Subscription within a API Management Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSubscription:
///     type: azure:apimanagement:Subscription
///     name: example
///     properties:
///       apiManagementName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       userId: ${exampleGetUser.id}
///       productId: ${exampleGetProduct.id}
///       displayName: Parser API
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: example-apim
///         resourceGroupName: example-resources
///   exampleGetProduct:
///     fn::invoke:
///       function: azure:apimanagement:getProduct
///       arguments:
///         productId: 00000000-0000-0000-0000-000000000000
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
///   exampleGetUser:
///     fn::invoke:
///       function: azure:apimanagement:getUser
///       arguments:
///         userId: 11111111-1111-1111-1111-111111111111
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
/// ```
///
/// ## Import
///
/// API Management Subscriptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/subscription:Subscription example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.ApiManagement/service/example-apim/subscriptions/subscription-name
/// ```
///
pub mod subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionArgs {
        /// Determines whether tracing can be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub allow_tracing: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the API which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `/apis` scope is used for the subscription and all apis are accessible.
        #[builder(into, default)]
        pub api_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the API Management Service where this Subscription should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The display name of this Subscription.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The primary subscription key to use for the subscription.
        #[builder(into, default)]
        pub primary_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Product which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `all_apis` scope is used for the subscription.
        #[builder(into, default)]
        pub product_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary subscription key to use for the subscription.
        #[builder(into, default)]
        pub secondary_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of this Subscription. Possible values are `active`, `cancelled`, `expired`, `rejected`, `submitted` and `suspended`. Defaults to `submitted`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// An Identifier which should used as the ID of this Subscription. If not specified a new Subscription ID will be generated. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subscription_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the User which should be assigned to this Subscription. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub user_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionResult {
        /// Determines whether tracing can be enabled. Defaults to `true`.
        pub allow_tracing: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the API which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `/apis` scope is used for the subscription and all apis are accessible.
        pub api_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the API Management Service where this Subscription should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The display name of this Subscription.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The primary subscription key to use for the subscription.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the Product which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `all_apis` scope is used for the subscription.
        pub product_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary subscription key to use for the subscription.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// The state of this Subscription. Possible values are `active`, `cancelled`, `expired`, `rejected`, `submitted` and `suspended`. Defaults to `submitted`.
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// An Identifier which should used as the ID of this Subscription. If not specified a new Subscription ID will be generated. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the User which should be assigned to this Subscription. Changing this forces a new resource to be created.
        pub user_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SubscriptionArgs) -> SubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_tracing_binding = args.allow_tracing.get_inner();
        let api_id_binding = args.api_id.get_inner();
        let api_management_name_binding = args.api_management_name.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let primary_key_binding = args.primary_key.get_inner();
        let product_id_binding = args.product_id.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let secondary_key_binding = args.secondary_key.get_inner();
        let state_binding = args.state.get_inner();
        let subscription_id_binding = args.subscription_id.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/subscription:Subscription".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowTracing".into(),
                    value: &allow_tracing_binding,
                },
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "primaryKey".into(),
                    value: &primary_key_binding,
                },
                register_interface::ObjectField {
                    name: "productId".into(),
                    value: &product_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryKey".into(),
                    value: &secondary_key_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowTracing".into(),
                },
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "productId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionId".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubscriptionResult {
            allow_tracing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowTracing").unwrap(),
            ),
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            product_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryKey").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionId").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}
