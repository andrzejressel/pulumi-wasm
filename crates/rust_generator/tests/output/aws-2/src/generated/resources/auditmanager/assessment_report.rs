/// Resource for managing an AWS Audit Manager Assessment Report.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assessment_report {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentReportArgs {
        /// Unique identifier of the assessment to create the report from.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub assessment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the assessment report.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the assessment report.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentReportResult {
        /// Unique identifier of the assessment to create the report from.
        ///
        /// The following arguments are optional:
        pub assessment_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the user who created the assessment report.
        pub author: pulumi_gestalt_rust::Output<String>,
        /// Description of the assessment report.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the assessment report.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Current status of the specified assessment report. Valid values are `COMPLETE`, `IN_PROGRESS`, and `FAILED`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssessmentReportArgs,
    ) -> AssessmentReportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assessment_id_binding = args.assessment_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/assessmentReport:AssessmentReport".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assessmentId".into(),
                    value: assessment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssessmentReportResult {
            assessment_id: o.get_field("assessmentId"),
            author: o.get_field("author"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            status: o.get_field("status"),
        }
    }
}
