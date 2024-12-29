/// Provides a Load Balancer Listener resource.
///
/// > **Note:** `aws.alb.Listener` is known as `aws.lb.Listener`. The functionality is identical.
///
/// ## Example Usage
///
/// ### Forward Action
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:LoadBalancer
///     name: front_end
///   frontEndTargetGroup:
///     type: aws:lb:TargetGroup
///     name: front_end
///   frontEndListener:
///     type: aws:lb:Listener
///     name: front_end
///     properties:
///       loadBalancerArn: ${frontEnd.arn}
///       port: '443'
///       protocol: HTTPS
///       sslPolicy: ELBSecurityPolicy-2016-08
///       certificateArn: arn:aws:iam::187416307283:server-certificate/test_cert_rab3wuqwgja25ct3n4jdj2tzu4
///       defaultActions:
///         - type: forward
///           targetGroupArn: ${frontEndTargetGroup.arn}
/// ```
///
/// To a NLB:
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:Listener
///     name: front_end
///     properties:
///       loadBalancerArn: ${frontEndAwsLb.arn}
///       port: '443'
///       protocol: TLS
///       sslPolicy: ELBSecurityPolicy-2016-08
///       certificateArn: arn:aws:iam::187416307283:server-certificate/test_cert_rab3wuqwgja25ct3n4jdj2tzu4
///       alpnPolicy: HTTP2Preferred
///       defaultActions:
///         - type: forward
///           targetGroupArn: ${frontEndAwsLbTargetGroup.arn}
/// ```
///
/// ### Redirect Action
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:LoadBalancer
///     name: front_end
///   frontEndListener:
///     type: aws:lb:Listener
///     name: front_end
///     properties:
///       loadBalancerArn: ${frontEnd.arn}
///       port: '80'
///       protocol: HTTP
///       defaultActions:
///         - type: redirect
///           redirect:
///             port: '443'
///             protocol: HTTPS
///             statusCode: HTTP_301
/// ```
///
/// ### Fixed-response Action
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:LoadBalancer
///     name: front_end
///   frontEndListener:
///     type: aws:lb:Listener
///     name: front_end
///     properties:
///       loadBalancerArn: ${frontEnd.arn}
///       port: '80'
///       protocol: HTTP
///       defaultActions:
///         - type: fixed-response
///           fixedResponse:
///             contentType: text/plain
///             messageBody: Fixed response content
///             statusCode: '200'
/// ```
///
/// ### Authenticate-cognito Action
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:LoadBalancer
///     name: front_end
///   frontEndTargetGroup:
///     type: aws:lb:TargetGroup
///     name: front_end
///   pool:
///     type: aws:cognito:UserPool
///   client:
///     type: aws:cognito:UserPoolClient
///   domain:
///     type: aws:cognito:UserPoolDomain
///   frontEndListener:
///     type: aws:lb:Listener
///     name: front_end
///     properties:
///       loadBalancerArn: ${frontEnd.arn}
///       port: '80'
///       protocol: HTTP
///       defaultActions:
///         - type: authenticate-cognito
///           authenticateCognito:
///             userPoolArn: ${pool.arn}
///             userPoolClientId: ${client.id}
///             userPoolDomain: ${domain.domain}
///         - type: forward
///           targetGroupArn: ${frontEndTargetGroup.arn}
/// ```
///
/// ### Authenticate-OIDC Action
///
/// ```yaml
/// resources:
///   frontEnd:
///     type: aws:lb:LoadBalancer
///     name: front_end
///   frontEndTargetGroup:
///     type: aws:lb:TargetGroup
///     name: front_end
///   frontEndListener:
///     type: aws:lb:Listener
///     name: front_end
///     properties:
///       loadBalancerArn: ${frontEnd.arn}
///       port: '80'
///       protocol: HTTP
///       defaultActions:
///         - type: authenticate-oidc
///           authenticateOidc:
///             authorizationEndpoint: https://example.com/authorization_endpoint
///             clientId: client_id
///             clientSecret: client_secret
///             issuer: https://example.com
///             tokenEndpoint: https://example.com/token_endpoint
///             userInfoEndpoint: https://example.com/user_info_endpoint
///         - type: forward
///           targetGroupArn: ${frontEndTargetGroup.arn}
/// ```
///
/// ### Gateway Load Balancer Listener
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lb:LoadBalancer
///     properties:
///       loadBalancerType: gateway
///       name: example
///       subnetMappings:
///         - subnetId: ${exampleAwsSubnet.id}
///   exampleTargetGroup:
///     type: aws:lb:TargetGroup
///     name: example
///     properties:
///       name: example
///       port: 6081
///       protocol: GENEVE
///       vpcId: ${exampleAwsVpc.id}
///       healthCheck:
///         port: 80
///         protocol: HTTP
///   exampleListener:
///     type: aws:lb:Listener
///     name: example
///     properties:
///       loadBalancerArn: ${example.id}
///       defaultActions:
///         - targetGroupArn: ${exampleTargetGroup.id}
///           type: forward
/// ```
///
/// ### Mutual TLS Authentication
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = load_balancer::create(
///         "example",
///         LoadBalancerArgs::builder().load_balancer_type("application").build_struct(),
///     );
///     let exampleListener = listener::create(
///         "exampleListener",
///         ListenerArgs::builder()
///             .default_actions(
///                 vec![
///                     ListenerDefaultAction::builder()
///                     .targetGroupArn("${exampleTargetGroup.id}"). type ("forward")
///                     .build_struct(),
///                 ],
///             )
///             .load_balancer_arn("${example.id}")
///             .mutual_authentication(
///                 ListenerMutualAuthentication::builder()
///                     .mode("verify")
///                     .trustStoreArn("...")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTargetGroup = target_group::create(
///         "exampleTargetGroup",
///         TargetGroupArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import listeners using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:alb/listener:Listener front_end arn:aws:elasticloadbalancing:us-west-2:187416307283:listener/app/front-end-alb/8e4497da625e2d8a/9ab28ade35828f96
/// ```
pub mod listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerArgs {
        /// Name of the Application-Layer Protocol Negotiation (ALPN) policy. Can be set if `protocol` is `TLS`. Valid values are `HTTP1Only`, `HTTP2Only`, `HTTP2Optional`, `HTTP2Preferred`, and `None`.
        #[builder(into, default)]
        pub alpn_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the default SSL server certificate. Exactly one certificate is required if the protocol is HTTPS. For adding additional SSL certificates, see the `aws.lb.ListenerCertificate` resource.
        #[builder(into, default)]
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for default actions. See below.
        #[builder(into)]
        pub default_actions: pulumi_wasm_rust::Output<
            Vec<super::super::types::alb::ListenerDefaultAction>,
        >,
        /// ARN of the load balancer.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub load_balancer_arn: pulumi_wasm_rust::Output<String>,
        /// The mutual authentication configuration information. See below.
        #[builder(into, default)]
        pub mutual_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::alb::ListenerMutualAuthentication>,
        >,
        /// Port on which the load balancer is listening. Not valid for Gateway Load Balancers.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Protocol for connections from clients to the load balancer. For Application Load Balancers, valid values are `HTTP` and `HTTPS`, with a default of `HTTP`. For Network Load Balancers, valid values are `TCP`, `TLS`, `UDP`, and `TCP_UDP`. Not valid to use `UDP` or `TCP_UDP` if dual-stack mode is enabled. Not valid for Gateway Load Balancers.
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the SSL Policy for the listener. Required if `protocol` is `HTTPS` or `TLS`. Default is `ELBSecurityPolicy-2016-08`.
        #[builder(into, default)]
        pub ssl_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// TCP idle timeout value in seconds. Can only be set if protocol is `TCP` on Network Load Balancer, or with a Gateway Load Balancer. Not supported for Application Load Balancers. Valid values are between `60` and `6000` inclusive. Default: `350`.
        #[builder(into, default)]
        pub tcp_idle_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct ListenerResult {
        /// Name of the Application-Layer Protocol Negotiation (ALPN) policy. Can be set if `protocol` is `TLS`. Valid values are `HTTP1Only`, `HTTP2Only`, `HTTP2Optional`, `HTTP2Preferred`, and `None`.
        pub alpn_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the listener (matches `id`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the default SSL server certificate. Exactly one certificate is required if the protocol is HTTPS. For adding additional SSL certificates, see the `aws.lb.ListenerCertificate` resource.
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for default actions. See below.
        pub default_actions: pulumi_wasm_rust::Output<
            Vec<super::super::types::alb::ListenerDefaultAction>,
        >,
        /// ARN of the load balancer.
        ///
        /// The following arguments are optional:
        pub load_balancer_arn: pulumi_wasm_rust::Output<String>,
        /// The mutual authentication configuration information. See below.
        pub mutual_authentication: pulumi_wasm_rust::Output<
            super::super::types::alb::ListenerMutualAuthentication,
        >,
        /// Port on which the load balancer is listening. Not valid for Gateway Load Balancers.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Protocol for connections from clients to the load balancer. For Application Load Balancers, valid values are `HTTP` and `HTTPS`, with a default of `HTTP`. For Network Load Balancers, valid values are `TCP`, `TLS`, `UDP`, and `TCP_UDP`. Not valid to use `UDP` or `TCP_UDP` if dual-stack mode is enabled. Not valid for Gateway Load Balancers.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Name of the SSL Policy for the listener. Required if `protocol` is `HTTPS` or `TLS`. Default is `ELBSecurityPolicy-2016-08`.
        pub ssl_policy: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// TCP idle timeout value in seconds. Can only be set if protocol is `TCP` on Network Load Balancer, or with a Gateway Load Balancer. Not supported for Application Load Balancers. Valid values are between `60` and `6000` inclusive. Default: `350`.
        pub tcp_idle_timeout_seconds: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ListenerArgs) -> ListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alpn_policy_binding = args.alpn_policy.get_inner();
        let certificate_arn_binding = args.certificate_arn.get_inner();
        let default_actions_binding = args.default_actions.get_inner();
        let load_balancer_arn_binding = args.load_balancer_arn.get_inner();
        let mutual_authentication_binding = args.mutual_authentication.get_inner();
        let port_binding = args.port.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let ssl_policy_binding = args.ssl_policy.get_inner();
        let tags_binding = args.tags.get_inner();
        let tcp_idle_timeout_seconds_binding = args.tcp_idle_timeout_seconds.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:alb/listener:Listener".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alpnPolicy".into(),
                    value: &alpn_policy_binding,
                },
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "defaultActions".into(),
                    value: &default_actions_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerArn".into(),
                    value: &load_balancer_arn_binding,
                },
                register_interface::ObjectField {
                    name: "mutualAuthentication".into(),
                    value: &mutual_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tcpIdleTimeoutSeconds".into(),
                    value: &tcp_idle_timeout_seconds_binding,
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
                    name: "loadBalancerArn".into(),
                },
                register_interface::ResultField {
                    name: "mutualAuthentication".into(),
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
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tcpIdleTimeoutSeconds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListenerResult {
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
            load_balancer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerArn").unwrap(),
            ),
            mutual_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mutualAuthentication").unwrap(),
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tcp_idle_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpIdleTimeoutSeconds").unwrap(),
            ),
        }
    }
}
