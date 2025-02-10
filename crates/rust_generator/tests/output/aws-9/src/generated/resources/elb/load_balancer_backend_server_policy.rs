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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod load_balancer_backend_server_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerBackendServerPolicyArgs {
        /// The instance port to apply the policy to.
        #[builder(into)]
        pub instance_port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The load balancer to attach the policy to.
        #[builder(into)]
        pub load_balancer_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of Policy Names to apply to the backend server.
        #[builder(into, default)]
        pub policy_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerBackendServerPolicyResult {
        /// The instance port to apply the policy to.
        pub instance_port: pulumi_gestalt_rust::Output<i32>,
        /// The load balancer to attach the policy to.
        pub load_balancer_name: pulumi_gestalt_rust::Output<String>,
        /// List of Policy Names to apply to the backend server.
        pub policy_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoadBalancerBackendServerPolicyArgs,
    ) -> LoadBalancerBackendServerPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_port_binding = args.instance_port.get_output(context);
        let load_balancer_name_binding = args.load_balancer_name.get_output(context);
        let policy_names_binding = args.policy_names.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elb/loadBalancerBackendServerPolicy:LoadBalancerBackendServerPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instancePort".into(),
                    value: instance_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerName".into(),
                    value: load_balancer_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyNames".into(),
                    value: policy_names_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoadBalancerBackendServerPolicyResult {
            instance_port: o.get_field("instancePort"),
            load_balancer_name: o.get_field("loadBalancerName"),
            policy_names: o.get_field("policyNames"),
        }
    }
}
