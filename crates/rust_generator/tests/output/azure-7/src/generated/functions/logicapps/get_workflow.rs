#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workflow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkflowArgs {
        /// The name of the Logic App Workflow.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Logic App Workflow exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkflowResult {
        /// The Access Endpoint for the Logic App Workflow
        pub access_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The list of access endpoint IP addresses of connector.
        pub connector_endpoint_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of connector.
        pub connector_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::logicapps::GetWorkflowIdentity>,
        >,
        /// The Azure location where the Logic App Workflow exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the integration account linked by this Logic App Workflow.
        pub logic_app_integration_account_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of Key-Value pairs.
        pub parameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of access endpoint IP addresses of workflow.
        pub workflow_endpoint_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The list of outgoing IP addresses of workflow.
        pub workflow_outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Schema used for this Logic App Workflow.
        pub workflow_schema: pulumi_gestalt_rust::Output<String>,
        /// The version of the Schema used for this Logic App Workflow. Defaults to `1.0.0.0`.
        pub workflow_version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkflowArgs,
    ) -> GetWorkflowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:logicapps/getWorkflow:getWorkflow".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetWorkflowResult {
            access_endpoint: o.get_field("accessEndpoint"),
            connector_endpoint_ip_addresses: o.get_field("connectorEndpointIpAddresses"),
            connector_outbound_ip_addresses: o.get_field("connectorOutboundIpAddresses"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            logic_app_integration_account_id: o
                .get_field("logicAppIntegrationAccountId"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            workflow_endpoint_ip_addresses: o.get_field("workflowEndpointIpAddresses"),
            workflow_outbound_ip_addresses: o.get_field("workflowOutboundIpAddresses"),
            workflow_schema: o.get_field("workflowSchema"),
            workflow_version: o.get_field("workflowVersion"),
        }
    }
}
