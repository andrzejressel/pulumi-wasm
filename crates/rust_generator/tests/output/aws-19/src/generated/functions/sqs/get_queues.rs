#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_queues {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueuesArgs {
        /// A string to use for filtering the list results. Only those queues whose name begins with the specified string are returned. Queue URLs and names are case-sensitive.
        #[builder(into, default)]
        pub queue_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQueuesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub queue_name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of queue URLs.
        pub queue_urls: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQueuesArgs,
    ) -> GetQueuesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let queue_name_prefix_binding = args.queue_name_prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sqs/getQueues:getQueues".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueNamePrefix".into(),
                    value: &queue_name_prefix_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQueuesResult {
            id: o.get_field("id"),
            queue_name_prefix: o.get_field("queueNamePrefix"),
            queue_urls: o.get_field("queueUrls"),
        }
    }
}
