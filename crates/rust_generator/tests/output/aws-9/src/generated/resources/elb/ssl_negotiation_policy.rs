/// Provides a load balancer SSL negotiation policy, which allows an ELB to control the ciphers and protocols that are supported during SSL negotiations between a client and a load balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = ssl_negotiation_policy::create(
///         "foo",
///         SslNegotiationPolicyArgs::builder()
///             .attributes(
///                 vec![
///                     SslNegotiationPolicyAttribute::builder().name("Protocol-TLSv1")
///                     .value("false").build_struct(),
///                     SslNegotiationPolicyAttribute::builder().name("Protocol-TLSv1.1")
///                     .value("false").build_struct(),
///                     SslNegotiationPolicyAttribute::builder().name("Protocol-TLSv1.2")
///                     .value("true").build_struct(),
///                     SslNegotiationPolicyAttribute::builder()
///                     .name("Server-Defined-Cipher-Order").value("true").build_struct(),
///                     SslNegotiationPolicyAttribute::builder()
///                     .name("ECDHE-RSA-AES128-GCM-SHA256").value("true").build_struct(),
///                     SslNegotiationPolicyAttribute::builder().name("AES128-GCM-SHA256")
///                     .value("true").build_struct(),
///                     SslNegotiationPolicyAttribute::builder().name("EDH-RSA-DES-CBC3-SHA")
///                     .value("false").build_struct(),
///                 ],
///             )
///             .lb_port(443)
///             .load_balancer("${lb.id}")
///             .name("foo-policy")
///             .build_struct(),
///     );
///     let lb = load_balancer::create(
///         "lb",
///         LoadBalancerArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .listeners(
///                 vec![
///                     LoadBalancerListener::builder().instancePort(8000)
///                     .instanceProtocol("https").lbPort(443).lbProtocol("https")
///                     .sslCertificateId("arn:aws:iam::123456789012:server-certificate/certName")
///                     .build_struct(),
///                 ],
///             )
///             .name("test-lb")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ssl_negotiation_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SslNegotiationPolicyArgs {
        /// An SSL Negotiation policy attribute. Each has two properties:
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::elb::SslNegotiationPolicyAttribute>>,
        >,
        /// The load balancer port to which the policy
        /// should be applied. This must be an active listener on the load
        /// balancer.
        #[builder(into)]
        pub lb_port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The load balancer to which the policy
        /// should be attached.
        #[builder(into)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SSL negotiation policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        ///
        /// To set your attributes, please see the [AWS Elastic Load Balancing Developer Guide](http://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/elb-security-policy-table.html) for a listing of the supported SSL protocols, SSL options, and SSL ciphers.
        ///
        /// > **NOTE:** The AWS documentation references Server Order Preference, which the AWS Elastic Load Balancing API refers to as `Server-Defined-Cipher-Order`. If you wish to set Server Order Preference, use this value instead.
        #[builder(into, default)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SslNegotiationPolicyResult {
        /// An SSL Negotiation policy attribute. Each has two properties:
        pub attributes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::elb::SslNegotiationPolicyAttribute>>,
        >,
        /// The load balancer port to which the policy
        /// should be applied. This must be an active listener on the load
        /// balancer.
        pub lb_port: pulumi_gestalt_rust::Output<i32>,
        /// The load balancer to which the policy
        /// should be attached.
        pub load_balancer: pulumi_gestalt_rust::Output<String>,
        /// The name of the SSL negotiation policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of arbitrary keys and values that, when changed, will trigger a redeployment.
        ///
        /// To set your attributes, please see the [AWS Elastic Load Balancing Developer Guide](http://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/elb-security-policy-table.html) for a listing of the supported SSL protocols, SSL options, and SSL ciphers.
        ///
        /// > **NOTE:** The AWS documentation references Server Order Preference, which the AWS Elastic Load Balancing API refers to as `Server-Defined-Cipher-Order`. If you wish to set Server Order Preference, use this value instead.
        pub triggers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SslNegotiationPolicyArgs,
    ) -> SslNegotiationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attributes_binding = args.attributes.get_output(context);
        let lb_port_binding = args.lb_port.get_output(context);
        let load_balancer_binding = args.load_balancer.get_output(context);
        let name_binding = args.name.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elb/sslNegotiationPolicy:SslNegotiationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbPort".into(),
                    value: lb_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancer".into(),
                    value: load_balancer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: triggers_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SslNegotiationPolicyResult {
            attributes: o.get_field("attributes"),
            lb_port: o.get_field("lbPort"),
            load_balancer: o.get_field("loadBalancer"),
            name: o.get_field("name"),
            triggers: o.get_field("triggers"),
        }
    }
}
