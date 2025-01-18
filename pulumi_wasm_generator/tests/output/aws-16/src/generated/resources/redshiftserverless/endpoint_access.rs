/// Creates a new Amazon Redshift Serverless Endpoint Access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint_access::create(
///         "example",
///         EndpointAccessArgs::builder()
///             .endpoint_name("example")
///             .workgroup_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Endpoint Access using the `endpoint_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/endpointAccess:EndpointAccess example example
/// ```
pub mod endpoint_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAccessArgs {
        /// The name of the endpoint.
        #[builder(into)]
        pub endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The owner Amazon Web Services account for the Amazon Redshift Serverless workgroup.
        #[builder(into, default)]
        pub owner_account: pulumi_wasm_rust::Output<Option<String>>,
        /// An array of VPC subnet IDs to associate with the endpoint.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// An array of security group IDs to associate with the workgroup.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the workgroup.
        #[builder(into)]
        pub workgroup_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointAccessResult {
        /// The DNS address of the VPC endpoint.
        pub address: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Endpoint Access.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the endpoint.
        pub endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The owner Amazon Web Services account for the Amazon Redshift Serverless workgroup.
        pub owner_account: pulumi_wasm_rust::Output<Option<String>>,
        /// The port that Amazon Redshift Serverless listens on.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// An array of VPC subnet IDs to associate with the endpoint.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The VPC endpoint or the Redshift Serverless workgroup. See `VPC Endpoint` below.
        pub vpc_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::redshiftserverless::EndpointAccessVpcEndpoint>,
        >,
        /// An array of security group IDs to associate with the workgroup.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the workgroup.
        pub workgroup_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointAccessArgs) -> EndpointAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoint_name_binding = args.endpoint_name.get_inner();
        let owner_account_binding = args.owner_account.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let vpc_security_group_ids_binding = args.vpc_security_group_ids.get_inner();
        let workgroup_name_binding = args.workgroup_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshiftserverless/endpointAccess:EndpointAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpointName".into(),
                    value: &endpoint_name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerAccount".into(),
                    value: &owner_account_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "workgroupName".into(),
                    value: &workgroup_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "address".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "endpointName".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccount".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "workgroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointAccessResult {
            address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("address").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            endpoint_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointName").unwrap(),
            ),
            owner_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccount").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            vpc_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpoints").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
            workgroup_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workgroupName").unwrap(),
            ),
        }
    }
}
