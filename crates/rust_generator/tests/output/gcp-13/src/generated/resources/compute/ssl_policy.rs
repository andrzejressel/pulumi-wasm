/// Represents a SSL policy. SSL policies give you the ability to control the
/// features of SSL that your SSL proxy or HTTPS load balancer negotiates.
///
///
/// To get more information about SslPolicy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/sslPolicies)
/// * How-to Guides
///     * [Using SSL Policies](https://cloud.google.com/compute/docs/load-balancing/ssl-policies)
///
/// ## Example Usage
///
/// ### Ssl Policy Basic
///
///
/// ```yaml
/// resources:
///   prod-ssl-policy:
///     type: gcp:compute:SSLPolicy
///     properties:
///       name: production-ssl-policy
///       profile: MODERN
///   nonprod-ssl-policy:
///     type: gcp:compute:SSLPolicy
///     properties:
///       name: nonprod-ssl-policy
///       profile: MODERN
///       minTlsVersion: TLS_1_2
///   custom-ssl-policy:
///     type: gcp:compute:SSLPolicy
///     properties:
///       name: custom-ssl-policy
///       minTlsVersion: TLS_1_2
///       profile: CUSTOM
///       customFeatures:
///         - TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
///         - TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
/// ```
///
/// ## Import
///
/// SslPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/sslPolicies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, SslPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/sSLPolicy:SSLPolicy default projects/{{project}}/global/sslPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/sSLPolicy:SSLPolicy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/sSLPolicy:SSLPolicy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ssl_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SSLPolicyArgs {
        /// Profile specifies the set of SSL features that can be used by the
        /// load balancer when negotiating SSL with clients. This can be one of
        /// `COMPATIBLE`, `MODERN`, `RESTRICTED`, or `CUSTOM`. If using `CUSTOM`,
        /// the set of SSL features to enable must be specified in the
        /// `customFeatures` field.
        /// See the [official documentation](https://cloud.google.com/compute/docs/load-balancing/ssl-policies#profilefeaturesupport)
        /// for which ciphers are available to use. **Note**: this argument
        /// *must* be present when using the `CUSTOM` profile. This argument
        /// *must not* be present when using any other profile.
        #[builder(into, default)]
        pub custom_features: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum version of SSL protocol that can be used by the clients
        /// to establish a connection with the load balancer.
        /// Default value is `TLS_1_0`.
        /// Possible values are: `TLS_1_0`, `TLS_1_1`, `TLS_1_2`.
        #[builder(into, default)]
        pub min_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Profile specifies the set of SSL features that can be used by the
        /// load balancer when negotiating SSL with clients. If using `CUSTOM`,
        /// the set of SSL features to enable must be specified in the
        /// `customFeatures` field.
        /// See the [official documentation](https://cloud.google.com/compute/docs/load-balancing/ssl-policies#profilefeaturesupport)
        /// for information on what cipher suites each profile provides. If
        /// `CUSTOM` is used, the `custom_features` attribute **must be set**.
        /// Default value is `COMPATIBLE`.
        /// Possible values are: `COMPATIBLE`, `MODERN`, `RESTRICTED`, `CUSTOM`.
        #[builder(into, default)]
        pub profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SSLPolicyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Profile specifies the set of SSL features that can be used by the
        /// load balancer when negotiating SSL with clients. This can be one of
        /// `COMPATIBLE`, `MODERN`, `RESTRICTED`, or `CUSTOM`. If using `CUSTOM`,
        /// the set of SSL features to enable must be specified in the
        /// `customFeatures` field.
        /// See the [official documentation](https://cloud.google.com/compute/docs/load-balancing/ssl-policies#profilefeaturesupport)
        /// for which ciphers are available to use. **Note**: this argument
        /// *must* be present when using the `CUSTOM` profile. This argument
        /// *must not* be present when using any other profile.
        pub custom_features: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list of features enabled in the SSL policy.
        pub enabled_features: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Fingerprint of this resource. A hash of the contents stored in this
        /// object. This field is used in optimistic locking.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The minimum version of SSL protocol that can be used by the clients
        /// to establish a connection with the load balancer.
        /// Default value is `TLS_1_0`.
        /// Possible values are: `TLS_1_0`, `TLS_1_1`, `TLS_1_2`.
        pub min_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Profile specifies the set of SSL features that can be used by the
        /// load balancer when negotiating SSL with clients. If using `CUSTOM`,
        /// the set of SSL features to enable must be specified in the
        /// `customFeatures` field.
        /// See the [official documentation](https://cloud.google.com/compute/docs/load-balancing/ssl-policies#profilefeaturesupport)
        /// for information on what cipher suites each profile provides. If
        /// `CUSTOM` is used, the `custom_features` attribute **must be set**.
        /// Default value is `COMPATIBLE`.
        /// Possible values are: `COMPATIBLE`, `MODERN`, `RESTRICTED`, `CUSTOM`.
        pub profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SSLPolicyArgs,
    ) -> SSLPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_features_binding_1 = args.custom_features.get_output(context);
        let custom_features_binding = custom_features_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let min_tls_version_binding_1 = args.min_tls_version.get_output(context);
        let min_tls_version_binding = min_tls_version_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let profile_binding_1 = args.profile.get_output(context);
        let profile_binding = profile_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/sSLPolicy:SSLPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customFeatures".into(),
                    value: &custom_features_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "minTlsVersion".into(),
                    value: &min_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profile".into(),
                    value: &profile_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SSLPolicyResult {
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            custom_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customFeatures"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledFeatures"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            min_tls_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minTlsVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("profile"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
