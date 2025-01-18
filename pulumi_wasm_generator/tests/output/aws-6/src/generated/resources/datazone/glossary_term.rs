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
pub mod glossary_term {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlossaryTermArgs {
        /// Identifier of domain.
        #[builder(into, default)]
        pub domain_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of glossary.
        #[builder(into)]
        pub glossary_identifier: pulumi_wasm_rust::Output<String>,
        /// Long description of entry.
        #[builder(into, default)]
        pub long_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of glossary term.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Short description of entry.
        #[builder(into, default)]
        pub short_description: pulumi_wasm_rust::Output<Option<String>>,
        /// If glossary term is ENABLED or DISABLED.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Object classifying the term relations through the following attributes:
        #[builder(into, default)]
        pub term_relations: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTermRelations>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlossaryTermResult {
        /// Time of glossary term creation.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Creator of glossary term.
        pub created_by: pulumi_wasm_rust::Output<String>,
        /// Identifier of domain.
        pub domain_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of glossary.
        pub glossary_identifier: pulumi_wasm_rust::Output<String>,
        /// Long description of entry.
        pub long_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of glossary term.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Short description of entry.
        pub short_description: pulumi_wasm_rust::Output<Option<String>>,
        /// If glossary term is ENABLED or DISABLED.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Object classifying the term relations through the following attributes:
        pub term_relations: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTermRelations>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GlossaryTermArgs) -> GlossaryTermResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_identifier_binding = args.domain_identifier.get_inner();
        let glossary_identifier_binding = args.glossary_identifier.get_inner();
        let long_description_binding = args.long_description.get_inner();
        let name_binding = args.name.get_inner();
        let short_description_binding = args.short_description.get_inner();
        let status_binding = args.status.get_inner();
        let term_relations_binding = args.term_relations.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "createdBy".into(),
                },
                register_interface::ResultField {
                    name: "domainIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "glossaryIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "longDescription".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "shortDescription".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "termRelations".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GlossaryTermResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            created_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdBy").unwrap(),
            ),
            domain_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainIdentifier").unwrap(),
            ),
            glossary_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("glossaryIdentifier").unwrap(),
            ),
            long_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("longDescription").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            short_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortDescription").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            term_relations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("termRelations").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
