/// This resource attaches a security group to an Elastic Network Interface (ENI).
/// It can be used to attach a security group to any existing ENI, be it a
/// secondary ENI or one attached as the primary interface on an instance.
///
/// > **NOTE on instances, interfaces, and security groups:** This provider currently
/// provides the capability to assign security groups via the [`aws.ec2.Instance`][1]
/// and the [`aws.ec2.NetworkInterface`][2] resources. Using this resource in
/// conjunction with security groups provided in-line in those resources will cause
/// conflicts, and will lead to spurious diffs and undefined behavior - please use
/// one or the other.
///
/// ## Example Usage
///
/// The following provides a very basic example of setting up an instance (provided
/// by `instance`) in the default security group, creating a security group
/// (provided by `sg`) and then attaching the security group to the instance's
/// primary network interface via the `aws.ec2.NetworkInterfaceSecurityGroupAttachment` resource,
/// named `sg_attachment`:
///
/// ```yaml
/// resources:
///   instance:
///     type: aws:ec2:Instance
///     properties:
///       instanceType: t2.micro
///       ami: ${ami.id}
///       tags:
///         type: test-instance
///   sg:
///     type: aws:ec2:SecurityGroup
///     properties:
///       tags:
///         type: test-security-group
///   sgAttachment:
///     type: aws:ec2:NetworkInterfaceSecurityGroupAttachment
///     name: sg_attachment
///     properties:
///       securityGroupId: ${sg.id}
///       networkInterfaceId: ${instance.primaryNetworkInterfaceId}
/// variables:
///   ami:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         filters:
///           - name: name
///             values:
///               - amzn-ami-hvm-*
///         owners:
///           - amazon
/// ```
///
/// In this example, `instance` is provided by the `aws.ec2.Instance` data source,
/// fetching an external instance, possibly not managed by this provider.
/// `sg_attachment` then attaches to the output instance's `network_interface_id`:
///
/// ```yaml
/// resources:
///   sg:
///     type: aws:ec2:SecurityGroup
///     properties:
///       tags:
///         type: test-security-group
///   sgAttachment:
///     type: aws:ec2:NetworkInterfaceSecurityGroupAttachment
///     name: sg_attachment
///     properties:
///       securityGroupId: ${sg.id}
///       networkInterfaceId: ${instance.networkInterfaceId}
/// variables:
///   instance:
///     fn::invoke:
///       function: aws:ec2:getInstance
///       arguments:
///         instanceId: i-1234567890abcdef0
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Interface Security Group attachments using the associated network interface ID and security group ID, separated by an underscore (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkInterfaceSecurityGroupAttachment:NetworkInterfaceSecurityGroupAttachment sg_attachment eni-1234567890abcdef0_sg-1234567890abcdef0
/// ```
pub mod network_interface_security_group_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceSecurityGroupAttachmentArgs {
        /// The ID of the network interface to attach to.
        #[builder(into)]
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group.
        #[builder(into)]
        pub security_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceSecurityGroupAttachmentResult {
        /// The ID of the network interface to attach to.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkInterfaceSecurityGroupAttachmentArgs,
    ) -> NetworkInterfaceSecurityGroupAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let security_group_id_binding = args.security_group_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/networkInterfaceSecurityGroupAttachment:NetworkInterfaceSecurityGroupAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkInterfaceSecurityGroupAttachmentResult {
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
        }
    }
}
