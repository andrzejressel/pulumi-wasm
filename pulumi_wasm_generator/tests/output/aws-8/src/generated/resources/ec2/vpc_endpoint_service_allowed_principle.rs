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
pub mod vpc_endpoint_service_allowed_principle {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointServiceAllowedPrincipleArgs {
        /// The ARN of the principal to allow permissions.
        #[builder(into)]
        pub principal_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the VPC endpoint service to allow permission.
        #[builder(into)]
        pub vpc_endpoint_service_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointServiceAllowedPrincipleResult {
        /// The ARN of the principal to allow permissions.
        pub principal_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC endpoint service to allow permission.
        pub vpc_endpoint_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcEndpointServiceAllowedPrincipleArgs,
    ) -> VpcEndpointServiceAllowedPrincipleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let principal_arn_binding = args.principal_arn.get_output(context).get_inner();
        let vpc_endpoint_service_id_binding = args
            .vpc_endpoint_service_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointServiceAllowedPrinciple:VpcEndpointServiceAllowedPrinciple"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "principalArn".into(),
                    value: &principal_arn_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: &vpc_endpoint_service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "principalArn".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointServiceId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointServiceAllowedPrincipleResult {
            principal_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalArn").unwrap(),
            ),
            vpc_endpoint_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointServiceId").unwrap(),
            ),
        }
    }
}
