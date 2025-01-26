/// Provides an AWS Backup plan resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:backup:Plan
///     properties:
///       name: my_example_backup_plan
///       rules:
///         - ruleName: my_example_backup_rule
///           targetVaultName: ${test.name}
///           schedule: cron(0 12 * * ? *)
///           lifecycle:
///             deleteAfter: 14
///       advancedBackupSettings:
///         - backupOptions:
///             WindowsVSS: enabled
///           resourceType: EC2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Plan using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/plan:Plan test <id>
/// ```
pub mod plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlanArgs {
        /// An object that specifies backup options for each resource type.
        #[builder(into, default)]
        pub advanced_backup_settings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::backup::PlanAdvancedBackupSetting>>,
        >,
        /// The display name of a backup plan.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A rule object that specifies a scheduled task that is used to back up a selection of resources.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::backup::PlanRule>,
        >,
        /// Metadata that you can assign to help organize the plans you create. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlanResult {
        /// An object that specifies backup options for each resource type.
        pub advanced_backup_settings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::backup::PlanAdvancedBackupSetting>>,
        >,
        /// The ARN of the backup plan.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The display name of a backup plan.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A rule object that specifies a scheduled task that is used to back up a selection of resources.
        pub rules: pulumi_wasm_rust::Output<Vec<super::super::types::backup::PlanRule>>,
        /// Metadata that you can assign to help organize the plans you create. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Unique, randomly generated, Unicode, UTF-8 encoded string that serves as the version ID of the backup plan.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PlanArgs,
    ) -> PlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_backup_settings_binding = args
            .advanced_backup_settings
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/plan:Plan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedBackupSettings".into(),
                    value: &advanced_backup_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PlanResult {
            advanced_backup_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedBackupSettings"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
