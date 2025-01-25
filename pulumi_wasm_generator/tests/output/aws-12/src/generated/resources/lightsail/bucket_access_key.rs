/// Provides a lightsail bucket access key. This is a set of credentials that allow API requests to be made to the lightsail bucket.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Bucket
///     properties:
///       name: mytestbucket
///       bundleId: small_1_0
///   testLightsailBucketAccessKeyAccessKey:
///     type: aws:lightsailBucketAccessKeyAccessKey
///     name: test
///     properties:
///       bucketName: ${testAwsLightsailBucketAccessKey.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_bucket_access_key` using the `id` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/bucketAccessKey:BucketAccessKey test example-bucket,AKIAIOSFODNN7EXAMPLE
/// ```
pub mod bucket_access_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketAccessKeyArgs {
        /// The name of the bucket that the new access key will belong to, and grant access to.
        #[builder(into)]
        pub bucket_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketAccessKeyResult {
        /// The ID of the access key.
        pub access_key_id: pulumi_wasm_rust::Output<String>,
        /// The name of the bucket that the new access key will belong to, and grant access to.
        pub bucket_name: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the access key was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The secret access key used to sign requests. This attribute is not available for imported resources. Note that this will be written to the state file.
        pub secret_access_key: pulumi_wasm_rust::Output<String>,
        /// The status of the access key.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BucketAccessKeyArgs,
    ) -> BucketAccessKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_name_binding = args.bucket_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/bucketAccessKey:BucketAccessKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessKeyId".into(),
                },
                register_interface::ResultField {
                    name: "bucketName".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "secretAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketAccessKeyResult {
            access_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessKeyId").unwrap(),
            ),
            bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketName").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            secret_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretAccessKey").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
