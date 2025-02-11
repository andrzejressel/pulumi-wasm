/// Creates a new Amazon Redshift endpoint access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAccessArgs {
        /// The cluster identifier of the cluster to access.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Redshift-managed VPC endpoint name.
        #[builder(into)]
        pub endpoint_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.
        #[builder(into, default)]
        pub resource_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.
        #[builder(into)]
        pub subnet_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointAccessResult {
        /// The DNS address of the endpoint.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier of the cluster to access.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Redshift-managed VPC endpoint name.
        pub endpoint_name: pulumi_gestalt_rust::Output<String>,
        /// The port number on which the cluster accepts incoming connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.
        pub subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// The connection endpoint for connecting to an Amazon Redshift cluster through the proxy. See details below.
        pub vpc_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redshift::EndpointAccessVpcEndpoint>,
        >,
        /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAccessArgs,
    ) -> EndpointAccessResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let endpoint_name_binding = args.endpoint_name.get_output(context);
        let resource_owner_binding = args.resource_owner.get_output(context);
        let subnet_group_name_binding = args.subnet_group_name.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/endpointAccess:EndpointAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointName".into(),
                    value: &endpoint_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceOwner".into(),
                    value: &resource_owner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetGroupName".into(),
                    value: &subnet_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointAccessResult {
            address: o.get_field("address"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            endpoint_name: o.get_field("endpointName"),
            port: o.get_field("port"),
            resource_owner: o.get_field("resourceOwner"),
            subnet_group_name: o.get_field("subnetGroupName"),
            vpc_endpoints: o.get_field("vpcEndpoints"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
