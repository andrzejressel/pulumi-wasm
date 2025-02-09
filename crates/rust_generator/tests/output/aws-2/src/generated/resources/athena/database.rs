/// Provides an Athena database.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .bucket("${example.id}")
///             .name("database_name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Athena Databases using their name. For example:
///
/// ```sh
/// $ pulumi import aws:athena/database:Database example example
/// ```
/// Certain resource arguments, like `encryption_configuration` and `bucket`, do not have an API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// That an Amazon S3 canned ACL should be set to control ownership of stored query results. See ACL Configuration below.
        #[builder(into, default)]
        pub acl_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::athena::DatabaseAclConfiguration>,
        >,
        /// Name of S3 bucket to save the results of the query execution.
        #[builder(into, default)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the database.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Encryption key block AWS Athena uses to decrypt the data in S3, such as an AWS Key Management Service (AWS KMS) key. See Encryption Configuration below.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::athena::DatabaseEncryptionConfiguration>,
        >,
        /// AWS account ID that you expect to be the owner of the Amazon S3 bucket.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean that indicates all tables should be deleted from the database so that the database can be destroyed without error. The tables are *not* recoverable.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the database to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of custom metadata properties for the database definition.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// That an Amazon S3 canned ACL should be set to control ownership of stored query results. See ACL Configuration below.
        pub acl_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::athena::DatabaseAclConfiguration>,
        >,
        /// Name of S3 bucket to save the results of the query execution.
        pub bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the database.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Encryption key block AWS Athena uses to decrypt the data in S3, such as an AWS Key Management Service (AWS KMS) key. See Encryption Configuration below.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::athena::DatabaseEncryptionConfiguration>,
        >,
        /// AWS account ID that you expect to be the owner of the Amazon S3 bucket.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean that indicates all tables should be deleted from the database so that the database can be destroyed without error. The tables are *not* recoverable.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the database to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of custom metadata properties for the database definition.
        pub properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let acl_configuration_binding = args.acl_configuration.get_output(context);
        let bucket_binding = args.bucket.get_output(context);
        let comment_binding = args.comment.get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let name_binding = args.name.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:athena/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aclConfiguration".into(),
                    value: acl_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: encryption_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBucketOwner".into(),
                    value: expected_bucket_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: properties_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatabaseResult {
            acl_configuration: o.get_field("aclConfiguration"),
            bucket: o.get_field("bucket"),
            comment: o.get_field("comment"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
            force_destroy: o.get_field("forceDestroy"),
            name: o.get_field("name"),
            properties: o.get_field("properties"),
        }
    }
}
