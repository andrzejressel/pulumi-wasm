/// Provides an AWS EIP Association as a top level resource, to associate and
/// disassociate Elastic IPs from AWS Instances and Network Interfaces.
///
/// > **NOTE:** Do not use this resource to associate an EIP to `aws.lb.LoadBalancer` or `aws.ec2.NatGateway` resources. Instead use the `allocation_id` available in those resources to allow AWS to manage the association, otherwise you will see `AuthFailure` errors.
///
/// > **NOTE:** `aws.ec2.EipAssociation` is useful in scenarios where EIPs are either
/// pre-existing or distributed to customers or users and therefore cannot be changed.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   eipAssoc:
///     type: aws:ec2:EipAssociation
///     name: eip_assoc
///     properties:
///       instanceId: ${web.id}
///       allocationId: ${example.id}
///   web:
///     type: aws:ec2:Instance
///     properties:
///       ami: ami-21f78e11
///       availabilityZone: us-west-2a
///       instanceType: t2.micro
///       tags:
///         Name: HelloWorld
///   example:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EIP Assocations using their association IDs. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/eipAssociation:EipAssociation test eipassoc-ab12c345
/// ```
pub mod eip_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EipAssociationArgs {
        /// The allocation ID. This is required for EC2-VPC.
        #[builder(into, default)]
        pub allocation_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to allow an Elastic IP to
        /// be re-associated. Defaults to `true` in VPC.
        #[builder(into, default)]
        pub allow_reassociation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the instance. This is required for
        /// EC2-Classic. For EC2-VPC, you can specify either the instance ID or the
        /// network interface ID, but not both. The operation fails if you specify an
        /// instance ID unless exactly one network interface is attached.
        #[builder(into, default)]
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the network interface. If the
        /// instance has more than one network interface, you must specify a network
        /// interface ID.
        #[builder(into, default)]
        pub network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The primary or secondary private IP address
        /// to associate with the Elastic IP address. If no private IP address is
        /// specified, the Elastic IP address is associated with the primary private IP
        /// address.
        #[builder(into, default)]
        pub private_ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The Elastic IP address. This is required for EC2-Classic.
        #[builder(into, default)]
        pub public_ip: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EipAssociationResult {
        /// The allocation ID. This is required for EC2-VPC.
        pub allocation_id: pulumi_wasm_rust::Output<String>,
        /// Whether to allow an Elastic IP to
        /// be re-associated. Defaults to `true` in VPC.
        pub allow_reassociation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the instance. This is required for
        /// EC2-Classic. For EC2-VPC, you can specify either the instance ID or the
        /// network interface ID, but not both. The operation fails if you specify an
        /// instance ID unless exactly one network interface is attached.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the network interface. If the
        /// instance has more than one network interface, you must specify a network
        /// interface ID.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The primary or secondary private IP address
        /// to associate with the Elastic IP address. If no private IP address is
        /// specified, the Elastic IP address is associated with the primary private IP
        /// address.
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        /// The Elastic IP address. This is required for EC2-Classic.
        pub public_ip: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EipAssociationArgs) -> EipAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_id_binding = args.allocation_id.get_inner();
        let allow_reassociation_binding = args.allow_reassociation.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let private_ip_address_binding = args.private_ip_address.get_inner();
        let public_ip_binding = args.public_ip.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/eipAssociation:EipAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationId".into(),
                    value: &allocation_id_binding,
                },
                register_interface::ObjectField {
                    name: "allowReassociation".into(),
                    value: &allow_reassociation_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpAddress".into(),
                    value: &private_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "publicIp".into(),
                    value: &public_ip_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocationId".into(),
                },
                register_interface::ResultField {
                    name: "allowReassociation".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "publicIp".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EipAssociationResult {
            allocation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationId").unwrap(),
            ),
            allow_reassociation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowReassociation").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddress").unwrap(),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIp").unwrap(),
            ),
        }
    }
}
