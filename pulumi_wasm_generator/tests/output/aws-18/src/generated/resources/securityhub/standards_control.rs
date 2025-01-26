/// Disable/enable Security Hub standards control in the current region.
///
/// The `aws.securityhub.StandardsControl` behaves differently from normal resources, in that
/// Pulumi does not _create_ this resource, but instead "adopts" it
/// into management. When you _delete_ this resource configuration, Pulumi "abandons" resource as is and just removes it from the state.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cisAwsFoundationsBenchmark = standards_subscription::create(
///         "cisAwsFoundationsBenchmark",
///         StandardsSubscriptionArgs::builder()
///             .standards_arn(
///                 "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
///             )
///             .build_struct(),
///     );
///     let ensureIamPasswordPolicyPreventsPasswordReuse = standards_control::create(
///         "ensureIamPasswordPolicyPreventsPasswordReuse",
///         StandardsControlArgs::builder()
///             .control_status("DISABLED")
///             .disabled_reason("We handle password policies within Okta")
///             .standards_control_arn(
///                 "arn:aws:securityhub:us-east-1:111111111111:control/cis-aws-foundations-benchmark/v/1.2.0/1.10",
///             )
///             .build_struct(),
///     );
///     let example = account::create("example", AccountArgs::builder().build_struct());
/// }
/// ```
pub mod standards_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardsControlArgs {
        /// The control status could be `ENABLED` or `DISABLED`. You have to specify `disabled_reason` argument for `DISABLED` control status.
        #[builder(into)]
        pub control_status: pulumi_wasm_rust::InputOrOutput<String>,
        /// A description of the reason why you are disabling a security standard control. If you specify this attribute, `control_status` will be set to `DISABLED` automatically.
        #[builder(into, default)]
        pub disabled_reason: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The standards control ARN. See the AWS documentation for how to list existing controls using [`get-enabled-standards`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/get-enabled-standards.html) and [`describe-standards-controls`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/describe-standards-controls.html).
        #[builder(into)]
        pub standards_control_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StandardsControlResult {
        /// The identifier of the security standard control.
        pub control_id: pulumi_wasm_rust::Output<String>,
        /// The control status could be `ENABLED` or `DISABLED`. You have to specify `disabled_reason` argument for `DISABLED` control status.
        pub control_status: pulumi_wasm_rust::Output<String>,
        /// The date and time that the status of the security standard control was most recently updated.
        pub control_status_updated_at: pulumi_wasm_rust::Output<String>,
        /// The standard control longer description. Provides information about what the control is checking for.
        pub description: pulumi_wasm_rust::Output<String>,
        /// A description of the reason why you are disabling a security standard control. If you specify this attribute, `control_status` will be set to `DISABLED` automatically.
        pub disabled_reason: pulumi_wasm_rust::Output<String>,
        /// The list of requirements that are related to this control.
        pub related_requirements: pulumi_wasm_rust::Output<Vec<String>>,
        /// A link to remediation information for the control in the Security Hub user documentation.
        pub remediation_url: pulumi_wasm_rust::Output<String>,
        /// The severity of findings generated from this security standard control.
        pub severity_rating: pulumi_wasm_rust::Output<String>,
        /// The standards control ARN. See the AWS documentation for how to list existing controls using [`get-enabled-standards`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/get-enabled-standards.html) and [`describe-standards-controls`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/describe-standards-controls.html).
        pub standards_control_arn: pulumi_wasm_rust::Output<String>,
        /// The standard control title.
        pub title: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StandardsControlArgs,
    ) -> StandardsControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let control_status_binding = args.control_status.get_output(context).get_inner();
        let disabled_reason_binding = args
            .disabled_reason
            .get_output(context)
            .get_inner();
        let standards_control_arn_binding = args
            .standards_control_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/standardsControl:StandardsControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "controlStatus".into(),
                    value: &control_status_binding,
                },
                register_interface::ObjectField {
                    name: "disabledReason".into(),
                    value: &disabled_reason_binding,
                },
                register_interface::ObjectField {
                    name: "standardsControlArn".into(),
                    value: &standards_control_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "controlId".into(),
                },
                register_interface::ResultField {
                    name: "controlStatus".into(),
                },
                register_interface::ResultField {
                    name: "controlStatusUpdatedAt".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disabledReason".into(),
                },
                register_interface::ResultField {
                    name: "relatedRequirements".into(),
                },
                register_interface::ResultField {
                    name: "remediationUrl".into(),
                },
                register_interface::ResultField {
                    name: "severityRating".into(),
                },
                register_interface::ResultField {
                    name: "standardsControlArn".into(),
                },
                register_interface::ResultField {
                    name: "title".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StandardsControlResult {
            control_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlId").unwrap(),
            ),
            control_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlStatus").unwrap(),
            ),
            control_status_updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlStatusUpdatedAt").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disabled_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabledReason").unwrap(),
            ),
            related_requirements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relatedRequirements").unwrap(),
            ),
            remediation_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remediationUrl").unwrap(),
            ),
            severity_rating: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severityRating").unwrap(),
            ),
            standards_control_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standardsControlArn").unwrap(),
            ),
            title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("title").unwrap(),
            ),
        }
    }
}
