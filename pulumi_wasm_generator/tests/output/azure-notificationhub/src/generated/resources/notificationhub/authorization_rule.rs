/// Manages an Authorization Rule associated with a Notification Hub within a Notification Hub Namespace.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("notificationhub-resources")
///             .build_struct(),
///     );
///     let exampleAuthorizationRule = authorization_rule::create(
///         "exampleAuthorizationRule",
///         AuthorizationRuleArgs::builder()
///             .listen(true)
///             .manage(true)
///             .name("management-auth-rule")
///             .namespace_name("${exampleNamespace.name}")
///             .notification_hub_name("${exampleHub.name}")
///             .resource_group_name("${example.name}")
///             .send(true)
///             .build_struct(),
///     );
///     let exampleHub = hub::create(
///         "exampleHub",
///         HubArgs::builder()
///             .location("${example.location}")
///             .name("mynotificationhub")
///             .namespace_name("${exampleNamespace.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder()
///             .location("${example.location}")
///             .name("myappnamespace")
///             .namespace_type("NotificationHub")
///             .resource_group_name("${example.name}")
///             .sku_name("Free")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Notification Hub Authorization Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:notificationhub/authorizationRule:AuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.NotificationHubs/namespaces/namespace1/notificationHubs/hub1/authorizationRules/rule1
/// ```
///
pub mod authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizationRuleArgs {
        /// Does this Authorization Rule have Listen access to the Notification Hub? Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_wasm_rust::Output<Option<bool>>,
        /// Does this Authorization Rule have Manage access to the Notification Hub? Defaults to `false`.
        ///
        /// > **NOTE:** If `manage` is set to `true` then both `send` and `listen` must also be set to `true`.
        #[builder(into, default)]
        pub manage: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name to use for this Authorization Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Notification Hub Namespace in which the Notification Hub exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Notification Hub for which the Authorization Rule should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub notification_hub_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Notification Hub Namespace exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Does this Authorization Rule have Send access to the Notification Hub? Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AuthorizationRuleResult {
        /// Does this Authorization Rule have Listen access to the Notification Hub? Defaults to `false`.
        pub listen: pulumi_wasm_rust::Output<Option<bool>>,
        /// Does this Authorization Rule have Manage access to the Notification Hub? Defaults to `false`.
        ///
        /// > **NOTE:** If `manage` is set to `true` then both `send` and `listen` must also be set to `true`.
        pub manage: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name to use for this Authorization Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Notification Hub Namespace in which the Notification Hub exists. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Notification Hub for which the Authorization Rule should be created. Changing this forces a new resource to be created.
        pub notification_hub_name: pulumi_wasm_rust::Output<String>,
        /// The Primary Access Key associated with this Authorization Rule.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The Primary Connetion String associated with this Authorization Rule.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Notification Hub Namespace exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Access Key associated with this Authorization Rule.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The Secondary Connetion String associated with this Authorization Rule.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// Does this Authorization Rule have Send access to the Notification Hub? Defaults to `false`.
        pub send: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AuthorizationRuleArgs) -> AuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let listen_binding = args.listen.get_inner();
        let manage_binding = args.manage.get_inner();
        let name_binding = args.name.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let notification_hub_name_binding = args.notification_hub_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let send_binding = args.send.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:notificationhub/authorizationRule:AuthorizationRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "listen".into(),
                    value: &listen_binding,
                },
                register_interface::ObjectField {
                    name: "manage".into(),
                    value: &manage_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationHubName".into(),
                    value: &notification_hub_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "send".into(),
                    value: &send_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "listen".into(),
                },
                register_interface::ResultField {
                    name: "manage".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "notificationHubName".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "send".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthorizationRuleResult {
            listen: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listen").unwrap(),
            ),
            manage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manage").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            notification_hub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationHubName").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            send: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("send").unwrap(),
            ),
        }
    }
}