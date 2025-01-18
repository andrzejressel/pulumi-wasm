pub mod get_secret_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretVersionArgs {
        /// Specifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
        #[builder(into)]
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the unique identifier of the version of the secret that you want to retrieve. Overrides `version_stage`.
        #[builder(into, default)]
        pub version_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the secret version that you want to retrieve by the staging label attached to the version. Defaults to `AWSCURRENT`.
        #[builder(into, default)]
        pub version_stage: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecretVersionResult {
        /// ARN of the secret.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Created date of the secret in UTC.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Decrypted part of the protected secret information that was originally provided as a binary.
        pub secret_binary: pulumi_wasm_rust::Output<String>,
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// Decrypted part of the protected secret information that was originally provided as a string.
        pub secret_string: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of this version of the secret.
        pub version_id: pulumi_wasm_rust::Output<String>,
        pub version_stage: pulumi_wasm_rust::Output<Option<String>>,
        pub version_stages: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSecretVersionArgs) -> GetSecretVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let secret_id_binding = args.secret_id.get_inner();
        let version_id_binding = args.version_id.get_inner();
        let version_stage_binding = args.version_stage.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "secretBinary".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
                register_interface::ResultField {
                    name: "secretString".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
                register_interface::ResultField {
                    name: "versionStage".into(),
                },
                register_interface::ResultField {
                    name: "versionStages".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSecretVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            secret_binary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretBinary").unwrap(),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
            secret_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretString").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
            version_stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionStage").unwrap(),
            ),
            version_stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionStages").unwrap(),
            ),
        }
    }
}
