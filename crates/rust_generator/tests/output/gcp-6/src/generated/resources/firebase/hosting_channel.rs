/// ## Example Usage
///
/// ### Firebasehosting Channel Basic
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hosting_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostingChannelArgs {
        /// Required. Immutable. A unique ID within the site that identifies the channel.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub channel_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time at which the channel will be automatically deleted. If null, the channel
        /// will not be automatically deleted. This field is present in the output whether it's
        /// set directly or via the `ttl` field.
        #[builder(into, default)]
        pub expire_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Text labels used for extra metadata and/or filtering
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of previous releases to retain on the channel for rollback or other
        /// purposes. Must be a number between 1-100. Defaults to 10 for new channels.
        #[builder(into, default)]
        pub retained_release_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Required. The ID of the site in which to create this channel.
        #[builder(into)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Input only. A time-to-live for this channel. Sets `expire_time` to the provided
        /// duration past the time of the request. A duration in seconds with up to nine fractional
        /// digits, terminated by 's'. Example: "86400s" (one day).
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostingChannelResult {
        /// Required. Immutable. A unique ID within the site that identifies the channel.
        ///
        ///
        /// - - -
        pub channel_id: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time at which the channel will be automatically deleted. If null, the channel
        /// will not be automatically deleted. This field is present in the output whether it's
        /// set directly or via the `ttl` field.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// Text labels used for extra metadata and/or filtering
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The fully-qualified resource name for the channel, in the format:
        /// sites/SITE_ID/channels/CHANNEL_ID
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The number of previous releases to retain on the channel for rollback or other
        /// purposes. Must be a number between 1-100. Defaults to 10 for new channels.
        pub retained_release_count: pulumi_gestalt_rust::Output<i32>,
        /// Required. The ID of the site in which to create this channel.
        pub site_id: pulumi_gestalt_rust::Output<String>,
        /// Input only. A time-to-live for this channel. Sets `expire_time` to the provided
        /// duration past the time of the request. A duration in seconds with up to nine fractional
        /// digits, terminated by 's'. Example: "86400s" (one day).
        pub ttl: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostingChannelArgs,
    ) -> HostingChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let channel_id_binding = args.channel_id.get_output(context);
        let expire_time_binding = args.expire_time.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let retained_release_count_binding = args
            .retained_release_count
            .get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/hostingChannel:HostingChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "channelId".into(),
                    value: &channel_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expireTime".into(),
                    value: &expire_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retainedReleaseCount".into(),
                    value: &retained_release_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostingChannelResult {
            channel_id: o.get_field("channelId"),
            effective_labels: o.get_field("effectiveLabels"),
            expire_time: o.get_field("expireTime"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            pulumi_labels: o.get_field("pulumiLabels"),
            retained_release_count: o.get_field("retainedReleaseCount"),
            site_id: o.get_field("siteId"),
            ttl: o.get_field("ttl"),
        }
    }
}
