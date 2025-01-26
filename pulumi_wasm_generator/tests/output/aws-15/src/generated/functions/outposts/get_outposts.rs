pub mod get_outposts {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostsArgs {
        /// Availability Zone name.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Availability Zone identifier.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// AWS Account identifier of the Outpost owner.
        #[builder(into, default)]
        pub owner_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Site identifier.
        #[builder(into, default)]
        pub site_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOutpostsArgs,
    ) -> GetOutpostsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let availability_zone_id_binding = args
            .availability_zone_id
            .get_output(context)
            .get_inner();
        let owner_id_binding = args.owner_id.get_output(context).get_inner();
        let site_id_binding = args.site_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:outposts/getOutposts:getOutposts".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOutpostsResult {
            arns: pulumi_wasm_rust::__private::into_domain(o.extract_field("arns")),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_wasm_rust::__private::into_domain(o.extract_field("ids")),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            site_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("siteId")),
        }
    }
}
