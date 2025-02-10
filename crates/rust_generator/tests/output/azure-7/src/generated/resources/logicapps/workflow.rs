/// Manages a Logic App Workflow.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("workflow-resources")
///             .build_struct(),
///     );
///     let exampleWorkflow = workflow::create(
///         "exampleWorkflow",
///         WorkflowArgs::builder()
///             .location("${example.location}")
///             .name("workflow1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Workflows can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/workflow:Workflow workflow1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Logic/workflows/workflow1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workflow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// A `access_control` block as defined below.
        #[builder(into, default)]
        pub access_control: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logicapps::WorkflowAccessControl>,
        >,
        /// Is the Logic App Workflow enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logicapps::WorkflowIdentity>,
        >,
        /// The ID of the Integration Service Environment to which this Logic App Workflow belongs. Changing this forces a new Logic App Workflow to be created.
        #[builder(into, default)]
        pub integration_service_environment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the Logic App Workflow exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the integration account linked by this Logic App Workflow.
        #[builder(into, default)]
        pub logic_app_integration_account_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of Key-Value pairs.
        ///
        /// > **NOTE:** Any parameters specified must exist in the Schema defined in `workflow_parameters`.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group in which the Logic App Workflow should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a map of Key-Value pairs of the Parameter Definitions to use for this Logic App Workflow. The key is the parameter name, and the value is a JSON encoded string of the parameter definition (see: <https://docs.microsoft.com/azure/logic-apps/logic-apps-workflow-definition-language#parameters>).
        #[builder(into, default)]
        pub workflow_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Schema to use for this Logic App Workflow. Defaults to `https://schema.management.azure.com/providers/Microsoft.Logic/schemas/2016-06-01/workflowdefinition.json#`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub workflow_schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the version of the Schema used for this Logic App Workflow. Defaults to `1.0.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub workflow_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// A `access_control` block as defined below.
        pub access_control: pulumi_gestalt_rust::Output<
            Option<super::super::types::logicapps::WorkflowAccessControl>,
        >,
        /// The Access Endpoint for the Logic App Workflow.
        pub access_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The list of access endpoint IP addresses of connector.
        pub connector_endpoint_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of connector.
        pub connector_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Is the Logic App Workflow enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::logicapps::WorkflowIdentity>,
        >,
        /// The ID of the Integration Service Environment to which this Logic App Workflow belongs. Changing this forces a new Logic App Workflow to be created.
        pub integration_service_environment_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the Logic App Workflow exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the integration account linked by this Logic App Workflow.
        pub logic_app_integration_account_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the name of the Logic App Workflow. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of Key-Value pairs.
        ///
        /// > **NOTE:** Any parameters specified must exist in the Schema defined in `workflow_parameters`.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group in which the Logic App Workflow should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The list of access endpoint IP addresses of workflow.
        pub workflow_endpoint_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of workflow.
        pub workflow_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies a map of Key-Value pairs of the Parameter Definitions to use for this Logic App Workflow. The key is the parameter name, and the value is a JSON encoded string of the parameter definition (see: <https://docs.microsoft.com/azure/logic-apps/logic-apps-workflow-definition-language#parameters>).
        pub workflow_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Schema to use for this Logic App Workflow. Defaults to `https://schema.management.azure.com/providers/Microsoft.Logic/schemas/2016-06-01/workflowdefinition.json#`. Changing this forces a new resource to be created.
        pub workflow_schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the version of the Schema used for this Logic App Workflow. Defaults to `1.0.0.0`. Changing this forces a new resource to be created.
        pub workflow_version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkflowArgs,
    ) -> WorkflowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_control_binding = args.access_control.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let integration_service_environment_id_binding = args
            .integration_service_environment_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let logic_app_integration_account_id_binding = args
            .logic_app_integration_account_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workflow_parameters_binding = args.workflow_parameters.get_output(context);
        let workflow_schema_binding = args.workflow_schema.get_output(context);
        let workflow_version_binding = args.workflow_version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessControl".into(),
                    value: access_control_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationServiceEnvironmentId".into(),
                    value: integration_service_environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logicAppIntegrationAccountId".into(),
                    value: logic_app_integration_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflowParameters".into(),
                    value: workflow_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflowSchema".into(),
                    value: workflow_schema_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflowVersion".into(),
                    value: workflow_version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkflowResult {
            access_control: o.get_field("accessControl"),
            access_endpoint: o.get_field("accessEndpoint"),
            connector_endpoint_ip_addresses: o.get_field("connectorEndpointIpAddresses"),
            connector_outbound_ip_addresses: o.get_field("connectorOutboundIpAddresses"),
            enabled: o.get_field("enabled"),
            identity: o.get_field("identity"),
            integration_service_environment_id: o
                .get_field("integrationServiceEnvironmentId"),
            location: o.get_field("location"),
            logic_app_integration_account_id: o
                .get_field("logicAppIntegrationAccountId"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            workflow_endpoint_ip_addresses: o.get_field("workflowEndpointIpAddresses"),
            workflow_outbound_ip_addresses: o.get_field("workflowOutboundIpAddresses"),
            workflow_parameters: o.get_field("workflowParameters"),
            workflow_schema: o.get_field("workflowSchema"),
            workflow_version: o.get_field("workflowVersion"),
        }
    }
}
