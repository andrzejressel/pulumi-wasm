pub mod get_hours_of_operation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHoursOfOperationArgs {
        /// Returns information on a specific Hours of Operation by hours of operation id
        #[builder(into, default)]
        pub hours_of_operation_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific Hours of Operation by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Hours of Operation.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetHoursOfOperationResult {
        /// ARN of the Hours of Operation.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below. Config blocks are documented below.
        pub configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetHoursOfOperationConfig>,
        >,
        /// Description of the Hours of Operation.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The identifier for the hours of operation.
        pub hours_of_operation_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Hours of Operation.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the Hours of Operation.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Time zone of the Hours of Operation.
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetHoursOfOperationArgs,
    ) -> GetHoursOfOperationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hours_of_operation_id_binding = args
            .hours_of_operation_id
            .get_output(context)
            .get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getHoursOfOperation:getHoursOfOperation".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hoursOfOperationId".into(),
                    value: &hours_of_operation_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetHoursOfOperationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configs"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            hours_of_operation_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hoursOfOperationId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeZone"),
            ),
        }
    }
}
