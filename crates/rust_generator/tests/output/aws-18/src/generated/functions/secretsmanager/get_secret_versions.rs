#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretVersionsArgs {
        /// If true, all deprecated secret versions are included in the response.
        /// If false, no deprecated secret versions are included in the response. If no value is specified, the default value is `false`.
        #[builder(into, default)]
        pub include_deprecated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSecretVersionsResult {
        /// ARN of the secret.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_deprecated: pulumi_gestalt_rust::Output<Option<bool>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// List of the versions of the secret. Attributes are specified below.
        pub versions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretsmanager::GetSecretVersionsVersion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSecretVersionsArgs,
    ) -> GetSecretVersionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let include_deprecated_binding = args.include_deprecated.get_output(context);
        let secret_id_binding = args.secret_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:secretsmanager/getSecretVersions:getSecretVersions".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeDeprecated".into(),
                    value: include_deprecated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretId".into(),
                    value: secret_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretVersionsResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            include_deprecated: o.get_field("includeDeprecated"),
            name: o.get_field("name"),
            secret_id: o.get_field("secretId"),
            versions: o.get_field("versions"),
        }
    }
}
