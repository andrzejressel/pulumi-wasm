/// Contains the data that describes an Identity Aware Proxy owned client.
///
/// > **Note:** Only internal org clients can be created via declarative tools. External clients must be
/// manually created via the GCP console. This restriction is due to the existing APIs and not lack of support
/// in this tool.
///
///
/// To get more information about Client, see:
///
/// * [API documentation](https://cloud.google.com/iap/docs/reference/rest/v1/projects.brands.identityAwareProxyClients)
/// * How-to Guides
///     * [Setting up IAP Client](https://cloud.google.com/iap/docs/authentication-howto)
///
///
///
/// ## Example Usage
///
/// ### Iap Client
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
///     let projectClient = client::create(
///         "projectClient",
///         ClientArgs::builder()
///             .brand("${projectBrand.name}")
///             .display_name("Test Client")
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
/// Client can be imported using any of these accepted formats:
///
/// * `{{brand}}/identityAwareProxyClients/{{client_id}}`
///
/// * `{{brand}}/{{client_id}}`
///
/// When using the `pulumi import` command, Client can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iap/client:Client default {{brand}}/identityAwareProxyClients/{{client_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/client:Client default {{brand}}/{{client_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod client {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClientArgs {
        /// Identifier of the brand to which this client
        /// is attached to. The format is
        /// `projects/{project_number}/brands/{brand_id}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub brand: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Human-friendly name given to the OAuth client.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClientResult {
        /// Identifier of the brand to which this client
        /// is attached to. The format is
        /// `projects/{project_number}/brands/{brand_id}`.
        ///
        ///
        /// - - -
        pub brand: pulumi_gestalt_rust::Output<String>,
        /// Output only. Unique identifier of the OAuth client.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// Human-friendly name given to the OAuth client.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Output only. Client secret of the OAuth client.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub secret: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClientArgs,
    ) -> ClientResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let brand_binding = args.brand.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iap/client:Client".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "brand".into(),
                    value: brand_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClientResult {
            brand: o.get_field("brand"),
            client_id: o.get_field("clientId"),
            display_name: o.get_field("displayName"),
            secret: o.get_field("secret"),
        }
    }
}
