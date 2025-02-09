/// ## Example Usage
///
/// ### Firebasehosting Release In Site
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
///             .message("Test release")
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
/// ### Firebasehosting Release In Channel
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
///             .site_id("site-with-channel")
///             .build_struct(),
///     );
///     let defaultHostingChannel = hosting_channel::create(
///         "defaultHostingChannel",
///         HostingChannelArgs::builder()
///             .channel_id("channel-id")
///             .site_id("${default.siteId}")
///             .build_struct(),
///     );
///     let defaultHostingRelease = hosting_release::create(
///         "defaultHostingRelease",
///         HostingReleaseArgs::builder()
///             .channel_id("${defaultHostingChannel.channelId}")
///             .message("Test release in channel")
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
/// ### Firebasehosting Release Disable
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
///             .message("Take down site")
///             .site_id("${default.siteId}")
///             .type_("SITE_DISABLE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Release can be imported using any of these accepted formats:
///
/// * `sites/{{site_id}}/channels/{{channel_id}}/releases/{{release_id}}`
///
/// * `sites/{{site_id}}/releases/{{release_id}}`
///
/// * `{{site_id}}/{{channel_id}}/{{release_id}}`
///
/// * `{{site_id}}/{{release_id}}`
///
/// When using the `pulumi import` command, Release can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingRelease:HostingRelease default sites/{{site_id}}/channels/{{channel_id}}/releases/{{release_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingRelease:HostingRelease default sites/{{site_id}}/releases/{{release_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingRelease:HostingRelease default {{site_id}}/{{channel_id}}/{{release_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingRelease:HostingRelease default {{site_id}}/{{release_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosting_release {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingReleaseArgs {
        /// The ID of the channel to which the release belongs. If not provided, the release will
        /// belong to the default "live" channel
        #[builder(into, default)]
        pub channel_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The deploy description when the release was created. The value can be up to 512 characters.
        #[builder(into, default)]
        pub message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. The ID of the site to which the release belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the release; indicates what happened to the content of the site. There is no need to specify
        /// `DEPLOY` or `ROLLBACK` type if a `version_name` is provided.
        /// DEPLOY: A version was uploaded to Firebase Hosting and released. Output only.
        /// ROLLBACK: The release points back to a previously deployed version. Output only.
        /// SITE_DISABLE: The release prevents the site from serving content. Firebase Hosting acts as if the site never existed
        /// Possible values are: `DEPLOY`, `ROLLBACK`, `SITE_DISABLE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier for a version, in the format: sites/SITE_ID/versions/VERSION_ID.
        /// The content of the version specified will be actively displayed on the appropriate URL.
        /// The Version must belong to the same site as in the `site_id`.
        /// This parameter must be empty if the `type` of the release is `SITE_DISABLE`.
        #[builder(into, default)]
        pub version_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostingReleaseResult {
        /// The ID of the channel to which the release belongs. If not provided, the release will
        /// belong to the default "live" channel
        pub channel_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The deploy description when the release was created. The value can be up to 512 characters.
        pub message: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the release, in either of the following formats:
        /// sites/SITE_ID/releases/RELEASE_ID
        /// sites/SITE_ID/channels/CHANNEL_ID/releases/RELEASE_ID
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the Release.
        pub release_id: pulumi_gestalt_rust::Output<String>,
        /// Required. The ID of the site to which the release belongs.
        ///
        ///
        /// - - -
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the release; indicates what happened to the content of the site. There is no need to specify
        /// `DEPLOY` or `ROLLBACK` type if a `version_name` is provided.
        /// DEPLOY: A version was uploaded to Firebase Hosting and released. Output only.
        /// ROLLBACK: The release points back to a previously deployed version. Output only.
        /// SITE_DISABLE: The release prevents the site from serving content. Firebase Hosting acts as if the site never existed
        /// Possible values are: `DEPLOY`, `ROLLBACK`, `SITE_DISABLE`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for a version, in the format: sites/SITE_ID/versions/VERSION_ID.
        /// The content of the version specified will be actively displayed on the appropriate URL.
        /// The Version must belong to the same site as in the `site_id`.
        /// This parameter must be empty if the `type` of the release is `SITE_DISABLE`.
        pub version_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostingReleaseArgs,
    ) -> HostingReleaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let channel_id_binding = args.channel_id.get_output(context);
        let message_binding = args.message.get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let type__binding = args.type_.get_output(context);
        let version_name_binding = args.version_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/hostingRelease:HostingRelease".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "channelId".into(),
                    value: channel_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "message".into(),
                    value: message_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: site_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionName".into(),
                    value: version_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostingReleaseResult {
            channel_id: o.get_field("channelId"),
            message: o.get_field("message"),
            name: o.get_field("name"),
            release_id: o.get_field("releaseId"),
            site_id: o.get_field("siteId"),
            type_: o.get_field("type"),
            version_name: o.get_field("versionName"),
        }
    }
}
