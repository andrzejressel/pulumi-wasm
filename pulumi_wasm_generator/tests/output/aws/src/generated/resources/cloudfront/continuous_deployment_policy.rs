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
pub mod continuous_deployment_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContinuousDeploymentPolicyArgs {
        /// Whether this continuous deployment policy is enabled.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// CloudFront domain name of the staging distribution. See `staging_distribution_dns_names`.
        #[builder(into, default)]
        pub staging_distribution_dns_names: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyStagingDistributionDnsNames,
            >,
        >,
        /// Parameters for routing production traffic from primary to staging distributions. See `traffic_config`.
        #[builder(into, default)]
        pub traffic_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ContinuousDeploymentPolicyResult {
        /// Whether this continuous deployment policy is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Current version of the continuous distribution policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Date and time the continuous deployment policy was last modified.
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        /// CloudFront domain name of the staging distribution. See `staging_distribution_dns_names`.
        pub staging_distribution_dns_names: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ContinuousDeploymentPolicyStagingDistributionDnsNames,
            >,
        >,
        /// Parameters for routing production traffic from primary to staging distributions. See `traffic_config`.
        pub traffic_config: pulumi_wasm_rust::Output<
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
        name: &str,
        args: ContinuousDeploymentPolicyArgs,
    ) -> ContinuousDeploymentPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let staging_distribution_dns_names_binding = args
            .staging_distribution_dns_names
            .get_inner();
        let traffic_config_binding = args.traffic_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/continuousDeploymentPolicy:ContinuousDeploymentPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "stagingDistributionDnsNames".into(),
                    value: &staging_distribution_dns_names_binding,
                },
                register_interface::ObjectField {
                    name: "trafficConfig".into(),
                    value: &traffic_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "stagingDistributionDnsNames".into(),
                },
                register_interface::ResultField {
                    name: "trafficConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContinuousDeploymentPolicyResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            staging_distribution_dns_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stagingDistributionDnsNames").unwrap(),
            ),
            traffic_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficConfig").unwrap(),
            ),
        }
    }
}