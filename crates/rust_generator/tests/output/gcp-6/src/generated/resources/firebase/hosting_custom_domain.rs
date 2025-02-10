/// ## Example Usage
///
/// ### Firebasehosting Customdomain Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = hosting_custom_domain::create(
///         "default",
///         HostingCustomDomainArgs::builder()
///             .custom_domain("custom.domain.com")
///             .project("my-project-name")
///             .site_id("site-id")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Customdomain Full
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
///             .site_id("site-id-full")
///             .build_struct(),
///     );
///     let defaultHostingCustomDomain = hosting_custom_domain::create(
///         "defaultHostingCustomDomain",
///         HostingCustomDomainArgs::builder()
///             .cert_preference("GROUPED")
///             .custom_domain("source.domain.com")
///             .project("my-project-name")
///             .redirect_target("destination.domain.com")
///             .site_id("${default.siteId}")
///             .wait_dns_verification(false)
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Customdomain Cloud Run
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
///     let defaultHostingCustomDomain = hosting_custom_domain::create(
///         "defaultHostingCustomDomain",
///         HostingCustomDomainArgs::builder()
///             .custom_domain("run.custom.domain.com")
///             .project("my-project-name")
///             .site_id("${default.siteId}")
///             .wait_dns_verification(false)
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
///
/// ## Import
///
/// CustomDomain can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/sites/{{site_id}}/customDomains/{{custom_domain}}`
///
/// * `sites/{{site_id}}/customDomains/{{custom_domain}}`
///
/// * `{{project}}/{{site_id}}/{{custom_domain}}`
///
/// * `{{site_id}}/{{custom_domain}}`
///
/// When using the `pulumi import` command, CustomDomain can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingCustomDomain:HostingCustomDomain default projects/{{project}}/sites/{{site_id}}/customDomains/{{custom_domain}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingCustomDomain:HostingCustomDomain default sites/{{site_id}}/customDomains/{{custom_domain}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingCustomDomain:HostingCustomDomain default {{project}}/{{site_id}}/{{custom_domain}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingCustomDomain:HostingCustomDomain default {{site_id}}/{{custom_domain}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosting_custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingCustomDomainArgs {
        /// A field that lets you specify which SSL certificate type Hosting creates
        /// for your domain name. Spark plan `CustomDomain`s only have access to the
        /// `GROUPED` cert type, while Blaze plan can select any option.
        /// Possible values are: `GROUPED`, `PROJECT_GROUPED`, `DEDICATED`.
        #[builder(into, default)]
        pub cert_preference: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the `CustomDomain`, which is the domain name you'd like to use with Firebase Hosting.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub custom_domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A domain name that this CustomDomain should direct traffic towards. If
        /// specified, Hosting will respond to requests against this CustomDomain
        /// with an HTTP 301 code, and route traffic to the specified `redirect_target`
        /// instead.
        #[builder(into, default)]
        pub redirect_target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the site in which to create this custom domain association.
        #[builder(into)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub wait_dns_verification: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct HostingCustomDomainResult {
        /// A field that lets you specify which SSL certificate type Hosting creates
        /// for your domain name. Spark plan `CustomDomain`s only have access to the
        /// `GROUPED` cert type, while Blaze plan can select any option.
        /// Possible values are: `GROUPED`, `PROJECT_GROUPED`, `DEDICATED`.
        pub cert_preference: pulumi_gestalt_rust::Output<String>,
        /// The SSL certificate Hosting has for this `CustomDomain`'s domain name.
        /// For new `CustomDomain`s, this often represents Hosting's intent to create
        /// a certificate, rather than an actual cert. Check the `state` field for
        /// more.
        /// Structure is documented below.
        pub certs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::firebase::HostingCustomDomainCert>,
        >,
        /// The `CustomDomain`'s create time.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The ID of the `CustomDomain`, which is the domain name you'd like to use with Firebase Hosting.
        ///
        ///
        /// - - -
        pub custom_domain: pulumi_gestalt_rust::Output<String>,
        /// The time the `CustomDomain` was deleted; null for `CustomDomains` that
        /// haven't been deleted. Deleted `CustomDomains` persist for approximately 30
        /// days, after which time Hosting removes them completely.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// A string that represents the current state of the `CustomDomain` and
        /// allows you to confirm its initial state in requests that would modify it.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The minimum time before a soft-deleted `CustomDomain` is completely removed
        /// from Hosting; null for `CustomDomains` that haven't been deleted.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// The host state of your domain name. Host state is determined by checking each
        /// IP address associated with your domain name to see if it's serving
        /// Hosting content.
        /// HOST_UNHOSTED:
        /// Your `CustomDomain`'s domain name isn't associated with any IP addresses.
        /// HOST_UNREACHABLE:
        /// Your `CustomDomain`'s domain name can't be reached. Hosting services' DNS
        /// queries to find your domain name's IP addresses resulted in errors. See
        /// your `CustomDomain`'s `issues` field for more details.
        /// HOST_MISMATCH:
        /// Your `CustomDomain`'s domain name has IP addresses that don't ultimately
        /// resolve to Hosting.
        /// HOST_CONFLICT:
        /// Your `CustomDomain`'s domain name has IP addresses that resolve to both
        /// Hosting and other services. To ensure consistent results, remove `A` and
        /// `AAAA` records related to non-Hosting services.
        /// HOST_ACTIVE:
        /// All requests against your `CustomDomain`'s domain name are served by
        /// Hosting. If the `CustomDomain`'s `OwnershipState` is also `ACTIVE`, Hosting
        /// serves your Hosting Site's content on the domain name.
        pub host_state: pulumi_gestalt_rust::Output<String>,
        /// A set of errors Hosting systems encountered when trying to establish
        /// Hosting's ability to serve secure content for your domain name. Resolve
        /// these issues to ensure your `CustomDomain` behaves properly.
        /// Structure is documented below.
        pub issues: pulumi_gestalt_rust::Output<
            Vec<super::super::types::firebase::HostingCustomDomainIssue>,
        >,
        /// The fully-qualified name of the `CustomDomain`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ownership state of your domain name. Ownership is determined at a
        /// Firebase project level, and established by adding `TXT` records to your
        /// domain name's DNS records.
        /// Ownership cascades to subdomains. Granting a project ownership of `foo.com`
        /// also grants that project ownership over `bar.foo.com`, unless you add
        /// specific `TXT` records to `bar.foo.com` that grant a different project
        /// ownership.
        /// If your `CustomDomain` is in an `OwnershipState` other than
        /// `OWNERSHIP_ACTIVE` for more than 30 days and it hasn't been updated in at
        /// least 30 days, Hosting's ownership systems delete the `CustomDomain`.
        /// OWNERSHIP_MISSING:
        /// Your `CustomDomain`'s domain name has no Hosting-related ownership records;
        /// no Firebase project has permission to act on the domain name's behalf.
        /// OWNERSHIP_UNREACHABLE:
        /// Your `CustomDomain`'s domain name can't be reached. Hosting services' DNS
        /// queries to find your domain name's ownership records resulted in errors.
        /// See your `CustomDomain`'s `issues` field for more details.
        /// OWNERSHIP_MISMATCH:
        /// Your `CustomDomain`'s domain name is owned by another Firebase project.
        /// Remove the conflicting `TXT` records and replace them with project-specific
        /// records for your current Firebase project.
        /// OWNERSHIP_CONFLICT:
        /// Your `CustomDomain`'s domain name has conflicting `TXT` records that
        /// indicate ownership by both your current Firebase project and another
        /// project. Remove the other project's ownership records to grant the current
        /// project ownership.
        /// OWNERSHIP_PENDING:
        /// Your `CustomDomain`'s DNS records are configured correctly. Hosting will
        /// transfer ownership of your domain to this `CustomDomain` within 24 hours.
        /// OWNERSHIP_ACTIVE:
        /// Your `CustomDomain`'s domain name has `TXT` records that grant its project
        /// permission to act on its behalf.
        pub ownership_state: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// if true, indicates that Hosting's systems are attmepting to
        /// make the `CustomDomain`'s state match your preferred state. This is most
        /// frequently `true` when initially provisioning a `CustomDomain` or when creating
        /// a new SSL certificate to match an updated `cert_preference`
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// A domain name that this CustomDomain should direct traffic towards. If
        /// specified, Hosting will respond to requests against this CustomDomain
        /// with an HTTP 301 code, and route traffic to the specified `redirect_target`
        /// instead.
        pub redirect_target: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of updates you should make to the domain name's DNS records to
        /// let Hosting serve secure content on its behalf.
        /// Structure is documented below.
        pub required_dns_updates: pulumi_gestalt_rust::Output<
            Vec<super::super::types::firebase::HostingCustomDomainRequiredDnsUpdate>,
        >,
        /// The ID of the site in which to create this custom domain association.
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// The last time the `CustomDomain` was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub wait_dns_verification: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostingCustomDomainArgs,
    ) -> HostingCustomDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cert_preference_binding = args.cert_preference.get_output(context);
        let custom_domain_binding = args.custom_domain.get_output(context);
        let project_binding = args.project.get_output(context);
        let redirect_target_binding = args.redirect_target.get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let wait_dns_verification_binding = args
            .wait_dns_verification
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/hostingCustomDomain:HostingCustomDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certPreference".into(),
                    value: cert_preference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDomain".into(),
                    value: custom_domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirectTarget".into(),
                    value: redirect_target_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: site_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitDnsVerification".into(),
                    value: wait_dns_verification_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostingCustomDomainResult {
            cert_preference: o.get_field("certPreference"),
            certs: o.get_field("certs"),
            create_time: o.get_field("createTime"),
            custom_domain: o.get_field("customDomain"),
            delete_time: o.get_field("deleteTime"),
            etag: o.get_field("etag"),
            expire_time: o.get_field("expireTime"),
            host_state: o.get_field("hostState"),
            issues: o.get_field("issues"),
            name: o.get_field("name"),
            ownership_state: o.get_field("ownershipState"),
            project: o.get_field("project"),
            reconciling: o.get_field("reconciling"),
            redirect_target: o.get_field("redirectTarget"),
            required_dns_updates: o.get_field("requiredDnsUpdates"),
            site_id: o.get_field("siteId"),
            update_time: o.get_field("updateTime"),
            wait_dns_verification: o.get_field("waitDnsVerification"),
        }
    }
}
