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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionArgs {
        /// Determines whether tracing can be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub allow_tracing: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the API which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `/apis` scope is used for the subscription and all apis are accessible.
        #[builder(into, default)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the API Management Service where this Subscription should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The display name of this Subscription.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The primary subscription key to use for the subscription.
        #[builder(into, default)]
        pub primary_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Product which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `all_apis` scope is used for the subscription.
        #[builder(into, default)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The secondary subscription key to use for the subscription.
        #[builder(into, default)]
        pub secondary_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The state of this Subscription. Possible values are `active`, `cancelled`, `expired`, `rejected`, `submitted` and `suspended`. Defaults to `submitted`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An Identifier which should used as the ID of this Subscription. If not specified a new Subscription ID will be generated. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the User which should be assigned to this Subscription. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionResult {
        /// Determines whether tracing can be enabled. Defaults to `true`.
        pub allow_tracing: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the API which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `/apis` scope is used for the subscription and all apis are accessible.
        pub api_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the API Management Service where this Subscription should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The display name of this Subscription.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The primary subscription key to use for the subscription.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Product which should be assigned to this Subscription. Changing this forces a new resource to be created.
        ///
        /// > **Info:** Only one of `product_id` and `api_id` can be set. If both are missing `all_apis` scope is used for the subscription.
        pub product_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary subscription key to use for the subscription.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// The state of this Subscription. Possible values are `active`, `cancelled`, `expired`, `rejected`, `submitted` and `suspended`. Defaults to `submitted`.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// An Identifier which should used as the ID of this Subscription. If not specified a new Subscription ID will be generated. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the User which should be assigned to this Subscription. Changing this forces a new resource to be created.
        pub user_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubscriptionArgs,
    ) -> SubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allow_tracing_binding = args.allow_tracing.get_output(context).get_inner();
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let primary_key_binding = args.primary_key.get_output(context).get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let secondary_key_binding = args.secondary_key.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/subscription:Subscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriptionResult {
            allow_tracing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowTracing"),
            ),
            api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiId"),
            ),
            api_management_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            product_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            subscription_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
            user_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userId"),
            ),
        }
    }
}
