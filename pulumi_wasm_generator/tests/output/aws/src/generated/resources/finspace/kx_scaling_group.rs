/// Resource for managing an AWS FinSpace Kx Scaling Group.
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
///     let example = kx_scaling_group::create(
///         "example",
///         KxScalingGroupArgs::builder()
///             .availability_zone_id("use1-az2")
///             .environment_id("${exampleAwsFinspaceKxEnvironment.id}")
///             .host_type("kx.sg.4xlarge")
///             .name("my-tf-kx-scalinggroup")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Scaling Group using the `id` (environment ID and scaling group name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxScalingGroup:KxScalingGroup example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-scalinggroup
/// ```
pub mod kx_scaling_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxScalingGroupArgs {
        /// The availability zone identifiers for the requested regions.
        #[builder(into)]
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the kdb environment, where you want to create the scaling group.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub host_type: pulumi_wasm_rust::Output<String>,
        /// Unique name for the scaling group that you want to create.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. You can add up to 50 tags to a scaling group.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxScalingGroupResult {
        /// Amazon Resource Name (ARN) identifier of the KX Scaling Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The availability zone identifiers for the requested regions.
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The list of Managed kdb clusters that are currently active in the given scaling group.
        pub clusters: pulumi_wasm_rust::Output<Vec<String>>,
        /// The timestamp at which the scaling group was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub created_timestamp: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the kdb environment, where you want to create the scaling group.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.
        ///
        /// The following arguments are optional:
        pub host_type: pulumi_wasm_rust::Output<String>,
        /// Last timestamp at which the scaling group was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_wasm_rust::Output<String>,
        /// Unique name for the scaling group that you want to create.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The status of scaling group.
        /// * `CREATING` – The scaling group creation is in progress.
        /// * `CREATE_FAILED` – The scaling group creation has failed.
        /// * `ACTIVE` – The scaling group is active.
        /// * `UPDATING` – The scaling group is in the process of being updated.
        /// * `UPDATE_FAILED` – The update action failed.
        /// * `DELETING` – The scaling group is in the process of being deleted.
        /// * `DELETE_FAILED` – The system failed to delete the scaling group.
        /// * `DELETED` – The scaling group is successfully deleted.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The error message when a failed state occurs.
        pub status_reason: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. You can add up to 50 tags to a scaling group.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KxScalingGroupArgs) -> KxScalingGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_id_binding = args.availability_zone_id.get_inner();
        let environment_id_binding = args.environment_id.get_inner();
        let host_type_binding = args.host_type.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxScalingGroup:KxScalingGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZoneId".into(),
                    value: &availability_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostType".into(),
                    value: &host_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "clusters".into(),
                },
                register_interface::ResultField {
                    name: "createdTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "hostType".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusReason".into(),
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
        KxScalingGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneId").unwrap(),
            ),
            clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusters").unwrap(),
            ),
            created_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTimestamp").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            host_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostType").unwrap(),
            ),
            last_modified_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTimestamp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusReason").unwrap(),
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