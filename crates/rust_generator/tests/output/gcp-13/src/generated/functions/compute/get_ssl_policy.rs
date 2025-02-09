#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetSslPolicyArgs,
    ) -> GetSslPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getSSLPolicy:getSSLPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSslPolicyResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            custom_features: o.get_field("customFeatures"),
            description: o.get_field("description"),
            enabled_features: o.get_field("enabledFeatures"),
            fingerprint: o.get_field("fingerprint"),
            id: o.get_field("id"),
            min_tls_version: o.get_field("minTlsVersion"),
            name: o.get_field("name"),
            profile: o.get_field("profile"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
        }
    }
}
