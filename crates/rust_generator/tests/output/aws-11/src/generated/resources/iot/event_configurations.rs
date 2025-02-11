/// Manages IoT event configurations.
///
/// > **NOTE:** Deleting this resource does not disable the event configurations, the resource in simply removed from state instead.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:EventConfigurations
///     properties:
///       eventConfigurations:
///         THING: true
///         THING_GROUP: false
///         THING_TYPE: false
///         THING_GROUP_MEMBERSHIP: false
///         THING_GROUP_HIERARCHY: false
///         THING_TYPE_ASSOCIATION: false
///         JOB: false
///         JOB_EXECUTION: false
///         POLICY: false
///         CERTIFICATE: true
///         CA_CERTIFICATE: false
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT Event Configurations using the AWS Region. For example:
///
/// ```sh
/// $ pulumi import aws:iot/eventConfigurations:EventConfigurations example us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_configurations {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventConfigurationsArgs {
        /// Map. The new event configuration values. You can use only these strings as keys: `THING_GROUP_HIERARCHY`, `THING_GROUP_MEMBERSHIP`, `THING_TYPE`, `THING_TYPE_ASSOCIATION`, `THING_GROUP`, `THING`, `POLICY`, `CA_CERTIFICATE`, `JOB_EXECUTION`, `CERTIFICATE`, `JOB`. Use boolean for values of mapping.
        #[builder(into)]
        pub event_configurations: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventConfigurationsResult {
        /// Map. The new event configuration values. You can use only these strings as keys: `THING_GROUP_HIERARCHY`, `THING_GROUP_MEMBERSHIP`, `THING_TYPE`, `THING_TYPE_ASSOCIATION`, `THING_GROUP`, `THING`, `POLICY`, `CA_CERTIFICATE`, `JOB_EXECUTION`, `CERTIFICATE`, `JOB`. Use boolean for values of mapping.
        pub event_configurations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventConfigurationsArgs,
    ) -> EventConfigurationsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let event_configurations_binding = args.event_configurations.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/eventConfigurations:EventConfigurations".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventConfigurations".into(),
                    value: &event_configurations_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventConfigurationsResult {
            event_configurations: o.get_field("eventConfigurations"),
        }
    }
}
