/// Resource for managing an AWS CloudFront Continuous Deployment Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   staging:
///     type: aws:cloudfront:Distribution
///     properties:
///       enabled: true
///       staging: true # ... other configuration ...
///   example:
///     type: aws:cloudfront:ContinuousDeploymentPolicy
///     properties:
///       enabled: true
///       stagingDistributionDnsNames:
///         items:
///           - ${staging.domainName}
///         quantity: 1
///       trafficConfig:
///         type: SingleWeight
///         singleWeightConfig:
///           weight: '0.01'
///   production:
///     type: aws:cloudfront:Distribution
///     properties:
///       enabled: true # NOTE: A continuous deployment policy cannot be associated to distribution
///       #   # on creation. Set this argument once the resource exists.
///       continuousDeploymentPolicyId: ${example.id}
/// ```
///
/// ### Single Weight Config with Session Stickiness
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudfront:ContinuousDeploymentPolicy
///     properties:
///       enabled: true
///       stagingDistributionDnsNames:
///         items:
///           - ${staging.domainName}
///         quantity: 1
///       trafficConfig:
///         type: SingleWeight
///         singleWeightConfig:
///           weight: '0.01'
///           sessionStickinessConfig:
///             idleTtl: 300
///             maximumTtl: 600
/// ```
///
/// ### Single Header Config
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudfront:ContinuousDeploymentPolicy
///     properties:
///       enabled: true
///       stagingDistributionDnsNames:
///         items:
///           - ${staging.domainName}
///         quantity: 1
///       trafficConfig:
///         type: SingleHeader
///         singleHeaderConfig:
///           header: aws-cf-cd-example
///           value: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Continuous Deployment Policy using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/continuousDeploymentPolicy:ContinuousDeploymentPolicy example abcd-1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod continuous_deployment_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContinuousDeploymentPolicyArgs {
        /// Whether this continuous deployment policy is enabled.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// CloudFront domain name of the staging distribution. See `staging_distribution_dns_names`.
        #[builder(into, default)]
        pub staging_distribution_dns_names: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyStagingDistributionDnsNames,
            >,
        >,
        /// Parameters for routing production traffic from primary to staging distributions. See `traffic_config`.
        #[builder(into, default)]
        pub traffic_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ContinuousDeploymentPolicyResult {
        /// Whether this continuous deployment policy is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Current version of the continuous distribution policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Date and time the continuous deployment policy was last modified.
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        /// CloudFront domain name of the staging distribution. See `staging_distribution_dns_names`.
        pub staging_distribution_dns_names: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyStagingDistributionDnsNames,
            >,
        >,
        /// Parameters for routing production traffic from primary to staging distributions. See `traffic_config`.
        pub traffic_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContinuousDeploymentPolicyArgs,
    ) -> ContinuousDeploymentPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let staging_distribution_dns_names_binding = args
            .staging_distribution_dns_names
            .get_output(context);
        let traffic_config_binding = args.traffic_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/continuousDeploymentPolicy:ContinuousDeploymentPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stagingDistributionDnsNames".into(),
                    value: staging_distribution_dns_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficConfig".into(),
                    value: traffic_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContinuousDeploymentPolicyResult {
            enabled: o.get_field("enabled"),
            etag: o.get_field("etag"),
            last_modified_time: o.get_field("lastModifiedTime"),
            staging_distribution_dns_names: o.get_field("stagingDistributionDnsNames"),
            traffic_config: o.get_field("trafficConfig"),
        }
    }
}
