#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// The name of the namespace.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// The username of the administrator for the first database created in the namespace.
        pub admin_username: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Namespace.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the first database created in the namespace.
        pub db_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. When specifying `default_iam_role_arn`, it also must be part of `iam_roles`.
        pub default_iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// A list of IAM roles to associate with the namespace.
        pub iam_roles: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Amazon Web Services Key Management Service key used to encrypt your data.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The types of logs the namespace can export. Available export types are `userlog`, `connectionlog`, and `useractivitylog`.
        pub log_exports: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Redshift Namespace ID.
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNamespaceArgs,
    ) -> GetNamespaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let namespace_name_binding = args.namespace_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:redshiftserverless/getNamespace:getNamespace".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: namespace_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNamespaceResult {
            admin_username: o.get_field("adminUsername"),
            arn: o.get_field("arn"),
            db_name: o.get_field("dbName"),
            default_iam_role_arn: o.get_field("defaultIamRoleArn"),
            iam_roles: o.get_field("iamRoles"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            log_exports: o.get_field("logExports"),
            namespace_id: o.get_field("namespaceId"),
            namespace_name: o.get_field("namespaceName"),
        }
    }
}
