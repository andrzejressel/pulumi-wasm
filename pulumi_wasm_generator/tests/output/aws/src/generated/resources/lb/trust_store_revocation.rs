/// Provides a ELBv2 Trust Store Revocation for use with Application Load Balancer Listener resources.
///
/// ## Example Usage
///
/// ### Trust Store With Revocations
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = trust_store::create(
///         "test",
///         TrustStoreArgs::builder()
///             .ca_certificates_bundle_s_3_bucket("...")
///             .ca_certificates_bundle_s_3_key("...")
///             .name("tf-example-lb-ts")
///             .build_struct(),
///     );
///     let testTrustStoreRevocation = trust_store_revocation::create(
///         "testTrustStoreRevocation",
///         TrustStoreRevocationArgs::builder()
///             .revocations_s_3_bucket("...")
///             .revocations_s_3_key("...")
///             .trust_store_arn("${test.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Trust Store Revocations using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:lb/trustStoreRevocation:TrustStoreRevocation example arn:aws:elasticloadbalancing:us-west-2:187416307283:truststore/my-trust-store/20cfe21448b66314,6
/// ```
pub mod trust_store_revocation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustStoreRevocationArgs {
        /// S3 Bucket name holding the client certificate CA bundle.
        #[builder(into)]
        pub revocations_s3_bucket: pulumi_wasm_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        #[builder(into)]
        pub revocations_s3_key: pulumi_wasm_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        #[builder(into, default)]
        pub revocations_s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Trust Store ARN.
        #[builder(into)]
        pub trust_store_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TrustStoreRevocationResult {
        /// AWS assigned RevocationId, (number).
        pub revocation_id: pulumi_wasm_rust::Output<i32>,
        /// S3 Bucket name holding the client certificate CA bundle.
        pub revocations_s3_bucket: pulumi_wasm_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        pub revocations_s3_key: pulumi_wasm_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        pub revocations_s3_object_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Trust Store ARN.
        pub trust_store_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TrustStoreRevocationArgs,
    ) -> TrustStoreRevocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let revocations_s3_bucket_binding = args.revocations_s3_bucket.get_inner();
        let revocations_s3_key_binding = args.revocations_s3_key.get_inner();
        let revocations_s3_object_version_binding = args
            .revocations_s3_object_version
            .get_inner();
        let trust_store_arn_binding = args.trust_store_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lb/trustStoreRevocation:TrustStoreRevocation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "revocationsS3Bucket".into(),
                    value: &revocations_s3_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "revocationsS3Key".into(),
                    value: &revocations_s3_key_binding,
                },
                register_interface::ObjectField {
                    name: "revocationsS3ObjectVersion".into(),
                    value: &revocations_s3_object_version_binding,
                },
                register_interface::ObjectField {
                    name: "trustStoreArn".into(),
                    value: &trust_store_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "revocationId".into(),
                },
                register_interface::ResultField {
                    name: "revocationsS3Bucket".into(),
                },
                register_interface::ResultField {
                    name: "revocationsS3Key".into(),
                },
                register_interface::ResultField {
                    name: "revocationsS3ObjectVersion".into(),
                },
                register_interface::ResultField {
                    name: "trustStoreArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrustStoreRevocationResult {
            revocation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revocationId").unwrap(),
            ),
            revocations_s3_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revocationsS3Bucket").unwrap(),
            ),
            revocations_s3_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revocationsS3Key").unwrap(),
            ),
            revocations_s3_object_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revocationsS3ObjectVersion").unwrap(),
            ),
            trust_store_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustStoreArn").unwrap(),
            ),
        }
    }
}