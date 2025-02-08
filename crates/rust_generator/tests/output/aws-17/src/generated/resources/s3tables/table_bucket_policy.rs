/// Resource for managing an Amazon S3 Tables Table Bucket Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleTableBucketPolicy:
///     type: aws:s3tables:TableBucketPolicy
///     name: example
///     properties:
///       resourcePolicy: ${example.json}
///       tableBucketArn: ${exampleAwsS3tablesTableBucket.arn}
///   test:
///     type: aws:s3tables:TableBucket
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
/// Using `pulumi import`, import S3 Tables Table Bucket Policy using the `table_bucket_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/tableBucketPolicy:TableBucketPolicy example 'arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket;example-namespace'
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod table_bucket_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableBucketPolicyArgs {
        /// Amazon Web Services resource-based policy document in JSON format.
        #[builder(into)]
        pub resource_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN referencing the Table Bucket that owns this policy.
        #[builder(into)]
        pub table_bucket_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TableBucketPolicyResult {
        /// Amazon Web Services resource-based policy document in JSON format.
        pub resource_policy: pulumi_gestalt_rust::Output<String>,
        /// ARN referencing the Table Bucket that owns this policy.
        pub table_bucket_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TableBucketPolicyArgs,
    ) -> TableBucketPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let resource_policy_binding = args
            .resource_policy
            .get_output(context)
            .get_inner();
        let table_bucket_arn_binding = args
            .table_bucket_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3tables/tableBucketPolicy:TableBucketPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourcePolicy".into(),
                    value: &resource_policy_binding,
                },
                register_interface::ObjectField {
                    name: "tableBucketArn".into(),
                    value: &table_bucket_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TableBucketPolicyResult {
            resource_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourcePolicy"),
            ),
            table_bucket_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableBucketArn"),
            ),
        }
    }
}
