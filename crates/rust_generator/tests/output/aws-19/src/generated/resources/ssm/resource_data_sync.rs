/// Provides a SSM resource data sync.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   hogeBucketV2:
///     type: aws:s3:BucketV2
///     name: hoge
///     properties:
///       bucket: tf-test-bucket-1234
///   hogeBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: hoge
///     properties:
///       bucket: ${hogeBucketV2.id}
///       policy: ${hoge.json}
///   foo:
///     type: aws:ssm:ResourceDataSync
///     properties:
///       name: foo
///       s3Destination:
///         bucketName: ${hogeBucketV2.bucket}
///         region: ${hogeBucketV2.region}
/// variables:
///   hoge:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: SSMBucketPermissionsCheck
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - ssm.amazonaws.com
///             actions:
///               - s3:GetBucketAcl
///             resources:
///               - arn:aws:s3:::tf-test-bucket-1234
///           - sid: SSMBucketDelivery
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - ssm.amazonaws.com
///             actions:
///               - s3:PutObject
///             resources:
///               - arn:aws:s3:::tf-test-bucket-1234/*
///             conditions:
///               - test: StringEquals
///                 variable: s3:x-amz-acl
///                 values:
///                   - bucket-owner-full-control
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceDataSyncArgs {
        /// Name for the configuration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Amazon S3 configuration details for the sync.
        #[builder(into)]
        pub s3_destination: pulumi_wasm_rust::InputOrOutput<
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourceDataSyncArgs,
    ) -> ResourceDataSyncResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let s3_destination_binding = args.s3_destination.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/resourceDataSync:ResourceDataSync".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceDataSyncResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            s3_destination: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3Destination"),
            ),
        }
    }
}
