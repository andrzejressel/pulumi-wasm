#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret_rotation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretRotationArgs {
        /// Specifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSecretRotationResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the secret.
        pub rotation_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Decrypted part of the protected secret information that was originally provided as a string.
        pub rotation_lambda_arn: pulumi_gestalt_rust::Output<String>,
        /// Decrypted part of the protected secret information that was originally provided as a binary. Base64 encoded.
        pub rotation_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::secretsmanager::GetSecretRotationRotationRule,
            >,
        >,
        pub secret_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretRotationArgs,
    ) -> GetSecretRotationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let secret_id_binding = args.secret_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:secretsmanager/getSecretRotation:getSecretRotation".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecretRotationResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            rotation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rotationEnabled"),
            ),
            rotation_lambda_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rotationLambdaArn"),
            ),
            rotation_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rotationRules"),
            ),
            secret_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretId"),
            ),
        }
    }
}
