pub mod get_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// The name of the namespace.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// The username of the administrator for the first database created in the namespace.
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Namespace.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the first database created in the namespace.
        pub db_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. When specifying `default_iam_role_arn`, it also must be part of `iam_roles`.
        pub default_iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// A list of IAM roles to associate with the namespace.
        pub iam_roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Amazon Web Services Key Management Service key used to encrypt your data.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The types of logs the namespace can export. Available export types are `userlog`, `connectionlog`, and `useractivitylog`.
        pub log_exports: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Redshift Namespace ID.
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNamespaceArgs) -> GetNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let namespace_name_binding = args.namespace_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshiftserverless/getNamespace:getNamespace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminUsername".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dbName".into(),
                },
                register_interface::ResultField {
                    name: "defaultIamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "iamRoles".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "logExports".into(),
                },
                register_interface::ResultField {
                    name: "namespaceId".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNamespaceResult {
            admin_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminUsername").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            db_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbName").unwrap(),
            ),
            default_iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultIamRoleArn").unwrap(),
            ),
            iam_roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoles").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            log_exports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logExports").unwrap(),
            ),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceId").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
        }
    }
}
