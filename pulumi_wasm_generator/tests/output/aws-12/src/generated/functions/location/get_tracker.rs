pub mod get_tracker {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerArgs {
        /// Key-value map of resource tags for the tracker.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the tracker resource.
        #[builder(into)]
        pub tracker_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTrackerResult {
        /// Timestamp for when the tracker resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional description for the tracker resource.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Position filtering method of the tracker resource.
        pub position_filtering: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the tracker.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// ARN for the tracker resource. Used when you need to specify a resource across all AWS.
        pub tracker_arn: pulumi_wasm_rust::Output<String>,
        pub tracker_name: pulumi_wasm_rust::Output<String>,
        /// Timestamp for when the tracker resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTrackerArgs) -> GetTrackerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_inner();
        let tracker_name_binding = args.tracker_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getTracker:getTracker".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trackerName".into(),
                    value: &tracker_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "positionFiltering".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trackerArn".into(),
                },
                register_interface::ResultField {
                    name: "trackerName".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTrackerResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            position_filtering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("positionFiltering").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tracker_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackerArn").unwrap(),
            ),
            tracker_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackerName").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
