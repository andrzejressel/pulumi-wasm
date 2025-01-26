/// Provides an EC2 Host resource. This allows Dedicated Hosts to be allocated, modified, and released.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod dedicated_host {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedHostArgs {
        /// The ID of the Outpost hardware asset on which to allocate the Dedicated Hosts. This parameter is supported only if you specify OutpostArn. If you are allocating the Dedicated Hosts in a Region, omit this parameter.
        #[builder(into, default)]
        pub asset_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. Valid values: `on`, `off`. Default: `on`.
        #[builder(into, default)]
        pub auto_placement: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Availability Zone in which to allocate the Dedicated Host.
        #[builder(into)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<String>,
        /// Indicates whether to enable or disable host recovery for the Dedicated Host. Valid values: `on`, `off`. Default: `off`.
        #[builder(into, default)]
        pub host_recovery: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family. Exactly one of `instance_family` or `instance_type` must be specified.
        #[builder(into, default)]
        pub instance_family: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only. Exactly one of `instance_family` or `instance_type` must be specified.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the AWS Outpost on which to allocate the Dedicated Host.
        #[builder(into, default)]
        pub outpost_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DedicatedHostResult {
        /// The ARN of the Dedicated Host.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the Outpost hardware asset on which to allocate the Dedicated Hosts. This parameter is supported only if you specify OutpostArn. If you are allocating the Dedicated Hosts in a Region, omit this parameter.
        pub asset_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. Valid values: `on`, `off`. Default: `on`.
        pub auto_placement: pulumi_wasm_rust::Output<Option<String>>,
        /// The Availability Zone in which to allocate the Dedicated Host.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to enable or disable host recovery for the Dedicated Host. Valid values: `on`, `off`. Default: `off`.
        pub host_recovery: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family. Exactly one of `instance_family` or `instance_type` must be specified.
        pub instance_family: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only. Exactly one of `instance_family` or `instance_type` must be specified.
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the AWS Outpost on which to allocate the Dedicated Host.
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the Dedicated Host.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DedicatedHostArgs,
    ) -> DedicatedHostResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let asset_id_binding = args.asset_id.get_output(context).get_inner();
        let auto_placement_binding = args.auto_placement.get_output(context).get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let host_recovery_binding = args.host_recovery.get_output(context).get_inner();
        let instance_family_binding = args
            .instance_family
            .get_output(context)
            .get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let outpost_arn_binding = args.outpost_arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/dedicatedHost:DedicatedHost".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assetId".into(),
                    value: &asset_id_binding,
                },
                register_interface::ObjectField {
                    name: "autoPlacement".into(),
                    value: &auto_placement_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "hostRecovery".into(),
                    value: &host_recovery_binding,
                },
                register_interface::ObjectField {
                    name: "instanceFamily".into(),
                    value: &instance_family_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DedicatedHostResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            asset_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("assetId"),
            ),
            auto_placement: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoPlacement"),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            host_recovery: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostRecovery"),
            ),
            instance_family: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceFamily"),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
