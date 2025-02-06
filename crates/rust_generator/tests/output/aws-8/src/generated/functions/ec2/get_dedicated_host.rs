pub mod get_dedicated_host {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDedicatedHostArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetDedicatedHostFilter>>,
        >,
        /// ID of the Dedicated Host.
        #[builder(into, default)]
        pub host_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDedicatedHostResult {
        /// ARN of the Dedicated Host.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Outpost hardware asset on which the Dedicated Host is allocated.
        pub asset_id: pulumi_gestalt_rust::Output<String>,
        /// Whether auto-placement is on or off.
        pub auto_placement: pulumi_gestalt_rust::Output<String>,
        /// Availability Zone of the Dedicated Host.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Number of cores on the Dedicated Host.
        pub cores: pulumi_gestalt_rust::Output<i32>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetDedicatedHostFilter>>,
        >,
        pub host_id: pulumi_gestalt_rust::Output<String>,
        /// Whether host recovery is enabled or disabled for the Dedicated Host.
        pub host_recovery: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Instance family supported by the Dedicated Host. For example, "m5".
        pub instance_family: pulumi_gestalt_rust::Output<String>,
        /// Instance type supported by the Dedicated Host. For example, "m5.large". If the host supports multiple instance types, no instanceType is returned.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the AWS Outpost on which the Dedicated Host is allocated.
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the Dedicated Host.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Number of sockets on the Dedicated Host.
        pub sockets: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Total number of vCPUs on the Dedicated Host.
        pub total_vcpus: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDedicatedHostArgs,
    ) -> GetDedicatedHostResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let host_id_binding = args.host_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getDedicatedHost:getDedicatedHost".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "hostId".into(),
                    value: &host_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDedicatedHostResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            asset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assetId"),
            ),
            auto_placement: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoPlacement"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            cores: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cores")),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            host_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostId"),
            ),
            host_recovery: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostRecovery"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceFamily"),
            ),
            instance_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            sockets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sockets"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            total_vcpus: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("totalVcpus"),
            ),
        }
    }
}
