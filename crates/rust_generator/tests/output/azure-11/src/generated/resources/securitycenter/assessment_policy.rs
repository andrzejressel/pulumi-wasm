/// Manages the Security Center Assessment Metadata for Azure Security Center.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = assessment_policy::create(
///         "example",
///         AssessmentPolicyArgs::builder()
///             .description("Test Description")
///             .display_name("Test Display Name")
///             .severity("Medium")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Security Assessments Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/assessmentPolicy:AssessmentPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/assessmentMetadata/metadata1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assessment_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentPolicyArgs {
        /// A list of the categories of resource that is at risk when the Security Center Assessment is unhealthy. Possible values are `Unknown`, `Compute`, `Data`, `IdentityAndAccess`, `IoT` and `Networking`.
        #[builder(into, default)]
        pub categories: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The description of the Security Center Assessment.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user-friendly display name of the Security Center Assessment.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The implementation effort which is used to remediate the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        #[builder(into, default)]
        pub implementation_effort: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description which is used to mitigate the security issue.
        #[builder(into, default)]
        pub remediation_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The severity level of the Security Center Assessment. Possible values are `Low`, `Medium` and `High`. Defaults to `Medium`.
        #[builder(into, default)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of the threat impacts for the Security Center Assessment. Possible values are `AccountBreach`, `DataExfiltration`, `DataSpillage`, `DenialOfService`, `ElevationOfPrivilege`, `MaliciousInsider`, `MissingCoverage` and `ThreatResistance`.
        #[builder(into, default)]
        pub threats: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The user impact of the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        #[builder(into, default)]
        pub user_impact: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentPolicyResult {
        /// A list of the categories of resource that is at risk when the Security Center Assessment is unhealthy. Possible values are `Unknown`, `Compute`, `Data`, `IdentityAndAccess`, `IoT` and `Networking`.
        pub categories: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The description of the Security Center Assessment.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The user-friendly display name of the Security Center Assessment.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The implementation effort which is used to remediate the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        pub implementation_effort: pulumi_gestalt_rust::Output<Option<String>>,
        /// The GUID as the name of the Security Center Assessment Policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The description which is used to mitigate the security issue.
        pub remediation_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The severity level of the Security Center Assessment. Possible values are `Low`, `Medium` and `High`. Defaults to `Medium`.
        pub severity: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of the threat impacts for the Security Center Assessment. Possible values are `AccountBreach`, `DataExfiltration`, `DataSpillage`, `DenialOfService`, `ElevationOfPrivilege`, `MaliciousInsider`, `MissingCoverage` and `ThreatResistance`.
        pub threats: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The user impact of the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        pub user_impact: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentPolicyArgs,
    ) -> AssessmentPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let categories_binding = args.categories.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let implementation_effort_binding = args
            .implementation_effort
            .get_output(context);
        let remediation_description_binding = args
            .remediation_description
            .get_output(context);
        let severity_binding = args.severity.get_output(context);
        let threats_binding = args.threats.get_output(context);
        let user_impact_binding = args.user_impact.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/assessmentPolicy:AssessmentPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "categories".into(),
                    value: categories_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "implementationEffort".into(),
                    value: implementation_effort_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remediationDescription".into(),
                    value: remediation_description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "severity".into(),
                    value: severity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threats".into(),
                    value: threats_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userImpact".into(),
                    value: user_impact_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssessmentPolicyResult {
            categories: o.get_field("categories"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            implementation_effort: o.get_field("implementationEffort"),
            name: o.get_field("name"),
            remediation_description: o.get_field("remediationDescription"),
            severity: o.get_field("severity"),
            threats: o.get_field("threats"),
            user_impact: o.get_field("userImpact"),
        }
    }
}
