/// Provides a VPC Endpoint Policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidrBlock: 10.0.0.0/16
///   exampleVpcEndpoint:
///     type: aws:ec2:VpcEndpoint
///     name: example
///     properties:
///       serviceName: ${example.serviceName}
///       vpcId: ${exampleVpc.id}
///   exampleVpcEndpointPolicy:
///     type: aws:ec2:VpcEndpointPolicy
///     name: example
///     properties:
///       vpcEndpointId: ${exampleVpcEndpoint.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: AllowAll
///               Effect: Allow
///               Principal:
///                 AWS: '*'
///               Action:
///                 - dynamodb:*
///               Resource: '*'
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getVpcEndpointService
///       arguments:
///         service: dynamodb
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Policies using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointPolicy:VpcEndpointPolicy example vpce-3ecf2a57
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointPolicyArgs {
        /// A policy to attach to the endpoint that controls access to the service. Defaults to full access. All `Gateway` and some `Interface` endpoints support policies - see the [relevant AWS documentation](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-endpoints-access.html) for more details.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The VPC Endpoint ID.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointPolicyResult {
        /// A policy to attach to the endpoint that controls access to the service. Defaults to full access. All `Gateway` and some `Interface` endpoints support policies - see the [relevant AWS documentation](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-endpoints-access.html) for more details.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The VPC Endpoint ID.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcEndpointPolicyArgs,
    ) -> VpcEndpointPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_output(context).get_inner();
        let vpc_endpoint_id_binding = args
            .vpc_endpoint_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointPolicy:VpcEndpointPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcEndpointPolicyResult {
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            vpc_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcEndpointId"),
            ),
        }
    }
}
