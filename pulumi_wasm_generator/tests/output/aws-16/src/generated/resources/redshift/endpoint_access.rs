/// Creates a new Amazon Redshift endpoint access.
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
///             .cluster_identifier("${exampleAwsRedshiftCluster.clusterIdentifier}")
///             .endpoint_name("example")
///             .subnet_group_name("${exampleAwsRedshiftSubnetGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift endpoint access using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/endpointAccess:EndpointAccess example example
/// ```
pub mod endpoint_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAccessArgs {
        /// The cluster identifier of the cluster to access.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Redshift-managed VPC endpoint name.
        #[builder(into)]
        pub endpoint_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.
        #[builder(into, default)]
        pub resource_owner: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.
        #[builder(into)]
        pub subnet_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EndpointAccessResult {
        /// The DNS address of the endpoint.
        pub address: pulumi_wasm_rust::Output<String>,
        /// The cluster identifier of the cluster to access.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Redshift-managed VPC endpoint name.
        pub endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The port number on which the cluster accepts incoming connections.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.
        pub subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// The connection endpoint for connecting to an Amazon Redshift cluster through the proxy. See details below.
        pub vpc_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::redshift::EndpointAccessVpcEndpoint>,
        >,
        /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EndpointAccessArgs,
    ) -> EndpointAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let endpoint_name_binding = args.endpoint_name.get_output(context).get_inner();
        let resource_owner_binding = args.resource_owner.get_output(context).get_inner();
        let subnet_group_name_binding = args
            .subnet_group_name
            .get_output(context)
            .get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/endpointAccess:EndpointAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "endpointName".into(),
                    value: &endpoint_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceOwner".into(),
                    value: &resource_owner_binding,
                },
                register_interface::ObjectField {
                    name: "subnetGroupName".into(),
                    value: &subnet_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "address".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "endpointName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwner".into(),
                },
                register_interface::ResultField {
                    name: "subnetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointAccessResult {
            address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("address").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            endpoint_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwner").unwrap(),
            ),
            subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetGroupName").unwrap(),
            ),
            vpc_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpoints").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
