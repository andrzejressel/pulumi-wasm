/// Manages a Template Deployment at a Management Group Scope.
///
/// > **Note:** Deleting a Deployment at the Management Group Scope will not delete any resources created by the deployment.
///
/// > **Note:** Deployments to a Management Group are always Incrementally applied. Existing resources that are not part of the template will not be removed.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGroupTemplateDeployment:
///     type: azure:management:GroupTemplateDeployment
///     name: example
///     properties:
///       name: example
///       location: West Europe
///       managementGroupId: ${example.id}
///       templateContent: |
///         {
///           "$schema": "https://schema.management.azure.com/schemas/2019-04-01/deploymentTemplate.json#",
///           "contentVersion": "1.0.0.0",
///           "parameters": {
///             "policyAssignmentName": {
///               "type": "string",
///               "defaultValue": "[guid(parameters('policyDefinitionID'), resourceGroup().name)]",
///               "metadata": {
///                 "description": "Specifies the name of the policy assignment, can be used defined or an idempotent name as the defaultValue provides."
///               }
///             },
///             "policyDefinitionID": {
///               "type": "string",
///               "metadata": {
///                 "description": "Specifies the ID of the policy definition or policy set definition being assigned."
///               }
///             }
///           },
///           "resources": [
///             {
///               "type": "Microsoft.Authorization/policyAssignments",
///               "name": "[parameters('policyAssignmentName')]",
///               "apiVersion": "2019-09-01",
///               "properties": {
///                 "scope": "[subscriptionResourceId('Microsoft.Resources/resourceGroups', resourceGroup().name)]",
///                 "policyDefinitionId": "[parameters('policyDefinitionID')]"
///               }
///             }
///           ]
///         }
///       parametersContent: |
///         {
///           "$schema": "https://schema.management.azure.com/schemas/2019-04-01/deploymentParameters.json#",
///           "contentVersion": "1.0.0.0",
///           "parameters": {
///             "policyDefinitionID": {
///               "value": "/providers/Microsoft.Authorization/policyDefinitions/0a914e76-4921-4c19-b460-a2d36003525a"
///             }
///           }
///         }
/// variables:
///   example:
///     fn::invoke:
///       function: azure:management:getGroup
///       arguments:
///         name: 00000000-0000-0000-0000-000000000000
/// ```
///
/// ```yaml
/// resources:
///   exampleGroupTemplateDeployment:
///     type: azure:management:GroupTemplateDeployment
///     name: example
///     properties:
///       name: example
///       location: West Europe
///       managementGroupId: ${example.id}
///       templateContent:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: templates/example-deploy-template.json
///           return: result
///       parametersContent:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: templates/example-deploy-params.json
///           return: result
/// variables:
///   example:
///     fn::invoke:
///       function: azure:management:getGroup
///       arguments:
///         name: 00000000-0000-0000-0000-000000000000
/// ```
///
/// ```yaml
/// resources:
///   exampleGroupTemplateDeployment:
///     type: azure:management:GroupTemplateDeployment
///     name: example
///     properties:
///       name: example
///       location: West Europe
///       managementGroupId: ${example.id}
///       templateSpecVersionId: ${exampleGetTemplateSpecVersion.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:management:getGroup
///       arguments:
///         name: 00000000-0000-0000-0000-000000000000
///   exampleGetTemplateSpecVersion:
///     fn::invoke:
///       function: azure:core:getTemplateSpecVersion
///       arguments:
///         name: exampleTemplateForManagementGroup
///         resourceGroupName: exampleResourceGroup
///         version: v1.0.9
/// ```
///
/// ## Import
///
/// Management Group Template Deployments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/groupTemplateDeployment:GroupTemplateDeployment example /providers/Microsoft.Management/managementGroups/my-management-group-id/providers/Microsoft.Resources/deployments/deploy1
/// ```
///
pub mod group_template_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Template should exist. Changing this forces a new Template to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Management Group to apply the Deployment Template to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Template Deployment. Changing this forces a new Template Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        #[builder(into, default)]
        pub parameters_content: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Template.
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
    pub struct GroupTemplateDeploymentResult {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Template should exist. Changing this forces a new Template to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Management Group to apply the Deployment Template to. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Template Deployment. Changing this forces a new Template Deployment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_wasm_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        pub parameters_content: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Template.
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
        args: GroupTemplateDeploymentArgs,
    ) -> GroupTemplateDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let debug_level_binding = args.debug_level.get_inner();
        let location_binding = args.location.get_inner();
        let management_group_id_binding = args.management_group_id.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_content_binding = args.parameters_content.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_content_binding = args.template_content.get_inner();
        let template_spec_version_id_binding = args.template_spec_version_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/groupTemplateDeployment:GroupTemplateDeployment"
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
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "debugLevel".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementGroupId".into(),
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
        GroupTemplateDeploymentResult {
            debug_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("debugLevel").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementGroupId").unwrap(),
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
