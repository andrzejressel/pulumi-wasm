/// Represents the settings related to security issues, such as data redaction and data retention. It may take hours for updates on the settings to propagate to all the related components and take effect.
/// Multiple security settings can be configured in each location. Each agent can specify the security settings to apply, and each setting can be applied to multiple agents in the same project and location.
///
///
/// To get more information about SecuritySettings, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.securitySettings)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Security Settings Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicSecuritySettings = cx_security_settings::create(
///         "basicSecuritySettings",
///         CxSecuritySettingsArgs::builder()
///             .display_name("dialogflowcx-security-settings")
///             .location("global")
///             .purge_data_types(vec![])
///             .retention_window_days(7)
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dialogflowcx Security Settings Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicSecuritySettings = cx_security_settings::create(
///         "basicSecuritySettings",
///         CxSecuritySettingsArgs::builder()
///             .audio_export_settings(
///                 CxSecuritySettingsAudioExportSettings::builder()
///                     .audioExportPattern("export")
///                     .audioFormat("OGG")
///                     .enableAudioRedaction(true)
///                     .gcsBucket("${bucket.id}")
///                     .build_struct(),
///             )
///             .deidentify_template("${deidentify.id}")
///             .display_name("dialogflowcx-security-settings")
///             .insights_export_settings(
///                 CxSecuritySettingsInsightsExportSettings::builder()
///                     .enableInsightsExport(true)
///                     .build_struct(),
///             )
///             .inspect_template("${inspect.id}")
///             .location("global")
///             .purge_data_types(vec!["DIALOGFLOW_HISTORY",])
///             .redaction_scope("REDACT_DISK_STORAGE")
///             .redaction_strategy("REDACT_WITH_SERVICE")
///             .retention_strategy("REMOVE_AFTER_CONVERSATION")
///             .build_struct(),
///     );
///     let bucket = bucket::create(
///         "bucket",
///         BucketArgs::builder()
///             .location("US")
///             .name("dialogflowcx-bucket")
///             .uniform_bucket_level_access(true)
///             .build_struct(),
///     );
///     let deidentify = prevention_deidentify_template::create(
///         "deidentify",
///         PreventionDeidentifyTemplateArgs::builder()
///             .deidentify_config(
///                 PreventionDeidentifyTemplateDeidentifyConfig::builder()
///                     .infoTypeTransformations(
///                         PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformations::builder()
///                             .transformations(
///                                 vec![
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation::builder()
///                                     .primitiveTransformation(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformation::builder()
///                                     .replaceConfig(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfig::builder()
///                                     .newValue(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfigNewValue::builder()
///                                     .stringValue("[REDACTED]").build_struct()).build_struct())
///                                     .build_struct()).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .display_name("dialogflowcx-deidentify-template")
///             .parent("projects/my-project-name/locations/global")
///             .build_struct(),
///     );
///     let inspect = prevention_inspect_template::create(
///         "inspect",
///         PreventionInspectTemplateArgs::builder()
///             .display_name("dialogflowcx-inspect-template")
///             .inspect_config(
///                 PreventionInspectTemplateInspectConfig::builder()
///                     .infoTypes(
///                         vec![
///                             PreventionInspectTemplateInspectConfigInfoType::builder()
///                             .name("EMAIL_ADDRESS").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name/locations/global")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SecuritySettings can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/securitySettings/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, SecuritySettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxSecuritySettings:CxSecuritySettings default projects/{{project}}/locations/{{location}}/securitySettings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxSecuritySettings:CxSecuritySettings default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxSecuritySettings:CxSecuritySettings default {{location}}/{{name}}
/// ```
///
pub mod cx_security_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxSecuritySettingsArgs {
        /// Controls audio export settings for post-conversation analytics when ingesting audio to conversations.
        /// If retention_strategy is set to REMOVE_AFTER_CONVERSATION or gcs_bucket is empty, audio export is disabled.
        /// If audio export is enabled, audio is recorded and saved to gcs_bucket, subject to retention policy of gcs_bucket.
        /// This setting won't effect audio input for implicit sessions via [Sessions.DetectIntent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/detectIntent#google.cloud.dialogflow.cx.v3.Sessions.DetectIntent).
        /// Structure is documented below.
        #[builder(into, default)]
        pub audio_export_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxSecuritySettingsAudioExportSettings>,
        >,
        /// [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.
        /// Note: deidentifyTemplate must be located in the same region as the SecuritySettings.
        /// Format: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>
        #[builder(into, default)]
        pub deidentify_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the security settings, unique within the location.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Controls conversation exporting settings to Insights after conversation is completed.
        /// If retentionStrategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here.
        /// Structure is documented below.
        #[builder(into, default)]
        pub insights_export_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::diagflow::CxSecuritySettingsInsightsExportSettings,
            >,
        >,
        /// [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.
        /// Note: inspectTemplate must be located in the same region as the SecuritySettings.
        /// Format: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>
        #[builder(into, default)]
        pub inspect_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The location these settings are located in. Settings can only be applied to an agent in the same location.
        /// See [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// List of types of data to remove when retention settings triggers purge.
        /// Each value may be one of: `DIALOGFLOW_HISTORY`.
        #[builder(into, default)]
        pub purge_data_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Defines what types of data to redact. If not set, defaults to not redacting any kind of data.
        /// * REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk.
        /// Possible values are: `REDACT_DISK_STORAGE`.
        #[builder(into, default)]
        pub redaction_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines how we redact data. If not set, defaults to not redacting.
        /// * REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted.
        /// Possible values are: `REDACT_WITH_SERVICE`.
        #[builder(into, default)]
        pub redaction_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines how long we retain persisted data that contains sensitive info. Only one of `retention_window_days` and `retention_strategy` may be set.
        /// * REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends.
        /// Possible values are: `REMOVE_AFTER_CONVERSATION`.
        #[builder(into, default)]
        pub retention_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.
        /// Only one of `retention_window_days` and `retention_strategy` may be set.
        #[builder(into, default)]
        pub retention_window_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct CxSecuritySettingsResult {
        /// Controls audio export settings for post-conversation analytics when ingesting audio to conversations.
        /// If retention_strategy is set to REMOVE_AFTER_CONVERSATION or gcs_bucket is empty, audio export is disabled.
        /// If audio export is enabled, audio is recorded and saved to gcs_bucket, subject to retention policy of gcs_bucket.
        /// This setting won't effect audio input for implicit sessions via [Sessions.DetectIntent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/detectIntent#google.cloud.dialogflow.cx.v3.Sessions.DetectIntent).
        /// Structure is documented below.
        pub audio_export_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::diagflow::CxSecuritySettingsAudioExportSettings>,
        >,
        /// [DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.
        /// Note: deidentifyTemplate must be located in the same region as the SecuritySettings.
        /// Format: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>
        pub deidentify_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The human-readable name of the security settings, unique within the location.
        ///
        ///
        /// - - -
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Controls conversation exporting settings to Insights after conversation is completed.
        /// If retentionStrategy is set to REMOVE_AFTER_CONVERSATION, Insights export is disabled no matter what you configure here.
        /// Structure is documented below.
        pub insights_export_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::diagflow::CxSecuritySettingsInsightsExportSettings,
            >,
        >,
        /// [DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.
        /// Note: inspectTemplate must be located in the same region as the SecuritySettings.
        /// Format: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>
        pub inspect_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The location these settings are located in. Settings can only be applied to an agent in the same location.
        /// See [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the settings.
        /// Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// List of types of data to remove when retention settings triggers purge.
        /// Each value may be one of: `DIALOGFLOW_HISTORY`.
        pub purge_data_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Defines what types of data to redact. If not set, defaults to not redacting any kind of data.
        /// * REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk.
        /// Possible values are: `REDACT_DISK_STORAGE`.
        pub redaction_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines how we redact data. If not set, defaults to not redacting.
        /// * REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted.
        /// Possible values are: `REDACT_WITH_SERVICE`.
        pub redaction_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines how long we retain persisted data that contains sensitive info. Only one of `retention_window_days` and `retention_strategy` may be set.
        /// * REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends.
        /// Possible values are: `REMOVE_AFTER_CONVERSATION`.
        pub retention_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Retains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.
        /// Only one of `retention_window_days` and `retention_strategy` may be set.
        pub retention_window_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CxSecuritySettingsArgs) -> CxSecuritySettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audio_export_settings_binding = args.audio_export_settings.get_inner();
        let deidentify_template_binding = args.deidentify_template.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let insights_export_settings_binding = args.insights_export_settings.get_inner();
        let inspect_template_binding = args.inspect_template.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let purge_data_types_binding = args.purge_data_types.get_inner();
        let redaction_scope_binding = args.redaction_scope.get_inner();
        let redaction_strategy_binding = args.redaction_strategy.get_inner();
        let retention_strategy_binding = args.retention_strategy.get_inner();
        let retention_window_days_binding = args.retention_window_days.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:diagflow/cxSecuritySettings:CxSecuritySettings".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "audioExportSettings".into(),
                    value: &audio_export_settings_binding,
                },
                register_interface::ObjectField {
                    name: "deidentifyTemplate".into(),
                    value: &deidentify_template_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "insightsExportSettings".into(),
                    value: &insights_export_settings_binding,
                },
                register_interface::ObjectField {
                    name: "inspectTemplate".into(),
                    value: &inspect_template_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "purgeDataTypes".into(),
                    value: &purge_data_types_binding,
                },
                register_interface::ObjectField {
                    name: "redactionScope".into(),
                    value: &redaction_scope_binding,
                },
                register_interface::ObjectField {
                    name: "redactionStrategy".into(),
                    value: &redaction_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "retentionStrategy".into(),
                    value: &retention_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "retentionWindowDays".into(),
                    value: &retention_window_days_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "audioExportSettings".into(),
                },
                register_interface::ResultField {
                    name: "deidentifyTemplate".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "insightsExportSettings".into(),
                },
                register_interface::ResultField {
                    name: "inspectTemplate".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "purgeDataTypes".into(),
                },
                register_interface::ResultField {
                    name: "redactionScope".into(),
                },
                register_interface::ResultField {
                    name: "redactionStrategy".into(),
                },
                register_interface::ResultField {
                    name: "retentionStrategy".into(),
                },
                register_interface::ResultField {
                    name: "retentionWindowDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CxSecuritySettingsResult {
            audio_export_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("audioExportSettings").unwrap(),
            ),
            deidentify_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deidentifyTemplate").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            insights_export_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insightsExportSettings").unwrap(),
            ),
            inspect_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inspectTemplate").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            purge_data_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purgeDataTypes").unwrap(),
            ),
            redaction_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redactionScope").unwrap(),
            ),
            redaction_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redactionStrategy").unwrap(),
            ),
            retention_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionStrategy").unwrap(),
            ),
            retention_window_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionWindowDays").unwrap(),
            ),
        }
    }
}
