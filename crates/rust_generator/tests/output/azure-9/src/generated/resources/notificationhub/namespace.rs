/// Manages a Notification Hub Namespace.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("notificationhub-resources")
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
/// Notification Hub Namespaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:notificationhub/namespace:Namespace namespace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.NotificationHubs/namespaces/namespace1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Is this Notification Hub Namespace enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region in which this Notification Hub Namespace should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name to use for this Notification Hub Namespace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Type of Namespace - possible values are `Messaging` or `NotificationHub`.
        #[builder(into)]
        pub namespace_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Notification Hub Namespace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SKU to use for this Notification Hub Namespace. Possible values are `Free`, `Basic` or `Standard`.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Is this Notification Hub Namespace enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region in which this Notification Hub Namespace should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name to use for this Notification Hub Namespace. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Type of Namespace - possible values are `Messaging` or `NotificationHub`.
        pub namespace_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the Notification Hub Namespace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ServiceBus Endpoint for this Notification Hub Namespace.
        pub servicebus_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the SKU to use for this Notification Hub Namespace. Possible values are `Free`, `Basic` or `Standard`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NamespaceArgs,
    ) -> NamespaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let namespace_type_binding_1 = args.namespace_type.get_output(context);
        let namespace_type_binding = namespace_type_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let sku_name_binding_1 = args.sku_name.get_output(context);
        let sku_name_binding = sku_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:notificationhub/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
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
                    name: "namespaceType".into(),
                    value: &namespace_type_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamespaceResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceType"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            servicebus_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicebusEndpoint"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
