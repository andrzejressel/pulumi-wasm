#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workgroup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkgroupArgs {
        /// The name of the workgroup associated with the database.
        #[builder(into)]
        pub workgroup_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkgroupResult {
        /// Amazon Resource Name (ARN) of the Redshift Serverless Workgroup.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The endpoint that is created from the workgroup. See `Endpoint` below.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redshiftserverless::GetWorkgroupEndpoint>,
        >,
        /// The value that specifies whether to turn on enhanced virtual private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC instead of over the internet.
        pub enhanced_vpc_routing: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// A value that specifies whether the workgroup can be accessed from a public network.
        pub publicly_accessible: pulumi_gestalt_rust::Output<bool>,
        /// An array of security group IDs to associate with the workgroup.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An array of VPC subnet IDs to associate with the workgroup. When set, must contain at least three subnets spanning three Availability Zones. A minimum number of IP addresses is required and scales with the Base Capacity. For more information, see the following [AWS document](https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-known-issues.html).
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Redshift Workgroup ID.
        pub workgroup_id: pulumi_gestalt_rust::Output<String>,
        pub workgroup_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkgroupArgs,
    ) -> GetWorkgroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let workgroup_name_binding = args.workgroup_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:redshiftserverless/getWorkgroup:getWorkgroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workgroupName".into(),
                    value: workgroup_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWorkgroupResult {
            arn: o.get_field("arn"),
            endpoints: o.get_field("endpoints"),
            enhanced_vpc_routing: o.get_field("enhancedVpcRouting"),
            id: o.get_field("id"),
            namespace_name: o.get_field("namespaceName"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            workgroup_id: o.get_field("workgroupId"),
            workgroup_name: o.get_field("workgroupName"),
        }
    }
}
