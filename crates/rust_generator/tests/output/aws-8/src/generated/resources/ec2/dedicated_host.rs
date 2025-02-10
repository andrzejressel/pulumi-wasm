/// Provides an EC2 Host resource. This allows Dedicated Hosts to be allocated, modified, and released.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = dedicated_host::create(
///         "test",
///         DedicatedHostArgs::builder()
///             .auto_placement("on")
///             .availability_zone("us-west-2a")
///             .host_recovery("on")
///             .instance_type("c5.18xlarge")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import hosts using the host `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/dedicatedHost:DedicatedHost example h-0385a99d0e4b20cbb
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dedicated_host {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedHostArgs {
        /// The ID of the Outpost hardware asset on which to allocate the Dedicated Hosts. This parameter is supported only if you specify OutpostArn. If you are allocating the Dedicated Hosts in a Region, omit this parameter.
        #[builder(into, default)]
        pub asset_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. Valid values: `on`, `off`. Default: `on`.
        #[builder(into, default)]
        pub auto_placement: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Availability Zone in which to allocate the Dedicated Host.
        #[builder(into)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether to enable or disable host recovery for the Dedicated Host. Valid values: `on`, `off`. Default: `off`.
        #[builder(into, default)]
        pub host_recovery: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family. Exactly one of `instance_family` or `instance_type` must be specified.
        #[builder(into, default)]
        pub instance_family: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only. Exactly one of `instance_family` or `instance_type` must be specified.
        #[builder(into, default)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the AWS Outpost on which to allocate the Dedicated Host.
        #[builder(into, default)]
        pub outpost_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DedicatedHostResult {
        /// The ARN of the Dedicated Host.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Outpost hardware asset on which to allocate the Dedicated Hosts. This parameter is supported only if you specify OutpostArn. If you are allocating the Dedicated Hosts in a Region, omit this parameter.
        pub asset_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. Valid values: `on`, `off`. Default: `on`.
        pub auto_placement: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Availability Zone in which to allocate the Dedicated Host.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether to enable or disable host recovery for the Dedicated Host. Valid values: `on`, `off`. Default: `off`.
        pub host_recovery: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family. Exactly one of `instance_family` or `instance_type` must be specified.
        pub instance_family: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only. Exactly one of `instance_family` or `instance_type` must be specified.
        pub instance_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the AWS Outpost on which to allocate the Dedicated Host.
        pub outpost_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the Dedicated Host.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedHostArgs,
    ) -> DedicatedHostResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let asset_id_binding = args.asset_id.get_output(context);
        let auto_placement_binding = args.auto_placement.get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let host_recovery_binding = args.host_recovery.get_output(context);
        let instance_family_binding = args.instance_family.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let outpost_arn_binding = args.outpost_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/dedicatedHost:DedicatedHost".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assetId".into(),
                    value: asset_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoPlacement".into(),
                    value: auto_placement_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostRecovery".into(),
                    value: host_recovery_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceFamily".into(),
                    value: instance_family_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outpostArn".into(),
                    value: outpost_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DedicatedHostResult {
            arn: o.get_field("arn"),
            asset_id: o.get_field("assetId"),
            auto_placement: o.get_field("autoPlacement"),
            availability_zone: o.get_field("availabilityZone"),
            host_recovery: o.get_field("hostRecovery"),
            instance_family: o.get_field("instanceFamily"),
            instance_type: o.get_field("instanceType"),
            outpost_arn: o.get_field("outpostArn"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
