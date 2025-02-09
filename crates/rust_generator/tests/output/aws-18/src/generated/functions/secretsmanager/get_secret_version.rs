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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretVersionArgs,
    ) -> GetSecretVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let secret_id_binding_1 = args.secret_id.get_output(context);
        let secret_id_binding = secret_id_binding_1.get_inner();
        let version_id_binding_1 = args.version_id.get_output(context);
        let version_id_binding = version_id_binding_1.get_inner();
        let version_stage_binding_1 = args.version_stage.get_output(context);
        let version_stage_binding = version_stage_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:secretsmanager/getSecretVersion:getSecretVersion".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
                },
                register_interface::ObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding,
                },
                register_interface::ObjectField {
                    name: "versionStage".into(),
                    value: &version_stage_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecretVersionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            secret_binary: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretBinary"),
            ),
            secret_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretId"),
            ),
            secret_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretString"),
            ),
            version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
            version_stage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionStage"),
            ),
            version_stages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionStages"),
            ),
        }
    }
}
