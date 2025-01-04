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
/// $ pulumi import gcp:securitycenter/sourceIamMember:SourceIamMember default organizations/{{organization}}/sources/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/sourceIamMember:SourceIamMember default {{organization}}/{{name}}
/// ```
///
pub mod source_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::SourceIamMemberCondition>,
        >,
        #[builder(into)]
        pub member: pulumi_wasm_rust::Output<String>,
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
    pub struct SourceIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::securitycenter::SourceIamMemberCondition>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub member: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: SourceIamMemberArgs) -> SourceIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let member_binding = args.member.get_inner();
        let organization_binding = args.organization.get_inner();
        let role_binding = args.role.get_inner();
        let source_binding = args.source.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/sourceIamMember:SourceIamMember".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "member".into(),
                    value: &member_binding,
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
                    name: "member".into(),
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
        SourceIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
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
