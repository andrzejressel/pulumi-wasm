/// Associates an SSM Document to an instance or EC2 tag.
///
/// ## Example Usage
///
/// ### Create an association for a specific instance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = association::create(
///         "example",
///         AssociationArgs::builder()
///             .name("${exampleAwsSsmDocument.name}")
///             .targets(
///                 vec![
///                     AssociationTarget::builder().key("InstanceIds")
///                     .values(vec!["${exampleAwsInstance.id}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create an association for all managed instances in an AWS account
///
/// To target all managed instances in an AWS account, set the `key` as `"InstanceIds"` with `values` set as `["*"]`. This example also illustrates how to use an Amazon owned SSM document named `AmazonCloudWatch-ManageAgent`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = association::create(
///         "example",
///         AssociationArgs::builder()
///             .name("AmazonCloudWatch-ManageAgent")
///             .targets(
///                 vec![
///                     AssociationTarget::builder().key("InstanceIds").values(vec!["*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create an association for a specific tag
///
/// This example shows how to target all managed instances that are assigned a tag key of `Environment` and value of `Development`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = association::create(
///         "example",
///         AssociationArgs::builder()
///             .name("AmazonCloudWatch-ManageAgent")
///             .targets(
///                 vec![
///                     AssociationTarget::builder().key("tag:Environment")
///                     .values(vec!["Development",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create an association with a specific schedule
///
/// This example shows how to schedule an association in various ways.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = association::create(
///         "example",
///         AssociationArgs::builder()
///             .name("${exampleAwsSsmDocument.name}")
///             .schedule_expression("cron(0 2 ? * SUN *)")
///             .targets(
///                 vec![
///                     AssociationTarget::builder().key("InstanceIds")
///                     .values(vec!["${exampleAwsInstance.id}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM associations using the `association_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/association:Association test-association 10abcdef-0abc-1234-5678-90abcdef123456
/// ```
pub mod association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssociationArgs {
        /// By default, when you create a new or update associations, the system runs it immediately and then according to the schedule you specified. Enable this option if you do not want an association to run immediately after you create or update it. This parameter is not supported for rate expressions. Default: `false`.
        #[builder(into, default)]
        pub apply_only_at_cron_interval: pulumi_wasm_rust::Output<Option<bool>>,
        /// The descriptive name for the association.
        #[builder(into, default)]
        pub association_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the target for the association. This target is required for associations that use an `Automation` document and target resources by using rate controls. This should be set to the SSM document `parameter` that will define how your automation will branch out.
        #[builder(into, default)]
        pub automation_target_parameter_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The compliance severity for the association. Can be one of the following: `UNSPECIFIED`, `LOW`, `MEDIUM`, `HIGH` or `CRITICAL`
        #[builder(into, default)]
        pub compliance_severity: pulumi_wasm_rust::Output<Option<String>>,
        /// The document version you want to associate with the target(s). Can be a specific version or the default version.
        #[builder(into, default)]
        pub document_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The instance ID to apply an SSM document to. Use `targets` with key `InstanceIds` for document schema versions 2.0 and above. Use the `targets` attribute instead.
        #[builder(into, default)]
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%.
        #[builder(into, default)]
        pub max_concurrency: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify a number, for example 10, or a percentage of the target set, for example 10%. If you specify a threshold of 3, the stop command is sent when the fourth error is returned. If you specify a threshold of 10% for 50 associations, the stop command is sent when the sixth error is returned.
        #[builder(into, default)]
        pub max_errors: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the SSM document to apply.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An output location block. Output Location is documented below.
        #[builder(into, default)]
        pub output_location: pulumi_wasm_rust::Output<
            Option<super::super::types::ssm::AssociationOutputLocation>,
        >,
        /// A block of arbitrary string parameters to pass to the SSM document.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html) that specifies when the association runs.
        #[builder(into, default)]
        pub schedule_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode for generating association compliance. You can specify `AUTO` or `MANUAL`.
        #[builder(into, default)]
        pub sync_compliance: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block containing the targets of the SSM association. Targets are documented below. AWS currently supports a maximum of 5 targets.
        #[builder(into, default)]
        pub targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ssm::AssociationTarget>>,
        >,
        /// The number of seconds to wait for the association status to be `Success`. If `Success` status is not reached within the given time, create opration will fail.
        ///
        /// Output Location (`output_location`) is an S3 bucket where you want to store the results of this association:
        #[builder(into, default)]
        pub wait_for_success_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct AssociationResult {
        /// By default, when you create a new or update associations, the system runs it immediately and then according to the schedule you specified. Enable this option if you do not want an association to run immediately after you create or update it. This parameter is not supported for rate expressions. Default: `false`.
        pub apply_only_at_cron_interval: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the SSM association
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the SSM association.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// The descriptive name for the association.
        pub association_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the target for the association. This target is required for associations that use an `Automation` document and target resources by using rate controls. This should be set to the SSM document `parameter` that will define how your automation will branch out.
        pub automation_target_parameter_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The compliance severity for the association. Can be one of the following: `UNSPECIFIED`, `LOW`, `MEDIUM`, `HIGH` or `CRITICAL`
        pub compliance_severity: pulumi_wasm_rust::Output<Option<String>>,
        /// The document version you want to associate with the target(s). Can be a specific version or the default version.
        pub document_version: pulumi_wasm_rust::Output<String>,
        /// The instance ID to apply an SSM document to. Use `targets` with key `InstanceIds` for document schema versions 2.0 and above. Use the `targets` attribute instead.
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%.
        pub max_concurrency: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify a number, for example 10, or a percentage of the target set, for example 10%. If you specify a threshold of 3, the stop command is sent when the fourth error is returned. If you specify a threshold of 10% for 50 associations, the stop command is sent when the sixth error is returned.
        pub max_errors: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the SSM document to apply.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An output location block. Output Location is documented below.
        pub output_location: pulumi_wasm_rust::Output<
            Option<super::super::types::ssm::AssociationOutputLocation>,
        >,
        /// A block of arbitrary string parameters to pass to the SSM document.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html) that specifies when the association runs.
        pub schedule_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode for generating association compliance. You can specify `AUTO` or `MANUAL`.
        pub sync_compliance: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A block containing the targets of the SSM association. Targets are documented below. AWS currently supports a maximum of 5 targets.
        pub targets: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssm::AssociationTarget>,
        >,
        /// The number of seconds to wait for the association status to be `Success`. If `Success` status is not reached within the given time, create opration will fail.
        ///
        /// Output Location (`output_location`) is an S3 bucket where you want to store the results of this association:
        pub wait_for_success_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssociationArgs) -> AssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apply_only_at_cron_interval_binding = args
            .apply_only_at_cron_interval
            .get_inner();
        let association_name_binding = args.association_name.get_inner();
        let automation_target_parameter_name_binding = args
            .automation_target_parameter_name
            .get_inner();
        let compliance_severity_binding = args.compliance_severity.get_inner();
        let document_version_binding = args.document_version.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let max_concurrency_binding = args.max_concurrency.get_inner();
        let max_errors_binding = args.max_errors.get_inner();
        let name_binding = args.name.get_inner();
        let output_location_binding = args.output_location.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let schedule_expression_binding = args.schedule_expression.get_inner();
        let sync_compliance_binding = args.sync_compliance.get_inner();
        let tags_binding = args.tags.get_inner();
        let targets_binding = args.targets.get_inner();
        let wait_for_success_timeout_seconds_binding = args
            .wait_for_success_timeout_seconds
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/association:Association".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applyOnlyAtCronInterval".into(),
                    value: &apply_only_at_cron_interval_binding,
                },
                register_interface::ObjectField {
                    name: "associationName".into(),
                    value: &association_name_binding,
                },
                register_interface::ObjectField {
                    name: "automationTargetParameterName".into(),
                    value: &automation_target_parameter_name_binding,
                },
                register_interface::ObjectField {
                    name: "complianceSeverity".into(),
                    value: &compliance_severity_binding,
                },
                register_interface::ObjectField {
                    name: "documentVersion".into(),
                    value: &document_version_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "maxConcurrency".into(),
                    value: &max_concurrency_binding,
                },
                register_interface::ObjectField {
                    name: "maxErrors".into(),
                    value: &max_errors_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outputLocation".into(),
                    value: &output_location_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleExpression".into(),
                    value: &schedule_expression_binding,
                },
                register_interface::ObjectField {
                    name: "syncCompliance".into(),
                    value: &sync_compliance_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targets".into(),
                    value: &targets_binding,
                },
                register_interface::ObjectField {
                    name: "waitForSuccessTimeoutSeconds".into(),
                    value: &wait_for_success_timeout_seconds_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applyOnlyAtCronInterval".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associationId".into(),
                },
                register_interface::ResultField {
                    name: "associationName".into(),
                },
                register_interface::ResultField {
                    name: "automationTargetParameterName".into(),
                },
                register_interface::ResultField {
                    name: "complianceSeverity".into(),
                },
                register_interface::ResultField {
                    name: "documentVersion".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "maxConcurrency".into(),
                },
                register_interface::ResultField {
                    name: "maxErrors".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputLocation".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "scheduleExpression".into(),
                },
                register_interface::ResultField {
                    name: "syncCompliance".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targets".into(),
                },
                register_interface::ResultField {
                    name: "waitForSuccessTimeoutSeconds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssociationResult {
            apply_only_at_cron_interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyOnlyAtCronInterval").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationId").unwrap(),
            ),
            association_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationName").unwrap(),
            ),
            automation_target_parameter_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationTargetParameterName").unwrap(),
            ),
            compliance_severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("complianceSeverity").unwrap(),
            ),
            document_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentVersion").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            max_concurrency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxConcurrency").unwrap(),
            ),
            max_errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxErrors").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            output_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputLocation").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            schedule_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduleExpression").unwrap(),
            ),
            sync_compliance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncCompliance").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targets").unwrap(),
            ),
            wait_for_success_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForSuccessTimeoutSeconds").unwrap(),
            ),
        }
    }
}