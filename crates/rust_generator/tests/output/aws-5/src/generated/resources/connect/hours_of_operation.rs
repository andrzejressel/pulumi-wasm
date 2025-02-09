/// Provides an Amazon Connect Hours of Operation resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:connect:HoursOfOperation
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Office Hours
///       description: Monday office hours
///       timeZone: EST
///       configs:
///         - day: MONDAY
///           endTime:
///             hours: 23
///             minutes: 8
///           startTime:
///             hours: 8
///             minutes: 0
///         - day: TUESDAY
///           endTime:
///             hours: 21
///             minutes: 0
///           startTime:
///             hours: 9
///             minutes: 0
///       tags:
///         Name: Example Hours of Operation
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Hours of Operations using the `instance_id` and `hours_of_operation_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/hoursOfOperation:HoursOfOperation example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hours_of_operation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HoursOfOperationArgs {
        /// One or more config blocks which define the configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below.
        #[builder(into)]
        pub configs: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::connect::HoursOfOperationConfig>,
        >,
        /// Specifies the description of the Hours of Operation.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Hours of Operation.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the Hours of Operation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the time zone of the Hours of Operation.
        #[builder(into)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HoursOfOperationResult {
        /// The Amazon Resource Name (ARN) of the Hours of Operation.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// One or more config blocks which define the configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below.
        pub configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::connect::HoursOfOperationConfig>,
        >,
        /// Specifies the description of the Hours of Operation.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier for the hours of operation.
        pub hours_of_operation_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Hours of Operation.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Hours of Operation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the time zone of the Hours of Operation.
        pub time_zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HoursOfOperationArgs,
    ) -> HoursOfOperationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configs_binding = args.configs.get_output(context);
        let description_binding = args.description.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/hoursOfOperation:HoursOfOperation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configs".into(),
                    value: configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: time_zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HoursOfOperationResult {
            arn: o.get_field("arn"),
            configs: o.get_field("configs"),
            description: o.get_field("description"),
            hours_of_operation_id: o.get_field("hoursOfOperationId"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
