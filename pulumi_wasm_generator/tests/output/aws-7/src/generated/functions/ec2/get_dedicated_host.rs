pub mod get_dedicated_host {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDedicatedHostArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetDedicatedHostFilter>>,
        >,
        /// ID of the Dedicated Host.
        #[builder(into, default)]
        pub host_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDedicatedHostResult {
        /// ARN of the Dedicated Host.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the Outpost hardware asset on which the Dedicated Host is allocated.
        pub asset_id: pulumi_wasm_rust::Output<String>,
        /// Whether auto-placement is on or off.
        pub auto_placement: pulumi_wasm_rust::Output<String>,
        /// Availability Zone of the Dedicated Host.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Number of cores on the Dedicated Host.
        pub cores: pulumi_wasm_rust::Output<i32>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetDedicatedHostFilter>>,
        >,
        pub host_id: pulumi_wasm_rust::Output<String>,
        /// Whether host recovery is enabled or disabled for the Dedicated Host.
        pub host_recovery: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Instance family supported by the Dedicated Host. For example, "m5".
        pub instance_family: pulumi_wasm_rust::Output<String>,
        /// Instance type supported by the Dedicated Host. For example, "m5.large". If the host supports multiple instance types, no instanceType is returned.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the AWS Outpost on which the Dedicated Host is allocated.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the AWS account that owns the Dedicated Host.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Number of sockets on the Dedicated Host.
        pub sockets: pulumi_wasm_rust::Output<i32>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Total number of vCPUs on the Dedicated Host.
        pub total_vcpus: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDedicatedHostArgs) -> GetDedicatedHostResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let host_id_binding = args.host_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getDedicatedHost:getDedicatedHost".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "assetId".into(),
                },
                register_interface::ResultField {
                    name: "autoPlacement".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "cores".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "hostId".into(),
                },
                register_interface::ResultField {
                    name: "hostRecovery".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceFamily".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "sockets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "totalVcpus".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDedicatedHostResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            asset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assetId").unwrap(),
            ),
            auto_placement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoPlacement").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            cores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cores").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            host_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostId").unwrap(),
            ),
            host_recovery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostRecovery").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceFamily").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            sockets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sockets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            total_vcpus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalVcpus").unwrap(),
            ),
        }
    }
}
