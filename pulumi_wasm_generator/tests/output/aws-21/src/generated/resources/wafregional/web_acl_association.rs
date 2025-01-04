/// Manages an association with WAF Regional Web ACL.
///
/// > **Note:** An Application Load Balancer can only be associated with one WAF Regional WebACL.
///
/// ## Example Usage
///
/// ### Application Load Balancer Association
///
/// ```yaml
/// resources:
///   ipset:
///     type: aws:wafregional:IpSet
///     properties:
///       name: tfIPSet
///       ipSetDescriptors:
///         - type: IPV4
///           value: 192.0.7.0/24
///   foo:
///     type: aws:wafregional:Rule
///     properties:
///       name: tfWAFRule
///       metricName: tfWAFRule
///       predicates:
///         - dataId: ${ipset.id}
///           negated: false
///           type: IPMatch
///   fooWebAcl:
///     type: aws:wafregional:WebAcl
///     name: foo
///     properties:
///       name: foo
///       metricName: foo
///       defaultAction:
///         type: ALLOW
///       rules:
///         - action:
///             type: BLOCK
///           priority: 1
///           ruleId: ${foo.id}
///   fooVpc:
///     type: aws:ec2:Vpc
///     name: foo
///     properties:
///       cidrBlock: 10.1.0.0/16
///   fooSubnet:
///     type: aws:ec2:Subnet
///     name: foo
///     properties:
///       vpcId: ${fooVpc.id}
///       cidrBlock: 10.1.1.0/24
///       availabilityZone: ${available.names[0]}
///   bar:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${fooVpc.id}
///       cidrBlock: 10.1.2.0/24
///       availabilityZone: ${available.names[1]}
///   fooLoadBalancer:
///     type: aws:alb:LoadBalancer
///     name: foo
///     properties:
///       internal: true
///       subnets:
///         - ${fooSubnet.id}
///         - ${bar.id}
///   fooWebAclAssociation:
///     type: aws:wafregional:WebAclAssociation
///     name: foo
///     properties:
///       resourceArn: ${fooLoadBalancer.arn}
///       webAclId: ${fooWebAcl.id}
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Web ACL Association using their `web_acl_id:resource_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/webAclAssociation:WebAclAssociation foo web_acl_id:resource_arn
/// ```
pub mod web_acl_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclAssociationArgs {
        /// ARN of the resource to associate with. For example, an Application Load Balancer or API Gateway Stage.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the WAF Regional WebACL to create an association.
        #[builder(into)]
        pub web_acl_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WebAclAssociationResult {
        /// ARN of the resource to associate with. For example, an Application Load Balancer or API Gateway Stage.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the WAF Regional WebACL to create an association.
        pub web_acl_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WebAclAssociationArgs) -> WebAclAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_arn_binding = args.resource_arn.get_inner();
        let web_acl_id_binding = args.web_acl_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/webAclAssociation:WebAclAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "webAclId".into(),
                    value: &web_acl_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "webAclId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebAclAssociationResult {
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            web_acl_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAclId").unwrap(),
            ),
        }
    }
}
