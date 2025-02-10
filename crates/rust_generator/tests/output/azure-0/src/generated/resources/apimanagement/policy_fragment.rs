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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyFragmentArgs,
    ) -> PolicyFragmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let format_binding = args.format.get_output(context);
        let name_binding = args.name.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/policyFragment:PolicyFragment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: api_management_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyFragmentResult {
            api_management_id: o.get_field("apiManagementId"),
            description: o.get_field("description"),
            format: o.get_field("format"),
            name: o.get_field("name"),
            value: o.get_field("value"),
        }
    }
}
