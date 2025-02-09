/// ## Example Usage
///
/// ### Region Security Policy Basic
///
///
/// ```yaml
/// resources:
///   region-sec-policy-basic:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       name: my-sec-policy-basic
///       description: basic region security policy
///       type: CLOUD_ARMOR
/// ```
/// ### Region Security Policy With Ddos Protection Config
///
///
/// ```yaml
/// resources:
///   region-sec-policy-ddos-protection:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       name: my-sec-policy-ddos-protection
///       description: with ddos protection config
///       type: CLOUD_ARMOR_NETWORK
///       ddosProtectionConfig:
///         ddosProtection: ADVANCED_PREVIEW
/// ```
/// ### Region Security Policy With User Defined Fields
///
///
/// ```yaml
/// resources:
///   region-sec-policy-user-defined-fields:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       name: my-sec-policy-user-defined-fields
///       description: with user defined fields
///       type: CLOUD_ARMOR_NETWORK
///       userDefinedFields:
///         - name: SIG1_AT_0
///           base: UDP
///           offset: 8
///           size: 2
///           mask: 0x8F00
///         - name: SIG2_AT_8
///           base: UDP
///           offset: 16
///           size: 4
///           mask: 0xFFFFFFFF
/// ```
/// ### Region Security Policy With Rules
///
///
/// ```yaml
/// resources:
///   region-sec-policy-with-rules:
///     type: gcp:compute:RegionSecurityPolicy
///     properties:
///       name: my-sec-policy-with-rules
///       description: basic region security policy with multiple rules
///       type: CLOUD_ARMOR
///       rules:
///         - action: deny
///           priority: '1000'
///           match:
///             expr:
///               expression: request.path.matches("/login.html") && token.recaptcha_session.score < 0.2
///         - action: deny
///           priority: '2147483647'
///           match:
///             versionedExpr: SRC_IPS_V1
///             config:
///               srcIpRanges:
///                 - '*'
///           description: default rule
/// ```
///
/// ## Import
///
/// RegionSecurityPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/securityPolicies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionSecurityPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicy:RegionSecurityPolicy default projects/{{project}}/regions/{{region}}/securityPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicy:RegionSecurityPolicy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicy:RegionSecurityPolicy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSecurityPolicy:RegionSecurityPolicy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_security_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionSecurityPolicyArgs {
        /// Configuration for Google Cloud Armor DDOS Proctection Config.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ddos_protection_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::RegionSecurityPolicyDdosProtectionConfig,
            >,
        >,
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.
        /// Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created Region Security Policy should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match "*"). If no rules are provided when creating a security policy, a default rule with action "allow" will be added.
        /// Structure is documented below.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionSecurityPolicyRule>>,
        >,
        /// The type indicates the intended use of the security policy.
        /// - CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.
        /// - CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.
        /// - CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.
        /// This field can be set only at resource creation time.
        /// Possible values are: `CLOUD_ARMOR`, `CLOUD_ARMOR_EDGE`, `CLOUD_ARMOR_NETWORK`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Definitions of user-defined fields for CLOUD_ARMOR_NETWORK policies.
        /// A user-defined field consists of up to 4 bytes extracted from a fixed offset in the packet, relative to the IPv4, IPv6, TCP, or UDP header, with an optional mask to select certain bits.
        /// Rules may then specify matching values for these fields.
        /// Structure is documented below.
        #[builder(into, default)]
        pub user_defined_fields: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::compute::RegionSecurityPolicyUserDefinedField>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct RegionSecurityPolicyResult {
        /// Configuration for Google Cloud Armor DDOS Proctection Config.
        /// Structure is documented below.
        pub ddos_protection_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::RegionSecurityPolicyDdosProtectionConfig,
            >,
        >,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fingerprint of this resource. This field is used internally during
        /// updates of this resource.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.
        /// Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Region in which the created Region Security Policy should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match "*"). If no rules are provided when creating a security policy, a default rule with action "allow" will be added.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::RegionSecurityPolicyRule>,
        >,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL for this resource with the resource id.
        pub self_link_with_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The type indicates the intended use of the security policy.
        /// - CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.
        /// - CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.
        /// - CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.
        /// This field can be set only at resource creation time.
        /// Possible values are: `CLOUD_ARMOR`, `CLOUD_ARMOR_EDGE`, `CLOUD_ARMOR_NETWORK`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Definitions of user-defined fields for CLOUD_ARMOR_NETWORK policies.
        /// A user-defined field consists of up to 4 bytes extracted from a fixed offset in the packet, relative to the IPv4, IPv6, TCP, or UDP header, with an optional mask to select certain bits.
        /// Rules may then specify matching values for these fields.
        /// Structure is documented below.
        pub user_defined_fields: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::compute::RegionSecurityPolicyUserDefinedField>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionSecurityPolicyArgs,
    ) -> RegionSecurityPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ddos_protection_config_binding_1 = args
            .ddos_protection_config
            .get_output(context);
        let ddos_protection_config_binding = ddos_protection_config_binding_1
            .get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let rules_binding_1 = args.rules.get_output(context);
        let rules_binding = rules_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let user_defined_fields_binding_1 = args.user_defined_fields.get_output(context);
        let user_defined_fields_binding = user_defined_fields_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionSecurityPolicy:RegionSecurityPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ddosProtectionConfig".into(),
                    value: &ddos_protection_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "userDefinedFields".into(),
                    value: &user_defined_fields_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionSecurityPolicyResult {
            ddos_protection_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ddosProtectionConfig"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            self_link_with_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLinkWithPolicyId"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            user_defined_fields: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userDefinedFields"),
            ),
        }
    }
}
