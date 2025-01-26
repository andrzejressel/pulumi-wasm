/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = snapshot_schedule::create(
///         "default",
///         SnapshotScheduleArgs::builder()
///             .definitions(vec!["rate(12 hours)",])
///             .identifier("tf-redshift-snapshot-schedule")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Snapshot Schedule using the `identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/snapshotSchedule:SnapshotSchedule default tf-redshift-snapshot-schedule
/// ```
pub mod snapshot_schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotScheduleArgs {
        /// The definition of the snapshot schedule. The definition is made up of schedule expressions, for example `cron(30 12 *)` or `rate(12 hours)`.
        #[builder(into)]
        pub definitions: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The description of the snapshot schedule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to destroy all associated clusters with this snapshot schedule on deletion. Must be enabled and applied before attempting deletion.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The snapshot schedule identifier. If omitted, this provider will assign a random, unique identifier.
        #[builder(into, default)]
        pub identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique
        /// identifier beginning with the specified prefix. Conflicts with `identifier`.
        #[builder(into, default)]
        pub identifier_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SnapshotScheduleResult {
        /// Amazon Resource Name (ARN) of the Redshift Snapshot Schedule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The definition of the snapshot schedule. The definition is made up of schedule expressions, for example `cron(30 12 *)` or `rate(12 hours)`.
        pub definitions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The description of the snapshot schedule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to destroy all associated clusters with this snapshot schedule on deletion. Must be enabled and applied before attempting deletion.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// The snapshot schedule identifier. If omitted, this provider will assign a random, unique identifier.
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// Creates a unique
        /// identifier beginning with the specified prefix. Conflicts with `identifier`.
        pub identifier_prefix: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SnapshotScheduleArgs,
    ) -> SnapshotScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definitions_binding = args.definitions.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let identifier_prefix_binding = args
            .identifier_prefix
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/snapshotSchedule:SnapshotSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definitions".into(),
                    value: &definitions_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "identifierPrefix".into(),
                    value: &identifier_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotScheduleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            definitions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("definitions"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            identifier_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identifierPrefix"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
