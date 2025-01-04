/// Manages an AWS Storage Gateway Tape Pool.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod tape_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TapePoolArgs {
        /// The name of the new custom tape pool.
        #[builder(into)]
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days). Default value is 0.
        #[builder(into, default)]
        pub retention_lock_time_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account. Possible values are `COMPLIANCE`, `GOVERNANCE`, and `NONE`. Default value is `NONE`.
        #[builder(into, default)]
        pub retention_lock_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class that corresponds to the pool. Possible values are `DEEP_ARCHIVE` or `GLACIER`.
        #[builder(into)]
        pub storage_class: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TapePoolResult {
        /// Volume Amazon Resource Name (ARN), e.g., `aws_storagegateway_tape_pool.example arn:aws:storagegateway:us-east-1:123456789012:tapepool/pool-12345678`.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the new custom tape pool.
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days). Default value is 0.
        pub retention_lock_time_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account. Possible values are `COMPLIANCE`, `GOVERNANCE`, and `NONE`. Default value is `NONE`.
        pub retention_lock_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class that corresponds to the pool. Possible values are `DEEP_ARCHIVE` or `GLACIER`.
        pub storage_class: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: TapePoolArgs) -> TapePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let pool_name_binding = args.pool_name.get_inner();
        let retention_lock_time_in_days_binding = args
            .retention_lock_time_in_days
            .get_inner();
        let retention_lock_type_binding = args.retention_lock_type.get_inner();
        let storage_class_binding = args.storage_class.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:storagegateway/tapePool:TapePool".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "poolName".into(),
                },
                register_interface::ResultField {
                    name: "retentionLockTimeInDays".into(),
                },
                register_interface::ResultField {
                    name: "retentionLockType".into(),
                },
                register_interface::ResultField {
                    name: "storageClass".into(),
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
        TapePoolResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolName").unwrap(),
            ),
            retention_lock_time_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionLockTimeInDays").unwrap(),
            ),
            retention_lock_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionLockType").unwrap(),
            ),
            storage_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageClass").unwrap(),
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
