#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretVersionArgs {
        /// Specifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the unique identifier of the version of the secret that you want to retrieve. Overrides `version_stage`.
        #[builder(into, default)]
        pub version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the secret version that you want to retrieve by the staging label attached to the version. Defaults to `AWSCURRENT`.
        #[builder(into, default)]
        pub version_stage: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecretVersionResult {
        /// ARN of the secret.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Created date of the secret in UTC.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Decrypted part of the protected secret information that was originally provided as a binary.
        pub secret_binary: pulumi_gestalt_rust::Output<String>,
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// Decrypted part of the protected secret information that was originally provided as a string.
        pub secret_string: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of this version of the secret.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        pub version_stage: pulumi_gestalt_rust::Output<Option<String>>,
        pub version_stages: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSecretVersionArgs,
    ) -> GetSecretVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let secret_id_binding = args.secret_id.get_output(context);
        let version_id_binding = args.version_id.get_output(context);
        let version_stage_binding = args.version_stage.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:secretsmanager/getSecretVersion:getSecretVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionStage".into(),
                    value: &version_stage_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretVersionResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            id: o.get_field("id"),
            secret_binary: o.get_field("secretBinary"),
            secret_id: o.get_field("secretId"),
            secret_string: o.get_field("secretString"),
            version_id: o.get_field("versionId"),
            version_stage: o.get_field("versionStage"),
            version_stages: o.get_field("versionStages"),
        }
    }
}
