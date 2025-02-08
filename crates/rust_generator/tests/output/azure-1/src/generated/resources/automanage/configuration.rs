/// Manages an Automanage Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-automanage
///       location: West Europe
///   exampleConfiguration:
///     type: azure:automanage:Configuration
///     name: example
///     properties:
///       name: example-acmp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       antimalware:
///         exclusions:
///           extensions: exe;dll
///           paths: C:\Windows\Temp;D:\Temp
///           processes: svchost.exe;notepad.exe
///         realTimeProtectionEnabled: true
///         scheduledScanEnabled: true
///         scheduledScanType: Quick
///         scheduledScanDay: 1
///         scheduledScanTimeInMinutes: 1339
///       azureSecurityBaseline:
///         assignmentType: ApplyAndAutoCorrect
///       automationAccountEnabled: true
///       backup:
///         policyName: acctest-backup-policy-%d
///         timeZone: UTC
///         instantRpRetentionRangeInDays: 2
///         schedulePolicy:
///           scheduleRunFrequency: Daily
///           scheduleRunDays:
///             - Monday
///             - Tuesday
///           scheduleRunTimes:
///             - 12:00
///           schedulePolicyType: SimpleSchedulePolicy
///         retentionPolicy:
///           retentionPolicyType: LongTermRetentionPolicy
///           dailySchedule:
///             retentionTimes:
///               - 12:00
///             retentionDuration:
///               count: 7
///               durationType: Days
///           weeklySchedule:
///             retentionTimes:
///               - 14:00
///             retentionDuration:
///               count: 4
///               durationType: Weeks
///       bootDiagnosticsEnabled: true
///       defenderForCloudEnabled: true
///       guestConfigurationEnabled: true
///       logAnalyticsEnabled: true
///       statusChangeAlertEnabled: true
///       tags:
///         env: test
/// ```
///
/// ## Import
///
/// Automanage Configuration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automanage/configuration:Configuration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AutoManage/configurationProfiles/configurationProfile1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// A `antimalware` block as defined below.
        #[builder(into, default)]
        pub antimalware: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automanage::ConfigurationAntimalware>,
        >,
        /// Whether the automation account is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub automation_account_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `azure_security_baseline` block as defined below.
        #[builder(into, default)]
        pub azure_security_baseline: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automanage::ConfigurationAzureSecurityBaseline>,
        >,
        /// A `backup` block as defined below.
        #[builder(into, default)]
        pub backup: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automanage::ConfigurationBackup>,
        >,
        /// Whether the boot diagnostics are enabled. Defaults to `false`.
        #[builder(into, default)]
        pub boot_diagnostics_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the defender for cloud is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub defender_for_cloud_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the guest configuration is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub guest_configuration_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Azure Region where the Automanage Configuration should exist. Changing this forces a new Automanage Configuration to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether log analytics are enabled. Defaults to `false`.
        #[builder(into, default)]
        pub log_analytics_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Automanage Configuration. Changing this forces a new Automanage Configuration to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Automanage Configuration should exist. Changing this forces a new Automanage Configuration to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the status change alert is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub status_change_alert_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// A `antimalware` block as defined below.
        pub antimalware: pulumi_gestalt_rust::Output<
            Option<super::super::types::automanage::ConfigurationAntimalware>,
        >,
        /// Whether the automation account is enabled. Defaults to `false`.
        pub automation_account_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `azure_security_baseline` block as defined below.
        pub azure_security_baseline: pulumi_gestalt_rust::Output<
            Option<super::super::types::automanage::ConfigurationAzureSecurityBaseline>,
        >,
        /// A `backup` block as defined below.
        pub backup: pulumi_gestalt_rust::Output<
            Option<super::super::types::automanage::ConfigurationBackup>,
        >,
        /// Whether the boot diagnostics are enabled. Defaults to `false`.
        pub boot_diagnostics_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the defender for cloud is enabled. Defaults to `false`.
        pub defender_for_cloud_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the guest configuration is enabled. Defaults to `false`.
        pub guest_configuration_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure Region where the Automanage Configuration should exist. Changing this forces a new Automanage Configuration to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Whether log analytics are enabled. Defaults to `false`.
        pub log_analytics_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Automanage Configuration. Changing this forces a new Automanage Configuration to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Automanage Configuration should exist. Changing this forces a new Automanage Configuration to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Whether the status change alert is enabled. Defaults to `false`.
        pub status_change_alert_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConfigurationArgs,
    ) -> ConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let antimalware_binding = args.antimalware.get_output(context).get_inner();
        let automation_account_enabled_binding = args
            .automation_account_enabled
            .get_output(context)
            .get_inner();
        let azure_security_baseline_binding = args
            .azure_security_baseline
            .get_output(context)
            .get_inner();
        let backup_binding = args.backup.get_output(context).get_inner();
        let boot_diagnostics_enabled_binding = args
            .boot_diagnostics_enabled
            .get_output(context)
            .get_inner();
        let defender_for_cloud_enabled_binding = args
            .defender_for_cloud_enabled
            .get_output(context)
            .get_inner();
        let guest_configuration_enabled_binding = args
            .guest_configuration_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let log_analytics_enabled_binding = args
            .log_analytics_enabled
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let status_change_alert_enabled_binding = args
            .status_change_alert_enabled
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automanage/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "antimalware".into(),
                    value: &antimalware_binding,
                },
                register_interface::ObjectField {
                    name: "automationAccountEnabled".into(),
                    value: &automation_account_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "azureSecurityBaseline".into(),
                    value: &azure_security_baseline_binding,
                },
                register_interface::ObjectField {
                    name: "backup".into(),
                    value: &backup_binding,
                },
                register_interface::ObjectField {
                    name: "bootDiagnosticsEnabled".into(),
                    value: &boot_diagnostics_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "defenderForCloudEnabled".into(),
                    value: &defender_for_cloud_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "guestConfigurationEnabled".into(),
                    value: &guest_configuration_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsEnabled".into(),
                    value: &log_analytics_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "statusChangeAlertEnabled".into(),
                    value: &status_change_alert_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationResult {
            antimalware: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("antimalware"),
            ),
            automation_account_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountEnabled"),
            ),
            azure_security_baseline: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureSecurityBaseline"),
            ),
            backup: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backup"),
            ),
            boot_diagnostics_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootDiagnosticsEnabled"),
            ),
            defender_for_cloud_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defenderForCloudEnabled"),
            ),
            guest_configuration_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guestConfigurationEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_analytics_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsEnabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            status_change_alert_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusChangeAlertEnabled"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
