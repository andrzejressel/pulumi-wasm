/// Resource for managing an AWS DataZone Glossary Term.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: example
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
///         - name: example
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
///   exampleDomain:
///     type: aws:datazone:Domain
///     name: example
///     properties:
///       name: example_name
///       domainExecutionRole: ${example.arn}
///   exampleSecurityGroup:
///     type: aws:ec2:SecurityGroup
///     name: example
///     properties:
///       name: example_name
///   exampleProject:
///     type: aws:datazone:Project
///     name: example
///     properties:
///       domainIdentifier: ${exampleDomain.id}
///       glossaryTerms:
///         - 2N8w6XJCwZf
///       name: example
///       skipDeletionCheck: true
///   exampleGlossary:
///     type: aws:datazone:Glossary
///     name: example
///     properties:
///       description: description
///       name: example
///       owningProjectIdentifier: ${exampleProject.id}
///       status: ENABLED
///       domainIdentifier: ${exampleProject.domainIdentifier}
///   exampleGlossaryTerm:
///     type: aws:datazone:GlossaryTerm
///     name: example
///     properties:
///       domainIdentifier: ${exampleDomain.id}
///       glossaryIdentifier: ${exampleGlossary.id}
///       name: example
///       status: ENABLED
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Glossary Term using a comma-delimited string combining the `domain_identifier`, `id`, and the `glossary_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/glossaryTerm:GlossaryTerm example domain-id,glossary-term-id,glossary-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod glossary_term {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlossaryTermArgs {
        /// Identifier of domain.
        #[builder(into, default)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of glossary.
        #[builder(into)]
        pub glossary_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Long description of entry.
        #[builder(into, default)]
        pub long_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of glossary term.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short description of entry.
        #[builder(into, default)]
        pub short_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If glossary term is ENABLED or DISABLED.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object classifying the term relations through the following attributes:
        #[builder(into, default)]
        pub term_relations: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::GlossaryTermTermRelations>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::GlossaryTermTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlossaryTermResult {
        /// Time of glossary term creation.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Creator of glossary term.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// Identifier of domain.
        pub domain_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of glossary.
        pub glossary_identifier: pulumi_gestalt_rust::Output<String>,
        /// Long description of entry.
        pub long_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of glossary term.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Short description of entry.
        pub short_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If glossary term is ENABLED or DISABLED.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object classifying the term relations through the following attributes:
        pub term_relations: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTermRelations>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GlossaryTermArgs,
    ) -> GlossaryTermResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let domain_identifier_binding_1 = args.domain_identifier.get_output(context);
        let domain_identifier_binding = domain_identifier_binding_1.get_inner();
        let glossary_identifier_binding_1 = args.glossary_identifier.get_output(context);
        let glossary_identifier_binding = glossary_identifier_binding_1.get_inner();
        let long_description_binding_1 = args.long_description.get_output(context);
        let long_description_binding = long_description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let short_description_binding_1 = args.short_description.get_output(context);
        let short_description_binding = short_description_binding_1.get_inner();
        let status_binding_1 = args.status.get_output(context);
        let status_binding = status_binding_1.get_inner();
        let term_relations_binding_1 = args.term_relations.get_output(context);
        let term_relations_binding = term_relations_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/glossaryTerm:GlossaryTerm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "glossaryIdentifier".into(),
                    value: &glossary_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "longDescription".into(),
                    value: &long_description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shortDescription".into(),
                    value: &short_description_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "termRelations".into(),
                    value: &term_relations_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlossaryTermResult {
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            created_by: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdBy"),
            ),
            domain_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainIdentifier"),
            ),
            glossary_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("glossaryIdentifier"),
            ),
            long_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("longDescription"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            short_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shortDescription"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            term_relations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("termRelations"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
