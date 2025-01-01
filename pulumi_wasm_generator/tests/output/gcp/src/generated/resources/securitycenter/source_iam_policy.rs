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
/// $ pulumi import gcp:securitycenter/sourceIamPolicy:SourceIamPolicy default organizations/{{organization}}/sources/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/sourceIamPolicy:SourceIamPolicy default {{organization}}/{{name}}
/// ```
///
pub mod source_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceIamPolicyArgs {
        /// The organization whose Cloud Security Command Center the Source
        /// lives in.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub organization: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub policy_data: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SourceIamPolicyResult {
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The organization whose Cloud Security Command Center the Source
        /// lives in.
        ///
        ///
        /// - - -
        pub organization: pulumi_wasm_rust::Output<String>,
        pub policy_data: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SourceIamPolicyArgs) -> SourceIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let organization_binding = args.organization.get_inner();
        let policy_data_binding = args.policy_data.get_inner();
        let source_binding = args.source.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/sourceIamPolicy:SourceIamPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
                register_interface::ObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
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
        SourceIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
        }
    }
}
