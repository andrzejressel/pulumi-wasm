/// AuthzPolicy is a resource that allows to forward traffic to a callout backend designed to scan the traffic for security purposes.
///
///
/// To get more information about AuthzPolicy, see:
///
/// * [API documentation](https://cloud.google.com/load-balancing/docs/reference/network-security/rest/v1beta1/projects.locations.authzPolicies)
///
/// ## Example Usage
///
/// ### Network Services Authz Policy Advanced
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let authzExtension = region_backend_service::create(
///         "authzExtension",
///         RegionBackendServiceArgs::builder()
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("authz-service")
///             .port_name("grpc")
///             .project("my-project-name")
///             .protocol("HTTP2")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let default = network::create(
///         "default",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("lb-network")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let defaultAddress = address::create(
///         "defaultAddress",
///         AddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("l7-ilb-ip-address")
///             .project("my-project-name")
///             .purpose("GCE_ENDPOINT")
///             .region("us-west1")
///             .subnetwork("${defaultSubnetwork.id}")
///             .build_struct(),
///     );
///     let defaultAuthzExtension = authz_extension::create(
///         "defaultAuthzExtension",
///         AuthzExtensionArgs::builder()
///             .authority("ext11.com")
///             .description("my description")
///             .fail_open(false)
///             .forward_headers(vec!["Authorization",])
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .location("us-west1")
///             .name("my-authz-ext")
///             .project("my-project-name")
///             .service("${authzExtension.selfLink}")
///             .timeout("0.1s")
///             .build_struct(),
///     );
///     let defaultAuthzPolicy = authz_policy::create(
///         "defaultAuthzPolicy",
///         AuthzPolicyArgs::builder()
///             .action("CUSTOM")
///             .custom_provider(
///                 AuthzPolicyCustomProvider::builder()
///                     .authzExtension(
///                         AuthzPolicyCustomProviderAuthzExtension::builder()
///                             .resources(vec!["${defaultAuthzExtension.id}",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("my description")
///             .location("us-west1")
///             .name("my-authz-policy")
///             .project("my-project-name")
///             .target(
///                 AuthzPolicyTarget::builder()
///                     .loadBalancingScheme("INTERNAL_MANAGED")
///                     .resources(vec!["${defaultForwardingRule.selfLink}",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultForwardingRule = forwarding_rule::create(
///         "defaultForwardingRule",
///         ForwardingRuleArgs::builder()
///             .ip_address("${defaultAddress.id}")
///             .ip_protocol("TCP")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("l7-ilb-forwarding-rule")
///             .network("${default.id}")
///             .port_range("80")
///             .project("my-project-name")
///             .region("us-west1")
///             .subnetwork("${defaultSubnetwork.id}")
///             .target("${defaultRegionTargetHttpProxy.id}")
///             .build_struct(),
///     );
///     let defaultRegionHealthCheck = region_health_check::create(
///         "defaultRegionHealthCheck",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder()
///                     .portSpecification("USE_SERVING_PORT")
///                     .build_struct(),
///             )
///             .name("l7-ilb-basic-check")
///             .project("my-project-name")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let defaultRegionTargetHttpProxy = region_target_http_proxy::create(
///         "defaultRegionTargetHttpProxy",
///         RegionTargetHttpProxyArgs::builder()
///             .name("l7-ilb-proxy")
///             .project("my-project-name")
///             .region("us-west1")
///             .url_map("${defaultRegionUrlMap.id}")
///             .build_struct(),
///     );
///     let defaultRegionUrlMap = region_url_map::create(
///         "defaultRegionUrlMap",
///         RegionUrlMapArgs::builder()
///             .default_service("${urlMap.id}")
///             .name("l7-ilb-map")
///             .project("my-project-name")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.1.2.0/24")
///             .name("backend-subnet")
///             .network("${default.id}")
///             .project("my-project-name")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let proxyOnly = subnetwork::create(
///         "proxyOnly",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.129.0.0/23")
///             .name("proxy-only-subnet")
///             .network("${default.id}")
///             .project("my-project-name")
///             .purpose("REGIONAL_MANAGED_PROXY")
///             .region("us-west1")
///             .role("ACTIVE")
///             .build_struct(),
///     );
///     let urlMap = region_backend_service::create(
///         "urlMap",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${defaultRegionHealthCheck.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("l7-ilb-backend-service")
///             .project("my-project-name")
///             .region("us-west1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AuthzPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/authzPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AuthzPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authzPolicy:AuthzPolicy default projects/{{project}}/locations/{{location}}/authzPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authzPolicy:AuthzPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authzPolicy:AuthzPolicy default {{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/authzPolicy:AuthzPolicy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authz_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthzPolicyArgs {
        /// When the action is CUSTOM, customProvider must be specified.
        /// When the action is ALLOW, only requests matching the policy will be allowed.
        /// When the action is DENY, only requests matching the policy will be denied.
        /// When a request arrives, the policies are evaluated in the following order:
        /// 1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.
        /// 2. If there are any DENY policies that match the request, the request is denied.
        /// 3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.
        /// 4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request.
        /// Possible values are: `ALLOW`, `DENY`, `CUSTOM`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required if the action is CUSTOM. Allows delegating authorization decisions to Cloud IAP or to Service Extensions. One
        /// of cloudIap or authzExtension must be specified.
        #[builder(into, default)]
        pub custom_provider: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networksecurity::AuthzPolicyCustomProvider>,
        >,
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of authorization HTTP rules to match against the incoming request.A policy match occurs when at least one HTTP
        /// rule matches the request or when no HTTP rules are specified in the policy. At least one HTTP Rule is required for Allow
        /// or Deny Action. Limited to 5 rules.
        #[builder(into, default)]
        pub http_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRule>>,
        >,
        /// Set of labels associated with the AuthzExtension resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier. Name of the AuthzPolicy resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the set of resources to which this policy should be applied to.
        /// Structure is documented below.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networksecurity::AuthzPolicyTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct AuthzPolicyResult {
        /// When the action is CUSTOM, customProvider must be specified.
        /// When the action is ALLOW, only requests matching the policy will be allowed.
        /// When the action is DENY, only requests matching the policy will be denied.
        /// When a request arrives, the policies are evaluated in the following order:
        /// 1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.
        /// 2. If there are any DENY policies that match the request, the request is denied.
        /// 3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.
        /// 4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request.
        /// Possible values are: `ALLOW`, `DENY`, `CUSTOM`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Required if the action is CUSTOM. Allows delegating authorization decisions to Cloud IAP or to Service Extensions. One
        /// of cloudIap or authzExtension must be specified.
        pub custom_provider: pulumi_gestalt_rust::Output<
            Option<super::super::types::networksecurity::AuthzPolicyCustomProvider>,
        >,
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of authorization HTTP rules to match against the incoming request.A policy match occurs when at least one HTTP
        /// rule matches the request or when no HTTP rules are specified in the policy. At least one HTTP Rule is required for Allow
        /// or Deny Action. Limited to 5 rules.
        pub http_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRule>>,
        >,
        /// Set of labels associated with the AuthzExtension resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. Name of the AuthzPolicy resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the set of resources to which this policy should be applied to.
        /// Structure is documented below.
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::networksecurity::AuthzPolicyTarget,
        >,
        /// The timestamp when the resource was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AuthzPolicyArgs,
    ) -> AuthzPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let custom_provider_binding = args
            .custom_provider
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let http_rules_binding = args.http_rules.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let target_binding = args.target.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/authzPolicy:AuthzPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "customProvider".into(),
                    value: &custom_provider_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "httpRules".into(),
                    value: &http_rules_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AuthzPolicyResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            custom_provider: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customProvider"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            http_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpRules"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("target"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
