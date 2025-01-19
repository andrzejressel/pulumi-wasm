/// A Cloud Security Command Center's (Cloud SCC) finding source. A finding
/// source is an entity or a mechanism that can produce a finding. A source is
/// like a container of findings that come from the same scanner, logger,
/// monitor, etc.
///
///
/// To get more information about Source, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v1/organizations.sources)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs)
///
/// ## Example Usage
///
/// ### Scc Source Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customSource = source::create(
///         "customSource",
///         SourceArgs::builder()
///             .description("My custom Cloud Security Command Center Finding Source")
///             .display_name("My Source")
///             .organization("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Source can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/sources/{{name}}`
///
/// * `{{organization}}/{{name}}`
///
/// When using the `pulumi import` command, Source can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/sourceIamBinding:SourceIamBinding default organizations/{{organization}}/sources/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/sourceIamBinding:SourceIamBinding default {{organization}}/{{name}}
/// ```
///
pub mod source_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::SourceIamBindingCondition>,
        >,
        #[builder(into)]
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The organization whose Cloud Security Command Center the Source
        /// lives in.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SourceIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::SourceIamBindingCondition>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The organization whose Cloud Security Command Center the Source
        /// lives in.
        ///
        ///
        /// - - -
        pub organization: pulumi_wasm_rust::Output<String>,
        pub role: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SourceIamBindingArgs) -> SourceIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let members_binding = args.members.get_inner();
        let organization_binding = args.organization.get_inner();
        let role_binding = args.role.get_inner();
        let source_binding = args.source.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/sourceIamBinding:SourceIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SourceIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
        }
    }
}
