/// Manages an AWS Storage Gateway Tape Pool.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tape_pool::create(
///         "example",
///         TapePoolArgs::builder()
///             .pool_name("example")
///             .storage_class("GLACIER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_storagegateway_tape_pool` using the volume Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:storagegateway/tapePool:TapePool example arn:aws:storagegateway:us-east-1:123456789012:tapepool/pool-12345678
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod tape_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TapePoolArgs {
        /// The name of the new custom tape pool.
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days). Default value is 0.
        #[builder(into, default)]
        pub retention_lock_time_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account. Possible values are `COMPLIANCE`, `GOVERNANCE`, and `NONE`. Default value is `NONE`.
        #[builder(into, default)]
        pub retention_lock_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class that corresponds to the pool. Possible values are `DEEP_ARCHIVE` or `GLACIER`.
        #[builder(into)]
        pub storage_class: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TapePoolResult {
        /// Volume Amazon Resource Name (ARN), e.g., `aws_storagegateway_tape_pool.example arn:aws:storagegateway:us-east-1:123456789012:tapepool/pool-12345678`.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the new custom tape pool.
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        /// Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days). Default value is 0.
        pub retention_lock_time_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account. Possible values are `COMPLIANCE`, `GOVERNANCE`, and `NONE`. Default value is `NONE`.
        pub retention_lock_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class that corresponds to the pool. Possible values are `DEEP_ARCHIVE` or `GLACIER`.
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TapePoolArgs,
    ) -> TapePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let pool_name_binding = args.pool_name.get_output(context).get_inner();
        let retention_lock_time_in_days_binding = args
            .retention_lock_time_in_days
            .get_output(context)
            .get_inner();
        let retention_lock_type_binding = args
            .retention_lock_type
            .get_output(context)
            .get_inner();
        let storage_class_binding = args.storage_class.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/tapePool:TapePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionLockTimeInDays".into(),
                    value: &retention_lock_time_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "retentionLockType".into(),
                    value: &retention_lock_type_binding,
                },
                register_interface::ObjectField {
                    name: "storageClass".into(),
                    value: &storage_class_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TapePoolResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            pool_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("poolName"),
            ),
            retention_lock_time_in_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionLockTimeInDays"),
            ),
            retention_lock_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionLockType"),
            ),
            storage_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageClass"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
