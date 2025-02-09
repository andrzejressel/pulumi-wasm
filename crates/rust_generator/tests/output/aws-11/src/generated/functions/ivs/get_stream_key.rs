#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_stream_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStreamKeyArgs {
        /// ARN of the Channel.
        #[builder(into)]
        pub channel_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetStreamKeyResult {
        /// ARN of the Stream Key.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub channel_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Stream Key value.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetStreamKeyArgs,
    ) -> GetStreamKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let channel_arn_binding_1 = args.channel_arn.get_output(context);
        let channel_arn_binding = channel_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ivs/getStreamKey:getStreamKey".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "channelArn".into(),
                    value: &channel_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStreamKeyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            channel_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("channelArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
