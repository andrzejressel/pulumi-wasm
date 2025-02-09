#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretArgs {
        /// ARN of the secret to retrieve.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the secret to retrieve.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags of the secret.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSecretResult {
        /// ARN of the secret.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Created date of the secret in UTC.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the secret.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key Management Service (KMS) Customer Master Key (CMK) associated with the secret.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Last updated date of the secret in UTC.
        pub last_changed_date: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource-based policy document that's attached to the secret.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Tags of the secret.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSecretArgs,
    ) -> GetSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:secretsmanager/getSecret:getSecret".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            last_changed_date: o.get_field("lastChangedDate"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
            tags: o.get_field("tags"),
        }
    }
}
