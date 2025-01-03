/// Resource for managing an AWS Audit Manager Assessment Report.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = assessment_report::create(
///         "test",
///         AssessmentReportArgs::builder()
///             .assessment_id("${testAwsAuditmanagerAssessment.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Assessment Reports using the assessment report `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/assessmentReport:AssessmentReport example abc123-de45
/// ```
pub mod assessment_report {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentReportArgs {
        /// Unique identifier of the assessment to create the report from.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assessment_id: pulumi_wasm_rust::Output<String>,
        /// Description of the assessment report.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the assessment report.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentReportResult {
        /// Unique identifier of the assessment to create the report from.
        ///
        /// The following arguments are optional:
        pub assessment_id: pulumi_wasm_rust::Output<String>,
        /// Name of the user who created the assessment report.
        pub author: pulumi_wasm_rust::Output<String>,
        /// Description of the assessment report.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the assessment report.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Current status of the specified assessment report. Valid values are `COMPLETE`, `IN_PROGRESS`, and `FAILED`.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssessmentReportArgs) -> AssessmentReportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assessment_id_binding = args.assessment_id.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/assessmentReport:AssessmentReport".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assessmentId".into(),
                    value: &assessment_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assessmentId".into(),
                },
                register_interface::ResultField {
                    name: "author".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssessmentReportResult {
            assessment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assessmentId").unwrap(),
            ),
            author: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("author").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
