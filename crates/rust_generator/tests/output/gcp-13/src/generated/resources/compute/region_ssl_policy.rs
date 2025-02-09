/// Represents a Regional SSL policy. SSL policies give you the ability to control the
/// features of SSL that your SSL proxy or HTTPS load balancer negotiates.
///
///
/// To get more information about RegionSslPolicy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionSslPolicies)
/// * How-to Guides
///     * [Using SSL Policies](https://cloud.google.com/compute/docs/load-balancing/ssl-policies)
///
/// ## Import
///
/// RegionSslPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/sslPolicies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionSslPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionSslPolicy:RegionSslPolicy default projects/{{project}}/regions/{{region}}/sslPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSslPolicy:RegionSslPolicy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSslPolicy:RegionSslPolicy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionSslPolicy:RegionSslPolicy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_ssl_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionSslPolicyArgs {
        /// A list of features enabled when the selected profile is CUSTOM. The
        /// method returns the set of features that can be specified in this
        /// list. This field must be empty if the profile is not CUSTOM.
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
        /// The region where the regional SSL policy resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionSslPolicyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// A list of features enabled when the selected profile is CUSTOM. The
        /// method returns the set of features that can be specified in this
        /// list. This field must be empty if the profile is not CUSTOM.
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
        /// The region where the regional SSL policy resides.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionSslPolicyArgs,
    ) -> RegionSslPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_features_binding = args.custom_features.get_output(context);
        let description_binding = args.description.get_output(context);
        let min_tls_version_binding = args.min_tls_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let profile_binding = args.profile.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionSslPolicy:RegionSslPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customFeatures".into(),
                    value: custom_features_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minTlsVersion".into(),
                    value: min_tls_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profile".into(),
                    value: profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionSslPolicyResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            custom_features: o.get_field("customFeatures"),
            description: o.get_field("description"),
            enabled_features: o.get_field("enabledFeatures"),
            fingerprint: o.get_field("fingerprint"),
            min_tls_version: o.get_field("minTlsVersion"),
            name: o.get_field("name"),
            profile: o.get_field("profile"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
        }
    }
}
