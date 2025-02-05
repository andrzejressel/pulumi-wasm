pub mod get_listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerArgs {
        /// ARN of the listener. Required if `load_balancer_arn` and `port` is not set.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the load balancer. Required if `arn` is not set.
        #[builder(into, default)]
        pub load_balancer_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Port of the listener. Required if `arn` is not set.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetListenerResult {
        pub alpn_policy: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        pub default_actions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::alb::GetListenerDefaultAction>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub load_balancer_arn: pulumi_wasm_rust::Output<String>,
        pub mutual_authentications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::alb::GetListenerMutualAuthentication>,
        >,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub protocol: pulumi_wasm_rust::Output<String>,
        pub ssl_policy: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetListenerArgs,
    ) -> GetListenerResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let load_balancer_arn_binding = args
            .load_balancer_arn
            .get_output(context)
            .get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:alb/getListener:getListener".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerArn".into(),
                    value: &load_balancer_arn_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetListenerResult {
            alpn_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("alpnPolicy"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            default_actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultActions"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            load_balancer_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancerArn"),
            ),
            mutual_authentications: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mutualAuthentications"),
            ),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            ssl_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sslPolicy"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
