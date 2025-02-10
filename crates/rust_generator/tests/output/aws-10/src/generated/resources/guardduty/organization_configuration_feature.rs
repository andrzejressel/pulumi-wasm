/// Provides a resource to manage a single Amazon GuardDuty [organization configuration feature](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-features-activation-model.html#guardduty-features).
///
/// > **NOTE:** Deleting this resource does not disable the organization configuration feature, the resource in simply removed from state instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let eksRuntimeMonitoring = organization_configuration_feature::create(
///         "eksRuntimeMonitoring",
///         OrganizationConfigurationFeatureArgs::builder()
///             .additional_configurations(
///                 vec![
///                     OrganizationConfigurationFeatureAdditionalConfiguration::builder()
///                     .autoEnable("NEW").name("EKS_ADDON_MANAGEMENT").build_struct(),
///                 ],
///             )
///             .auto_enable("ALL")
///             .detector_id("${example.id}")
///             .name("EKS_RUNTIME_MONITORING")
///             .build_struct(),
///     );
///     let example = detector::create(
///         "example",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_configuration_feature {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationFeatureArgs {
        /// Additional feature configuration block for features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING`. See below.
        #[builder(into, default)]
        pub additional_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::guardduty::OrganizationConfigurationFeatureAdditionalConfiguration,
                >,
            >,
        >,
        /// The status of the feature that is configured for the member accounts within the organization. Valid values: `NEW`, `ALL`, `NONE`.
        #[builder(into)]
        pub auto_enable: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the detector that configures the delegated administrator.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the feature that will be configured for the organization. Valid values: `S3_DATA_EVENTS`, `EKS_AUDIT_LOGS`, `EBS_MALWARE_PROTECTION`, `RDS_LOGIN_EVENTS`, `EKS_RUNTIME_MONITORING`, `LAMBDA_NETWORK_LOGS`, `RUNTIME_MONITORING`. Only one of two features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING` can be added, adding both features will cause an error. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorFeatureConfiguration.html) for the current list of supported values.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationFeatureResult {
        /// Additional feature configuration block for features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING`. See below.
        pub additional_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::guardduty::OrganizationConfigurationFeatureAdditionalConfiguration,
                >,
            >,
        >,
        /// The status of the feature that is configured for the member accounts within the organization. Valid values: `NEW`, `ALL`, `NONE`.
        pub auto_enable: pulumi_gestalt_rust::Output<String>,
        /// The ID of the detector that configures the delegated administrator.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the feature that will be configured for the organization. Valid values: `S3_DATA_EVENTS`, `EKS_AUDIT_LOGS`, `EBS_MALWARE_PROTECTION`, `RDS_LOGIN_EVENTS`, `EKS_RUNTIME_MONITORING`, `LAMBDA_NETWORK_LOGS`, `RUNTIME_MONITORING`. Only one of two features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING` can be added, adding both features will cause an error. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorFeatureConfiguration.html) for the current list of supported values.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationFeatureArgs,
    ) -> OrganizationConfigurationFeatureResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_configurations_binding = args
            .additional_configurations
            .get_output(context);
        let auto_enable_binding = args.auto_enable.get_output(context);
        let detector_id_binding = args.detector_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/organizationConfigurationFeature:OrganizationConfigurationFeature"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalConfigurations".into(),
                    value: additional_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnable".into(),
                    value: auto_enable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorId".into(),
                    value: detector_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationConfigurationFeatureResult {
            additional_configurations: o.get_field("additionalConfigurations"),
            auto_enable: o.get_field("autoEnable"),
            detector_id: o.get_field("detectorId"),
            name: o.get_field("name"),
        }
    }
}
