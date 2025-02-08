/// ## Example Usage
///
/// ### Firebasehosting Version Redirect
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
///             .site_id("site-id")
///             .build_struct(),
///     );
///     let defaultHostingRelease = hosting_release::create(
///         "defaultHostingRelease",
///         HostingReleaseArgs::builder()
///             .message("Redirect to Google")
///             .site_id("${default.siteId}")
///             .version_name("${defaultHostingVersion.name}")
///             .build_struct(),
///     );
///     let defaultHostingVersion = hosting_version::create(
///         "defaultHostingVersion",
///         HostingVersionArgs::builder()
///             .config(
///                 HostingVersionConfig::builder()
///                     .redirects(
///                         vec![
///                             HostingVersionConfigRedirect::builder().glob("/google/**")
///                             .location("https://www.google.com").statusCode(302)
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .site_id("${default.siteId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Version Headers
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:HostingSite
///     properties:
///       project: my-project-name
///       siteId: site-id
///   defaultHostingVersion:
///     type: gcp:firebase:HostingVersion
///     name: default
///     properties:
///       siteId: ${default.siteId}
///       config:
///         headers:
///           - glob: /headers/**
///             headers:
///               my-header: my-value
///   defaultHostingRelease:
///     type: gcp:firebase:HostingRelease
///     name: default
///     properties:
///       siteId: ${default.siteId}
///       versionName: ${defaultHostingVersion.name}
///       message: With custom headers
/// ```
/// ### Firebasehosting Version Headers Regex
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:HostingSite
///     properties:
///       project: my-project-name
///       siteId: site-id
///   defaultHostingVersion:
///     type: gcp:firebase:HostingVersion
///     name: default
///     properties:
///       siteId: ${default.siteId}
///       config:
///         headers:
///           - regex: ^~/headers$
///             headers:
///               my-header: my-value
///   defaultHostingRelease:
///     type: gcp:firebase:HostingRelease
///     name: default
///     properties:
///       siteId: ${default.siteId}
///       versionName: ${defaultHostingVersion.name}
///       message: With custom headers
/// ```
/// ### Firebasehosting Version Path
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
///             .site_id("site-id")
///             .build_struct(),
///     );
///     let defaultHostingRelease = hosting_release::create(
///         "defaultHostingRelease",
///         HostingReleaseArgs::builder()
///             .message("Path Rewrite")
///             .site_id("${default.siteId}")
///             .version_name("${defaultHostingVersion.name}")
///             .build_struct(),
///     );
///     let defaultHostingVersion = hosting_version::create(
///         "defaultHostingVersion",
///         HostingVersionArgs::builder()
///             .config(
///                 HostingVersionConfig::builder()
///                     .rewrites(
///                         vec![
///                             HostingVersionConfigRewrite::builder().glob("**")
///                             .path("/index.html").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .site_id("${default.siteId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Version Cloud Run
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
///             .site_id("site-id")
///             .build_struct(),
///     );
///     let defaultHostingRelease = hosting_release::create(
///         "defaultHostingRelease",
///         HostingReleaseArgs::builder()
///             .message("Cloud Run Integration")
///             .site_id("${default.siteId}")
///             .version_name("${defaultHostingVersion.name}")
///             .build_struct(),
///     );
///     let defaultHostingVersion = hosting_version::create(
///         "defaultHostingVersion",
///         HostingVersionArgs::builder()
///             .config(
///                 HostingVersionConfig::builder()
///                     .rewrites(
///                         vec![
///                             HostingVersionConfigRewrite::builder().glob("/hello/**")
///                             .run(HostingVersionConfigRewriteRun::builder()
///                             .region("${defaultService.location}")
///                             .serviceId("${defaultService.name}").build_struct())
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .site_id("${default.siteId}")
///             .build_struct(),
///     );
///     let defaultService = service::create(
///         "defaultService",
///         ServiceArgs::builder()
///             .deletion_protection(true)
///             .ingress("INGRESS_TRAFFIC_ALL")
///             .location("us-central1")
///             .name("cloud-run-service-via-hosting")
///             .project("my-project-name")
///             .template(
///                 ServiceTemplate::builder()
///                     .containers(
///                         vec![
///                             ServiceTemplateContainer::builder()
///                             .image("us-docker.pkg.dev/cloudrun/container/hello")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Version Cloud Functions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:HostingSite
///     properties:
///       project: my-project-name
///       siteId: site-id
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       project: my-project-name
///       name: site-id-function-source
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: function-source.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: function-source.zip
///   function:
///     type: gcp:cloudfunctions:Function
///     properties:
///       project: my-project-name
///       name: cloud-function-via-hosting
///       description: A Cloud Function connected to Firebase Hosing
///       runtime: nodejs16
///       availableMemoryMb: 128
///       sourceArchiveBucket: ${bucket.name}
///       sourceArchiveObject: ${object.name}
///       triggerHttp: true
///       entryPoint: helloHttp
///   defaultHostingVersion:
///     type: gcp:firebase:HostingVersion
///     name: default
///     properties:
///       siteId: ${default.siteId}
///       config:
///         rewrites:
///           - glob: /hello/**
///             function: ${function.name}
///   defaultHostingRelease:
///     type: gcp:firebase:HostingRelease
///     name: default
///     properties:
///       siteId: ${default.siteId}
///       versionName: ${defaultHostingVersion.name}
///       message: Cloud Functions Integration
/// ```
///
/// ## Import
///
/// Version can be imported using any of these accepted formats:
///
/// * `sites/{{site_id}}/versions/{{version_id}}`
///
/// * `{{site_id}}/{{version_id}}`
///
/// When using the `pulumi import` command, Version can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingVersion:HostingVersion default sites/{{site_id}}/versions/{{version_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingVersion:HostingVersion default {{site_id}}/{{version_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod hosting_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingVersionArgs {
        /// The configuration for the behavior of the site. This configuration exists in the `firebase.json` file.
        /// Structure is documented below.
        #[builder(into, default)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::firebase::HostingVersionConfig>,
        >,
        /// Required. The ID of the site in which to create this Version.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HostingVersionResult {
        /// The configuration for the behavior of the site. This configuration exists in the `firebase.json` file.
        /// Structure is documented below.
        pub config: pulumi_gestalt_rust::Output<
            Option<super::super::types::firebase::HostingVersionConfig>,
        >,
        /// The fully-qualified resource name for the version, in the format:
        /// sites/SITE_ID/versions/VERSION_ID
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. The ID of the site in which to create this Version.
        ///
        ///
        /// - - -
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// The ID for the version as in sites/SITE_ID/versions/VERSION_ID
        pub version_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HostingVersionArgs,
    ) -> HostingVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let config_binding = args.config.get_output(context).get_inner();
        let site_id_binding = args.site_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/hostingVersion:HostingVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostingVersionResult {
            config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("config"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            site_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("siteId"),
            ),
            version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
        }
    }
}
