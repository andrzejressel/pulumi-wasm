/// Manages the Security Center Assessment Metadata for Azure Security Center.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod assessment_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentPolicyArgs {
        /// A list of the categories of resource that is at risk when the Security Center Assessment is unhealthy. Possible values are `Unknown`, `Compute`, `Data`, `IdentityAndAccess`, `IoT` and `Networking`.
        #[builder(into, default)]
        pub categories: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The description of the Security Center Assessment.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// The user-friendly display name of the Security Center Assessment.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The implementation effort which is used to remediate the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        #[builder(into, default)]
        pub implementation_effort: pulumi_wasm_rust::Output<Option<String>>,
        /// The description which is used to mitigate the security issue.
        #[builder(into, default)]
        pub remediation_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The severity level of the Security Center Assessment. Possible values are `Low`, `Medium` and `High`. Defaults to `Medium`.
        #[builder(into, default)]
        pub severity: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of the threat impacts for the Security Center Assessment. Possible values are `AccountBreach`, `DataExfiltration`, `DataSpillage`, `DenialOfService`, `ElevationOfPrivilege`, `MaliciousInsider`, `MissingCoverage` and `ThreatResistance`.
        #[builder(into, default)]
        pub threats: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The user impact of the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        #[builder(into, default)]
        pub user_impact: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentPolicyResult {
        /// A list of the categories of resource that is at risk when the Security Center Assessment is unhealthy. Possible values are `Unknown`, `Compute`, `Data`, `IdentityAndAccess`, `IoT` and `Networking`.
        pub categories: pulumi_wasm_rust::Output<Vec<String>>,
        /// The description of the Security Center Assessment.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The user-friendly display name of the Security Center Assessment.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The implementation effort which is used to remediate the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        pub implementation_effort: pulumi_wasm_rust::Output<Option<String>>,
        /// The GUID as the name of the Security Center Assessment Policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The description which is used to mitigate the security issue.
        pub remediation_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The severity level of the Security Center Assessment. Possible values are `Low`, `Medium` and `High`. Defaults to `Medium`.
        pub severity: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of the threat impacts for the Security Center Assessment. Possible values are `AccountBreach`, `DataExfiltration`, `DataSpillage`, `DenialOfService`, `ElevationOfPrivilege`, `MaliciousInsider`, `MissingCoverage` and `ThreatResistance`.
        pub threats: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The user impact of the Security Center Assessment. Possible values are `Low`, `Moderate` and `High`.
        pub user_impact: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssessmentPolicyArgs) -> AssessmentPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let categories_binding = args.categories.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let implementation_effort_binding = args.implementation_effort.get_inner();
        let remediation_description_binding = args.remediation_description.get_inner();
        let severity_binding = args.severity.get_inner();
        let threats_binding = args.threats.get_inner();
        let user_impact_binding = args.user_impact.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/assessmentPolicy:AssessmentPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "categories".into(),
                    value: &categories_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "implementationEffort".into(),
                    value: &implementation_effort_binding,
                },
                register_interface::ObjectField {
                    name: "remediationDescription".into(),
                    value: &remediation_description_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "threats".into(),
                    value: &threats_binding,
                },
                register_interface::ObjectField {
                    name: "userImpact".into(),
                    value: &user_impact_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "categories".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "implementationEffort".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "remediationDescription".into(),
                },
                register_interface::ResultField {
                    name: "severity".into(),
                },
                register_interface::ResultField {
                    name: "threats".into(),
                },
                register_interface::ResultField {
                    name: "userImpact".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssessmentPolicyResult {
            categories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("categories").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            implementation_effort: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("implementationEffort").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            remediation_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remediationDescription").unwrap(),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severity").unwrap(),
            ),
            threats: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threats").unwrap(),
            ),
            user_impact: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userImpact").unwrap(),
            ),
        }
    }
}