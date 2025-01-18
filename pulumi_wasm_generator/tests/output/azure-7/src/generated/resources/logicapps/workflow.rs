/// Manages a Logic App Workflow.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod workflow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// A `access_control` block as defined below.
        #[builder(into, default)]
        pub access_control: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::WorkflowAccessControl>,
        >,
        /// Is the Logic App Workflow enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::WorkflowIdentity>,
        >,
        /// The ID of the Integration Service Environment to which this Logic App Workflow belongs. Changing this forces a new Logic App Workflow to be created.
        #[builder(into, default)]
        pub integration_service_environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Logic App Workflow exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the integration account linked by this Logic App Workflow.
        #[builder(into, default)]
        pub logic_app_integration_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of Key-Value pairs.
        ///
        /// > **NOTE:** Any parameters specified must exist in the Schema defined in `workflow_parameters`.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group in which the Logic App Workflow should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a map of Key-Value pairs of the Parameter Definitions to use for this Logic App Workflow. The key is the parameter name, and the value is a JSON encoded string of the parameter definition (see: <https://docs.microsoft.com/azure/logic-apps/logic-apps-workflow-definition-language#parameters>).
        #[builder(into, default)]
        pub workflow_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Schema to use for this Logic App Workflow. Defaults to `https://schema.management.azure.com/providers/Microsoft.Logic/schemas/2016-06-01/workflowdefinition.json#`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub workflow_schema: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the version of the Schema used for this Logic App Workflow. Defaults to `1.0.0.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub workflow_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// A `access_control` block as defined below.
        pub access_control: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::WorkflowAccessControl>,
        >,
        /// The Access Endpoint for the Logic App Workflow.
        pub access_endpoint: pulumi_wasm_rust::Output<String>,
        /// The list of access endpoint IP addresses of connector.
        pub connector_endpoint_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of connector.
        pub connector_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Is the Logic App Workflow enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::WorkflowIdentity>,
        >,
        /// The ID of the Integration Service Environment to which this Logic App Workflow belongs. Changing this forces a new Logic App Workflow to be created.
        pub integration_service_environment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the Logic App Workflow exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the integration account linked by this Logic App Workflow.
        pub logic_app_integration_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Logic App Workflow. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of Key-Value pairs.
        ///
        /// > **NOTE:** Any parameters specified must exist in the Schema defined in `workflow_parameters`.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group in which the Logic App Workflow should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The list of access endpoint IP addresses of workflow.
        pub workflow_endpoint_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of workflow.
        pub workflow_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies a map of Key-Value pairs of the Parameter Definitions to use for this Logic App Workflow. The key is the parameter name, and the value is a JSON encoded string of the parameter definition (see: <https://docs.microsoft.com/azure/logic-apps/logic-apps-workflow-definition-language#parameters>).
        pub workflow_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Schema to use for this Logic App Workflow. Defaults to `https://schema.management.azure.com/providers/Microsoft.Logic/schemas/2016-06-01/workflowdefinition.json#`. Changing this forces a new resource to be created.
        pub workflow_schema: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the version of the Schema used for this Logic App Workflow. Defaults to `1.0.0.0`. Changing this forces a new resource to be created.
        pub workflow_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkflowArgs) -> WorkflowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_control_binding = args.access_control.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let identity_binding = args.identity.get_inner();
        let integration_service_environment_id_binding = args
            .integration_service_environment_id
            .get_inner();
        let location_binding = args.location.get_inner();
        let logic_app_integration_account_id_binding = args
            .logic_app_integration_account_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let workflow_parameters_binding = args.workflow_parameters.get_inner();
        let workflow_schema_binding = args.workflow_schema.get_inner();
        let workflow_version_binding = args.workflow_version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessControl".into(),
                    value: &access_control_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "integrationServiceEnvironmentId".into(),
                    value: &integration_service_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logicAppIntegrationAccountId".into(),
                    value: &logic_app_integration_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
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
                    name: "workflowParameters".into(),
                    value: &workflow_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "workflowSchema".into(),
                    value: &workflow_schema_binding,
                },
                register_interface::ObjectField {
                    name: "workflowVersion".into(),
                    value: &workflow_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessControl".into(),
                },
                register_interface::ResultField {
                    name: "accessEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "connectorEndpointIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "connectorOutboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "integrationServiceEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "logicAppIntegrationAccountId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workflowEndpointIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "workflowOutboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "workflowParameters".into(),
                },
                register_interface::ResultField {
                    name: "workflowSchema".into(),
                },
                register_interface::ResultField {
                    name: "workflowVersion".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkflowResult {
            access_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessControl").unwrap(),
            ),
            access_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessEndpoint").unwrap(),
            ),
            connector_endpoint_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorEndpointIpAddresses").unwrap(),
            ),
            connector_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorOutboundIpAddresses").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            integration_service_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationServiceEnvironmentId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logic_app_integration_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicAppIntegrationAccountId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workflow_endpoint_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflowEndpointIpAddresses").unwrap(),
            ),
            workflow_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflowOutboundIpAddresses").unwrap(),
            ),
            workflow_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflowParameters").unwrap(),
            ),
            workflow_schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflowSchema").unwrap(),
            ),
            workflow_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workflowVersion").unwrap(),
            ),
        }
    }
}
