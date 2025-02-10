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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_template_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupTemplateDeploymentArgs {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        #[builder(into, default)]
        pub debug_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Template should exist. Changing this forces a new Template to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Management Group to apply the Deployment Template to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Template Deployment. Changing this forces a new Template Deployment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        #[builder(into, default)]
        pub parameters_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Template.
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
    pub struct GroupTemplateDeploymentResult {
        /// The Debug Level which should be used for this Resource Group Template Deployment. Possible values are `none`, `requestContent`, `responseContent` and `requestContent, responseContent`.
        pub debug_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Template should exist. Changing this forces a new Template to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Management Group to apply the Deployment Template to. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Template Deployment. Changing this forces a new Template Deployment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_gestalt_rust::Output<String>,
        /// The contents of the ARM Template parameters file - containing a JSON list of parameters.
        pub parameters_content: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Template.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupTemplateDeploymentArgs,
    ) -> GroupTemplateDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let debug_level_binding = args.debug_level.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_group_id_binding = args.management_group_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_content_binding = args.parameters_content.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_content_binding = args.template_content.get_output(context);
        let template_spec_version_id_binding = args
            .template_spec_version_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:management/groupTemplateDeployment:GroupTemplateDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "debugLevel".into(),
                    value: debug_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupId".into(),
                    value: management_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parametersContent".into(),
                    value: parameters_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateContent".into(),
                    value: template_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateSpecVersionId".into(),
                    value: template_spec_version_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupTemplateDeploymentResult {
            debug_level: o.get_field("debugLevel"),
            location: o.get_field("location"),
            management_group_id: o.get_field("managementGroupId"),
            name: o.get_field("name"),
            output_content: o.get_field("outputContent"),
            parameters_content: o.get_field("parametersContent"),
            tags: o.get_field("tags"),
            template_content: o.get_field("templateContent"),
            template_spec_version_id: o.get_field("templateSpecVersionId"),
        }
    }
}
