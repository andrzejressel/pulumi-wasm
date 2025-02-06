/// Provides a Cognito User Pool Domain resource.
///
/// ## Example Usage
///
/// ### Amazon Cognito domain
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod user_pool_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoolDomainArgs {
        /// The ARN of an ISSUED ACM certificate in us-east-1 for a custom domain.
        #[builder(into, default)]
        pub certificate_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// For custom domains, this is the fully-qualified domain name, such as auth.example.com. For Amazon Cognito prefix domains, this is the prefix alone, such as auth.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::InputOrOutput<String>,
        /// The user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserPoolDomainResult {
        /// The AWS account ID for the user pool owner.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of an ISSUED ACM certificate in us-east-1 for a custom domain.
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon CloudFront endpoint (e.g. `dpp0gtxikpq3y.cloudfront.net`) that you use as the target of the alias that you set up with your Domain Name Service (DNS) provider.
        pub cloudfront_distribution: pulumi_wasm_rust::Output<String>,
        /// The URL of the CloudFront distribution. This is required to generate the ALIAS `aws.route53.Record`
        pub cloudfront_distribution_arn: pulumi_wasm_rust::Output<String>,
        /// The Route 53 hosted zone ID of the CloudFront distribution.
        pub cloudfront_distribution_zone_id: pulumi_wasm_rust::Output<String>,
        /// For custom domains, this is the fully-qualified domain name, such as auth.example.com. For Amazon Cognito prefix domains, this is the prefix alone, such as auth.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The S3 bucket where the static files for this domain are stored.
        pub s3_bucket: pulumi_wasm_rust::Output<String>,
        /// The user pool ID.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// The app version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UserPoolDomainArgs,
    ) -> UserPoolDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_arn_binding = args
            .certificate_arn
            .get_output(context)
            .get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/userPoolDomain:UserPoolDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserPoolDomainResult {
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            cloudfront_distribution: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudfrontDistribution"),
            ),
            cloudfront_distribution_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudfrontDistributionArn"),
            ),
            cloudfront_distribution_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudfrontDistributionZoneId"),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            s3_bucket: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3Bucket"),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
