/// Manages an Api Management Policy Fragment.
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
///   exampleService:
///     type: azure:apimanagement:Service
///     name: example
///     properties:
///       name: example-apim
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       publisherName: pub1
///       publisherEmail: pub1@email.com
///       skuName: Developer_1
///   examplePolicyFragment:
///     type: azure:apimanagement:PolicyFragment
///     name: example
///     properties:
///       apiManagementId: ${exampleService.id}
///       name: example-policy-fragment
///       format: xml
///       value:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: policy-fragment-1.xml
///           return: result
/// ```
///
/// ## Import
///
/// Api Management Policy Fragments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/policyFragment:PolicyFragment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/instance1/policyFragments/policyFragment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_fragment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyFragmentArgs {
        /// The id of the API Management Service. Changing this forces a new Api Management Policy Fragment to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Policy Fragment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The format of the Policy Fragment. Possible values are `xml` or `rawxml`. Default is `xml`.
        ///
        /// > **NOTE:** The `value` property will be updated to reflect the corresponding format when `format` is updated.
        #[builder(into, default)]
        pub format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Api Management Policy Fragment. Changing this forces a new Api Management Policy Fragment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the Policy Fragment.
        ///
        /// > **NOTE:** Be aware of the two format possibilities. If the `value` is not applied and continues to cause a diff the format could be wrong.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyFragmentResult {
        /// The id of the API Management Service. Changing this forces a new Api Management Policy Fragment to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Policy Fragment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The format of the Policy Fragment. Possible values are `xml` or `rawxml`. Default is `xml`.
        ///
        /// > **NOTE:** The `value` property will be updated to reflect the corresponding format when `format` is updated.
        pub format: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Api Management Policy Fragment. Changing this forces a new Api Management Policy Fragment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The value of the Policy Fragment.
        ///
        /// > **NOTE:** Be aware of the two format possibilities. If the `value` is not applied and continues to cause a diff the format could be wrong.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PolicyFragmentArgs,
    ) -> PolicyFragmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args
            .api_management_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let format_binding = args.format.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/policyFragment:PolicyFragment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "format".into(),
                    value: &format_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyFragmentResult {
            api_management_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("format"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
