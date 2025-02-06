/// The SSH public key information associated with a Google account.
///
///
/// To get more information about SSHPublicKey, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/oslogin/rest/v1/users.sshPublicKeys)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/oslogin)
///
/// ## Example Usage
///
/// ### Os Login Ssh Key Basic
///
///
/// ```yaml
/// resources:
///   cache:
///     type: gcp:oslogin:SshPublicKey
///     properties:
///       user: ${me.email}
///       key:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/id_rsa.pub
///           return: result
/// variables:
///   me:
///     fn::invoke:
///       function: gcp:organizations:getClientOpenIdUserInfo
///       arguments: {}
/// ```
///
/// ## Import
///
/// SSHPublicKey can be imported using any of these accepted formats:
///
/// * `users/{{user}}/sshPublicKeys/{{fingerprint}}`
///
/// * `{{user}}/{{fingerprint}}`
///
/// When using the `pulumi import` command, SSHPublicKey can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:oslogin/sshPublicKey:SshPublicKey default users/{{user}}/sshPublicKeys/{{fingerprint}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oslogin/sshPublicKey:SshPublicKey default {{user}}/{{fingerprint}}
/// ```
///
pub mod ssh_public_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SshPublicKeyArgs {
        /// An expiration time in microseconds since epoch.
        #[builder(into, default)]
        pub expiration_time_usec: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Public key text in SSH format, defined by RFC4253 section 6.6.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project ID of the Google Cloud Platform project.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user email.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SshPublicKeyResult {
        /// An expiration time in microseconds since epoch.
        pub expiration_time_usec: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SHA-256 fingerprint of the SSH public key.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Public key text in SSH format, defined by RFC4253 section 6.6.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The project ID of the Google Cloud Platform project.
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user email.
        ///
        ///
        /// - - -
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SshPublicKeyArgs,
    ) -> SshPublicKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let expiration_time_usec_binding = args
            .expiration_time_usec
            .get_output(context)
            .get_inner();
        let key_binding = args.key.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:oslogin/sshPublicKey:SshPublicKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expirationTimeUsec".into(),
                    value: &expiration_time_usec_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SshPublicKeyResult {
            expiration_time_usec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationTimeUsec"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            user: pulumi_gestalt_rust::__private::into_domain(o.extract_field("user")),
        }
    }
}
