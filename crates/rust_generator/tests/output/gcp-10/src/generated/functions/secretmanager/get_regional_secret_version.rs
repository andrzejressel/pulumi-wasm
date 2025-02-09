#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_regional_secret_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionalSecretVersionArgs {
        /// If set to 'true', the secret data is
        /// expected to be base64-encoded string.
        #[builder(into, default)]
        pub is_secret_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Location of Secret Manager regional secret resource.
        /// It must be provided when the `secret` field provided consists of only the name of the regional secret.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project to get the secret version for. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The regional secret to get the secret version for.
        /// This can be either the reference of the regional secret as in `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}` or only the name of the regional secret as in `{{secret_id}}`. If only the name of the regional secret is provided, the location must also be provided.
        #[builder(into)]
        pub secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the regional secret to get. If it
        /// is not provided, the latest version is retrieved.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionalSecretVersionResult {
        /// The time at which the regional secret was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The customer-managed encryption configuration of the regional secret. Structure is documented below.
        pub customer_managed_encryptions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::secretmanager::GetRegionalSecretVersionCustomerManagedEncryption,
            >,
        >,
        /// The time at which the regional secret was destroyed. Only present if state is DESTROYED.
        pub destroy_time: pulumi_gestalt_rust::Output<String>,
        /// True if the current state of the regional SecretVersion is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub is_secret_data_base64: pulumi_gestalt_rust::Output<Option<bool>>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the regional SecretVersion. Format:
        /// `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}`
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
        context: &pulumi_gestalt_rust::Context,
        args: GetRegionalSecretVersionArgs,
    ) -> GetRegionalSecretVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let is_secret_data_base64_binding = args
            .is_secret_data_base64
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let secret_binding = args.secret.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:secretmanager/getRegionalSecretVersion:getRegionalSecretVersion"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isSecretDataBase64".into(),
                    value: is_secret_data_base64_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secret".into(),
                    value: secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegionalSecretVersionResult {
            create_time: o.get_field("createTime"),
            customer_managed_encryptions: o.get_field("customerManagedEncryptions"),
            destroy_time: o.get_field("destroyTime"),
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
            is_secret_data_base64: o.get_field("isSecretDataBase64"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            secret: o.get_field("secret"),
            secret_data: o.get_field("secretData"),
            version: o.get_field("version"),
        }
    }
}
