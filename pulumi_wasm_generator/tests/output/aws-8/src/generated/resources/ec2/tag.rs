/// Manages an individual EC2 resource tag. This resource should only be used in cases where EC2 resources are created outside the provider (e.g. AMIs), being shared via Resource Access Manager (RAM), or implicitly created by other means (e.g. Transit Gateway VPN Attachments).
///
/// > **NOTE:** This tagging resource should not be combined with the providers resource for managing the parent resource. For example, using `aws.ec2.Vpc` and `aws.ec2.Tag` to manage tags of the same VPC will cause a perpetual difference where the `aws.ec2.Vpc` resource will try to remove the tag being added by the `aws.ec2.Tag` resource.
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:TransitGateway
///   exampleCustomerGateway:
///     type: aws:ec2:CustomerGateway
///     name: example
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.0.0.1
///       type: ipsec.1
///   exampleVpnConnection:
///     type: aws:ec2:VpnConnection
///     name: example
///     properties:
///       customerGatewayId: ${exampleCustomerGateway.id}
///       transitGatewayId: ${example.id}
///       type: ${exampleCustomerGateway.type}
///   exampleTag:
///     type: aws:ec2:Tag
///     name: example
///     properties:
///       resourceId: ${exampleVpnConnection.transitGatewayAttachmentId}
///       key: Name
///       value: Hello World
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_tag` using the EC2 resource identifier and key, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/tag:Tag example tgw-attach-1234567890abcdef,Name
/// ```
pub mod tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// The tag name.
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// The ID of the EC2 resource to manage the tag for.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The value of the tag.
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// The tag name.
        pub key: pulumi_wasm_rust::Output<String>,
        /// The ID of the EC2 resource to manage the tag for.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The value of the tag.
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TagArgs) -> TagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_binding = args.key.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/tag:Tag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagResult {
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
