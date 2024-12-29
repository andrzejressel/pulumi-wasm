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
pub mod event_configurations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventConfigurationsArgs {
        /// Map. The new event configuration values. You can use only these strings as keys: `THING_GROUP_HIERARCHY`, `THING_GROUP_MEMBERSHIP`, `THING_TYPE`, `THING_TYPE_ASSOCIATION`, `THING_GROUP`, `THING`, `POLICY`, `CA_CERTIFICATE`, `JOB_EXECUTION`, `CERTIFICATE`, `JOB`. Use boolean for values of mapping.
        #[builder(into)]
        pub event_configurations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventConfigurationsResult {
        /// Map. The new event configuration values. You can use only these strings as keys: `THING_GROUP_HIERARCHY`, `THING_GROUP_MEMBERSHIP`, `THING_TYPE`, `THING_TYPE_ASSOCIATION`, `THING_GROUP`, `THING`, `POLICY`, `CA_CERTIFICATE`, `JOB_EXECUTION`, `CERTIFICATE`, `JOB`. Use boolean for values of mapping.
        pub event_configurations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EventConfigurationsArgs,
    ) -> EventConfigurationsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let event_configurations_binding = args.event_configurations.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/eventConfigurations:EventConfigurations".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventConfigurations".into(),
                    value: &event_configurations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventConfigurations".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventConfigurationsResult {
            event_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventConfigurations").unwrap(),
            ),
        }
    }
}
