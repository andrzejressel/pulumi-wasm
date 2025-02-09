#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretVersionArgs {
        /// If set to 'true', the secret data is
        /// expected to be base64-encoded string.
        #[builder(into, default)]
        pub is_secret_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The project to get the secret version for. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The secret to get the secret version for.
        #[builder(into)]
        pub secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the secret to get. If it
        /// is not provided, the latest version is retrieved.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecretVersionResult {
        /// The time at which the Secret was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The time at which the Secret was destroyed. Only present if state is DESTROYED.
        pub destroy_time: pulumi_gestalt_rust::Output<String>,
        /// True if the current state of the SecretVersion is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub is_secret_data_base64: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource name of the SecretVersion. Format:
        /// `projects/{{project}}/secrets/{{secret_id}}/versions/{{version}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub secret: pulumi_gestalt_rust::Output<String>,
        /// The secret data. No larger than 64KiB.
        pub secret_data: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretVersionArgs,
    ) -> GetSecretVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let is_secret_data_base64_binding_1 = args
            .is_secret_data_base64
            .get_output(context);
        let is_secret_data_base64_binding = is_secret_data_base64_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let secret_binding_1 = args.secret.get_output(context);
        let secret_binding = secret_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:secretmanager/getSecretVersion:getSecretVersion".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "isSecretDataBase64".into(),
                    value: &is_secret_data_base64_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "secret".into(),
                    value: &secret_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecretVersionResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            destroy_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destroyTime"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            is_secret_data_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isSecretDataBase64"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secret"),
            ),
            secret_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretData"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
