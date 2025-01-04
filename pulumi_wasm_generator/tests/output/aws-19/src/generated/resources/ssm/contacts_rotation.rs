/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = contacts_rotation::create(
///         "example",
///         ContactsRotationArgs::builder()
///             .contact_ids(vec!["${exampleAwsSsmcontactsContact.arn}",])
///             .name("rotation")
///             .recurrence(
///                 ContactsRotationRecurrence::builder()
///                     .dailySettings(
///                         vec![
///                             ContactsRotationRecurrenceDailySetting::builder()
///                             .hourOfDay(9).minuteOfHour(0).build_struct(),
///                         ],
///                     )
///                     .numberOfOnCalls(1)
///                     .recurrenceMultiplier(1)
///                     .build_struct(),
///             )
///             .time_zone_id("Australia/Sydney")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with Weekly Settings and Shift Coverages Fields
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssm:ContactsRotation
///     properties:
///       contactIds:
///         - ${exampleAwsSsmcontactsContact.arn}
///       name: rotation
///       recurrence:
///         numberOfOnCalls: 1
///         recurrenceMultiplier: 1
///         weeklySettings:
///           - dayOfWeek: WED
///             handOffTime:
///               hourOfDay: 4
///               minuteOfHour: 25
///           - dayOfWeek: FRI
///             handOffTime:
///               hourOfDay: 15
///               minuteOfHour: 57
///         shiftCoverages:
///           - mapBlockKey: MON
///             coverageTimes:
///               - start:
///                   hourOfDay: 1
///                   minuteOfHour: 0
///                 end:
///                   hourOfDay: 23
///                   minuteOfHour: 0
///       startTime: 2023-07-20T02:21:49+00:00
///       timeZoneId: Australia/Sydney
///       tags:
///         key1: tag1
///         key2: tag2
///     options:
///       dependsOn:
///         - ${exampleAwsSsmincidentsReplicationSet}
/// ```
///
/// ### Usage with Monthly Settings Fields
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = contacts_rotation::create(
///         "example",
///         ContactsRotationArgs::builder()
///             .contact_ids(vec!["${exampleAwsSsmcontactsContact.arn}",])
///             .name("rotation")
///             .recurrence(
///                 ContactsRotationRecurrence::builder()
///                     .monthlySettings(
///                         vec![
///                             ContactsRotationRecurrenceMonthlySetting::builder()
///                             .dayOfMonth(20)
///                             .handOffTime(ContactsRotationRecurrenceMonthlySettingHandOffTime::builder()
///                             .hourOfDay(8).minuteOfHour(0).build_struct()).build_struct(),
///                             ContactsRotationRecurrenceMonthlySetting::builder()
///                             .dayOfMonth(13)
///                             .handOffTime(ContactsRotationRecurrenceMonthlySettingHandOffTime::builder()
///                             .hourOfDay(12).minuteOfHour(34).build_struct())
///                             .build_struct(),
///                         ],
///                     )
///                     .numberOfOnCalls(1)
///                     .recurrenceMultiplier(1)
///                     .build_struct(),
///             )
///             .time_zone_id("Australia/Sydney")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeGuru Profiler Profiling Group using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/contactsRotation:ContactsRotation example arn:aws:ssm-contacts:us-east-1:012345678910:rotation/example
/// ```
pub mod contacts_rotation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactsRotationArgs {
        /// Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
        #[builder(into)]
        pub contact_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name for the rotation.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about when an on-call rotation is in effect and how long the rotation period lasts. Exactly one of either `daily_settings`, `monthly_settings`, or `weekly_settings` must be populated. See Recurrence for more details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::ssm::ContactsRotationRecurrence>,
        >,
        /// The date and time, in RFC 3339 format, that the rotation goes into effect.
        #[builder(into, default)]
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
        #[builder(into)]
        pub time_zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContactsRotationResult {
        /// The Amazon Resource Name (ARN) of the rotation.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
        pub contact_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name for the rotation.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Information about when an on-call rotation is in effect and how long the rotation period lasts. Exactly one of either `daily_settings`, `monthly_settings`, or `weekly_settings` must be populated. See Recurrence for more details.
        ///
        /// The following arguments are optional:
        pub recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::ssm::ContactsRotationRecurrence>,
        >,
        /// The date and time, in RFC 3339 format, that the rotation goes into effect.
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
        pub time_zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContactsRotationArgs) -> ContactsRotationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_ids_binding = args.contact_ids.get_inner();
        let name_binding = args.name.get_inner();
        let recurrence_binding = args.recurrence.get_inner();
        let start_time_binding = args.start_time.get_inner();
        let tags_binding = args.tags.get_inner();
        let time_zone_id_binding = args.time_zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/contactsRotation:ContactsRotation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactIds".into(),
                    value: &contact_ids_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recurrence".into(),
                    value: &recurrence_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeZoneId".into(),
                    value: &time_zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "contactIds".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recurrence".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeZoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContactsRotationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            contact_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactIds").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recurrence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurrence").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            time_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZoneId").unwrap(),
            ),
        }
    }
}
