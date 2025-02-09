/// Creates a new Amazon Redshift Serverless Workgroup.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workgroup::create(
///         "example",
///         WorkgroupArgs::builder()
///             .namespace_name("concurrency-scaling")
///             .workgroup_name("concurrency-scaling")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Workgroups using the `workgroup_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/workgroup:Workgroup example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workgroup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkgroupArgs {
        /// The base data warehouse capacity of the workgroup in Redshift Processing Units (RPUs).
        #[builder(into, default)]
        pub base_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An array of parameters to set for more control over a serverless database. See `Config Parameter` below.
        #[builder(into, default)]
        pub config_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::redshiftserverless::WorkgroupConfigParameter>,
            >,
        >,
        /// The value that specifies whether to turn on enhanced virtual private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC instead of over the internet.
        #[builder(into, default)]
        pub enhanced_vpc_routing: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The maximum data-warehouse capacity Amazon Redshift Serverless uses to serve queries, specified in Redshift Processing Units (RPUs).
        #[builder(into, default)]
        pub max_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the namespace.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The port number on which the cluster accepts incoming connections.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A value that specifies whether the workgroup can be accessed from a public network.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An array of security group IDs to associate with the workgroup.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An array of VPC subnet IDs to associate with the workgroup. When set, must contain at least three subnets spanning three Availability Zones. A minimum number of IP addresses is required and scales with the Base Capacity. For more information, see the following [AWS document](https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-known-issues.html).
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the workgroup.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub workgroup_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkgroupResult {
        /// Amazon Resource Name (ARN) of the Redshift Serverless Workgroup.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The base data warehouse capacity of the workgroup in Redshift Processing Units (RPUs).
        pub base_capacity: pulumi_gestalt_rust::Output<i32>,
        /// An array of parameters to set for more control over a serverless database. See `Config Parameter` below.
        pub config_parameters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redshiftserverless::WorkgroupConfigParameter>,
        >,
        /// The endpoint that is created from the workgroup. See `Endpoint` below.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redshiftserverless::WorkgroupEndpoint>,
        >,
        /// The value that specifies whether to turn on enhanced virtual private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC instead of over the internet.
        pub enhanced_vpc_routing: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The maximum data-warehouse capacity Amazon Redshift Serverless uses to serve queries, specified in Redshift Processing Units (RPUs).
        pub max_capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the namespace.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The port number on which the cluster accepts incoming connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// A value that specifies whether the workgroup can be accessed from a public network.
        pub publicly_accessible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An array of security group IDs to associate with the workgroup.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An array of VPC subnet IDs to associate with the workgroup. When set, must contain at least three subnets spanning three Availability Zones. A minimum number of IP addresses is required and scales with the Base Capacity. For more information, see the following [AWS document](https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-known-issues.html).
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Redshift Workgroup ID.
        pub workgroup_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the workgroup.
        ///
        /// The following arguments are optional:
        pub workgroup_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkgroupArgs,
    ) -> WorkgroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_capacity_binding = args.base_capacity.get_output(context);
        let config_parameters_binding = args.config_parameters.get_output(context);
        let enhanced_vpc_routing_binding = args.enhanced_vpc_routing.get_output(context);
        let max_capacity_binding = args.max_capacity.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let port_binding = args.port.get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workgroup_name_binding = args.workgroup_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshiftserverless/workgroup:Workgroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseCapacity".into(),
                    value: base_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configParameters".into(),
                    value: config_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enhancedVpcRouting".into(),
                    value: enhanced_vpc_routing_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxCapacity".into(),
                    value: max_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publiclyAccessible".into(),
                    value: publicly_accessible_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workgroupName".into(),
                    value: workgroup_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkgroupResult {
            arn: o.get_field("arn"),
            base_capacity: o.get_field("baseCapacity"),
            config_parameters: o.get_field("configParameters"),
            endpoints: o.get_field("endpoints"),
            enhanced_vpc_routing: o.get_field("enhancedVpcRouting"),
            max_capacity: o.get_field("maxCapacity"),
            namespace_name: o.get_field("namespaceName"),
            port: o.get_field("port"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            workgroup_id: o.get_field("workgroupId"),
            workgroup_name: o.get_field("workgroupName"),
        }
    }
}
