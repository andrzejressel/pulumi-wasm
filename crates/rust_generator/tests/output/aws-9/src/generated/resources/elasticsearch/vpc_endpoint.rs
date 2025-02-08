/// Manages an [AWS Elasticsearch VPC Endpoint](https://docs.aws.amazon.com/elasticsearch-service/latest/APIReference/API_CreateVpcEndpoint.html). Creates an Amazon elasticsearch Service-managed VPC endpoint.
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
/// Using `pulumi import`, import elasticsearch VPC endpoint connections using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticsearch/vpcEndpoint:VpcEndpoint example endpoint-id
/// ```
#[allow(clippy::doc_lazy_continuation)]
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
            super::super::types::elasticsearch::VpcEndpointVpcOptions,
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
            super::super::types::elasticsearch::VpcEndpointVpcOptions,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcEndpointArgs,
    ) -> VpcEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let domain_arn_binding = args.domain_arn.get_output(context).get_inner();
        let vpc_options_binding = args.vpc_options.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticsearch/vpcEndpoint:VpcEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainArn".into(),
                    value: &domain_arn_binding,
                },
                register_interface::ObjectField {
                    name: "vpcOptions".into(),
                    value: &vpc_options_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcEndpointResult {
            domain_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainArn"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            vpc_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcOptions"),
            ),
        }
    }
}
