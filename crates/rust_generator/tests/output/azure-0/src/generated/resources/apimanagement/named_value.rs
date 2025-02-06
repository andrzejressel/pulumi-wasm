/// Manages an API Management Named Value.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleNamedValue = named_value::create(
///         "exampleNamedValue",
///         NamedValueArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .display_name("ExampleProperty")
///             .name("example-apimg")
///             .resource_group_name("${example.name}")
///             .value("Example Value")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("pub1@email.com")
///             .publisher_name("pub1")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Properties can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/namedValue:NamedValue example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.ApiManagement/service/example-apim/namedValues/example-apimp
/// ```
///
pub mod named_value {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamedValueArgs {
        /// The name of the API Management Service in which the API Management Named Value should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The display name of this API Management Named Value.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the API Management Named Value. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Named Value should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies whether the API Management Named Value is secret. Valid values are `true` or `false`. The default value is `false`.
        ///
        /// > **NOTE:** setting the field `secret` to `true` doesn't make this field sensitive in the provider, instead it marks the value as secret and encrypts the value in Azure.
        #[builder(into, default)]
        pub secret: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of tags to be applied to the API Management Named Value.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The value of this API Management Named Value.
        #[builder(into, default)]
        pub value: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `value_from_key_vault` block as defined below. If specified, `secret` must also be set to `true`.
        #[builder(into, default)]
        pub value_from_key_vault: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apimanagement::NamedValueValueFromKeyVault>,
        >,
    }
    #[allow(dead_code)]
    pub struct NamedValueResult {
        /// The name of the API Management Service in which the API Management Named Value should exist. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The display name of this API Management Named Value.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API Management Named Value. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Named Value should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the API Management Named Value is secret. Valid values are `true` or `false`. The default value is `false`.
        ///
        /// > **NOTE:** setting the field `secret` to `true` doesn't make this field sensitive in the provider, instead it marks the value as secret and encrypts the value in Azure.
        pub secret: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of tags to be applied to the API Management Named Value.
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The value of this API Management Named Value.
        pub value: pulumi_wasm_rust::Output<Option<String>>,
        /// A `value_from_key_vault` block as defined below. If specified, `secret` must also be set to `true`.
        pub value_from_key_vault: pulumi_wasm_rust::Output<
            Option<super::super::types::apimanagement::NamedValueValueFromKeyVault>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NamedValueArgs,
    ) -> NamedValueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let secret_binding = args.secret.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let value_from_key_vault_binding = args
            .value_from_key_vault
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/namedValue:NamedValue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
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
                    name: "secret".into(),
                    value: &secret_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
                register_interface::ObjectField {
                    name: "valueFromKeyVault".into(),
                    value: &value_from_key_vault_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamedValueResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secret: pulumi_wasm_rust::__private::into_domain(o.extract_field("secret")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            value: pulumi_wasm_rust::__private::into_domain(o.extract_field("value")),
            value_from_key_vault: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("valueFromKeyVault"),
            ),
        }
    }
}
