pub mod get_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// The name of the namespace.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNamespaceArgs,
    ) -> GetNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshiftserverless/getNamespace:getNamespace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNamespaceResult {
            admin_username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminUsername"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            db_name: pulumi_wasm_rust::__private::into_domain(o.extract_field("dbName")),
            default_iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultIamRoleArn"),
            ),
            iam_roles: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRoles"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            log_exports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logExports"),
            ),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
        }
    }
}
