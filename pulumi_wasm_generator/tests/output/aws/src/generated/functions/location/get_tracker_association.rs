pub mod get_tracker_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerAssociationArgs {
        /// ARN of the geofence collection associated to tracker resource.
        #[builder(into)]
        pub consumer_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the tracker resource associated with a geofence collection.
        #[builder(into)]
        pub tracker_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTrackerAssociationResult {
        pub consumer_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub tracker_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTrackerAssociationArgs) -> GetTrackerAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let consumer_arn_binding = args.consumer_arn.get_inner();
        let tracker_name_binding = args.tracker_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getTrackerAssociation:getTrackerAssociation".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "consumerArn".into(),
                    value: &consumer_arn_binding,
                },
                register_interface::ObjectField {
                    name: "trackerName".into(),
                    value: &tracker_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "consumerArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "trackerName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTrackerAssociationResult {
            consumer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumerArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            tracker_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackerName").unwrap(),
            ),
        }
    }
}
