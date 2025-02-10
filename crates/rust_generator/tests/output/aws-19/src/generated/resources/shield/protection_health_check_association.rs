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
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield protection health check association resources using the `shield_protection_id` and `health_check_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:shield/protectionHealthCheckAssociation:ProtectionHealthCheckAssociation example ff9592dc-22f3-4e88-afa1-7b29fde9669a+arn:aws:route53:::healthcheck/3742b175-edb9-46bc-9359-f53e3b794b1b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod protection_health_check_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionHealthCheckAssociationArgs {
        /// The ARN (Amazon Resource Name) of the Route53 Health Check resource which will be associated to the protected resource.
        #[builder(into)]
        pub health_check_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the protected resource.
        #[builder(into)]
        pub shield_protection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectionHealthCheckAssociationResult {
        /// The ARN (Amazon Resource Name) of the Route53 Health Check resource which will be associated to the protected resource.
        pub health_check_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the protected resource.
        pub shield_protection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectionHealthCheckAssociationArgs,
    ) -> ProtectionHealthCheckAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let health_check_arn_binding = args.health_check_arn.get_output(context);
        let shield_protection_id_binding = args.shield_protection_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:shield/protectionHealthCheckAssociation:ProtectionHealthCheckAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckArn".into(),
                    value: health_check_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shieldProtectionId".into(),
                    value: shield_protection_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProtectionHealthCheckAssociationResult {
            health_check_arn: o.get_field("healthCheckArn"),
            shield_protection_id: o.get_field("shieldProtectionId"),
        }
    }
}
