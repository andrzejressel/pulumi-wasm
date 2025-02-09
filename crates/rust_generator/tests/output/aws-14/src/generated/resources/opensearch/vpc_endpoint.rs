/// Manages an [AWS Opensearch VPC Endpoint](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_CreateVpcEndpoint.html). Creates an Amazon OpenSearch Service-managed VPC endpoint.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = vpc_endpoint::create(
///         "foo",
///         VpcEndpointArgs::builder()
///             .domain_arn("${domain1.arn}")
///             .vpc_options(
///                 VpcEndpointVpcOptions::builder()
///                     .securityGroupIds(vec!["${test.id}", "${test2.id}",])
///                     .subnetIds(vec!["${testAwsSubnet.id}", "${test2AwsSubnet.id}",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearch VPC endpoint connections using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/vpcEndpoint:VpcEndpoint example endpoint-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointArgs {
        /// Specifies the Amazon Resource Name (ARN) of the domain to create the endpoint for
        #[builder(into)]
        pub domain_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Options to specify the subnets and security groups for the endpoint.
        #[builder(into)]
        pub vpc_options: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::opensearch::VpcEndpointVpcOptions,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointResult {
        /// Specifies the Amazon Resource Name (ARN) of the domain to create the endpoint for
        pub domain_arn: pulumi_gestalt_rust::Output<String>,
        /// The connection endpoint ID for connecting to the domain.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Options to specify the subnets and security groups for the endpoint.
        pub vpc_options: pulumi_gestalt_rust::Output<
            super::super::types::opensearch::VpcEndpointVpcOptions,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointArgs,
    ) -> VpcEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_arn_binding = args.domain_arn.get_output(context);
        let vpc_options_binding = args.vpc_options.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/vpcEndpoint:VpcEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainArn".into(),
                    value: domain_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcOptions".into(),
                    value: vpc_options_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointResult {
            domain_arn: o.get_field("domainArn"),
            endpoint: o.get_field("endpoint"),
            vpc_options: o.get_field("vpcOptions"),
        }
    }
}
