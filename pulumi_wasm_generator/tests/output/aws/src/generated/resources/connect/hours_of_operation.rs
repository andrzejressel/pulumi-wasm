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
pub mod hours_of_operation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HoursOfOperationArgs {
        /// One or more config blocks which define the configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below.
        #[builder(into)]
        pub configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::connect::HoursOfOperationConfig>,
        >,
        /// Specifies the description of the Hours of Operation.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Hours of Operation.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags to apply to the Hours of Operation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the time zone of the Hours of Operation.
        #[builder(into)]
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct HoursOfOperationResult {
        /// The Amazon Resource Name (ARN) of the Hours of Operation.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// One or more config blocks which define the configuration information for the hours of operation: day, start time, and end time . Config blocks are documented below.
        pub configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::connect::HoursOfOperationConfig>,
        >,
        /// Specifies the description of the Hours of Operation.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier for the hours of operation.
        pub hours_of_operation_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Hours of Operation.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Hours of Operation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the time zone of the Hours of Operation.
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HoursOfOperationArgs) -> HoursOfOperationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configs_binding = args.configs.get_inner();
        let description_binding = args.description.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/hoursOfOperation:HoursOfOperation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configs".into(),
                    value: &configs_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configs".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "hoursOfOperationId".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HoursOfOperationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configs").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            hours_of_operation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hoursOfOperationId").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
        }
    }
}