/// Provides a resource to manage a single Amazon GuardDuty [organization configuration feature](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-features-activation-model.html#guardduty-features).
///
/// > **NOTE:** Deleting this resource does not disable the organization configuration feature, the resource in simply removed from state instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod organization_configuration_feature {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationFeatureArgs {
        /// Additional feature configuration block for features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING`. See below.
        #[builder(into, default)]
        pub additional_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::guardduty::OrganizationConfigurationFeatureAdditionalConfiguration,
                >,
            >,
        >,
        /// The status of the feature that is configured for the member accounts within the organization. Valid values: `NEW`, `ALL`, `NONE`.
        #[builder(into)]
        pub auto_enable: pulumi_wasm_rust::Output<String>,
        /// The ID of the detector that configures the delegated administrator.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// The name of the feature that will be configured for the organization. Valid values: `S3_DATA_EVENTS`, `EKS_AUDIT_LOGS`, `EBS_MALWARE_PROTECTION`, `RDS_LOGIN_EVENTS`, `EKS_RUNTIME_MONITORING`, `LAMBDA_NETWORK_LOGS`, `RUNTIME_MONITORING`. Only one of two features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING` can be added, adding both features will cause an error. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorFeatureConfiguration.html) for the current list of supported values.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationFeatureResult {
        /// Additional feature configuration block for features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING`. See below.
        pub additional_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::guardduty::OrganizationConfigurationFeatureAdditionalConfiguration,
                >,
            >,
        >,
        /// The status of the feature that is configured for the member accounts within the organization. Valid values: `NEW`, `ALL`, `NONE`.
        pub auto_enable: pulumi_wasm_rust::Output<String>,
        /// The ID of the detector that configures the delegated administrator.
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// The name of the feature that will be configured for the organization. Valid values: `S3_DATA_EVENTS`, `EKS_AUDIT_LOGS`, `EBS_MALWARE_PROTECTION`, `RDS_LOGIN_EVENTS`, `EKS_RUNTIME_MONITORING`, `LAMBDA_NETWORK_LOGS`, `RUNTIME_MONITORING`. Only one of two features `EKS_RUNTIME_MONITORING` or `RUNTIME_MONITORING` can be added, adding both features will cause an error. Refer to the [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/APIReference/API_DetectorFeatureConfiguration.html) for the current list of supported values.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationConfigurationFeatureArgs,
    ) -> OrganizationConfigurationFeatureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_configurations_binding = args
            .additional_configurations
            .get_inner();
        let auto_enable_binding = args.auto_enable.get_inner();
        let detector_id_binding = args.detector_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/organizationConfigurationFeature:OrganizationConfigurationFeature"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalConfigurations".into(),
                    value: &additional_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "autoEnable".into(),
                    value: &auto_enable_binding,
                },
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "autoEnable".into(),
                },
                register_interface::ResultField {
                    name: "detectorId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationConfigurationFeatureResult {
            additional_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalConfigurations").unwrap(),
            ),
            auto_enable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoEnable").unwrap(),
            ),
            detector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
