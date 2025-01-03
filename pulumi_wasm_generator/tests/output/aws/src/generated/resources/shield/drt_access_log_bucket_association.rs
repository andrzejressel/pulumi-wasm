/// Resource for managing an AWS Shield DRT Access Log Bucket Association.
/// Up to 10 log buckets can be associated for DRT Access sharing with the Shield Response Team (SRT).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = drt_access_role_arn_association::create(
///         "test",
///         DrtAccessRoleArnAssociationArgs::builder()
///             .role_arn(
///                 "arn:aws:iam:${current.name}:${currentAwsCallerIdentity.accountId}:${shieldDrtAccessRoleName}",
///             )
///             .build_struct(),
///     );
///     let testDrtAccessLogBucketAssociation = drt_access_log_bucket_association::create(
///         "testDrtAccessLogBucketAssociation",
///         DrtAccessLogBucketAssociationArgs::builder()
///             .log_bucket("${shieldDrtAccessLogBucket}")
///             .role_arn_association_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield DRT access log bucket associations using the `log_bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation example example-bucket
/// ```
pub mod drt_access_log_bucket_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DrtAccessLogBucketAssociationArgs {
        /// The Amazon S3 bucket that contains the logs that you want to share.
        #[builder(into)]
        pub log_bucket: pulumi_wasm_rust::Output<String>,
        /// The ID of the Role Arn association used for allowing Shield DRT Access.
        #[builder(into)]
        pub role_arn_association_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::shield::DrtAccessLogBucketAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DrtAccessLogBucketAssociationResult {
        /// The Amazon S3 bucket that contains the logs that you want to share.
        pub log_bucket: pulumi_wasm_rust::Output<String>,
        /// The ID of the Role Arn association used for allowing Shield DRT Access.
        pub role_arn_association_id: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::shield::DrtAccessLogBucketAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DrtAccessLogBucketAssociationArgs,
    ) -> DrtAccessLogBucketAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_bucket_binding = args.log_bucket.get_inner();
        let role_arn_association_id_binding = args.role_arn_association_id.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logBucket".into(),
                    value: &log_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "roleArnAssociationId".into(),
                    value: &role_arn_association_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "logBucket".into(),
                },
                register_interface::ResultField {
                    name: "roleArnAssociationId".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DrtAccessLogBucketAssociationResult {
            log_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logBucket").unwrap(),
            ),
            role_arn_association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArnAssociationId").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
