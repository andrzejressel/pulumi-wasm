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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlossaryArgs {
        /// Description of the glossary. Must have a length between 0 and 4096.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub domain_identifier: pulumi_wasm_rust::Output<String>,
        /// Name of the glossary. Must have length between 1 and 256.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the project that owns business glossary. Must follow regex of ^[a-zA-Z0-9_-]{1,36}$.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub owning_project_identifier: pulumi_wasm_rust::Output<String>,
        /// Status of business glossary. Valid values are DISABLED and ENABLED.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GlossaryResult {
        /// Description of the glossary. Must have a length between 0 and 4096.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub domain_identifier: pulumi_wasm_rust::Output<String>,
        /// Name of the glossary. Must have length between 1 and 256.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the project that owns business glossary. Must follow regex of ^[a-zA-Z0-9_-]{1,36}$.
        ///
        /// The following arguments are optional:
        pub owning_project_identifier: pulumi_wasm_rust::Output<String>,
        /// Status of business glossary. Valid values are DISABLED and ENABLED.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GlossaryArgs) -> GlossaryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let domain_identifier_binding = args.domain_identifier.get_inner();
        let name_binding = args.name.get_inner();
        let owning_project_identifier_binding = args
            .owning_project_identifier
            .get_inner();
        let status_binding = args.status.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domainIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owningProjectIdentifier".into(),
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
        GlossaryResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainIdentifier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owning_project_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owningProjectIdentifier").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
