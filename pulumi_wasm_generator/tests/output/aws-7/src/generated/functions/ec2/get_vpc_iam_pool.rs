pub mod get_vpc_iam_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcIamPoolArgs {
        /// Tags that are required to create resources in using this pool.
        #[builder(into, default)]
        pub allocation_resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolFilter>>,
        >,
        /// ID of the IPAM pool.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the IPAM pool you would like information on.
        #[builder(into, default)]
        pub ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcIamPoolResult {
        /// IP protocol assigned to this pool.
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is `10.0.0.0/8` and you enter 16 here, new allocations will default to `10.0.0.0/16`.
        pub allocation_default_netmask_length: pulumi_wasm_rust::Output<i32>,
        /// The maximum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_max_netmask_length: pulumi_wasm_rust::Output<i32>,
        /// The minimum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_min_netmask_length: pulumi_wasm_rust::Output<i32>,
        /// Tags that are required to create resources in using this pool.
        pub allocation_resource_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of the pool
        pub arn: pulumi_wasm_rust::Output<String>,
        /// If enabled, IPAM will continuously look for resources within the CIDR range of this pool and automatically import them as allocations into your IPAM.
        pub auto_import: pulumi_wasm_rust::Output<bool>,
        /// Limits which service in AWS that the pool can be used in. `ec2` for example, allows users to use space for Elastic IP addresses and VPCs.
        pub aws_service: pulumi_wasm_rust::Output<String>,
        /// Description for the IPAM pool.
        pub description: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolFilter>>,
        >,
        /// ID of the IPAM pool.
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        pub ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the scope the pool belongs to.
        pub ipam_scope_id: pulumi_wasm_rust::Output<String>,
        pub ipam_scope_type: pulumi_wasm_rust::Output<String>,
        /// Locale is the Region where your pool is available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region.
        pub locale: pulumi_wasm_rust::Output<String>,
        pub pool_depth: pulumi_wasm_rust::Output<i32>,
        /// Defines whether or not IPv6 pool space is publicly advertisable over the internet.
        pub publicly_advertisable: pulumi_wasm_rust::Output<bool>,
        /// ID of the source IPAM pool.
        pub source_ipam_pool_id: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpcIamPoolArgs) -> GetVpcIamPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_resource_tags_binding = args.allocation_resource_tags.get_inner();
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcIamPool:getVpcIamPool".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationResourceTags".into(),
                    value: &allocation_resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressFamily".into(),
                },
                register_interface::ResultField {
                    name: "allocationDefaultNetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "allocationMaxNetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "allocationMinNetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "allocationResourceTags".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoImport".into(),
                },
                register_interface::ResultField {
                    name: "awsService".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "ipamScopeId".into(),
                },
                register_interface::ResultField {
                    name: "ipamScopeType".into(),
                },
                register_interface::ResultField {
                    name: "locale".into(),
                },
                register_interface::ResultField {
                    name: "poolDepth".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAdvertisable".into(),
                },
                register_interface::ResultField {
                    name: "sourceIpamPoolId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpcIamPoolResult {
            address_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressFamily").unwrap(),
            ),
            allocation_default_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationDefaultNetmaskLength").unwrap(),
            ),
            allocation_max_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationMaxNetmaskLength").unwrap(),
            ),
            allocation_min_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationMinNetmaskLength").unwrap(),
            ),
            allocation_resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationResourceTags").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_import: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoImport").unwrap(),
            ),
            aws_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsService").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamPoolId").unwrap(),
            ),
            ipam_scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamScopeId").unwrap(),
            ),
            ipam_scope_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamScopeType").unwrap(),
            ),
            locale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locale").unwrap(),
            ),
            pool_depth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolDepth").unwrap(),
            ),
            publicly_advertisable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAdvertisable").unwrap(),
            ),
            source_ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIpamPoolId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
