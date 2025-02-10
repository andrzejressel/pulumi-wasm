/// Resource for managing an AWS Lightsail Distribution.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// Below is a basic example with a bucket as an origin.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = bucket::create(
///         "test",
///         BucketArgs::builder().bundle_id("small_1_0").name("test-bucket").build_struct(),
///     );
///     let testDistribution = distribution::create(
///         "testDistribution",
///         DistributionArgs::builder()
///             .bundle_id("small_1_0")
///             .cache_behavior_settings(
///                 DistributionCacheBehaviorSettings::builder()
///                     .allowedHttpMethods("GET,HEAD,OPTIONS,PUT,PATCH,POST,DELETE")
///                     .cachedHttpMethods("GET,HEAD")
///                     .defaultTtl(86400)
///                     .forwardedCookies(
///                         DistributionCacheBehaviorSettingsForwardedCookies::builder()
///                             .option("none")
///                             .build_struct(),
///                     )
///                     .forwardedHeaders(
///                         DistributionCacheBehaviorSettingsForwardedHeaders::builder()
///                             .option("default")
///                             .build_struct(),
///                     )
///                     .forwardedQueryStrings(
///                         DistributionCacheBehaviorSettingsForwardedQueryStrings::builder()
///                             .option(false)
///                             .build_struct(),
///                     )
///                     .maximumTtl(31536000)
///                     .minimumTtl(0)
///                     .build_struct(),
///             )
///             .default_cache_behavior(
///                 DistributionDefaultCacheBehavior::builder()
///                     .behavior("cache")
///                     .build_struct(),
///             )
///             .name("test-distribution")
///             .origin(
///                 DistributionOrigin::builder()
///                     .name("${test.name}")
///                     .regionName("${test.region}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### instance origin example
///
/// Below is an example of an instance as the origin.
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:StaticIpAttachment
///     properties:
///       staticIpName: ${testStaticIp.name}
///       instanceName: ${testInstance.name}
///   testStaticIp:
///     type: aws:lightsail:StaticIp
///     name: test
///     properties:
///       name: test-static-ip
///   testInstance:
///     type: aws:lightsail:Instance
///     name: test
///     properties:
///       name: test-instance
///       availabilityZone: ${available.names[0]}
///       blueprintId: amazon_linux_2
///       bundleId: micro_1_0
///   testDistribution:
///     type: aws:lightsail:Distribution
///     name: test
///     properties:
///       name: test-distribution
///       bundleId: small_1_0
///       origin:
///         name: ${testInstance.name}
///         regionName: ${available.id}
///       defaultCacheBehavior:
///         behavior: cache
///     options:
///       dependsOn:
///         - ${test}
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///         filters:
///           - name: opt-in-status
///             values:
///               - opt-in-not-required
/// ```
///
/// ### lb origin example
///
/// Below is an example with a load balancer as an origin
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Lb
///     properties:
///       name: test-load-balancer
///       healthCheckPath: /
///       instancePort: '80'
///       tags:
///         foo: bar
///   testInstance:
///     type: aws:lightsail:Instance
///     name: test
///     properties:
///       name: test-instance
///       availabilityZone: ${available.names[0]}
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///   testLbAttachment:
///     type: aws:lightsail:LbAttachment
///     name: test
///     properties:
///       lbName: ${test.name}
///       instanceName: ${testInstance.name}
///   testDistribution:
///     type: aws:lightsail:Distribution
///     name: test
///     properties:
///       name: test-distribution
///       bundleId: small_1_0
///       origin:
///         name: ${test.name}
///         regionName: ${available.id}
///       defaultCacheBehavior:
///         behavior: cache
///     options:
///       dependsOn:
///         - ${testLbAttachment}
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///         filters:
///           - name: opt-in-status
///             values:
///               - opt-in-not-required
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lightsail Distribution using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/distribution:Distribution example rft-8012925589
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod distribution {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DistributionArgs {
        /// Bundle ID to use for the distribution.
        #[builder(into)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An object that describes the cache behavior settings of the distribution. Detailed below
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub cache_behavior_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lightsail::DistributionCacheBehaviorSettings>,
        >,
        /// A set of configuration blocks that describe the per-path cache behavior of the distribution. Detailed below
        #[builder(into, default)]
        pub cache_behaviors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lightsail::DistributionCacheBehavior>>,
        >,
        /// The name of the SSL/TLS certificate attached to the distribution, if any.
        #[builder(into, default)]
        pub certificate_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object that describes the default cache behavior of the distribution. Detailed below
        #[builder(into)]
        pub default_cache_behavior: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::lightsail::DistributionDefaultCacheBehavior,
        >,
        /// The IP address type of the distribution. Default: `dualstack`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the distribution is enabled. Default: `true`.
        #[builder(into, default)]
        pub is_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the distribution.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object that describes the origin resource of the distribution, such as a Lightsail instance, bucket, or load balancer. Detailed below
        #[builder(into)]
        pub origin: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::lightsail::DistributionOrigin,
        >,
        /// Map of tags for the Lightsail Distribution. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DistributionResult {
        /// The alternate domain names of the distribution.
        pub alternative_domain_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Amazon Resource Name (ARN) of the distribution.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Bundle ID to use for the distribution.
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        /// An object that describes the cache behavior settings of the distribution. Detailed below
        ///
        /// The following arguments are optional:
        pub cache_behavior_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::lightsail::DistributionCacheBehaviorSettings>,
        >,
        /// A set of configuration blocks that describe the per-path cache behavior of the distribution. Detailed below
        pub cache_behaviors: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lightsail::DistributionCacheBehavior>>,
        >,
        /// The name of the SSL/TLS certificate attached to the distribution, if any.
        pub certificate_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp when the distribution was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Object that describes the default cache behavior of the distribution. Detailed below
        pub default_cache_behavior: pulumi_gestalt_rust::Output<
            super::super::types::lightsail::DistributionDefaultCacheBehavior,
        >,
        /// The domain name of the distribution.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The IP address type of the distribution. Default: `dualstack`.
        pub ip_address_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether the distribution is enabled. Default: `true`.
        pub is_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An object that describes the location of the distribution, such as the AWS Region and Availability Zone. Detailed below
        pub locations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::lightsail::DistributionLocation>,
        >,
        /// Name of the distribution.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Object that describes the origin resource of the distribution, such as a Lightsail instance, bucket, or load balancer. Detailed below
        pub origin: pulumi_gestalt_rust::Output<
            super::super::types::lightsail::DistributionOrigin,
        >,
        /// The public DNS of the origin.
        pub origin_public_dns: pulumi_gestalt_rust::Output<String>,
        /// The Lightsail resource type (e.g., Distribution).
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// The status of the distribution.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The support code. Include this code in your email to support when you have questions about your Lightsail distribution. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the Lightsail Distribution. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DistributionArgs,
    ) -> DistributionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bundle_id_binding = args.bundle_id.get_output(context);
        let cache_behavior_settings_binding = args
            .cache_behavior_settings
            .get_output(context);
        let cache_behaviors_binding = args.cache_behaviors.get_output(context);
        let certificate_name_binding = args.certificate_name.get_output(context);
        let default_cache_behavior_binding = args
            .default_cache_behavior
            .get_output(context);
        let ip_address_type_binding = args.ip_address_type.get_output(context);
        let is_enabled_binding = args.is_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let origin_binding = args.origin.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/distribution:Distribution".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleId".into(),
                    value: bundle_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheBehaviorSettings".into(),
                    value: cache_behavior_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheBehaviors".into(),
                    value: cache_behaviors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateName".into(),
                    value: certificate_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultCacheBehavior".into(),
                    value: default_cache_behavior_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressType".into(),
                    value: ip_address_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isEnabled".into(),
                    value: is_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "origin".into(),
                    value: origin_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DistributionResult {
            alternative_domain_names: o.get_field("alternativeDomainNames"),
            arn: o.get_field("arn"),
            bundle_id: o.get_field("bundleId"),
            cache_behavior_settings: o.get_field("cacheBehaviorSettings"),
            cache_behaviors: o.get_field("cacheBehaviors"),
            certificate_name: o.get_field("certificateName"),
            created_at: o.get_field("createdAt"),
            default_cache_behavior: o.get_field("defaultCacheBehavior"),
            domain_name: o.get_field("domainName"),
            ip_address_type: o.get_field("ipAddressType"),
            is_enabled: o.get_field("isEnabled"),
            locations: o.get_field("locations"),
            name: o.get_field("name"),
            origin: o.get_field("origin"),
            origin_public_dns: o.get_field("originPublicDns"),
            resource_type: o.get_field("resourceType"),
            status: o.get_field("status"),
            support_code: o.get_field("supportCode"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
