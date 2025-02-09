/// Provides a resource to allow a principal to discover a VPC endpoint service.
///
/// > **NOTE on VPC Endpoint Services and VPC Endpoint Service Allowed Principals:** This provider provides
/// both a standalone VPC Endpoint Service Allowed Principal resource
/// and a VPC Endpoint Service resource with an `allowed_principals` attribute. Do not use the same principal ARN in both
/// a VPC Endpoint Service resource and a VPC Endpoint Service Allowed Principal resource. Doing so will cause a conflict
/// and will overwrite the association.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   allowMeToFoo:
///     type: aws:ec2:VpcEndpointServiceAllowedPrinciple
///     name: allow_me_to_foo
///     properties:
///       vpcEndpointServiceId: ${foo.id}
///       principalArn: ${current.arn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint_service_allowed_principle {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointServiceAllowedPrincipleArgs {
        /// The ARN of the principal to allow permissions.
        #[builder(into)]
        pub principal_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC endpoint service to allow permission.
        #[builder(into)]
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointServiceAllowedPrincipleResult {
        /// The ARN of the principal to allow permissions.
        pub principal_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC endpoint service to allow permission.
        pub vpc_endpoint_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointServiceAllowedPrincipleArgs,
    ) -> VpcEndpointServiceAllowedPrincipleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let principal_arn_binding = args.principal_arn.get_output(context);
        let vpc_endpoint_service_id_binding = args
            .vpc_endpoint_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointServiceAllowedPrinciple:VpcEndpointServiceAllowedPrinciple"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalArn".into(),
                    value: principal_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: vpc_endpoint_service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointServiceAllowedPrincipleResult {
            principal_arn: o.get_field("principalArn"),
            vpc_endpoint_service_id: o.get_field("vpcEndpointServiceId"),
        }
    }
}
