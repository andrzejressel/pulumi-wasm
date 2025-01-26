/// Attaches a load balancer policy to an ELB backend server.
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
///   wu-tang-backend-auth-policies-443:
///     type: aws:elb:LoadBalancerBackendServerPolicy
///     properties:
///       loadBalancerName: ${["wu-tang"].name}
///       instancePort: 443
///       policyNames:
///         - ${["wu-tang-root-ca-backend-auth-policy"].policyName}
/// ```
pub mod load_balancer_backend_server_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerBackendServerPolicyArgs {
        /// The instance port to apply the policy to.
        #[builder(into)]
        pub instance_port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The load balancer to attach the policy to.
        #[builder(into)]
        pub load_balancer_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of Policy Names to apply to the backend server.
        #[builder(into, default)]
        pub policy_names: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerBackendServerPolicyResult {
        /// The instance port to apply the policy to.
        pub instance_port: pulumi_wasm_rust::Output<i32>,
        /// The load balancer to attach the policy to.
        pub load_balancer_name: pulumi_wasm_rust::Output<String>,
        /// List of Policy Names to apply to the backend server.
        pub policy_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoadBalancerBackendServerPolicyArgs,
    ) -> LoadBalancerBackendServerPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_port_binding = args.instance_port.get_output(context).get_inner();
        let load_balancer_name_binding = args
            .load_balancer_name
            .get_output(context)
            .get_inner();
        let policy_names_binding = args.policy_names.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elb/loadBalancerBackendServerPolicy:LoadBalancerBackendServerPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instancePort".into(),
                    value: &instance_port_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerName".into(),
                    value: &load_balancer_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "instancePort".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerName".into(),
                },
                register_interface::ResultField {
                    name: "policyNames".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoadBalancerBackendServerPolicyResult {
            instance_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instancePort").unwrap(),
            ),
            load_balancer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerName").unwrap(),
            ),
            policy_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyNames").unwrap(),
            ),
        }
    }
}
