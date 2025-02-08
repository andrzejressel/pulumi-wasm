/// Provides a resource to manage a single Amazon GuardDuty [detector feature](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-features-activation-model.html#guardduty-features).
///
/// > **NOTE:** Deleting this resource does not disable the detector feature, the resource in simply removed from state instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let eksRuntimeMonitoring = detector_feature::create(
///         "eksRuntimeMonitoring",
///         DetectorFeatureArgs::builder()
///             .additional_configurations(
///                 vec![
///                     DetectorFeatureAdditionalConfiguration::builder()
///                     .name("EKS_ADDON_MANAGEMENT").status("ENABLED").build_struct(),
///                 ],
///             )
///             .detector_id("${example.id}")
///             .name("EKS_RUNTIME_MONITORING")
///             .status("ENABLED")
///             .build_struct(),
///     );
///     let example = detector::create(
///         "example",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod detector_feature {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DetectorFeatureArgs {
        /// Additional feature configuration block for features`EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING`. See below.
        #[builder(into, default)]
        pub additional_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::guardduty::DetectorFeatureAdditionalConfiguration,
                >,
            >,
        >,
        /// Amazon GuardDuty detector ID.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the detector feature. Valid values: `S3_DATA_EVENTS`, `EKS_AUDIT_LOGS`, `EBS_MALWARE_PROTECTION`, `RDS_LOGIN_EVENTS`, `EKS_RUNTIME_MONITORING`, `LAMBDA_NETWORK_LOGS`, `RUNTIME_MONITORING`. Only one of two features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING` can be added, adding both features will cause an error. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorFeatureConfiguration.html) for the current list of supported values.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The status of the detector feature. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DetectorFeatureResult {
        /// Additional feature configuration block for features`EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING`. See below.
        pub additional_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::guardduty::DetectorFeatureAdditionalConfiguration,
                >,
            >,
        >,
        /// Amazon GuardDuty detector ID.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the detector feature. Valid values: `S3_DATA_EVENTS`, `EKS_AUDIT_LOGS`, `EBS_MALWARE_PROTECTION`, `RDS_LOGIN_EVENTS`, `EKS_RUNTIME_MONITORING`, `LAMBDA_NETWORK_LOGS`, `RUNTIME_MONITORING`. Only one of two features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING` can be added, adding both features will cause an error. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorFeatureConfiguration.html) for the current list of supported values.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The status of the detector feature. Valid values: `ENABLED`, `DISABLED`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DetectorFeatureArgs,
    ) -> DetectorFeatureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_configurations_binding = args
            .additional_configurations
            .get_output(context)
            .get_inner();
        let detector_id_binding = args.detector_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/detectorFeature:DetectorFeature".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalConfigurations".into(),
                    value: &additional_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DetectorFeatureResult {
            additional_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalConfigurations"),
            ),
            detector_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectorId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
