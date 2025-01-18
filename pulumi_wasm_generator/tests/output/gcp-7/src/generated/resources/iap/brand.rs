/// ## Example Usage
///
/// ### Iap Brand
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let project = project::create(
///         "project",
///         ProjectArgs::builder()
///             .deletion_policy("DELETE")
///             .name("my-project")
///             .org_id("123456789")
///             .project_id("my-project")
///             .build_struct(),
///     );
///     let projectBrand = brand::create(
///         "projectBrand",
///         BrandArgs::builder()
///             .application_title("Cloud IAP protected Application")
///             .project("${projectService.project}")
///             .support_email("support@example.com")
///             .build_struct(),
///     );
///     let projectService = service::create(
///         "projectService",
///         ServiceArgs::builder()
///             .project("${project.projectId}")
///             .service("iap.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Brand can be imported using any of these accepted formats:
///
/// * `projects/{{project_id}}/brands/{{brand_id}}`
///
/// * `projects/{{project_number}}/brands/{{brand_id}}`
///
/// * `{{project_number}}/{{brand_id}}`
///
/// When using the `pulumi import` command, Brand can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iap/brand:Brand default projects/{{project_id}}/brands/{{brand_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/brand:Brand default projects/{{project_number}}/brands/{{brand_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/brand:Brand default {{project_number}}/{{brand_id}}
/// ```
///
pub mod brand {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BrandArgs {
        /// Application name displayed on OAuth consent screen.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub application_title: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Support email displayed on the OAuth consent screen. Can be either a
        /// user or group email. When a user email is specified, the caller must
        /// be the user with the associated email address. When a group email is
        /// specified, the caller can be either a user or a service account which
        /// is an owner of the specified group in Cloud Identity.
        #[builder(into)]
        pub support_email: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BrandResult {
        /// Application name displayed on OAuth consent screen.
        ///
        ///
        /// - - -
        pub application_title: pulumi_wasm_rust::Output<String>,
        /// Output only. Identifier of the brand, in the format `projects/{project_number}/brands/{brand_id}`
        /// NOTE: The name can also be expressed as `projects/{project_id}/brands/{brand_id}`, e.g. when importing.
        /// NOTE: The brand identification corresponds to the project number as only one
        /// brand can be created per project.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the brand is only intended for usage inside the GSuite organization only.
        pub org_internal_only: pulumi_wasm_rust::Output<bool>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Support email displayed on the OAuth consent screen. Can be either a
        /// user or group email. When a user email is specified, the caller must
        /// be the user with the associated email address. When a group email is
        /// specified, the caller can be either a user or a service account which
        /// is an owner of the specified group in Cloud Identity.
        pub support_email: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BrandArgs) -> BrandResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_title_binding = args.application_title.get_inner();
        let project_binding = args.project.get_inner();
        let support_email_binding = args.support_email.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iap/brand:Brand".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationTitle".into(),
                    value: &application_title_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "supportEmail".into(),
                    value: &support_email_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationTitle".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgInternalOnly".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "supportEmail".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BrandResult {
            application_title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationTitle").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_internal_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgInternalOnly").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            support_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportEmail").unwrap(),
            ),
        }
    }
}
