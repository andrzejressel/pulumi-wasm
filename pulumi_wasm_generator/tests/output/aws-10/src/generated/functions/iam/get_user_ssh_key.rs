pub mod get_user_ssh_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserSshKeyArgs {
        /// Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use `SSH`. To retrieve the public key in PEM format, use `PEM`.
        #[builder(into)]
        pub encoding: pulumi_wasm_rust::InputOrOutput<String>,
        /// Unique identifier for the SSH public key.
        #[builder(into)]
        pub ssh_public_key_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the IAM user associated with the SSH public key.
        #[builder(into)]
        pub username: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserSshKeyResult {
        pub encoding: pulumi_wasm_rust::Output<String>,
        /// MD5 message digest of the SSH public key.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// SSH public key.
        pub public_key: pulumi_wasm_rust::Output<String>,
        pub ssh_public_key_id: pulumi_wasm_rust::Output<String>,
        /// Status of the SSH public key. Active means that the key can be used for authentication with an CodeCommit repository. Inactive means that the key cannot be used.
        pub status: pulumi_wasm_rust::Output<String>,
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserSshKeyArgs,
    ) -> GetUserSshKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encoding_binding = args.encoding.get_output(context).get_inner();
        let ssh_public_key_id_binding = args
            .ssh_public_key_id
            .get_output(context)
            .get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getUserSshKey:getUserSshKey".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encoding".into(),
                    value: &encoding_binding,
                },
                register_interface::ObjectField {
                    name: "sshPublicKeyId".into(),
                    value: &ssh_public_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encoding".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "sshPublicKeyId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserSshKeyResult {
            encoding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encoding").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            ssh_public_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshPublicKeyId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
