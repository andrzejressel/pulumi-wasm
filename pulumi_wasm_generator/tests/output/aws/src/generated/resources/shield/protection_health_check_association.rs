/// Creates an association between a Route53 Health Check and a Shield Advanced protected resource.
/// This association uses the health of your applications to improve responsiveness and accuracy in attack detection and mitigation.
///
/// Blog post: [AWS Shield Advanced now supports Health Based Detection](https://aws.amazon.com/about-aws/whats-new/2020/02/aws-shield-advanced-now-supports-health-based-detection/)
///
/// ## Example Usage
///
/// ### Create an association between a protected EIP and a Route53 Health Check
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///       tags:
///         Name: example
///   exampleProtection:
///     type: aws:shield:Protection
///     name: example
///     properties:
///       name: example-protection
///       resourceArn: arn:${currentGetPartition.partition}:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
///   exampleHealthCheck:
///     type: aws:route53:HealthCheck
///     name: example
///     properties:
///       ipAddress: ${example.publicIp}
///       port: 80
///       type: HTTP
///       resourcePath: /ready
///       failureThreshold: '3'
///       requestInterval: '30'
///       tags:
///         Name: tf-example-health-check
///   exampleProtectionHealthCheckAssociation:
///     type: aws:shield:ProtectionHealthCheckAssociation
///     name: example
///     properties:
///       healthCheckArn: ${exampleHealthCheck.arn}
///       shieldProtectionId: ${exampleProtection.id}
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getRegion
///       Arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       Function: aws:getPartition
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield protection health check association resources using the `shield_protection_id` and `health_check_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:shield/protectionHealthCheckAssociation:ProtectionHealthCheckAssociation example ff9592dc-22f3-4e88-afa1-7b29fde9669a+arn:aws:route53:::healthcheck/3742b175-edb9-46bc-9359-f53e3b794b1b
/// ```
pub mod protection_health_check_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionHealthCheckAssociationArgs {
        /// The ARN (Amazon Resource Name) of the Route53 Health Check resource which will be associated to the protected resource.
        #[builder(into)]
        pub health_check_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the protected resource.
        #[builder(into)]
        pub shield_protection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectionHealthCheckAssociationResult {
        /// The ARN (Amazon Resource Name) of the Route53 Health Check resource which will be associated to the protected resource.
        pub health_check_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the protected resource.
        pub shield_protection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProtectionHealthCheckAssociationArgs,
    ) -> ProtectionHealthCheckAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let health_check_arn_binding = args.health_check_arn.get_inner();
        let shield_protection_id_binding = args.shield_protection_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/protectionHealthCheckAssociation:ProtectionHealthCheckAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "healthCheckArn".into(),
                    value: &health_check_arn_binding,
                },
                register_interface::ObjectField {
                    name: "shieldProtectionId".into(),
                    value: &shield_protection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "healthCheckArn".into(),
                },
                register_interface::ResultField {
                    name: "shieldProtectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProtectionHealthCheckAssociationResult {
            health_check_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckArn").unwrap(),
            ),
            shield_protection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shieldProtectionId").unwrap(),
            ),
        }
    }
}