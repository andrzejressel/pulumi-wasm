/// Resource for managing an Amazon S3 Tables Table Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleTablePolicy:
///     type: aws:s3tables:TablePolicy
///     name: example
///     properties:
///       resourcePolicy: ${example.json}
///       name: ${test.name}
///       namespace: ${test.namespace}
///       tableBucketArn: ${test.tableBucketArn}
///   exampleTable:
///     type: aws:s3tables:Table
///     name: example
///     properties:
///       name: example-table
///       namespace: ${exampleNamespace}
///       tableBucketArn: ${exampleNamespace.tableBucketArn}
///       format: ICEBERG
///   exampleNamespace:
///     type: aws:s3tables:Namespace
///     name: example
///     properties:
///       namespace:
///         - example-namespace
///       tableBucketArn: ${exampleTableBucket.arn}
///   exampleTableBucket:
///     type: aws:s3tables:TableBucket
///     name: example
///     properties:
///       name: example-bucket
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Tables Table Policy using the `table_bucket_arn`, the value of `namespace`, and the value of `name`, separated by a semicolon (`;`). For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/tablePolicy:TablePolicy example 'arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket;example-namespace;example-table'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TablePolicyArgs {
        /// Name of the table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the namespace for this table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        #[builder(into)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Web Services resource-based policy document in JSON format.
        #[builder(into)]
        pub resource_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN referencing the Table Bucket that contains this Namespace.
        #[builder(into)]
        pub table_bucket_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TablePolicyResult {
        /// Name of the table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of the namespace for this table.
        /// Must be between 1 and 255 characters in length.
        /// Can consist of lowercase letters, numbers, and underscores, and must begin and end with a lowercase letter or number.
        pub namespace: pulumi_gestalt_rust::Output<String>,
        /// Amazon Web Services resource-based policy document in JSON format.
        pub resource_policy: pulumi_gestalt_rust::Output<String>,
        /// ARN referencing the Table Bucket that contains this Namespace.
        pub table_bucket_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TablePolicyArgs,
    ) -> TablePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let resource_policy_binding = args.resource_policy.get_output(context);
        let table_bucket_arn_binding = args.table_bucket_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3tables/tablePolicy:TablePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: namespace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourcePolicy".into(),
                    value: resource_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableBucketArn".into(),
                    value: table_bucket_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TablePolicyResult {
            name: o.get_field("name"),
            namespace: o.get_field("namespace"),
            resource_policy: o.get_field("resourcePolicy"),
            table_bucket_arn: o.get_field("tableBucketArn"),
        }
    }
}
