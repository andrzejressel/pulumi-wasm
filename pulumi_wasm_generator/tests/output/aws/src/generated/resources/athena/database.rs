/// Provides an Athena database.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// That an Amazon S3 canned ACL should be set to control ownership of stored query results. See ACL Configuration below.
        #[builder(into, default)]
        pub acl_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::athena::DatabaseAclConfiguration>,
        >,
        /// Name of S3 bucket to save the results of the query execution.
        #[builder(into, default)]
        pub bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the database.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Encryption key block AWS Athena uses to decrypt the data in S3, such as an AWS Key Management Service (AWS KMS) key. See Encryption Configuration below.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::athena::DatabaseEncryptionConfiguration>,
        >,
        /// AWS account ID that you expect to be the owner of the Amazon S3 bucket.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean that indicates all tables should be deleted from the database so that the database can be destroyed without error. The tables are *not* recoverable.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the database to create.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of custom metadata properties for the database definition.
        #[builder(into, default)]
        pub properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// That an Amazon S3 canned ACL should be set to control ownership of stored query results. See ACL Configuration below.
        pub acl_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::athena::DatabaseAclConfiguration>,
        >,
        /// Name of S3 bucket to save the results of the query execution.
        pub bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the database.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Encryption key block AWS Athena uses to decrypt the data in S3, such as an AWS Key Management Service (AWS KMS) key. See Encryption Configuration below.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::athena::DatabaseEncryptionConfiguration>,
        >,
        /// AWS account ID that you expect to be the owner of the Amazon S3 bucket.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean that indicates all tables should be deleted from the database so that the database can be destroyed without error. The tables are *not* recoverable.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the database to create.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of custom metadata properties for the database definition.
        pub properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatabaseArgs) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let acl_configuration_binding = args.acl_configuration.get_inner();
        let bucket_binding = args.bucket.get_inner();
        let comment_binding = args.comment.get_inner();
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let name_binding = args.name.get_inner();
        let properties_binding = args.properties.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:athena/database:Database".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aclConfiguration".into(),
                    value: &acl_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aclConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "expectedBucketOwner".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseResult {
            acl_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aclConfiguration").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
        }
    }
}