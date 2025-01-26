/// Provides a load balancer policy, which can be attached to an ELB listener or backend server.
///
/// ## Example Usage
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
///   wu-tang-ca-pubkey-policy:
///     type: aws:elb:LoadBalancerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       policyName: wu-tang-ca-pubkey-policy
///       policyTypeName: PublicKeyPolicyType
///       policyAttributes:
///         - name: PublicKey
///           value:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: wu-tang-pubkey
///               return: result
///   wu-tang-root-ca-backend-auth-policy:
///     type: aws:elb:LoadBalancerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       policyName: wu-tang-root-ca-backend-auth-policy
///       policyTypeName: BackendServerAuthenticationPolicyType
///       policyAttributes:
///         - name: PublicKeyPolicyName
///           value: ${["wu-tang-root-ca-pubkey-policy"].policyName}
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
///   wu-tang-ssl-tls-1-1:
///     type: aws:elb:LoadBalancerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       policyName: wu-tang-ssl
///       policyTypeName: SSLNegotiationPolicyType
///       policyAttributes:
///         - name: Reference-Security-Policy
///           value: ELBSecurityPolicy-TLS-1-1-2017-01
///   wu-tang-backend-auth-policies-443:
///     type: aws:elb:LoadBalancerBackendServerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       instancePort: 443
///       policyNames:
///         - ${["wu-tang-root-ca-backend-auth-policy"].policyName}
///   wu-tang-listener-policies-443:
///     type: aws:elb:ListenerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       loadBalancerPort: 443
///       policyNames:
///         - ${["wu-tang-ssl"].policyName}
/// ```
pub mod load_balancer_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerPolicyArgs {
        /// The load balancer on which the policy is defined.
        #[builder(into)]
        pub load_balancer_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Policy attribute to apply to the policy.
        #[builder(into, default)]
        pub policy_attributes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::elb::LoadBalancerPolicyPolicyAttribute>>,
        >,
        /// The name of the load balancer policy.
        #[builder(into)]
        pub policy_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The policy type.
        #[builder(into)]
        pub policy_type_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerPolicyResult {
        /// The load balancer on which the policy is defined.
        pub load_balancer_name: pulumi_wasm_rust::Output<String>,
        /// Policy attribute to apply to the policy.
        pub policy_attributes: pulumi_wasm_rust::Output<
            Vec<super::super::types::elb::LoadBalancerPolicyPolicyAttribute>,
        >,
        /// The name of the load balancer policy.
        pub policy_name: pulumi_wasm_rust::Output<String>,
        /// The policy type.
        pub policy_type_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoadBalancerPolicyArgs,
    ) -> LoadBalancerPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let load_balancer_name_binding = args
            .load_balancer_name
            .get_output(context)
            .get_inner();
        let policy_attributes_binding = args
            .policy_attributes
            .get_output(context)
            .get_inner();
        let policy_name_binding = args.policy_name.get_output(context).get_inner();
        let policy_type_name_binding = args
            .policy_type_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elb/loadBalancerPolicy:LoadBalancerPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadBalancerName".into(),
                    value: &load_balancer_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyAttributes".into(),
                    value: &policy_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "policyName".into(),
                    value: &policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyTypeName".into(),
                    value: &policy_type_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "loadBalancerName".into(),
                },
                register_interface::ResultField {
                    name: "policyAttributes".into(),
                },
                register_interface::ResultField {
                    name: "policyName".into(),
                },
                register_interface::ResultField {
                    name: "policyTypeName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoadBalancerPolicyResult {
            load_balancer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerName").unwrap(),
            ),
            policy_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyAttributes").unwrap(),
            ),
            policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyName").unwrap(),
            ),
            policy_type_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyTypeName").unwrap(),
            ),
        }
    }
}
