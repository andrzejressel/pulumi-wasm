/// Creates a new Amazon Redshift Serverless Namespace.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = namespace::create(
///         "example",
///         NamespaceArgs::builder().namespace_name("concurrency-scaling").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Namespaces using the `namespace_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/namespace:Namespace example example
/// ```
pub mod namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// ID of the KMS key used to encrypt the namespace's admin credentials secret.
        #[builder(into, default)]
        pub admin_password_secret_kms_key_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The password of the administrator for the first database created in the namespace.
        /// Conflicts with `manage_admin_password`.
        #[builder(into, default)]
        pub admin_user_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The username of the administrator for the first database created in the namespace.
        #[builder(into, default)]
        pub admin_username: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the first database created in the namespace.
        #[builder(into, default)]
        pub db_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. When specifying `default_iam_role_arn`, it also must be part of `iam_roles`.
        #[builder(into, default)]
        pub default_iam_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of IAM roles to associate with the namespace.
        #[builder(into, default)]
        pub iam_roles: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ARN of the Amazon Web Services Key Management Service key used to encrypt your data.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The types of logs the namespace can export. Available export types are `userlog`, `connectionlog`, and `useractivitylog`.
        #[builder(into, default)]
        pub log_exports: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether to use AWS SecretManager to manage namespace's admin credentials.
        /// Conflicts with `admin_user_password`.
        #[builder(into, default)]
        pub manage_admin_password: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the namespace.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        pub admin_password_secret_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the KMS key used to encrypt the namespace's admin credentials secret.
        pub admin_password_secret_kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The password of the administrator for the first database created in the namespace.
        /// Conflicts with `manage_admin_password`.
        pub admin_user_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The username of the administrator for the first database created in the namespace.
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Namespace.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the first database created in the namespace.
        pub db_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. When specifying `default_iam_role_arn`, it also must be part of `iam_roles`.
        pub default_iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IAM roles to associate with the namespace.
        pub iam_roles: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN of the Amazon Web Services Key Management Service key used to encrypt your data.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The types of logs the namespace can export. Available export types are `userlog`, `connectionlog`, and `useractivitylog`.
        pub log_exports: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to use AWS SecretManager to manage namespace's admin credentials.
        /// Conflicts with `admin_user_password`.
        pub manage_admin_password: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Redshift Namespace ID.
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        /// The name of the namespace.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NamespaceArgs,
    ) -> NamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_password_secret_kms_key_id_binding = args
            .admin_password_secret_kms_key_id
            .get_output(context)
            .get_inner();
        let admin_user_password_binding = args
            .admin_user_password
            .get_output(context)
            .get_inner();
        let admin_username_binding = args.admin_username.get_output(context).get_inner();
        let db_name_binding = args.db_name.get_output(context).get_inner();
        let default_iam_role_arn_binding = args
            .default_iam_role_arn
            .get_output(context)
            .get_inner();
        let iam_roles_binding = args.iam_roles.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let log_exports_binding = args.log_exports.get_output(context).get_inner();
        let manage_admin_password_binding = args
            .manage_admin_password
            .get_output(context)
            .get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshiftserverless/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminPasswordSecretKmsKeyId".into(),
                    value: &admin_password_secret_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "adminUserPassword".into(),
                    value: &admin_user_password_binding,
                },
                register_interface::ObjectField {
                    name: "adminUsername".into(),
                    value: &admin_username_binding,
                },
                register_interface::ObjectField {
                    name: "dbName".into(),
                    value: &db_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultIamRoleArn".into(),
                    value: &default_iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoles".into(),
                    value: &iam_roles_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "logExports".into(),
                    value: &log_exports_binding,
                },
                register_interface::ObjectField {
                    name: "manageAdminPassword".into(),
                    value: &manage_admin_password_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamespaceResult {
            admin_password_secret_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminPasswordSecretArn"),
            ),
            admin_password_secret_kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminPasswordSecretKmsKeyId"),
            ),
            admin_user_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminUserPassword"),
            ),
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
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            log_exports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logExports"),
            ),
            manage_admin_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("manageAdminPassword"),
            ),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
