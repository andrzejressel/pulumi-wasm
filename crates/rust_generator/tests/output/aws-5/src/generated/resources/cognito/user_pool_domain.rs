/// Provides a Cognito User Pool Domain resource.
///
/// ## Example Usage
///
/// ### Amazon Cognito domain
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_pool::create(
///         "example",
///         UserPoolArgs::builder().name("example-pool").build_struct(),
///     );
///     let main = user_pool_domain::create(
///         "main",
///         UserPoolDomainArgs::builder()
///             .domain("example-domain")
///             .user_pool_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Custom Cognito domain
///
/// ```yaml
/// resources:
///   main:
///     type: aws:cognito:UserPoolDomain
///     properties:
///       domain: auth.example.com
///       certificateArn: ${cert.arn}
///       userPoolId: ${exampleUserPool.id}
///   exampleUserPool:
///     type: aws:cognito:UserPool
///     name: example
///     properties:
///       name: example-pool
///   auth-cognito-A:
///     type: aws:route53:Record
///     properties:
///       name: ${main.domain}
///       type: A
///       zoneId: ${example.zoneId}
///       aliases:
///         - evaluateTargetHealth: false
///           name: ${main.cloudfrontDistribution}
///           zoneId: ${main.cloudfrontDistributionZoneId}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:route53:getZone
///       arguments:
///         name: example.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User Pool Domains using the `domain`. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/userPoolDomain:UserPoolDomain main auth.example.org
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_pool_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoolDomainArgs {
        /// The ARN of an ISSUED ACM certificate in us-east-1 for a custom domain.
        #[builder(into, default)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// For custom domains, this is the fully-qualified domain name, such as auth.example.com. For Amazon Cognito prefix domains, this is the prefix alone, such as auth.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserPoolDomainResult {
        /// The AWS account ID for the user pool owner.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an ISSUED ACM certificate in us-east-1 for a custom domain.
        pub certificate_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon CloudFront endpoint (e.g. `dpp0gtxikpq3y.cloudfront.net`) that you use as the target of the alias that you set up with your Domain Name Service (DNS) provider.
        pub cloudfront_distribution: pulumi_gestalt_rust::Output<String>,
        /// The URL of the CloudFront distribution. This is required to generate the ALIAS `aws.route53.Record`
        pub cloudfront_distribution_arn: pulumi_gestalt_rust::Output<String>,
        /// The Route 53 hosted zone ID of the CloudFront distribution.
        pub cloudfront_distribution_zone_id: pulumi_gestalt_rust::Output<String>,
        /// For custom domains, this is the fully-qualified domain name, such as auth.example.com. For Amazon Cognito prefix domains, this is the prefix alone, such as auth.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The S3 bucket where the static files for this domain are stored.
        pub s3_bucket: pulumi_gestalt_rust::Output<String>,
        /// The user pool ID.
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The app version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPoolDomainArgs,
    ) -> UserPoolDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_arn_binding = args.certificate_arn.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/userPoolDomain:UserPoolDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateArn".into(),
                    value: certificate_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: user_pool_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserPoolDomainResult {
            aws_account_id: o.get_field("awsAccountId"),
            certificate_arn: o.get_field("certificateArn"),
            cloudfront_distribution: o.get_field("cloudfrontDistribution"),
            cloudfront_distribution_arn: o.get_field("cloudfrontDistributionArn"),
            cloudfront_distribution_zone_id: o.get_field("cloudfrontDistributionZoneId"),
            domain: o.get_field("domain"),
            s3_bucket: o.get_field("s3Bucket"),
            user_pool_id: o.get_field("userPoolId"),
            version: o.get_field("version"),
        }
    }
}
