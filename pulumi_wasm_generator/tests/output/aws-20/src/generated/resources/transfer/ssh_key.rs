/// Provides a AWS Transfer User SSH Key resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   examplePrivateKey:
///     type: tls:PrivateKey
///     name: example
///     properties:
///       algorithm: RSA
///       rsaBits: 4096
///   exampleSshKey:
///     type: aws:transfer:SshKey
///     name: example
///     properties:
///       serverId: ${exampleServer.id}
///       userName: ${exampleUser.userName}
///       body:
///         fn::invoke:
///           function: std:trimspace
///           arguments:
///             input: ${examplePrivateKey.publicKeyOpenssh}
///           return: result
///   exampleServer:
///     type: aws:transfer:Server
///     name: example
///     properties:
///       identityProviderType: SERVICE_MANAGED
///       tags:
///         NAME: tf-acc-test-transfer-server
///   exampleUser:
///     type: aws:transfer:User
///     name: example
///     properties:
///       serverId: ${exampleServer.id}
///       userName: tftestuser
///       role: ${exampleRole.arn}
///       tags:
///         NAME: tftestuser
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: tf-test-transfer-user-iam-role
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: tf-test-transfer-user-iam-policy
///       role: ${exampleRole.id}
///       policy: ${example.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - transfer.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: AllowFullAccesstoS3
///             effect: Allow
///             actions:
///               - s3:*
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer SSH Public Key using the `server_id` and `user_name` and `ssh_public_key_id` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/sshKey:SshKey bar s-12345678/test-username/key-12345
/// ```
pub mod ssh_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SshKeyArgs {
        /// The public key portion of an SSH key pair.
        #[builder(into)]
        pub body: pulumi_wasm_rust::Output<String>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The name of the user account that is assigned to one or more servers.
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SshKeyResult {
        /// The public key portion of an SSH key pair.
        pub body: pulumi_wasm_rust::Output<String>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        pub server_id: pulumi_wasm_rust::Output<String>,
        pub ssh_key_id: pulumi_wasm_rust::Output<String>,
        /// The name of the user account that is assigned to one or more servers.
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SshKeyArgs) -> SshKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let body_binding = args.body.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/sshKey:SshKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "sshKeyId".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SshKeyResult {
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            ssh_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshKeyId").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}
