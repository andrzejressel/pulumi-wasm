/// Billing information for a project.
///
///
/// To get more information about ProjectInfo, see:
///
/// * [API documentation](https://cloud.google.com/billing/docs/reference/rest/v1/projects)
/// * How-to Guides
///     * [Enable, disable, or change billing for a project](https://cloud.google.com/billing/docs/how-to/modify-project)
///
/// ## Example Usage
///
/// ### Billing Project Info Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = project_info::create(
///         "default",
///         ProjectInfoArgs::builder()
///             .billing_account("000000-0000000-0000000-000000")
///             .project("${project.projectId}")
///             .build_struct(),
///     );
///     let project = project::create(
///         "project",
///         ProjectArgs::builder()
///             .deletion_policy("DELETE")
///             .name("tf-test_88717")
///             .org_id("123456789")
///             .project_id("tf-test_81126")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ProjectInfo can be imported using any of these accepted formats:
///
/// * `projects/{{project}}`
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, ProjectInfo can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:billing/projectInfo:ProjectInfo default projects/{{project}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:billing/projectInfo:ProjectInfo default {{project}}
/// ```
///
pub mod project_info {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectInfoArgs {
        /// The ID of the billing account associated with the project, if
        /// any. Set to empty string to disable billing for the project.
        /// For example, `"012345-567890-ABCDEF"` or `""`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub billing_account: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProjectInfoResult {
        /// The ID of the billing account associated with the project, if
        /// any. Set to empty string to disable billing for the project.
        /// For example, `"012345-567890-ABCDEF"` or `""`.
        ///
        ///
        /// - - -
        pub billing_account: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProjectInfoArgs,
    ) -> ProjectInfoResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:billing/projectInfo:ProjectInfo".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "billingAccount".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectInfoResult {
            billing_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingAccount").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
