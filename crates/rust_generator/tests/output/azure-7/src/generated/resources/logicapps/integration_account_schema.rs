/// Manages a Logic App Integration Account Schema.
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
///   exampleIntegrationAccount:
///     type: azure:logicapps:IntegrationAccount
///     name: example
///     properties:
///       name: example-ia
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleIntegrationAccountSchema:
///     type: azure:logicapps:IntegrationAccountSchema
///     name: example
///     properties:
///       name: example-ias
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${exampleIntegrationAccount.name}
///       content:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: testdata/integration_account_schema_content.xsd
///           return: result
/// ```
///
/// ## Import
///
/// Logic App Integration Account Schemas can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountSchema:IntegrationAccountSchema example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/schemas/schema1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account_schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountSchemaArgs {
        /// The content of the Logic App Integration Account Schema.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The file name of the Logic App Integration Account Schema.
        #[builder(into, default)]
        pub file_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Schema to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metadata of the Logic App Integration Account Schema.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Schema. Changing this forces a new Logic App Integration Account Schema to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Schema should exist. Changing this forces a new Logic App Integration Account Schema to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountSchemaResult {
        /// The content of the Logic App Integration Account Schema.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The file name of the Logic App Integration Account Schema.
        pub file_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Schema to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Schema.
        pub metadata: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Schema. Changing this forces a new Logic App Integration Account Schema to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Schema should exist. Changing this forces a new Logic App Integration Account Schema to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountSchemaArgs,
    ) -> IntegrationAccountSchemaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_binding = args.content.get_output(context);
        let file_name_binding = args.file_name.get_output(context);
        let integration_account_name_binding = args
            .integration_account_name
            .get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountSchema:IntegrationAccountSchema"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: &content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileName".into(),
                    value: &file_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationAccountSchemaResult {
            content: o.get_field("content"),
            file_name: o.get_field("fileName"),
            integration_account_name: o.get_field("integrationAccountName"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
