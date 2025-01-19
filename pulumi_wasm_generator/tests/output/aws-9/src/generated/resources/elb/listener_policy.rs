/// Attaches a load balancer policy to an ELB Listener.
///
/// ## Example Usage
///
/// ### Custom Policy
///
/// ```yaml
/// resources:
///   wu-tang:
///     type: aws:elb:LoadBalancer
///     properties:
///       name: wu-tang
///       availabilityZones:
///         - us-east-1a
///       listeners:
///         - instancePort: 443
///           instanceProtocol: http
///           lbPort: 443
///           lbProtocol: https
///           sslCertificateId: arn:aws:iam::000000000000:server-certificate/wu-tang.net
///       tags:
///         Name: wu-tang
///   wu-tang-ssl:
///     type: aws:elb:LoadBalancerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       policyName: wu-tang-ssl
///       policyTypeName: SSLNegotiationPolicyType
///       policyAttributes:
///         - name: ECDHE-ECDSA-AES128-GCM-SHA256
///           value: 'true'
///         - name: Protocol-TLSv1.2
///           value: 'true'
///   wu-tang-listener-policies-443:
///     type: aws:elb:ListenerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       loadBalancerPort: 443
///       policyNames:
///         - ${["wu-tang-ssl"].policyName}
/// ```
///
/// This example shows how to customize the TLS settings of an HTTPS listener.
///
/// ### AWS Predefined Security Policy
///
/// ```yaml
/// resources:
///   wu-tang:
///     type: aws:elb:LoadBalancer
///     properties:
///       name: wu-tang
///       availabilityZones:
///         - us-east-1a
///       listeners:
///         - instancePort: 443
///           instanceProtocol: http
///           lbPort: 443
///           lbProtocol: https
///           sslCertificateId: arn:aws:iam::000000000000:server-certificate/wu-tang.net
///       tags:
///         Name: wu-tang
///   wu-tang-ssl-tls-1-1:
///     type: aws:elb:LoadBalancerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       policyName: wu-tang-ssl
///       policyTypeName: SSLNegotiationPolicyType
///       policyAttributes:
///         - name: Reference-Security-Policy
///           value: ELBSecurityPolicy-TLS-1-1-2017-01
///   wu-tang-listener-policies-443:
///     type: aws:elb:ListenerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       loadBalancerPort: 443
///       policyNames:
///         - ${["wu-tang-ssl-tls-1-1"].policyName}
/// ```
///
/// This example shows how to add a [Predefined Security Policy for ELBs](https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-security-policy-table.html)
pub mod listener_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerPolicyArgs {
        /// The load balancer to attach the policy to.
        #[builder(into)]
        pub load_balancer_name: pulumi_wasm_rust::Output<String>,
        /// The load balancer listener port to apply the policy to.
        #[builder(into)]
        pub load_balancer_port: pulumi_wasm_rust::Output<i32>,
        /// List of Policy Names to apply to the backend server.
        #[builder(into, default)]
        pub policy_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Map of arbitrary keys and values that, when changed, will trigger an update.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListenerPolicyResult {
        /// The load balancer to attach the policy to.
        pub load_balancer_name: pulumi_wasm_rust::Output<String>,
        /// The load balancer listener port to apply the policy to.
        pub load_balancer_port: pulumi_wasm_rust::Output<i32>,
        /// List of Policy Names to apply to the backend server.
        pub policy_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Map of arbitrary keys and values that, when changed, will trigger an update.
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ListenerPolicyArgs) -> ListenerPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let load_balancer_name_binding = args.load_balancer_name.get_inner();
        let load_balancer_port_binding = args.load_balancer_port.get_inner();
        let policy_names_binding = args.policy_names.get_inner();
        let triggers_binding = args.triggers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elb/listenerPolicy:ListenerPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadBalancerName".into(),
                    value: &load_balancer_name_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerPort".into(),
                    value: &load_balancer_port_binding,
                },
                register_interface::ObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "loadBalancerName".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerPort".into(),
                },
                register_interface::ResultField {
                    name: "policyNames".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListenerPolicyResult {
            load_balancer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerName").unwrap(),
            ),
            load_balancer_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerPort").unwrap(),
            ),
            policy_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyNames").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
