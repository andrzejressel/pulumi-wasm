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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableBucketPolicyArgs,
    ) -> TableBucketPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_policy_binding = args.resource_policy.get_output(context);
        let table_bucket_arn_binding = args.table_bucket_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3tables/tableBucketPolicy:TableBucketPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
        TableBucketPolicyResult {
            resource_policy: o.get_field("resourcePolicy"),
            table_bucket_arn: o.get_field("tableBucketArn"),
        }
    }
}
