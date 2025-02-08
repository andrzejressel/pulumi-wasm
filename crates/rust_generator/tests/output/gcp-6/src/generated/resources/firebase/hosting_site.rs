/// ## Example Usage
///
/// ### Firebasehosting Site Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = hosting_site::create(
///         "default",
///         HostingSiteArgs::builder()
///             .project("my-project-name")
///             .site_id("site-no-app")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Site Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = web_app::create(
///         "default",
///         WebAppArgs::builder()
///             .display_name("Test web app for Firebase Hosting")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let full = hosting_site::create(
///         "full",
///         HostingSiteArgs::builder()
///             .app_id("${default.appId}")
///             .project("my-project-name")
///             .site_id("site-with-app")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Site can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/sites/{{site_id}}`
///
/// * `{{project}}/{{site_id}}`
///
/// * `sites/{{site_id}}`
///
/// * `{{site_id}}`
///
/// When using the `pulumi import` command, Site can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingSite:HostingSite default projects/{{project}}/sites/{{site_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingSite:HostingSite default {{project}}/{{site_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingSite:HostingSite default sites/{{site_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingSite:HostingSite default {{site_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod hosting_site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingSiteArgs {
        /// Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id)
        /// associated with the Hosting site.
        #[builder(into, default)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Immutable. A globally unique identifier for the Hosting site. This identifier is
        /// used to construct the Firebase-provisioned subdomains for the site, so it must also be a valid
        /// domain name label.
        #[builder(into, default)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostingSiteResult {
        /// Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id)
        /// associated with the Hosting site.
        pub app_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default URL for the site in the form of https://{name}.web.app
        pub default_url: pulumi_gestalt_rust::Output<String>,
        /// Output only. The fully-qualified resource name of the Hosting site, in
        /// the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID PROJECT_IDENTIFIER: the
        /// Firebase project's
        /// [`ProjectNumber`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its
        /// [`ProjectId`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_id).
        /// Learn more about using project identifiers in Google's
        /// [AIP 2510 standard](https://google.aip.dev/cloud/2510).
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. A globally unique identifier for the Hosting site. This identifier is
        /// used to construct the Firebase-provisioned subdomains for the site, so it must also be a valid
        /// domain name label.
        pub site_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HostingSiteArgs,
    ) -> HostingSiteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_id_binding = args.app_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let site_id_binding = args.site_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/hostingSite:HostingSite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appId".into(),
                    value: &app_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostingSiteResult {
            app_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appId"),
            ),
            default_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultUrl"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            site_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteId"),
            ),
        }
    }
}
