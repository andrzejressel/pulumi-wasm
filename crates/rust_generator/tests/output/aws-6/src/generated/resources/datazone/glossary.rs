/// Resource for managing an AWS DataZone Glossary.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   domainExecutionRole:
///     type: aws:iam:Role
///     name: domain_execution_role
///     properties:
///       name: example_name
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: datazone.amazonaws.com
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: cloudformation.amazonaws.com
///       inlinePolicies:
///         - name: example_name
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - datazone:*
///                     - ram:*
///                     - sso:*
///                     - kms:*
///                   Effect: Allow
///                   Resource: '*'
///   test:
///     type: aws:datazone:Domain
///     properties:
///       name: example_name
///       domainExecutionRole: ${domainExecutionRole.arn}
///   testSecurityGroup:
///     type: aws:ec2:SecurityGroup
///     name: test
///     properties:
///       name: example_name
///   testProject:
///     type: aws:datazone:Project
///     name: test
///     properties:
///       domainIdentifier: ${test.id}
///       glossaryTerms:
///         - 2N8w6XJCwZf
///       name: example_name
///       description: desc
///       skipDeletionCheck: true
///   testGlossary:
///     type: aws:datazone:Glossary
///     name: test
///     properties:
///       description: description
///       name: example_name
///       owningProjectIdentifier: ${testProject.id}
///       status: DISABLED
///       domainIdentifier: ${testProject.domainIdentifier}
/// ```
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = glossary::create(
///         "test",
///         GlossaryArgs::builder()
///             .description("description")
///             .domain_identifier("${testAwsDatazoneProject.domainIdentifier}")
///             .name("example_name")
///             .owning_project_identifier("${testAwsDatazoneProject.id}")
///             .status("DISABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Glossary using the import Datazone Glossary using a comma-delimited string combining the domain id, glossary id, and the id of the project it's under. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/glossary:Glossary example domain-id,glossary-id,owning-project-identifier
/// ```
pub mod glossary {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlossaryArgs {
        /// Description of the glossary. Must have a length between 0 and 4096.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the glossary. Must have length between 1 and 256.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the project that owns business glossary. Must follow regex of ^[a-zA-Z0-9_-]{1,36}$.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub owning_project_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Status of business glossary. Valid values are DISABLED and ENABLED.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GlossaryResult {
        /// Description of the glossary. Must have a length between 0 and 4096.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// Name of the glossary. Must have length between 1 and 256.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the project that owns business glossary. Must follow regex of ^[a-zA-Z0-9_-]{1,36}$.
        ///
        /// The following arguments are optional:
        pub owning_project_identifier: pulumi_gestalt_rust::Output<String>,
        /// Status of business glossary. Valid values are DISABLED and ENABLED.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GlossaryArgs,
    ) -> GlossaryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let domain_identifier_binding = args
            .domain_identifier
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let owning_project_identifier_binding = args
            .owning_project_identifier
            .get_output(context)
            .get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/glossary:Glossary".into(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "owningProjectIdentifier".into(),
                    value: &owning_project_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlossaryResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            domain_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainIdentifier"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owning_project_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("owningProjectIdentifier"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
