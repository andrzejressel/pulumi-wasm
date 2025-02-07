/// Resource for managing an AWS Shield DRT Access Log Bucket Association.
/// Up to 10 log buckets can be associated for DRT Access sharing with the Shield Response Team (SRT).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DrtAccessLogBucketAssociationArgs {
        /// The Amazon S3 bucket that contains the logs that you want to share.
        #[builder(into)]
        pub log_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Role Arn association used for allowing Shield DRT Access.
        #[builder(into)]
        pub role_arn_association_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::shield::DrtAccessLogBucketAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DrtAccessLogBucketAssociationResult {
        /// The Amazon S3 bucket that contains the logs that you want to share.
        pub log_bucket: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Role Arn association used for allowing Shield DRT Access.
        pub role_arn_association_id: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::shield::DrtAccessLogBucketAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DrtAccessLogBucketAssociationArgs,
    ) -> DrtAccessLogBucketAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let log_bucket_binding = args.log_bucket.get_output(context).get_inner();
        let role_arn_association_id_binding = args
            .role_arn_association_id
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DrtAccessLogBucketAssociationResult {
            log_bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logBucket"),
            ),
            role_arn_association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArnAssociationId"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
