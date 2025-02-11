#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcEndpointArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcEndpointFilter>>,
        >,
        /// ID of the specific VPC Endpoint to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service name of the specific VPC Endpoint to retrieve. For AWS services the service name is usually in the form `com.amazonaws.<region>.<service>` (the SageMaker Notebook service is an exception to this rule, the service name is in the form `aws.sagemaker.<region>.notebook`).
        #[builder(into, default)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// State of the specific VPC Endpoint to retrieve.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the specific VPC Endpoint to retrieve.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC in which the specific VPC Endpoint is used.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVpcEndpointResult {
        /// ARN of the VPC endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of CIDR blocks for the exposed AWS service. Applicable for endpoints of type `Gateway`.
        pub cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// DNS entries for the VPC Endpoint. Applicable for endpoints of type `Interface`. DNS entry blocks are documented below.
        pub dns_entries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcEndpointDnsEntry>,
        >,
        /// DNS options for the VPC Endpoint. DNS options blocks are documented below.
        pub dns_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcEndpointDnsOption>,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcEndpointFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        /// One or more network interfaces for the VPC Endpoint. Applicable for endpoints of type `Interface`.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the AWS account that owns the VPC endpoint.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Policy document associated with the VPC Endpoint. Applicable for endpoints of type `Gateway`.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Prefix list ID of the exposed AWS service. Applicable for endpoints of type `Gateway`.
        pub prefix_list_id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not the VPC is associated with a private hosted zone - `true` or `false`. Applicable for endpoints of type `Interface`.
        pub private_dns_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not the VPC Endpoint is being managed by its service - `true` or `false`.
        pub requester_managed: pulumi_gestalt_rust::Output<bool>,
        /// One or more route tables associated with the VPC Endpoint. Applicable for endpoints of type `Gateway`.
        pub route_table_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more security groups associated with the network interfaces. Applicable for endpoints of type `Interface`.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        /// One or more subnets in which the VPC Endpoint is located. Applicable for endpoints of type `Interface`.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Endpoint type, `Gateway` or `Interface`.
        pub vpc_endpoint_type: pulumi_gestalt_rust::Output<String>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcEndpointArgs,
    ) -> GetVpcEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let state_binding = args.state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getVpcEndpoint:getVpcEndpoint".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcEndpointResult {
            arn: o.get_field("arn"),
            cidr_blocks: o.get_field("cidrBlocks"),
            dns_entries: o.get_field("dnsEntries"),
            dns_options: o.get_field("dnsOptions"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ip_address_type: o.get_field("ipAddressType"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            owner_id: o.get_field("ownerId"),
            policy: o.get_field("policy"),
            prefix_list_id: o.get_field("prefixListId"),
            private_dns_enabled: o.get_field("privateDnsEnabled"),
            requester_managed: o.get_field("requesterManaged"),
            route_table_ids: o.get_field("routeTableIds"),
            security_group_ids: o.get_field("securityGroupIds"),
            service_name: o.get_field("serviceName"),
            state: o.get_field("state"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            vpc_endpoint_type: o.get_field("vpcEndpointType"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
