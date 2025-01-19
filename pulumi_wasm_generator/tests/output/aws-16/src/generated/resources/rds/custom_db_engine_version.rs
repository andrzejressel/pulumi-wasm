/// Provides an custom engine version (CEV) resource for Amazon RDS Custom. For additional information, see [Working with CEVs for RDS Custom for Oracle](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev.html) and [Working with CEVs for RDS Custom for SQL Server](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev-sqlserver.html) in the the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Welcome.html).
///
/// ## Example Usage
///
/// ### RDS Custom for Oracle Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: KMS symmetric key for RDS Custom for Oracle
///   exampleCustomDbEngineVersion:
///     type: aws:rds:CustomDbEngineVersion
///     name: example
///     properties:
///       databaseInstallationFilesS3BucketName: DOC-EXAMPLE-BUCKET
///       databaseInstallationFilesS3Prefix: 1915_GI/
///       engine: custom-oracle-ee-cdb
///       engineVersion: 19.cdb_cev1
///       kmsKeyId: ${example.arn}
///       manifest: |2
///           {
///         	"databaseInstallationFileNames":["V982063-01.zip"]
///           }
///       tags:
///         Name: example
///         Key: value
/// ```
///
/// ### RDS Custom for Oracle External Manifest Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: KMS symmetric key for RDS Custom for Oracle
///   exampleCustomDbEngineVersion:
///     type: aws:rds:CustomDbEngineVersion
///     name: example
///     properties:
///       databaseInstallationFilesS3BucketName: DOC-EXAMPLE-BUCKET
///       databaseInstallationFilesS3Prefix: 1915_GI/
///       engine: custom-oracle-ee-cdb
///       engineVersion: 19.cdb_cev1
///       kmsKeyId: ${example.arn}
///       filename: manifest_1915_GI.json
///       manifestHash:
///         fn::invoke:
///           function: std:filebase64sha256
///           arguments:
///             input: ${json}
///           return: result
///       tags:
///         Name: example
///         Key: value
/// ```
///
/// ### RDS Custom for SQL Server Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = custom_db_engine_version::create(
///         "test",
///         CustomDbEngineVersionArgs::builder()
///             .engine("custom-sqlserver-se")
///             .engine_version("15.00.4249.2.cev-1")
///             .source_image_id("ami-0aa12345678a12ab1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RDS Custom for SQL Server Usage with AMI from another region
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami_copy::create(
///         "example",
///         AmiCopyArgs::builder()
///             .description("A copy of ami-xxxxxxxx")
///             .name("sqlserver-se-2019-15.00.4249.2")
///             .source_ami_id("ami-xxxxxxxx")
///             .source_ami_region("us-east-1")
///             .build_struct(),
///     );
///     let test = custom_db_engine_version::create(
///         "test",
///         CustomDbEngineVersionArgs::builder()
///             .engine("custom-sqlserver-se")
///             .engine_version("15.00.4249.2.cev-1")
///             .source_image_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import custom engine versions for Amazon RDS custom using the `engine` and `engine_version` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:rds/customDbEngineVersion:CustomDbEngineVersion example custom-oracle-ee-cdb:19.cdb_cev1
/// ```
pub mod custom_db_engine_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDbEngineVersionArgs {
        /// The name of the Amazon S3 bucket that contains the database installation files.
        #[builder(into, default)]
        pub database_installation_files_s3_bucket_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The prefix for the Amazon S3 bucket that contains the database installation files.
        #[builder(into, default)]
        pub database_installation_files_s3_prefix: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The description of the CEV.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the database engine. Valid values are `custom-oracle*`, `custom-sqlserver*`.
        #[builder(into)]
        pub engine: pulumi_wasm_rust::Output<String>,
        /// The version of the database engine.
        #[builder(into)]
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The name of the manifest file within the local filesystem. Conflicts with `manifest`.
        #[builder(into, default)]
        pub filename: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the AWS KMS key that is used to encrypt the database installation files. Required for RDS Custom for Oracle.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The manifest file, in JSON format, that contains the list of database installation files. Conflicts with `filename`.
        #[builder(into, default)]
        pub manifest: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the manifest source specified with `filename`. The usual way to set this is filebase64sha256("manifest.json") where "manifest.json" is the local filename of the manifest source.
        #[builder(into, default)]
        pub manifest_hash: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AMI to create the CEV from. Required for RDS Custom for SQL Server. For RDS Custom for Oracle, you can specify an AMI ID that was used in a different Oracle CEV.
        #[builder(into, default)]
        pub source_image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the CEV. Valid values are `available`, `inactive`, `inactive-except-restore`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomDbEngineVersionResult {
        /// The Amazon Resource Name (ARN) for the custom engine version.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time that the CEV was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The name of the Amazon S3 bucket that contains the database installation files.
        pub database_installation_files_s3_bucket_name: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The prefix for the Amazon S3 bucket that contains the database installation files.
        pub database_installation_files_s3_prefix: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The name of the DB parameter group family for the CEV.
        pub db_parameter_group_family: pulumi_wasm_rust::Output<String>,
        /// The description of the CEV.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the database engine. Valid values are `custom-oracle*`, `custom-sqlserver*`.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// The version of the database engine.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The name of the manifest file within the local filesystem. Conflicts with `manifest`.
        pub filename: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AMI that was created with the CEV.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the AWS KMS key that is used to encrypt the database installation files. Required for RDS Custom for Oracle.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The major version of the database engine.
        pub major_engine_version: pulumi_wasm_rust::Output<String>,
        /// The manifest file, in JSON format, that contains the list of database installation files. Conflicts with `filename`.
        pub manifest: pulumi_wasm_rust::Output<Option<String>>,
        /// The returned manifest file, in JSON format, service generated and often different from input `manifest`.
        pub manifest_computed: pulumi_wasm_rust::Output<String>,
        /// Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the manifest source specified with `filename`. The usual way to set this is filebase64sha256("manifest.json") where "manifest.json" is the local filename of the manifest source.
        pub manifest_hash: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AMI to create the CEV from. Required for RDS Custom for SQL Server. For RDS Custom for Oracle, you can specify an AMI ID that was used in a different Oracle CEV.
        pub source_image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the CEV. Valid values are `available`, `inactive`, `inactive-except-restore`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        name: &str,
        args: CustomDbEngineVersionArgs,
    ) -> CustomDbEngineVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_installation_files_s3_bucket_name_binding = args
            .database_installation_files_s3_bucket_name
            .get_inner();
        let database_installation_files_s3_prefix_binding = args
            .database_installation_files_s3_prefix
            .get_inner();
        let description_binding = args.description.get_inner();
        let engine_binding = args.engine.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let filename_binding = args.filename.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let manifest_binding = args.manifest.get_inner();
        let manifest_hash_binding = args.manifest_hash.get_inner();
        let source_image_id_binding = args.source_image_id.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/customDbEngineVersion:CustomDbEngineVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseInstallationFilesS3BucketName".into(),
                    value: &database_installation_files_s3_bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "databaseInstallationFilesS3Prefix".into(),
                    value: &database_installation_files_s3_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "filename".into(),
                    value: &filename_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "manifest".into(),
                    value: &manifest_binding,
                },
                register_interface::ObjectField {
                    name: "manifestHash".into(),
                    value: &manifest_hash_binding,
                },
                register_interface::ObjectField {
                    name: "sourceImageId".into(),
                    value: &source_image_id_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "databaseInstallationFilesS3BucketName".into(),
                },
                register_interface::ResultField {
                    name: "databaseInstallationFilesS3Prefix".into(),
                },
                register_interface::ResultField {
                    name: "dbParameterGroupFamily".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "filename".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "majorEngineVersion".into(),
                },
                register_interface::ResultField {
                    name: "manifest".into(),
                },
                register_interface::ResultField {
                    name: "manifestComputed".into(),
                },
                register_interface::ResultField {
                    name: "manifestHash".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomDbEngineVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            database_installation_files_s3_bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseInstallationFilesS3BucketName").unwrap(),
            ),
            database_installation_files_s3_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseInstallationFilesS3Prefix").unwrap(),
            ),
            db_parameter_group_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbParameterGroupFamily").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            filename: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filename").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            major_engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("majorEngineVersion").unwrap(),
            ),
            manifest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manifest").unwrap(),
            ),
            manifest_computed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manifestComputed").unwrap(),
            ),
            manifest_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manifestHash").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
