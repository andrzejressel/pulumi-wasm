/// Manages a Resource Group Template Deployment.
///
/// > **Note:** This resource will automatically attempt to delete resources deployed by the ARM Template when it is deleted. This behavior can be disabled in the provider `features` block by setting the `delete_nested_items_during_deletion` field to `false` within the `template_deployment` block.
///
/// ## Import
///
/// Resource Group Template Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourceGroupTemplateDeployment:ResourceGroupTemplateDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Resources/deployments/template1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_group_template_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Deployment Mode for this Resource Group Template Deployment. Possible values are `Complete` (where resources in the Resource Group not specified in the ARM Template will be destroyed) and `Incremental` (where resources are additive only).
        ///
        /// > **Note:** If `deployment_mode` is set to `Complete` then resources within this Resource Group which are not defined in the ARM Template will be deleted.
        #[builder(into)]
        pub deployment_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Resource Group Template Deployment. Changing this forces a new Resource Group Template Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        ///
        /// > An example of how to pass variables into an ARM Template can be seen in the example.
        #[builder(into, default)]
        pub parameters_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Resource Group Template Deployment should exist. Changing this forces a new Resource Group Template Deployment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Resource Group Template Deployment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Resource Group. Cannot be specified with `template_spec_version_id`.
        #[builder(into, default)]
        pub template_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Template Spec Version to deploy. Cannot be specified with `template_content`.
        #[builder(into, default)]
        pub template_spec_version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupTemplateDeploymentResult {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Deployment Mode for this Resource Group Template Deployment. Possible values are `Complete` (where resources in the Resource Group not specified in the ARM Template will be destroyed) and `Incremental` (where resources are additive only).
        ///
        /// > **Note:** If `deployment_mode` is set to `Complete` then resources within this Resource Group which are not defined in the ARM Template will be deleted.
        pub deployment_mode: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Resource Group Template Deployment. Changing this forces a new Resource Group Template Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_gestalt_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        ///
        /// > An example of how to pass variables into an ARM Template can be seen in the example.
        pub parameters_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Resource Group Template Deployment should exist. Changing this forces a new Resource Group Template Deployment to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Resource Group Template Deployment.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Resource Group. Cannot be specified with `template_spec_version_id`.
        pub template_content: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Template Spec Version to deploy. Cannot be specified with `template_content`.
        pub template_spec_version_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceGroupTemplateDeploymentArgs,
    ) -> ResourceGroupTemplateDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let debug_level_binding_1 = args.debug_level.get_output(context);
        let debug_level_binding = debug_level_binding_1.get_inner();
        let deployment_mode_binding_1 = args.deployment_mode.get_output(context);
        let deployment_mode_binding = deployment_mode_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_content_binding_1 = args.parameters_content.get_output(context);
        let parameters_content_binding = parameters_content_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let template_content_binding_1 = args.template_content.get_output(context);
        let template_content_binding = template_content_binding_1.get_inner();
        let template_spec_version_id_binding_1 = args
            .template_spec_version_id
            .get_output(context);
        let template_spec_version_id_binding = template_spec_version_id_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourceGroupTemplateDeployment:ResourceGroupTemplateDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "debugLevel".into(),
                    value: &debug_level_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentMode".into(),
                    value: &deployment_mode_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
        ResourceGroupTemplateDeploymentResult {
            debug_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("debugLevel"),
            ),
            deployment_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentMode"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            output_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputContent"),
            ),
            parameters_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parametersContent"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
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
