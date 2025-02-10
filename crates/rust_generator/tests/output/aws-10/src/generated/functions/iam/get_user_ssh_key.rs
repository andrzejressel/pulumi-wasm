#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user_ssh_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserSshKeyArgs {
        /// Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use `SSH`. To retrieve the public key in PEM format, use `PEM`.
        #[builder(into)]
        pub encoding: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique identifier for the SSH public key.
        #[builder(into)]
        pub ssh_public_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the IAM user associated with the SSH public key.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserSshKeyResult {
        pub encoding: pulumi_gestalt_rust::Output<String>,
        /// MD5 message digest of the SSH public key.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// SSH public key.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        pub ssh_public_key_id: pulumi_gestalt_rust::Output<String>,
        /// Status of the SSH public key. Active means that the key can be used for authentication with an CodeCommit repository. Inactive means that the key cannot be used.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserSshKeyArgs,
    ) -> GetUserSshKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encoding_binding = args.encoding.get_output(context);
        let ssh_public_key_id_binding = args.ssh_public_key_id.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getUserSshKey:getUserSshKey".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encoding".into(),
                    value: encoding_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshPublicKeyId".into(),
                    value: ssh_public_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: username_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserSshKeyResult {
            encoding: o.get_field("encoding"),
            fingerprint: o.get_field("fingerprint"),
            id: o.get_field("id"),
            public_key: o.get_field("publicKey"),
            ssh_public_key_id: o.get_field("sshPublicKeyId"),
            status: o.get_field("status"),
            username: o.get_field("username"),
        }
    }
}
