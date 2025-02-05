pub mod get_workflow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkflowArgs {
        /// The name of the Logic App Workflow.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Logic App Workflow exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkflowResult {
        /// The Access Endpoint for the Logic App Workflow
        pub access_endpoint: pulumi_wasm_rust::Output<String>,
        /// The list of access endpoint IP addresses of connector.
        pub connector_endpoint_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of connector.
        pub connector_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::logicapps::GetWorkflowIdentity>,
        >,
        /// The Azure location where the Logic App Workflow exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the integration account linked by this Logic App Workflow.
        pub logic_app_integration_account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of Key-Value pairs.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of access endpoint IP addresses of workflow.
        pub workflow_endpoint_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of workflow.
        pub workflow_outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Schema used for this Logic App Workflow.
        pub workflow_schema: pulumi_wasm_rust::Output<String>,
        /// The version of the Schema used for this Logic App Workflow. Defaults to `1.0.0.0`.
        pub workflow_version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetWorkflowArgs,
    ) -> GetWorkflowResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:logicapps/getWorkflow:getWorkflow".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkflowResult {
            access_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessEndpoint"),
            ),
            connector_endpoint_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectorEndpointIpAddresses"),
            ),
            connector_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectorOutboundIpAddresses"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logic_app_integration_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logicAppIntegrationAccountId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            workflow_endpoint_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workflowEndpointIpAddresses"),
            ),
            workflow_outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workflowOutboundIpAddresses"),
            ),
            workflow_schema: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workflowSchema"),
            ),
            workflow_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workflowVersion"),
            ),
        }
    }
}
