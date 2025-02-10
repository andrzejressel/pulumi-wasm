#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_hosting_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHostingChannelArgs {
        /// The ID of the channel. Use `channel_id = "live"` for the default channel of a site.
        #[builder(into)]
        pub channel_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the site this channel belongs to.
        #[builder(into)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetHostingChannelResult {
        pub channel_id: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fully-qualified resource name for the channel, in the format: `sites/{{site_id}}/channels/{{channel_id}}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub retained_release_count: pulumi_gestalt_rust::Output<i32>,
        pub site_id: pulumi_gestalt_rust::Output<String>,
        pub ttl: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetHostingChannelArgs,
    ) -> GetHostingChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let channel_id_binding = args.channel_id.get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:firebase/getHostingChannel:getHostingChannel".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "channelId".into(),
                    value: channel_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: site_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetHostingChannelResult {
            channel_id: o.get_field("channelId"),
            effective_labels: o.get_field("effectiveLabels"),
            expire_time: o.get_field("expireTime"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            pulumi_labels: o.get_field("pulumiLabels"),
            retained_release_count: o.get_field("retainedReleaseCount"),
            site_id: o.get_field("siteId"),
            ttl: o.get_field("ttl"),
        }
    }
}
