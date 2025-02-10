#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_listener {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerArgs {
        /// ID or Amazon Resource Name (ARN) of the listener
        #[builder(into)]
        pub listener_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID or Amazon Resource Name (ARN) of the service network
        #[builder(into)]
        pub service_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of tags associated with the listener.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetListenerResult {
        /// ARN of the listener.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the listener was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The actions for the default listener rule.
        pub default_actions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vpclattice::GetListenerDefaultAction>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The date and time the listener was last updated.
        pub last_updated_at: pulumi_gestalt_rust::Output<String>,
        /// The ID of the listener.
        pub listener_id: pulumi_gestalt_rust::Output<String>,
        pub listener_identifier: pulumi_gestalt_rust::Output<String>,
        /// The name of the listener.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The listener port.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The listener protocol. Either `HTTPS` or `HTTP`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the service.
        pub service_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the service.
        pub service_id: pulumi_gestalt_rust::Output<String>,
        pub service_identifier: pulumi_gestalt_rust::Output<String>,
        /// List of tags associated with the listener.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetListenerArgs,
    ) -> GetListenerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let listener_identifier_binding = args.listener_identifier.get_output(context);
        let service_identifier_binding = args.service_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:vpclattice/getListener:getListener".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerIdentifier".into(),
                    value: listener_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceIdentifier".into(),
                    value: service_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetListenerResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            default_actions: o.get_field("defaultActions"),
            id: o.get_field("id"),
            last_updated_at: o.get_field("lastUpdatedAt"),
            listener_id: o.get_field("listenerId"),
            listener_identifier: o.get_field("listenerIdentifier"),
            name: o.get_field("name"),
            port: o.get_field("port"),
            protocol: o.get_field("protocol"),
            service_arn: o.get_field("serviceArn"),
            service_id: o.get_field("serviceId"),
            service_identifier: o.get_field("serviceIdentifier"),
            tags: o.get_field("tags"),
        }
    }
}
