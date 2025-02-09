/// Provides a resource to manage an Amazon GuardDuty detector.
///
/// > **NOTE:** Deleting this resource is equivalent to "disabling" GuardDuty for an AWS region, which removes all existing findings. You can set the `enable` attribute to `false` to instead "suspend" monitoring and feedback reporting while keeping existing data. See the [Suspending or Disabling Amazon GuardDuty documentation](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_suspend-disable.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myDetector = detector::create(
///         "myDetector",
///         DetectorArgs::builder()
///             .datasources(
///                 DetectorDatasources::builder()
///                     .kubernetes(
///                         DetectorDatasourcesKubernetes::builder()
///                             .auditLogs(
///                                 DetectorDatasourcesKubernetesAuditLogs::builder()
///                                     .enable(false)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .malwareProtection(
///                         DetectorDatasourcesMalwareProtection::builder()
///                             .scanEc2InstanceWithFindings(
///                                 DetectorDatasourcesMalwareProtectionScanEc2InstanceWithFindings::builder()
///                                     .ebsVolumes(
///                                         DetectorDatasourcesMalwareProtectionScanEc2InstanceWithFindingsEbsVolumes::builder()
///                                             .enable(true)
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .s3Logs(
///                         DetectorDatasourcesS3Logs::builder().enable(true).build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .enable(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty detectors using the detector ID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/detector:Detector MyDetector 00b00fd5aecc0ab60a708659477e9617
/// ```
/// The ID of the detector can be retrieved via the [AWS CLI](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/guardduty/list-detectors.html) using `aws guardduty list-detectors`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod detector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DetectorArgs {
        /// Describes which data sources will be enabled for the detector. See Data Sources below for more details. [Deprecated](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-feature-object-api-changes-march2023.html) in favor of `aws.guardduty.DetectorFeature` resources.
        #[builder(into, default)]
        pub datasources: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::guardduty::DetectorDatasources>,
        >,
        /// Enable monitoring and feedback reporting. Setting to `false` is equivalent to "suspending" GuardDuty. Defaults to `true`.
        #[builder(into, default)]
        pub enable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the frequency of notifications sent for subsequent finding occurrences. If the detector is a GuardDuty member account, the value is determined by the GuardDuty primary account and cannot be modified, otherwise defaults to `SIX_HOURS`. For standalone and GuardDuty primary accounts, it must be configured in this provider to enable drift detection. Valid values for standalone and primary accounts: `FIFTEEN_MINUTES`, `ONE_HOUR`, `SIX_HOURS`. See [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_findings_cloudwatch.html#guardduty_findings_cloudwatch_notification_frequency) for more information.
        #[builder(into, default)]
        pub finding_publishing_frequency: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DetectorResult {
        /// The AWS account ID of the GuardDuty detector
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the GuardDuty detector
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Describes which data sources will be enabled for the detector. See Data Sources below for more details. [Deprecated](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty-feature-object-api-changes-march2023.html) in favor of `aws.guardduty.DetectorFeature` resources.
        pub datasources: pulumi_gestalt_rust::Output<
            super::super::types::guardduty::DetectorDatasources,
        >,
        /// Enable monitoring and feedback reporting. Setting to `false` is equivalent to "suspending" GuardDuty. Defaults to `true`.
        pub enable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the frequency of notifications sent for subsequent finding occurrences. If the detector is a GuardDuty member account, the value is determined by the GuardDuty primary account and cannot be modified, otherwise defaults to `SIX_HOURS`. For standalone and GuardDuty primary accounts, it must be configured in this provider to enable drift detection. Valid values for standalone and primary accounts: `FIFTEEN_MINUTES`, `ONE_HOUR`, `SIX_HOURS`. See [AWS Documentation](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_findings_cloudwatch.html#guardduty_findings_cloudwatch_notification_frequency) for more information.
        pub finding_publishing_frequency: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DetectorArgs,
    ) -> DetectorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let datasources_binding = args.datasources.get_output(context);
        let enable_binding = args.enable.get_output(context);
        let finding_publishing_frequency_binding = args
            .finding_publishing_frequency
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/detector:Detector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datasources".into(),
                    value: datasources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enable".into(),
                    value: enable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "findingPublishingFrequency".into(),
                    value: finding_publishing_frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DetectorResult {
            account_id: o.get_field("accountId"),
            arn: o.get_field("arn"),
            datasources: o.get_field("datasources"),
            enable: o.get_field("enable"),
            finding_publishing_frequency: o.get_field("findingPublishingFrequency"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
