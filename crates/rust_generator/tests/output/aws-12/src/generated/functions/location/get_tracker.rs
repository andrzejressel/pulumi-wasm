pub mod get_tracker {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrackerArgs {
        /// Key-value map of resource tags for the tracker.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the tracker resource.
        #[builder(into)]
        pub tracker_name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTrackerArgs,
    ) -> GetTrackerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let tracker_name_binding = args.tracker_name.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTrackerResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            position_filtering: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("positionFiltering"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tracker_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trackerArn"),
            ),
            tracker_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trackerName"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
