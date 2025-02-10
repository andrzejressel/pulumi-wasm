/// Resource for managing an AWS DataZone Project.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:datazone:Project
///     properties:
///       domainId: ${testAwsDatazoneDomain.id}
///       glossaryTerms:
///         - 2N8w6XJCwZf
///       name: name
///       description: desc
///       skipDeletionCheck: true
/// ```
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = project::create(
///         "test",
///         ProjectArgs::builder()
///             .domain_identifier("${testAwsDatazoneDomain.id}")
///             .name("name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Project using a colon-delimited string combining `domain_id` and `id`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/project:Project example domain-1234:project-1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// Description of project.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of domain which the project is part of. Must follow the regex of `^dzd[-_][a-zA-Z0-9_-]{1,36}$`.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of glossary terms that can be used in the project. The list cannot be empty or include over 20 values. Each value must follow the regex of `[a-zA-Z0-9_-]{1,36}$`.
        #[builder(into, default)]
        pub glossary_terms: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the project. Must follow the regex of `^[\w -]+$`. and have a length of at most 64.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional flag to delete all child entities within the project.
        #[builder(into, default)]
        pub skip_deletion_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::ProjectTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// Timestamp of when the project was made.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Creator of the project.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// Description of project.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of domain which the project is part of. Must follow the regex of `^dzd[-_][a-zA-Z0-9_-]{1,36}$`.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// List of error messages if operation cannot be completed.
        pub failure_reasons: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datazone::ProjectFailureReason>,
        >,
        /// List of glossary terms that can be used in the project. The list cannot be empty or include over 20 values. Each value must follow the regex of `[a-zA-Z0-9_-]{1,36}$`.
        pub glossary_terms: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Timestamp of when the project was last updated.
        pub last_updated_at: pulumi_gestalt_rust::Output<String>,
        /// Name of the project. Must follow the regex of `^[\w -]+$`. and have a length of at most 64.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Enum that conveys state of project. Can be `ACTIVE`, `DELETING`, or `DELETE_FAILED`.
        pub project_status: pulumi_gestalt_rust::Output<String>,
        /// Optional flag to delete all child entities within the project.
        pub skip_deletion_check: pulumi_gestalt_rust::Output<Option<bool>>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::ProjectTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let domain_identifier_binding = args.domain_identifier.get_output(context);
        let glossary_terms_binding = args.glossary_terms.get_output(context);
        let name_binding = args.name.get_output(context);
        let skip_deletion_check_binding = args.skip_deletion_check.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: domain_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "glossaryTerms".into(),
                    value: glossary_terms_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipDeletionCheck".into(),
                    value: skip_deletion_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            created_at: o.get_field("createdAt"),
            created_by: o.get_field("createdBy"),
            description: o.get_field("description"),
            domain_identifier: o.get_field("domainIdentifier"),
            failure_reasons: o.get_field("failureReasons"),
            glossary_terms: o.get_field("glossaryTerms"),
            last_updated_at: o.get_field("lastUpdatedAt"),
            name: o.get_field("name"),
            project_status: o.get_field("projectStatus"),
            skip_deletion_check: o.get_field("skipDeletionCheck"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
