/// Uploads an SSH public key and associates it with the specified IAM user.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let user = user::create(
///         "user",
///         UserArgs::builder().name("test-user").path("/").build_struct(),
///     );
///     let userSshKey = ssh_key::create(
///         "userSshKey",
///         SshKeyArgs::builder()
///             .encoding("SSH")
///             .public_key(
///                 "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQD3F6tyPEFEzV0LX3X8BsXdMsQz1x2cEikKDEY0aIj41qgxMCP/iteneqXSIFZBp5vizPvaoIR3Um9xK7PGoW8giupGn+EPuxIA4cDM4vzOqOkiMPhz5XK0whEjkVzTo4+S0puvDZuwIsdiW9mxhJc7tgBNL0cYlWSYVkz4G/fslNfRPW5mYAM49f4fhtxPb5ok4Q2Lg9dPKVHO/Bgeu5woMc7RY0p1ej6D4CKFE6lymSDJpW0YHX/wqE9+cfEauh7xZcG0q9t2ta6F6fmX0agvpFyZo8aFbXeUBr7osSCJNgvavWbM/06niWrOvYX2xwWdhXmXSrbX8ZbabVohBK41 mytest@mydomain.com",
///             )
///             .username("${user.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSH public keys using the `username`, `ssh_public_key_id`, and `encoding`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/sshKey:SshKey user user:APKAJNCNNJICVN7CFKCA:SSH
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ssh_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SshKeyArgs {
        /// Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use `SSH`. To retrieve the public key in PEM format, use `PEM`.
        #[builder(into)]
        pub encoding: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SSH public key. The public key must be encoded in ssh-rsa format or PEM format.
        #[builder(into)]
        pub public_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The status to assign to the SSH public key. Active means the key can be used for authentication with an AWS CodeCommit repository. Inactive means the key cannot be used. Default is `active`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the IAM user to associate the SSH public key with.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SshKeyResult {
        /// Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use `SSH`. To retrieve the public key in PEM format, use `PEM`.
        pub encoding: pulumi_gestalt_rust::Output<String>,
        /// The MD5 message digest of the SSH public key.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The SSH public key. The public key must be encoded in ssh-rsa format or PEM format.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the SSH public key.
        pub ssh_public_key_id: pulumi_gestalt_rust::Output<String>,
        /// The status to assign to the SSH public key. Active means the key can be used for authentication with an AWS CodeCommit repository. Inactive means the key cannot be used. Default is `active`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The name of the IAM user to associate the SSH public key with.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SshKeyArgs,
    ) -> SshKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encoding_binding = args.encoding.get_output(context);
        let public_key_binding = args.public_key.get_output(context);
        let status_binding = args.status.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/sshKey:SshKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encoding".into(),
                    value: encoding_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKey".into(),
                    value: public_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: username_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SshKeyResult {
            encoding: o.get_field("encoding"),
            fingerprint: o.get_field("fingerprint"),
            public_key: o.get_field("publicKey"),
            ssh_public_key_id: o.get_field("sshPublicKeyId"),
            status: o.get_field("status"),
            username: o.get_field("username"),
        }
    }
}
