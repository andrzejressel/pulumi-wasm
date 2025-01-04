pub mod get_outposts {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostsArgs {
        /// Availability Zone name.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Availability Zone identifier.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS Account identifier of the Outpost owner.
        #[builder(into, default)]
        pub owner_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Site identifier.
        #[builder(into, default)]
        pub site_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOutpostsResult {
        /// Set of Amazon Resource Names (ARNs).
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of identifiers.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub site_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOutpostsArgs) -> GetOutpostsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding = args.availability_zone.get_inner();
        let availability_zone_id_binding = args.availability_zone_id.get_inner();
        let owner_id_binding = args.owner_id.get_inner();
        let site_id_binding = args.site_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:outposts/getOutposts:getOutposts".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZoneId".into(),
                    value: &availability_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arns".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ids".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "siteId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOutpostsResult {
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            site_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteId").unwrap(),
            ),
        }
    }
}
