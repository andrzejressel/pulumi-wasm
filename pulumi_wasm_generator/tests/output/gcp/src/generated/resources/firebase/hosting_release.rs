/// ## Example Usage
///
/// ### Firebasehosting Release In Site
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod hosting_release {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingReleaseArgs {
        /// The ID of the channel to which the release belongs. If not provided, the release will
        /// belong to the default "live" channel
        #[builder(into, default)]
        pub channel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The deploy description when the release was created. The value can be up to 512 characters.
        #[builder(into, default)]
        pub message: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. The ID of the site to which the release belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// The type of the release; indicates what happened to the content of the site. There is no need to specify
        /// `DEPLOY` or `ROLLBACK` type if a `version_name` is provided.
        /// DEPLOY: A version was uploaded to Firebase Hosting and released. Output only.
        /// ROLLBACK: The release points back to a previously deployed version. Output only.
        /// SITE_DISABLE: The release prevents the site from serving content. Firebase Hosting acts as if the site never existed
        /// Possible values are: `DEPLOY`, `ROLLBACK`, `SITE_DISABLE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier for a version, in the format: sites/SITE_ID/versions/VERSION_ID.
        /// The content of the version specified will be actively displayed on the appropriate URL.
        /// The Version must belong to the same site as in the `site_id`.
        /// This parameter must be empty if the `type` of the release is `SITE_DISABLE`.
        #[builder(into, default)]
        pub version_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostingReleaseResult {
        /// The ID of the channel to which the release belongs. If not provided, the release will
        /// belong to the default "live" channel
        pub channel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The deploy description when the release was created. The value can be up to 512 characters.
        pub message: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier for the release, in either of the following formats:
        /// sites/SITE_ID/releases/RELEASE_ID
        /// sites/SITE_ID/channels/CHANNEL_ID/releases/RELEASE_ID
        pub name: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the Release.
        pub release_id: pulumi_wasm_rust::Output<String>,
        /// Required. The ID of the site to which the release belongs.
        ///
        ///
        /// - - -
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// The type of the release; indicates what happened to the content of the site. There is no need to specify
        /// `DEPLOY` or `ROLLBACK` type if a `version_name` is provided.
        /// DEPLOY: A version was uploaded to Firebase Hosting and released. Output only.
        /// ROLLBACK: The release points back to a previously deployed version. Output only.
        /// SITE_DISABLE: The release prevents the site from serving content. Firebase Hosting acts as if the site never existed
        /// Possible values are: `DEPLOY`, `ROLLBACK`, `SITE_DISABLE`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for a version, in the format: sites/SITE_ID/versions/VERSION_ID.
        /// The content of the version specified will be actively displayed on the appropriate URL.
        /// The Version must belong to the same site as in the `site_id`.
        /// This parameter must be empty if the `type` of the release is `SITE_DISABLE`.
        pub version_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HostingReleaseArgs) -> HostingReleaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let channel_id_binding = args.channel_id.get_inner();
        let message_binding = args.message.get_inner();
        let site_id_binding = args.site_id.get_inner();
        let type__binding = args.type_.get_inner();
        let version_name_binding = args.version_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/hostingRelease:HostingRelease".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "channelId".into(),
                    value: &channel_id_binding,
                },
                register_interface::ObjectField {
                    name: "message".into(),
                    value: &message_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "versionName".into(),
                    value: &version_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "channelId".into(),
                },
                register_interface::ResultField {
                    name: "message".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "releaseId".into(),
                },
                register_interface::ResultField {
                    name: "siteId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "versionName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostingReleaseResult {
            channel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channelId").unwrap(),
            ),
            message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("message").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            release_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseId").unwrap(),
            ),
            site_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            version_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionName").unwrap(),
            ),
        }
    }
}
