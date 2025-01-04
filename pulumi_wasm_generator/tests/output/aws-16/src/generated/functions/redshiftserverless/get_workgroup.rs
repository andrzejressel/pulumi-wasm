pub mod get_workgroup {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkgroupArgs {
        /// The name of the workgroup associated with the database.
        #[builder(into)]
        pub workgroup_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkgroupResult {
        /// Amazon Resource Name (ARN) of the Redshift Serverless Workgroup.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The endpoint that is created from the workgroup. See `Endpoint` below.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redshiftserverless::GetWorkgroupEndpoint>,
        >,
        /// The value that specifies whether to turn on enhanced virtual private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC instead of over the internet.
        pub enhanced_vpc_routing: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// A value that specifies whether the workgroup can be accessed from a public network.
        pub publicly_accessible: pulumi_wasm_rust::Output<bool>,
        /// An array of security group IDs to associate with the workgroup.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// An array of VPC subnet IDs to associate with the workgroup. When set, must contain at least three subnets spanning three Availability Zones. A minimum number of IP addresses is required and scales with the Base Capacity. For more information, see the following [AWS document](https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-known-issues.html).
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Redshift Workgroup ID.
        pub workgroup_id: pulumi_wasm_rust::Output<String>,
        pub workgroup_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetWorkgroupArgs) -> GetWorkgroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let workgroup_name_binding = args.workgroup_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshiftserverless/getWorkgroup:getWorkgroup".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "workgroupName".into(),
                    value: &workgroup_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "enhancedVpcRouting".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "workgroupId".into(),
                },
                register_interface::ResultField {
                    name: "workgroupName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWorkgroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            enhanced_vpc_routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enhancedVpcRouting").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            workgroup_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workgroupId").unwrap(),
            ),
            workgroup_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workgroupName").unwrap(),
            ),
        }
    }
}
