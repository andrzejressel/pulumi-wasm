#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_listener {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerArgs {
        /// ARN of the listener. Required if `load_balancer_arn` and `port` is not set.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the load balancer. Required if `arn` is not set.
        #[builder(into, default)]
        pub load_balancer_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Port of the listener. Required if `arn` is not set.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetListenerResult {
        pub alpn_policy: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        pub default_actions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetListenerDefaultAction>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub load_balancer_arn: pulumi_gestalt_rust::Output<String>,
        pub mutual_authentications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetListenerMutualAuthentication>,
        >,
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub protocol: pulumi_gestalt_rust::Output<String>,
        pub ssl_policy: pulumi_gestalt_rust::Output<String>,
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
        let arn_binding = args.arn.get_output(context);
        let load_balancer_arn_binding = args.load_balancer_arn.get_output(context);
        let port_binding = args.port.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lb/getListener:getListener".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerArn".into(),
                    value: load_balancer_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetListenerResult {
            alpn_policy: o.get_field("alpnPolicy"),
            arn: o.get_field("arn"),
            certificate_arn: o.get_field("certificateArn"),
            default_actions: o.get_field("defaultActions"),
            id: o.get_field("id"),
            load_balancer_arn: o.get_field("loadBalancerArn"),
            mutual_authentications: o.get_field("mutualAuthentications"),
            port: o.get_field("port"),
            protocol: o.get_field("protocol"),
            ssl_policy: o.get_field("sslPolicy"),
            tags: o.get_field("tags"),
        }
    }
}
