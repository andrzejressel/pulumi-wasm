/// Provides a SSM resource data sync.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hoge = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:GetBucketAcl",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["ssm.amazonaws.com",]). type ("Service")
///                     .build_struct(),])
///                     .resources(vec!["arn:aws:s3:::tf-test-bucket-1234",])
///                     .sid("SSMBucketPermissionsCheck").build_struct(),
///                     GetPolicyDocumentStatement::builder().actions(vec!["s3:PutObject",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals").values(vec!["bucket-owner-full-control",])
///                     .variable("s3:x-amz-acl").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["ssm.amazonaws.com",]). type ("Service")
///                     .build_struct(),])
///                     .resources(vec!["arn:aws:s3:::tf-test-bucket-1234/*",])
///                     .sid("SSMBucketDelivery").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let foo = resource_data_sync::create(
///         "foo",
///         ResourceDataSyncArgs::builder()
///             .name("foo")
///             .s_3_destination(
///                 ResourceDataSyncS3Destination::builder()
///                     .bucketName("${hogeBucketV2.bucket}")
///                     .region("${hogeBucketV2.region}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let hogeBucketPolicy = bucket_policy::create(
///         "hogeBucketPolicy",
///         BucketPolicyArgs::builder()
///             .bucket("${hogeBucketV2.id}")
///             .policy("${hoge.json}")
///             .build_struct(),
///     );
///     let hogeBucketV2 = bucket_v_2::create(
///         "hogeBucketV2",
///         BucketV2Args::builder().bucket("tf-test-bucket-1234").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM resource data sync using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/resourceDataSync:ResourceDataSync example example-name
/// ```
pub mod resource_data_sync {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceDataSyncArgs {
        /// Name for the configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon S3 configuration details for the sync.
        #[builder(into)]
        pub s3_destination: pulumi_wasm_rust::Output<
            super::super::types::ssm::ResourceDataSyncS3Destination,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceDataSyncResult {
        /// Name for the configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Amazon S3 configuration details for the sync.
        pub s3_destination: pulumi_wasm_rust::Output<
            super::super::types::ssm::ResourceDataSyncS3Destination,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceDataSyncArgs) -> ResourceDataSyncResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let s3_destination_binding = args.s3_destination.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/resourceDataSync:ResourceDataSync".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "s3Destination".into(),
                    value: &s3_destination_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "s3Destination".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceDataSyncResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            s3_destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Destination").unwrap(),
            ),
        }
    }
}
