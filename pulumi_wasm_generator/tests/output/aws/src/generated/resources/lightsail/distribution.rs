/// Resource for managing an AWS Lightsail Distribution.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// Below is a basic example with a bucket as an origin.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let available = get_availability_zones::invoke(
///         GetAvailabilityZonesArgs::builder()
///             .filters(
///                 vec![
///                     GetAvailabilityZonesFilter::builder().name("opt-in-status")
///                     .values(vec!["opt-in-not-required",]).build_struct(),
///                 ],
///             )
///             .state("available")
///             .build_struct(),
///     );
///     let test = static_ip_attachment::create(
///         "test",
///         StaticIpAttachmentArgs::builder()
///             .instance_name("${testInstance.name}")
///             .static_ip_name("${testStaticIp.name}")
///             .build_struct(),
///     );
///     let testDistribution = distribution::create(
///         "testDistribution",
///         DistributionArgs::builder()
///             .bundle_id("small_1_0")
///             .default_cache_behavior(
///                 DistributionDefaultCacheBehavior::builder()
///                     .behavior("cache")
///                     .build_struct(),
///             )
///             .name("test-distribution")
///             .origin(
///                 DistributionOrigin::builder()
///                     .name("${testInstance.name}")
///                     .regionName("${available.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let testInstance = instance::create(
///         "testInstance",
///         InstanceArgs::builder()
///             .availability_zone("${available.names[0]}")
///             .blueprint_id("amazon_linux_2")
///             .bundle_id("micro_1_0")
///             .name("test-instance")
///             .build_struct(),
///     );
///     let testStaticIp = static_ip::create(
///         "testStaticIp",
///         StaticIpArgs::builder().name("test-static-ip").build_struct(),
///     );
/// }
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
///       dependson:
///         - ${testLbAttachment}
/// variables:
///   available:
///     fn::invoke:
///       Function: aws:getAvailabilityZones
///       Arguments:
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
pub mod distribution {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DistributionArgs {
        /// Bundle ID to use for the distribution.
        #[builder(into)]
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// An object that describes the cache behavior settings of the distribution. Detailed below
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub cache_behavior_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::lightsail::DistributionCacheBehaviorSettings>,
        >,
        /// A set of configuration blocks that describe the per-path cache behavior of the distribution. Detailed below
        #[builder(into, default)]
        pub cache_behaviors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lightsail::DistributionCacheBehavior>>,
        >,
        /// The name of the SSL/TLS certificate attached to the distribution, if any.
        #[builder(into, default)]
        pub certificate_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Object that describes the default cache behavior of the distribution. Detailed below
        #[builder(into)]
        pub default_cache_behavior: pulumi_wasm_rust::Output<
            super::super::types::lightsail::DistributionDefaultCacheBehavior,
        >,
        /// The IP address type of the distribution. Default: `dualstack`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the distribution is enabled. Default: `true`.
        #[builder(into, default)]
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the distribution.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Object that describes the origin resource of the distribution, such as a Lightsail instance, bucket, or load balancer. Detailed below
        #[builder(into)]
        pub origin: pulumi_wasm_rust::Output<
            super::super::types::lightsail::DistributionOrigin,
        >,
        /// Map of tags for the Lightsail Distribution. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DistributionResult {
        /// The alternate domain names of the distribution.
        pub alternative_domain_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Amazon Resource Name (ARN) of the distribution.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Bundle ID to use for the distribution.
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// An object that describes the cache behavior settings of the distribution. Detailed below
        ///
        /// The following arguments are optional:
        pub cache_behavior_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::lightsail::DistributionCacheBehaviorSettings>,
        >,
        /// A set of configuration blocks that describe the per-path cache behavior of the distribution. Detailed below
        pub cache_behaviors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lightsail::DistributionCacheBehavior>>,
        >,
        /// The name of the SSL/TLS certificate attached to the distribution, if any.
        pub certificate_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The timestamp when the distribution was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Object that describes the default cache behavior of the distribution. Detailed below
        pub default_cache_behavior: pulumi_wasm_rust::Output<
            super::super::types::lightsail::DistributionDefaultCacheBehavior,
        >,
        /// The domain name of the distribution.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The IP address type of the distribution. Default: `dualstack`.
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the distribution is enabled. Default: `true`.
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An object that describes the location of the distribution, such as the AWS Region and Availability Zone. Detailed below
        pub locations: pulumi_wasm_rust::Output<
            Vec<super::super::types::lightsail::DistributionLocation>,
        >,
        /// Name of the distribution.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Object that describes the origin resource of the distribution, such as a Lightsail instance, bucket, or load balancer. Detailed below
        pub origin: pulumi_wasm_rust::Output<
            super::super::types::lightsail::DistributionOrigin,
        >,
        /// The public DNS of the origin.
        pub origin_public_dns: pulumi_wasm_rust::Output<String>,
        /// The Lightsail resource type (e.g., Distribution).
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The status of the distribution.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The support code. Include this code in your email to support when you have questions about your Lightsail distribution. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_wasm_rust::Output<String>,
        /// Map of tags for the Lightsail Distribution. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DistributionArgs) -> DistributionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bundle_id_binding = args.bundle_id.get_inner();
        let cache_behavior_settings_binding = args.cache_behavior_settings.get_inner();
        let cache_behaviors_binding = args.cache_behaviors.get_inner();
        let certificate_name_binding = args.certificate_name.get_inner();
        let default_cache_behavior_binding = args.default_cache_behavior.get_inner();
        let ip_address_type_binding = args.ip_address_type.get_inner();
        let is_enabled_binding = args.is_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let origin_binding = args.origin.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/distribution:Distribution".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "cacheBehaviorSettings".into(),
                    value: &cache_behavior_settings_binding,
                },
                register_interface::ObjectField {
                    name: "cacheBehaviors".into(),
                    value: &cache_behaviors_binding,
                },
                register_interface::ObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultCacheBehavior".into(),
                    value: &default_cache_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
                },
                register_interface::ObjectField {
                    name: "isEnabled".into(),
                    value: &is_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "origin".into(),
                    value: &origin_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alternativeDomainNames".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "cacheBehaviorSettings".into(),
                },
                register_interface::ResultField {
                    name: "cacheBehaviors".into(),
                },
                register_interface::ResultField {
                    name: "certificateName".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "defaultCacheBehavior".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "isEnabled".into(),
                },
                register_interface::ResultField {
                    name: "locations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "origin".into(),
                },
                register_interface::ResultField {
                    name: "originPublicDns".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "supportCode".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DistributionResult {
            alternative_domain_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternativeDomainNames").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            cache_behavior_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheBehaviorSettings").unwrap(),
            ),
            cache_behaviors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheBehaviors").unwrap(),
            ),
            certificate_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateName").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            default_cache_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultCacheBehavior").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            is_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isEnabled").unwrap(),
            ),
            locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origin").unwrap(),
            ),
            origin_public_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originPublicDns").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            support_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportCode").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}