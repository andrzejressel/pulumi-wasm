/// Manages the GuardDuty Organization Configuration in the current AWS Region. The AWS account utilizing this resource must have been assigned as a delegated Organization administrator account, e.g., via the `aws.guardduty.OrganizationAdminAccount` resource. More information about Organizations support in GuardDuty can be found in the [GuardDuty User Guide](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_organizations.html).
///
/// > **NOTE:** This is an advanced resource. The provider will automatically assume management of the GuardDuty Organization Configuration without import and perform no actions on removal from the resource configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = detector::create(
///         "example",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
///     let exampleOrganizationConfiguration = organization_configuration::create(
///         "exampleOrganizationConfiguration",
///         OrganizationConfigurationArgs::builder()
///             .auto_enable_organization_members("ALL")
///             .datasources(
///                 OrganizationConfigurationDatasources::builder()
///                     .kubernetes(
///                         OrganizationConfigurationDatasourcesKubernetes::builder()
///                             .auditLogs(
///                                 OrganizationConfigurationDatasourcesKubernetesAuditLogs::builder()
///                                     .enable(true)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .malwareProtection(
///                         OrganizationConfigurationDatasourcesMalwareProtection::builder()
///                             .scanEc2InstanceWithFindings(
///                                 OrganizationConfigurationDatasourcesMalwareProtectionScanEc2InstanceWithFindings::builder()
///                                     .ebsVolumes(
///                                         OrganizationConfigurationDatasourcesMalwareProtectionScanEc2InstanceWithFindingsEbsVolumes::builder()
///                                             .autoEnable(true)
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .s3Logs(
///                         OrganizationConfigurationDatasourcesS3Logs::builder()
///                             .autoEnable(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .detector_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty Organization Configurations using the GuardDuty Detector ID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/organizationConfiguration:OrganizationConfiguration example 00b00fd5aecc0ab60a708659477e9617
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// *Deprecated:* Use `auto_enable_organization_members` instead. When this setting is enabled, all new accounts that are created in, or added to, the organization are added as a member accounts of the organization’s GuardDuty delegated administrator and GuardDuty is enabled in that AWS Region.
        #[builder(into, default)]
        pub auto_enable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates the auto-enablement configuration of GuardDuty for the member accounts in the organization. Valid values are `ALL`, `NEW`, `NONE`.
        #[builder(into, default)]
        pub auto_enable_organization_members: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration for the collected datasources.
        #[builder(into, default)]
        pub datasources: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::guardduty::OrganizationConfigurationDatasources>,
        >,
        /// The detector ID of the GuardDuty account.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// *Deprecated:* Use `auto_enable_organization_members` instead. When this setting is enabled, all new accounts that are created in, or added to, the organization are added as a member accounts of the organization’s GuardDuty delegated administrator and GuardDuty is enabled in that AWS Region.
        pub auto_enable: pulumi_gestalt_rust::Output<bool>,
        /// Indicates the auto-enablement configuration of GuardDuty for the member accounts in the organization. Valid values are `ALL`, `NEW`, `NONE`.
        pub auto_enable_organization_members: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the collected datasources.
        pub datasources: pulumi_gestalt_rust::Output<
            super::super::types::guardduty::OrganizationConfigurationDatasources,
        >,
        /// The detector ID of the GuardDuty account.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_enable_binding = args.auto_enable.get_output(context);
        let auto_enable_organization_members_binding = args
            .auto_enable_organization_members
            .get_output(context);
        let datasources_binding = args.datasources.get_output(context);
        let detector_id_binding = args.detector_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnable".into(),
                    value: auto_enable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnableOrganizationMembers".into(),
                    value: auto_enable_organization_members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datasources".into(),
                    value: datasources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorId".into(),
                    value: detector_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationConfigurationResult {
            auto_enable: o.get_field("autoEnable"),
            auto_enable_organization_members: o
                .get_field("autoEnableOrganizationMembers"),
            datasources: o.get_field("datasources"),
            detector_id: o.get_field("detectorId"),
        }
    }
}
