/// Manages an Azure Custom Provider.
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
///     let exampleCustomProvider = custom_provider::create(
///         "exampleCustomProvider",
///         CustomProviderArgs::builder()
///             .location("${example.location}")
///             .name("example_provider")
///             .resource_group_name("${example.name}")
///             .resource_types(
///                 vec![
///                     CustomProviderResourceType::builder()
///                     .endpoint("https://testendpoint.com/").name("dEf1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Custom Provider can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/customProvider:CustomProvider example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.CustomProviders/resourceProviders/example
/// ```
///
pub mod custom_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomProviderArgs {
        /// Any number of `action` block as defined below. One of `resource_type` or `action` must be specified.
        #[builder(into, default)]
        pub actions: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::core::CustomProviderAction>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Custom Provider. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Custom Provider. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Any number of `resource_type` block as defined below. One of `resource_type` or `action` must be specified.
        #[builder(into, default)]
        pub resource_types: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::core::CustomProviderResourceType>>,
        >,
        /// A mapping of tags to assign to the resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Any number of `validation` block as defined below.
        #[builder(into, default)]
        pub validations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::core::CustomProviderValidation>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomProviderResult {
        /// Any number of `action` block as defined below. One of `resource_type` or `action` must be specified.
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::CustomProviderAction>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Custom Provider. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Custom Provider. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Any number of `resource_type` block as defined below. One of `resource_type` or `action` must be specified.
        pub resource_types: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::CustomProviderResourceType>>,
        >,
        /// A mapping of tags to assign to the resource. Changing this forces a new resource to be created.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Any number of `validation` block as defined below.
        pub validations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::CustomProviderValidation>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomProviderArgs,
    ) -> CustomProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let resource_types_binding = args.resource_types.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let validations_binding = args.validations.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/customProvider:CustomProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
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
                    name: "resourceTypes".into(),
                    value: &resource_types_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "validations".into(),
                    value: &validations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypes".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "validations".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomProviderResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypes").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            validations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validations").unwrap(),
            ),
        }
    }
}
