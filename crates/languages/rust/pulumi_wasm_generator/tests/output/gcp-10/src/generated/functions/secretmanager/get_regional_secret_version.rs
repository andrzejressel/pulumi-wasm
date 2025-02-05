pub mod get_regional_secret_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionalSecretVersionArgs {
        /// If set to 'true', the secret data is
        /// expected to be base64-encoded string.
        #[builder(into, default)]
        pub is_secret_data_base64: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Location of Secret Manager regional secret resource.
        /// It must be provided when the `secret` field provided consists of only the name of the regional secret.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The project to get the secret version for. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The regional secret to get the secret version for.
        /// This can be either the reference of the regional secret as in `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}` or only the name of the regional secret as in `{{secret_id}}`. If only the name of the regional secret is provided, the location must also be provided.
        #[builder(into)]
        pub secret: pulumi_wasm_rust::InputOrOutput<String>,
        /// The version of the regional secret to get. If it
        /// is not provided, the latest version is retrieved.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionalSecretVersionResult {
        /// The time at which the regional secret was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The customer-managed encryption configuration of the regional secret. Structure is documented below.
        pub customer_managed_encryptions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::secretmanager::GetRegionalSecretVersionCustomerManagedEncryption,
            >,
        >,
        /// The time at which the regional secret was destroyed. Only present if state is DESTROYED.
        pub destroy_time: pulumi_wasm_rust::Output<String>,
        /// True if the current state of the regional SecretVersion is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub is_secret_data_base64: pulumi_wasm_rust::Output<Option<bool>>,
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the regional SecretVersion. Format:
        /// `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}`
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<String>,
        /// The secret data. No larger than 64KiB.
        pub secret_data: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegionalSecretVersionArgs,
    ) -> GetRegionalSecretVersionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let is_secret_data_base64_binding = args
            .is_secret_data_base64
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let secret_binding = args.secret.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:secretmanager/getRegionalSecretVersion:getRegionalSecretVersion"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "isSecretDataBase64".into(),
                    value: &is_secret_data_base64_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
        GetRegionalSecretVersionResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            customer_managed_encryptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerManagedEncryptions"),
            ),
            destroy_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destroyTime"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            is_secret_data_base64: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isSecretDataBase64"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            secret: pulumi_wasm_rust::__private::into_domain(o.extract_field("secret")),
            secret_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secretData"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
