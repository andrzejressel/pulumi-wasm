/// Resource for managing an AWS FinSpace Kx Scaling Group.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_scaling_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxScalingGroupArgs {
        /// The availability zone identifiers for the requested regions.
        #[builder(into)]
        pub availability_zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for the kdb environment, where you want to create the scaling group.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub host_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique name for the scaling group that you want to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. You can add up to 50 tags to a scaling group.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxScalingGroupResult {
        /// Amazon Resource Name (ARN) identifier of the KX Scaling Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The availability zone identifiers for the requested regions.
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The list of Managed kdb clusters that are currently active in the given scaling group.
        pub clusters: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The timestamp at which the scaling group was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the kdb environment, where you want to create the scaling group.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// The memory and CPU capabilities of the scaling group host on which FinSpace Managed kdb clusters will be placed.
        ///
        /// The following arguments are optional:
        pub host_type: pulumi_gestalt_rust::Output<String>,
        /// Last timestamp at which the scaling group was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the scaling group that you want to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The status of scaling group.
        /// * `CREATING` – The scaling group creation is in progress.
        /// * `CREATE_FAILED` – The scaling group creation has failed.
        /// * `ACTIVE` – The scaling group is active.
        /// * `UPDATING` – The scaling group is in the process of being updated.
        /// * `UPDATE_FAILED` – The update action failed.
        /// * `DELETING` – The scaling group is in the process of being deleted.
        /// * `DELETE_FAILED` – The system failed to delete the scaling group.
        /// * `DELETED` – The scaling group is successfully deleted.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The error message when a failed state occurs.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. You can add up to 50 tags to a scaling group.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: KxScalingGroupArgs,
    ) -> KxScalingGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let availability_zone_id_binding_1 = args
            .availability_zone_id
            .get_output(context);
        let availability_zone_id_binding = availability_zone_id_binding_1.get_inner();
        let environment_id_binding_1 = args.environment_id.get_output(context);
        let environment_id_binding = environment_id_binding_1.get_inner();
        let host_type_binding_1 = args.host_type.get_output(context);
        let host_type_binding = host_type_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxScalingGroup:KxScalingGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        KxScalingGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            clusters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusters"),
            ),
            created_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTimestamp"),
            ),
            environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentId"),
            ),
            host_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostType"),
            ),
            last_modified_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedTimestamp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusReason"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
