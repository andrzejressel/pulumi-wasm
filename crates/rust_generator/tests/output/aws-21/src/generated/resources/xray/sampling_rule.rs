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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SamplingRuleArgs {
        /// Matches attributes derived from the request.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The percentage of matching requests to instrument, after the reservoir is exhausted.
        #[builder(into)]
        pub fixed_rate: pulumi_gestalt_rust::InputOrOutput<f64>,
        /// Matches the hostname from a request URL.
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Matches the HTTP method of a request.
        #[builder(into)]
        pub http_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The priority of the sampling rule.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.
        #[builder(into)]
        pub reservoir_size: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Matches the ARN of the AWS resource on which the service runs.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the sampling rule.
        #[builder(into, default)]
        pub rule_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Matches the `name` that the service uses to identify itself in segments.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Matches the `origin` that the service uses to identify its type in segments.
        #[builder(into)]
        pub service_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Matches the path from a request URL.
        #[builder(into)]
        pub url_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the sampling rule format (`1` )
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct SamplingRuleResult {
        /// The ARN of the sampling rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Matches attributes derived from the request.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The percentage of matching requests to instrument, after the reservoir is exhausted.
        pub fixed_rate: pulumi_gestalt_rust::Output<f64>,
        /// Matches the hostname from a request URL.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// Matches the HTTP method of a request.
        pub http_method: pulumi_gestalt_rust::Output<String>,
        /// The priority of the sampling rule.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.
        pub reservoir_size: pulumi_gestalt_rust::Output<i32>,
        /// Matches the ARN of the AWS resource on which the service runs.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the sampling rule.
        pub rule_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Matches the `name` that the service uses to identify itself in segments.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Matches the `origin` that the service uses to identify its type in segments.
        pub service_type: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Matches the path from a request URL.
        pub url_path: pulumi_gestalt_rust::Output<String>,
        /// The version of the sampling rule format (`1` )
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SamplingRuleArgs,
    ) -> SamplingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_output(context).get_inner();
        let fixed_rate_binding = args.fixed_rate.get_output(context).get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let http_method_binding = args.http_method.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let reservoir_size_binding = args.reservoir_size.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let rule_name_binding = args.rule_name.get_output(context).get_inner();
        let service_name_binding = args.service_name.get_output(context).get_inner();
        let service_type_binding = args.service_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let url_path_binding = args.url_path.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SamplingRuleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            fixed_rate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fixedRate"),
            ),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            http_method: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpMethod"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            reservoir_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservoirSize"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            rule_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleName"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            service_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            url_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("urlPath"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
