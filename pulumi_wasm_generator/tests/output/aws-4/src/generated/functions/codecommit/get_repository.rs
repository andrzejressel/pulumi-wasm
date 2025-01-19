pub mod get_repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// Name for the repository. This needs to be less than 100 characters.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        /// ARN of the repository.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// URL to use for cloning the repository over HTTPS.
        pub clone_url_http: pulumi_wasm_rust::Output<String>,
        /// URL to use for cloning the repository over SSH.
        pub clone_url_ssh: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the encryption key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// ID of the repository.
        pub repository_id: pulumi_wasm_rust::Output<String>,
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRepositoryArgs) -> GetRepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let repository_name_binding = args.repository_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codecommit/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cloneUrlHttp".into(),
                },
                register_interface::ResultField {
                    name: "cloneUrlSsh".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRepositoryResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            clone_url_http: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloneUrlHttp").unwrap(),
            ),
            clone_url_ssh: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloneUrlSsh").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            repository_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryId").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
            ),
        }
    }
}
