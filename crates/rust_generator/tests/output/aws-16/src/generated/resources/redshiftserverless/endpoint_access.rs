/// Creates a new Amazon Redshift Serverless Endpoint Access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint_access::create(
///         "example",
///         EndpointAccessArgs::builder()
///             .endpoint_name("example")
///             .workgroup_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Endpoint Access using the `endpoint_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/endpointAccess:EndpointAccess example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAccessArgs {
        /// The name of the endpoint.
        #[builder(into)]
        pub endpoint_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The owner Amazon Web Services account for the Amazon Redshift Serverless workgroup.
        #[builder(into, default)]
        pub owner_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of VPC subnet IDs to associate with the endpoint.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// An array of security group IDs to associate with the workgroup.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name of the workgroup.
        #[builder(into)]
        pub workgroup_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointAccessResult {
        /// The DNS address of the VPC endpoint.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Endpoint Access.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the endpoint.
        pub endpoint_name: pulumi_gestalt_rust::Output<String>,
        /// The owner Amazon Web Services account for the Amazon Redshift Serverless workgroup.
        pub owner_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// The port that Amazon Redshift Serverless listens on.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// An array of VPC subnet IDs to associate with the endpoint.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The VPC endpoint or the Redshift Serverless workgroup. See `VPC Endpoint` below.
        pub vpc_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redshiftserverless::EndpointAccessVpcEndpoint>,
        >,
        /// An array of security group IDs to associate with the workgroup.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the workgroup.
        pub workgroup_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAccessArgs,
    ) -> EndpointAccessResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_name_binding = args.endpoint_name.get_output(context);
        let owner_account_binding = args.owner_account.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let workgroup_name_binding = args.workgroup_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshiftserverless/endpointAccess:EndpointAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointName".into(),
                    value: endpoint_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerAccount".into(),
                    value: owner_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: vpc_security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workgroupName".into(),
                    value: workgroup_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointAccessResult {
            address: o.get_field("address"),
            arn: o.get_field("arn"),
            endpoint_name: o.get_field("endpointName"),
            owner_account: o.get_field("ownerAccount"),
            port: o.get_field("port"),
            subnet_ids: o.get_field("subnetIds"),
            vpc_endpoints: o.get_field("vpcEndpoints"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
            workgroup_name: o.get_field("workgroupName"),
        }
    }
}
