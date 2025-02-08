/// Resource for managing an AWS Audit Manager Assessment.
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
///     let test = assessment::create(
///         "test",
///         AssessmentArgs::builder()
///             .assessment_reports_destination(
///                 AssessmentAssessmentReportsDestination::builder()
///                     .destination("s3://${testAwsS3Bucket.id}")
///                     .destinationType("S3")
///                     .build_struct(),
///             )
///             .framework_id("${testAwsAuditmanagerFramework.id}")
///             .name("example")
///             .roles(
///                 vec![
///                     AssessmentRole::builder().roleArn("${testAwsIamRole.arn}")
///                     .roleType("PROCESS_OWNER").build_struct(),
///                 ],
///             )
///             .scope(
///                 AssessmentScope::builder()
///                     .awsAccounts(
///                         vec![
///                             AssessmentScopeAwsAccount::builder()
///                             .id("${current.accountId}").build_struct(),
///                         ],
///                     )
///                     .awsServices(
///                         vec![
///                             AssessmentScopeAwsService::builder().serviceName("S3")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Assessments using the assessment `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/assessment:Assessment example abc123-de45
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assessment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentArgs {
        /// Assessment report storage destination configuration. See `assessment_reports_destination` below.
        #[builder(into, default)]
        pub assessment_reports_destination: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::auditmanager::AssessmentAssessmentReportsDestination,
            >,
        >,
        /// Description of the assessment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier of the framework the assessment will be created from.
        #[builder(into)]
        pub framework_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the assessment.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of roles for the assessment. See `roles` below.
        #[builder(into)]
        pub roles: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::auditmanager::AssessmentRole>,
        >,
        /// Amazon Web Services accounts and services that are in scope for the assessment. See `scope` below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::auditmanager::AssessmentScope>,
        >,
        /// A map of tags to assign to the assessment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AssessmentResult {
        /// Amazon Resource Name (ARN) of the assessment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Assessment report storage destination configuration. See `assessment_reports_destination` below.
        pub assessment_reports_destination: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::auditmanager::AssessmentAssessmentReportsDestination,
            >,
        >,
        /// Description of the assessment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier of the framework the assessment will be created from.
        pub framework_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the assessment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of roles for the assessment. See `roles` below.
        pub roles: pulumi_gestalt_rust::Output<
            Vec<super::super::types::auditmanager::AssessmentRole>,
        >,
        /// Complete list of all roles with access to the assessment. This includes both roles explicitly configured via the `roles` block, and any roles which have access to all Audit Manager assessments by default.
        pub roles_alls: pulumi_gestalt_rust::Output<
            Vec<super::super::types::auditmanager::AssessmentRolesAll>,
        >,
        /// Amazon Web Services accounts and services that are in scope for the assessment. See `scope` below.
        ///
        /// The following arguments are optional:
        pub scope: pulumi_gestalt_rust::Output<
            Option<super::super::types::auditmanager::AssessmentScope>,
        >,
        /// Status of the assessment. Valid values are `ACTIVE` and `INACTIVE`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the assessment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AssessmentArgs,
    ) -> AssessmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let assessment_reports_destination_binding = args
            .assessment_reports_destination
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let framework_id_binding = args.framework_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let roles_binding = args.roles.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/assessment:Assessment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assessmentReportsDestination".into(),
                    value: &assessment_reports_destination_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "frameworkId".into(),
                    value: &framework_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roles".into(),
                    value: &roles_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AssessmentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            assessment_reports_destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assessmentReportsDestination"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            framework_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frameworkId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            roles: pulumi_gestalt_rust::__private::into_domain(o.extract_field("roles")),
            roles_alls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rolesAlls"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
