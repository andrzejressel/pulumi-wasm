/// ## Example Usage
///
/// ### Iap Brand
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod brand {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BrandArgs {
        /// Application name displayed on OAuth consent screen.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub application_title: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Support email displayed on the OAuth consent screen. Can be either a
        /// user or group email. When a user email is specified, the caller must
        /// be the user with the associated email address. When a group email is
        /// specified, the caller can be either a user or a service account which
        /// is an owner of the specified group in Cloud Identity.
        #[builder(into)]
        pub support_email: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BrandResult {
        /// Application name displayed on OAuth consent screen.
        ///
        ///
        /// - - -
        pub application_title: pulumi_gestalt_rust::Output<String>,
        /// Output only. Identifier of the brand, in the format `projects/{project_number}/brands/{brand_id}`
        /// NOTE: The name can also be expressed as `projects/{project_id}/brands/{brand_id}`, e.g. when importing.
        /// NOTE: The brand identification corresponds to the project number as only one
        /// brand can be created per project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the brand is only intended for usage inside the GSuite organization only.
        pub org_internal_only: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Support email displayed on the OAuth consent screen. Can be either a
        /// user or group email. When a user email is specified, the caller must
        /// be the user with the associated email address. When a group email is
        /// specified, the caller can be either a user or a service account which
        /// is an owner of the specified group in Cloud Identity.
        pub support_email: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BrandArgs,
    ) -> BrandResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_title_binding = args.application_title.get_output(context);
        let project_binding = args.project.get_output(context);
        let support_email_binding = args.support_email.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iap/brand:Brand".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationTitle".into(),
                    value: &application_title_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportEmail".into(),
                    value: &support_email_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BrandResult {
            application_title: o.get_field("applicationTitle"),
            name: o.get_field("name"),
            org_internal_only: o.get_field("orgInternalOnly"),
            project: o.get_field("project"),
            support_email: o.get_field("supportEmail"),
        }
    }
}
