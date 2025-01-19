/// ## Example Usage
///
/// ### Firebasehosting Channel Basic
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
///             .channel_id("channel-basic")
///             .site_id("${default.siteId}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebasehosting Channel Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:firebase:HostingSite
///     properties:
///       project: my-project-name
///       siteId: site-with-channel
///   full:
///     type: gcp:firebase:HostingChannel
///     properties:
///       siteId: ${default.siteId}
///       channelId: channel-full
///       ttl: 86400s
///       retainedReleaseCount: 20
///       labels:
///         some-key: some-value
/// ```
///
/// ## Import
///
/// Channel can be imported using any of these accepted formats:
///
/// * `sites/{{site_id}}/channels/{{channel_id}}`
///
/// * `{{site_id}}/{{channel_id}}`
///
/// When using the `pulumi import` command, Channel can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingChannel:HostingChannel default sites/{{site_id}}/channels/{{channel_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/hostingChannel:HostingChannel default {{site_id}}/{{channel_id}}
/// ```
///
pub mod hosting_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingChannelArgs {
        /// Required. Immutable. A unique ID within the site that identifies the channel.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub channel_id: pulumi_wasm_rust::Output<String>,
        /// The time at which the channel will be automatically deleted. If null, the channel
        /// will not be automatically deleted. This field is present in the output whether it's
        /// set directly or via the `ttl` field.
        #[builder(into, default)]
        pub expire_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Text labels used for extra metadata and/or filtering
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of previous releases to retain on the channel for rollback or other
        /// purposes. Must be a number between 1-100. Defaults to 10 for new channels.
        #[builder(into, default)]
        pub retained_release_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Required. The ID of the site in which to create this channel.
        #[builder(into)]
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// Input only. A time-to-live for this channel. Sets `expire_time` to the provided
        /// duration past the time of the request. A duration in seconds with up to nine fractional
        /// digits, terminated by 's'. Example: "86400s" (one day).
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostingChannelResult {
        /// Required. Immutable. A unique ID within the site that identifies the channel.
        ///
        ///
        /// - - -
        pub channel_id: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time at which the channel will be automatically deleted. If null, the channel
        /// will not be automatically deleted. This field is present in the output whether it's
        /// set directly or via the `ttl` field.
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// Text labels used for extra metadata and/or filtering
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The fully-qualified resource name for the channel, in the format:
        /// sites/SITE_ID/channels/CHANNEL_ID
        pub name: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The number of previous releases to retain on the channel for rollback or other
        /// purposes. Must be a number between 1-100. Defaults to 10 for new channels.
        pub retained_release_count: pulumi_wasm_rust::Output<i32>,
        /// Required. The ID of the site in which to create this channel.
        pub site_id: pulumi_wasm_rust::Output<String>,
        /// Input only. A time-to-live for this channel. Sets `expire_time` to the provided
        /// duration past the time of the request. A duration in seconds with up to nine fractional
        /// digits, terminated by 's'. Example: "86400s" (one day).
        pub ttl: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HostingChannelArgs) -> HostingChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let channel_id_binding = args.channel_id.get_inner();
        let expire_time_binding = args.expire_time.get_inner();
        let labels_binding = args.labels.get_inner();
        let retained_release_count_binding = args.retained_release_count.get_inner();
        let site_id_binding = args.site_id.get_inner();
        let ttl_binding = args.ttl.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/hostingChannel:HostingChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "channelId".into(),
                    value: &channel_id_binding,
                },
                register_interface::ObjectField {
                    name: "expireTime".into(),
                    value: &expire_time_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "retainedReleaseCount".into(),
                    value: &retained_release_count_binding,
                },
                register_interface::ObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "channelId".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "retainedReleaseCount".into(),
                },
                register_interface::ResultField {
                    name: "siteId".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostingChannelResult {
            channel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channelId").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            retained_release_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retainedReleaseCount").unwrap(),
            ),
            site_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteId").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ttl").unwrap()),
        }
    }
}
