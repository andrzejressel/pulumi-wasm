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
pub mod resource_group_template_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The Deployment Mode for this Resource Group Template Deployment. Possible values are `Complete` (where resources in the Resource Group not specified in the ARM Template will be destroyed) and `Incremental` (where resources are additive only).
        ///
        /// > **Note:** If `deployment_mode` is set to `Complete` then resources within this Resource Group which are not defined in the ARM Template will be deleted.
        #[builder(into)]
        pub deployment_mode: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Resource Group Template Deployment. Changing this forces a new Resource Group Template Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        ///
        /// > An example of how to pass variables into an ARM Template can be seen in the example.
        #[builder(into, default)]
        pub parameters_content: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Resource Group Template Deployment should exist. Changing this forces a new Resource Group Template Deployment to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Resource Group Template Deployment.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Resource Group. Cannot be specified with `template_spec_version_id`.
        #[builder(into, default)]
        pub template_content: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Template Spec Version to deploy. Cannot be specified with `template_content`.
        #[builder(into, default)]
        pub template_spec_version_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupTemplateDeploymentResult {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The Deployment Mode for this Resource Group Template Deployment. Possible values are `Complete` (where resources in the Resource Group not specified in the ARM Template will be destroyed) and `Incremental` (where resources are additive only).
        ///
        /// > **Note:** If `deployment_mode` is set to `Complete` then resources within this Resource Group which are not defined in the ARM Template will be deleted.
        pub deployment_mode: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Resource Group Template Deployment. Changing this forces a new Resource Group Template Deployment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_wasm_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        ///
        /// > An example of how to pass variables into an ARM Template can be seen in the example.
        pub parameters_content: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Resource Group Template Deployment should exist. Changing this forces a new Resource Group Template Deployment to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Resource Group Template Deployment.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The contents of the ARM Template which should be deployed into this Resource Group. Cannot be specified with `template_spec_version_id`.
        pub template_content: pulumi_wasm_rust::Output<String>,
        /// The ID of the Template Spec Version to deploy. Cannot be specified with `template_content`.
        pub template_spec_version_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResourceGroupTemplateDeploymentArgs,
    ) -> ResourceGroupTemplateDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let debug_level_binding = args.debug_level.get_inner();
        let deployment_mode_binding = args.deployment_mode.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_content_binding = args.parameters_content.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_content_binding = args.template_content.get_inner();
        let template_spec_version_id_binding = args.template_spec_version_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourceGroupTemplateDeployment:ResourceGroupTemplateDeployment"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "debugLevel".into(),
                },
                register_interface::ResultField {
                    name: "deploymentMode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputContent".into(),
                },
                register_interface::ResultField {
                    name: "parametersContent".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "templateContent".into(),
                },
                register_interface::ResultField {
                    name: "templateSpecVersionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceGroupTemplateDeploymentResult {
            debug_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("debugLevel").unwrap(),
            ),
            deployment_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentMode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            output_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputContent").unwrap(),
            ),
            parameters_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parametersContent").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            template_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateContent").unwrap(),
            ),
            template_spec_version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateSpecVersionId").unwrap(),
            ),
        }
    }
}