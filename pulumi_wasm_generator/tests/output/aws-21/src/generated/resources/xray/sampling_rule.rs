/// Creates and manages an AWS XRay Sampling Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:xray:SamplingRule
///     properties:
///       ruleName: example
///       priority: 9999
///       version: 1
///       reservoirSize: 1
///       fixedRate: 0.05
///       urlPath: '*'
///       host: '*'
///       httpMethod: '*'
///       serviceType: '*'
///       serviceName: '*'
///       resourceArn: '*'
///       attributes:
///         Hello: Tris
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import XRay Sampling Rules using the name. For example:
///
/// ```sh
/// $ pulumi import aws:xray/samplingRule:SamplingRule example example
/// ```
pub mod sampling_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SamplingRuleArgs {
        /// Matches attributes derived from the request.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The percentage of matching requests to instrument, after the reservoir is exhausted.
        #[builder(into)]
        pub fixed_rate: pulumi_wasm_rust::Output<f64>,
        /// Matches the hostname from a request URL.
        #[builder(into)]
        pub host: pulumi_wasm_rust::Output<String>,
        /// Matches the HTTP method of a request.
        #[builder(into)]
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// The priority of the sampling rule.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.
        #[builder(into)]
        pub reservoir_size: pulumi_wasm_rust::Output<i32>,
        /// Matches the ARN of the AWS resource on which the service runs.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the sampling rule.
        #[builder(into, default)]
        pub rule_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Matches the `name` that the service uses to identify itself in segments.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Matches the `origin` that the service uses to identify its type in segments.
        #[builder(into)]
        pub service_type: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Matches the path from a request URL.
        #[builder(into)]
        pub url_path: pulumi_wasm_rust::Output<String>,
        /// The version of the sampling rule format (`1` )
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct SamplingRuleResult {
        /// The ARN of the sampling rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Matches attributes derived from the request.
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The percentage of matching requests to instrument, after the reservoir is exhausted.
        pub fixed_rate: pulumi_wasm_rust::Output<f64>,
        /// Matches the hostname from a request URL.
        pub host: pulumi_wasm_rust::Output<String>,
        /// Matches the HTTP method of a request.
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// The priority of the sampling rule.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.
        pub reservoir_size: pulumi_wasm_rust::Output<i32>,
        /// Matches the ARN of the AWS resource on which the service runs.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the sampling rule.
        pub rule_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Matches the `name` that the service uses to identify itself in segments.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Matches the `origin` that the service uses to identify its type in segments.
        pub service_type: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Matches the path from a request URL.
        pub url_path: pulumi_wasm_rust::Output<String>,
        /// The version of the sampling rule format (`1` )
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SamplingRuleArgs) -> SamplingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_inner();
        let fixed_rate_binding = args.fixed_rate.get_inner();
        let host_binding = args.host.get_inner();
        let http_method_binding = args.http_method.get_inner();
        let priority_binding = args.priority.get_inner();
        let reservoir_size_binding = args.reservoir_size.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let rule_name_binding = args.rule_name.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let service_type_binding = args.service_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let url_path_binding = args.url_path.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:xray/samplingRule:SamplingRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "fixedRate".into(),
                    value: &fixed_rate_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "reservoirSize".into(),
                    value: &reservoir_size_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceType".into(),
                    value: &service_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "urlPath".into(),
                    value: &url_path_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "fixedRate".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "httpMethod".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "reservoirSize".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "ruleName".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "serviceType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "urlPath".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SamplingRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            fixed_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fixedRate").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            http_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpMethod").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            reservoir_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservoirSize").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            rule_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleName").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            service_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            url_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlPath").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
