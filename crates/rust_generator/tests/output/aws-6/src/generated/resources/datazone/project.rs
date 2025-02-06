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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let domain_identifier_binding = args
            .domain_identifier
            .get_output(context)
            .get_inner();
        let glossary_terms_binding = args.glossary_terms.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let skip_deletion_check_binding = args
            .skip_deletion_check
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "glossaryTerms".into(),
                    value: &glossary_terms_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "skipDeletionCheck".into(),
                    value: &skip_deletion_check_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectResult {
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            created_by: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdBy"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            domain_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainIdentifier"),
            ),
            failure_reasons: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failureReasons"),
            ),
            glossary_terms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("glossaryTerms"),
            ),
            last_updated_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedAt"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectStatus"),
            ),
            skip_deletion_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipDeletionCheck"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
