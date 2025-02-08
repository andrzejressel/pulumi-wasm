#[allow(clippy::doc_lazy_continuation)]
pub mod get_ssl_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSslPolicyArgs {
        /// The name of the SSL Policy.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSslPolicyResult {
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// If the `profile` is `CUSTOM`, these are the custom encryption
        /// ciphers supported by the profile. If the `profile` is *not* `CUSTOM`, this
        /// attribute will be empty.
        pub custom_features: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Description of this SSL Policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The set of enabled encryption ciphers as a result of the policy config
        pub enabled_features: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Fingerprint of this resource.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The minimum supported TLS version of this policy.
        pub min_tls_version: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Google-curated or custom profile used by this policy.
        pub profile: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSslPolicyArgs,
    ) -> GetSslPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getSSLPolicy:getSSLPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSslPolicyResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
