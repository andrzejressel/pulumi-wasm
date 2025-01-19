pub mod get_listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerArgs {
        /// ARN of the listener. Required if `load_balancer_arn` and `port` is not set.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the load balancer. Required if `arn` is not set.
        #[builder(into, default)]
        pub load_balancer_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Port of the listener. Required if `arn` is not set.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetListenerResult {
        pub alpn_policy: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        pub default_actions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lb::GetListenerDefaultAction>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub load_balancer_arn: pulumi_wasm_rust::Output<String>,
        pub mutual_authentications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lb::GetListenerMutualAuthentication>,
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
    pub fn invoke(args: GetListenerArgs) -> GetListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let load_balancer_arn_binding = args.load_balancer_arn.get_inner();
        let port_binding = args.port.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lb/getListener:getListener".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alpnPolicy".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "defaultActions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerArn".into(),
                },
                register_interface::ResultField {
                    name: "mutualAuthentications".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "sslPolicy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetListenerResult {
            alpn_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alpnPolicy").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            default_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultActions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            load_balancer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerArn").unwrap(),
            ),
            mutual_authentications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mutualAuthentications").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            ssl_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslPolicy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
