/// Manages a Notification Hub within a Notification Hub Namespace.
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
/// Notification Hubs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:notificationhub/hub:Hub hub1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.NotificationHubs/namespaces/namespace1/notificationHubs/hub1
/// ```
///
pub mod hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HubArgs {
        /// A `apns_credential` block as defined below.
        ///
        /// > **NOTE:** Removing the `apns_credential` block will currently force a recreation of this resource [due to this bug in the Azure SDK for Go](https://github.com/Azure/azure-sdk-for-go/issues/2246) - we'll remove this limitation when the SDK bug is fixed.
        #[builder(into, default)]
        pub apns_credential: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notificationhub::HubApnsCredential>,
        >,
        /// A `browser_credential` block as defined below.
        #[builder(into, default)]
        pub browser_credential: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notificationhub::HubBrowserCredential>,
        >,
        /// A `gcm_credential` block as defined below.
        ///
        /// > **NOTE:** Removing the `gcm_credential` block will currently force a recreation of this resource [due to this bug in the Azure SDK for Go](https://github.com/Azure/azure-sdk-for-go/issues/2246) - we'll remove this limitation when the SDK bug is fixed.
        #[builder(into, default)]
        pub gcm_credential: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notificationhub::HubGcmCredential>,
        >,
        /// The Azure Region in which this Notification Hub Namespace exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name to use for this Notification Hub. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Notification Hub Namespace in which to create this Notification Hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Notification Hub Namespace exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HubResult {
        /// A `apns_credential` block as defined below.
        ///
        /// > **NOTE:** Removing the `apns_credential` block will currently force a recreation of this resource [due to this bug in the Azure SDK for Go](https://github.com/Azure/azure-sdk-for-go/issues/2246) - we'll remove this limitation when the SDK bug is fixed.
        pub apns_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::notificationhub::HubApnsCredential>,
        >,
        /// A `browser_credential` block as defined below.
        pub browser_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::notificationhub::HubBrowserCredential>,
        >,
        /// A `gcm_credential` block as defined below.
        ///
        /// > **NOTE:** Removing the `gcm_credential` block will currently force a recreation of this resource [due to this bug in the Azure SDK for Go](https://github.com/Azure/azure-sdk-for-go/issues/2246) - we'll remove this limitation when the SDK bug is fixed.
        pub gcm_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::notificationhub::HubGcmCredential>,
        >,
        /// The Azure Region in which this Notification Hub Namespace exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name to use for this Notification Hub. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Notification Hub Namespace in which to create this Notification Hub. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Notification Hub Namespace exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HubArgs,
    ) -> HubResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apns_credential_binding = args
            .apns_credential
            .get_output(context)
            .get_inner();
        let browser_credential_binding = args
            .browser_credential
            .get_output(context)
            .get_inner();
        let gcm_credential_binding = args.gcm_credential.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:notificationhub/hub:Hub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apnsCredential".into(),
                    value: &apns_credential_binding,
                },
                register_interface::ObjectField {
                    name: "browserCredential".into(),
                    value: &browser_credential_binding,
                },
                register_interface::ObjectField {
                    name: "gcmCredential".into(),
                    value: &gcm_credential_binding,
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
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HubResult {
            apns_credential: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apnsCredential"),
            ),
            browser_credential: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("browserCredential"),
            ),
            gcm_credential: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gcmCredential"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
