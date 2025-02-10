#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountKeyArgs {
        /// The name of the service account key. This must have format
        /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{KEYID}`, where `{ACCOUNT}`
        /// is the email address or unique id of the service account.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project that the service account is present in.
        /// Defaults to the provider project configuration.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The output format of the public key requested. TYPE_X509_PEM_FILE is the default output format.
        #[builder(into, default)]
        pub public_key_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAccountKeyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_algorithm: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The public key, base64 encoded
        pub public_key: pulumi_gestalt_rust::Output<String>,
        pub public_key_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountKeyArgs,
    ) -> GetAccountKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let public_key_type_binding = args.public_key_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:serviceaccount/getAccountKey:getAccountKey".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKeyType".into(),
                    value: public_key_type_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountKeyResult {
            id: o.get_field("id"),
            key_algorithm: o.get_field("keyAlgorithm"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            public_key: o.get_field("publicKey"),
            public_key_type: o.get_field("publicKeyType"),
        }
    }
}
