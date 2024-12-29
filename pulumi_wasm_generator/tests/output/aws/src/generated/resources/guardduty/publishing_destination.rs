/// Provides a resource to manage a GuardDuty PublishingDestination. Requires an existing GuardDuty Detector.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bucketPol = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["s3:PutObject",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["guardduty.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).resources(vec!["${gdBucket.arn}/*",])
///                     .sid("Allow PutObject").build_struct(),
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:GetBucketLocation",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["guardduty.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).resources(vec!["${gdBucket.arn}",])
///                     .sid("Allow GetBucketLocation").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let currentGetRegion = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let kmsPol = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["kms:GenerateDataKey",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["guardduty.amazonaws.com",]). type ("Service")
///                     .build_struct(),])
///                     .resources(vec!["arn:aws:kms:${currentGetRegion.name}:${current.accountId}:key/*",])
///                     .sid("Allow GuardDuty to encrypt findings").build_struct(),
///                     GetPolicyDocumentStatement::builder().actions(vec!["kms:*",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["arn:aws:iam::${current.accountId}:root",]). type
///                     ("AWS").build_struct(),])
///                     .resources(vec!["arn:aws:kms:${currentGetRegion.name}:${current.accountId}:key/*",])
///                     .sid("Allow all users to modify/delete key (test only)")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let gdBucket = bucket_v_2::create(
///         "gdBucket",
///         BucketV2Args::builder().bucket("example").force_destroy(true).build_struct(),
///     );
///     let gdBucketAcl = bucket_acl_v_2::create(
///         "gdBucketAcl",
///         BucketAclV2Args::builder().acl("private").bucket("${gdBucket.id}").build_struct(),
///     );
///     let gdBucketPolicy = bucket_policy::create(
///         "gdBucketPolicy",
///         BucketPolicyArgs::builder()
///             .bucket("${gdBucket.id}")
///             .policy("${bucketPol.json}")
///             .build_struct(),
///     );
///     let gdKey = key::create(
///         "gdKey",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Temporary key for AccTest of TF")
///             .policy("${kmsPol.json}")
///             .build_struct(),
///     );
///     let test = publishing_destination::create(
///         "test",
///         PublishingDestinationArgs::builder()
///             .destination_arn("${gdBucket.arn}")
///             .detector_id("${testGd.id}")
///             .kms_key_arn("${gdKey.arn}")
///             .build_struct(),
///     );
///     let testGd = detector::create(
///         "testGd",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** Please do not use this simple example for Bucket-Policy and KMS Key Policy in a production environment. It is much too open for such a use-case. Refer to the AWS documentation here: https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_exportfindings.html
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty PublishingDestination using the master GuardDuty detector ID and PublishingDestinationID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/publishingDestination:PublishingDestination test a4b86f26fa42e7e7cf0d1c333ea77777:a4b86f27a0e464e4a7e0516d242f1234
/// ```
pub mod publishing_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublishingDestinationArgs {
        /// The bucket arn and prefix under which the findings get exported. Bucket-ARN is required, the prefix is optional and will be `AWSLogs/[Account-ID]/GuardDuty/[Region]/` if not provided
        #[builder(into)]
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// Currently there is only "S3" available as destination type which is also the default value
        ///
        /// > **Note:** In case of missing permissions (S3 Bucket Policy _or_ KMS Key permissions) the resource will fail to create. If the permissions are changed after resource creation, this can be asked from the AWS API via the "DescribePublishingDestination" call (https://docs.aws.amazon.com/cli/latest/reference/guardduty/describe-publishing-destination.html).
        #[builder(into, default)]
        pub destination_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The detector ID of the GuardDuty.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the KMS key used to encrypt GuardDuty findings. GuardDuty enforces this to be encrypted.
        #[builder(into)]
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PublishingDestinationResult {
        /// The bucket arn and prefix under which the findings get exported. Bucket-ARN is required, the prefix is optional and will be `AWSLogs/[Account-ID]/GuardDuty/[Region]/` if not provided
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// Currently there is only "S3" available as destination type which is also the default value
        ///
        /// > **Note:** In case of missing permissions (S3 Bucket Policy _or_ KMS Key permissions) the resource will fail to create. If the permissions are changed after resource creation, this can be asked from the AWS API via the "DescribePublishingDestination" call (https://docs.aws.amazon.com/cli/latest/reference/guardduty/describe-publishing-destination.html).
        pub destination_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The detector ID of the GuardDuty.
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the KMS key used to encrypt GuardDuty findings. GuardDuty enforces this to be encrypted.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PublishingDestinationArgs,
    ) -> PublishingDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_arn_binding = args.destination_arn.get_inner();
        let destination_type_binding = args.destination_type.get_inner();
        let detector_id_binding = args.detector_id.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/publishingDestination:PublishingDestination".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "destinationType".into(),
                    value: &destination_type_binding,
                },
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationType".into(),
                },
                register_interface::ResultField {
                    name: "detectorId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PublishingDestinationResult {
            destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationArn").unwrap(),
            ),
            destination_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationType").unwrap(),
            ),
            detector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorId").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
        }
    }
}
