/// Manages a Subscription Template Deployment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscription_template_deployment::create(
///         "example",
///         SubscriptionTemplateDeploymentArgs::builder()
///             .location("West Europe")
///             .name("example-deployment")
///             .template_content(
///                 " {\n   \"$schema\": \"https://schema.management.azure.com/schemas/2015-01-01/deploymentTemplate.json#\",\n   \"contentVersion\": \"1.0.0.0\",\n   \"parameters\": {},\n   \"variables\": {},\n   \"resources\": [\n     {\n       \"type\": \"Microsoft.Resources/resourceGroups\",\n       \"apiVersion\": \"2018-05-01\",\n       \"location\": \"West Europe\",\n       \"name\": \"some-resource-group\",\n       \"properties\": {}\n     }\n   ]\n }",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Subscription Template Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/subscriptionTemplateDeployment:SubscriptionTemplateDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Resources/deployments/template1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription_template_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Subscription Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Subscription Template Deployment should exist. Changing this forces a new Subscription Template Deployment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Subscription Template Deployment. Changing this forces a new Subscription Template Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        #[builder(into, default)]
        pub parameters_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Subscription Template Deployment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Subscription.
        #[builder(into, default)]
        pub template_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Template Spec Version to deploy into the Subscription. Cannot be specified with `template_content`.
        #[builder(into, default)]
        pub template_spec_version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionTemplateDeploymentResult {
        /// The Debug Level which should be used for this Subscription Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Subscription Template Deployment should exist. Changing this forces a new Subscription Template Deployment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Subscription Template Deployment. Changing this forces a new Subscription Template Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_gestalt_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        pub parameters_content: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Subscription Template Deployment.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Subscription.
        pub template_content: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Template Spec Version to deploy into the Subscription. Cannot be specified with `template_content`.
        pub template_spec_version_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubscriptionTemplateDeploymentArgs,
    ) -> SubscriptionTemplateDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let debug_level_binding = args.debug_level.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_content_binding = args
            .parameters_content
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let template_content_binding = args
            .template_content
            .get_output(context)
            .get_inner();
        let template_spec_version_id_binding = args
            .template_spec_version_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/subscriptionTemplateDeployment:SubscriptionTemplateDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "debugLevel".into(),
                    value: &debug_level_binding,
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
                    name: "parametersContent".into(),
                    value: &parameters_content_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "templateContent".into(),
                    value: &template_content_binding,
                },
                register_interface::ObjectField {
                    name: "templateSpecVersionId".into(),
                    value: &template_spec_version_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriptionTemplateDeploymentResult {
            debug_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("debugLevel"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputContent"),
            ),
            parameters_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parametersContent"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            template_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateContent"),
            ),
            template_spec_version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateSpecVersionId"),
            ),
        }
    }
}
