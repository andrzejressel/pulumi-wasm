/// Manages an object variable in Azure Automation
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-example-rg
///       location: West Europe
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: tfex-example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleVariableObject:
///     type: azure:automation:VariableObject
///     name: example
///     properties:
///       name: tfex-example-var
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
///       value:
///         fn::toJSON:
///           greeting: Hello, Terraform Basic Test.
///           language: en
/// ```
///
/// ## Import
///
/// Automation Object Variable can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/variableObject:VariableObject example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/tfex-example-rg/providers/Microsoft.Automation/automationAccounts/tfex-example-account/variables/tfex-example-var
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod variable_object {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VariableObjectArgs {
        /// The name of the automation account in which the Variable is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the Automation Variable.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Automation Variable is encrypted. Defaults to `false`.
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Automation Variable. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Automation Variable. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The value of the Automation Variable as a `jsonencode()` string.
        #[builder(into, default)]
        pub value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VariableObjectResult {
        /// The name of the automation account in which the Variable is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The description of the Automation Variable.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if the Automation Variable is encrypted. Defaults to `false`.
        pub encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Automation Variable. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Automation Variable. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The value of the Automation Variable as a `jsonencode()` string.
        pub value: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VariableObjectArgs,
    ) -> VariableObjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let encrypted_binding = args.encrypted.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/variableObject:VariableObject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: automation_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encrypted".into(),
                    value: encrypted_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VariableObjectResult {
            automation_account_name: o.get_field("automationAccountName"),
            description: o.get_field("description"),
            encrypted: o.get_field("encrypted"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            value: o.get_field("value"),
        }
    }
}
