#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_hours_of_operation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHoursOfOperationArgs {
        /// Returns information on a specific Hours of Operation by hours of operation id
        #[builder(into, default)]
        pub hours_of_operation_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific Hours of Operation by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Hours of Operation.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetHoursOfOperationResult {
        /// ARN of the Hours of Operation.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below. Config blocks are documented below.
        pub configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetHoursOfOperationConfig>,
        >,
        /// Description of the Hours of Operation.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the hours of operation.
        pub hours_of_operation_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Hours of Operation.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the Hours of Operation.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Time zone of the Hours of Operation.
        pub time_zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetHoursOfOperationArgs,
    ) -> GetHoursOfOperationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hours_of_operation_id_binding = args
            .hours_of_operation_id
            .get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getHoursOfOperation:getHoursOfOperation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hoursOfOperationId".into(),
                    value: hours_of_operation_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetHoursOfOperationResult {
            arn: o.get_field("arn"),
            configs: o.get_field("configs"),
            description: o.get_field("description"),
            hours_of_operation_id: o.get_field("hoursOfOperationId"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
