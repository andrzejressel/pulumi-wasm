/// Creates a new Amazon Redshift endpoint authorization.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint_authorization::create(
///         "example",
///         EndpointAuthorizationArgs::builder()
///             .account("01234567910")
///             .cluster_identifier("${exampleAwsRedshiftCluster.clusterIdentifier}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift endpoint authorization using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/endpointAuthorization:EndpointAuthorization example 01234567910:cluster-example-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAuthorizationArgs {
        /// The Amazon Web Services account ID to grant access to.
        #[builder(into)]
        pub account: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The cluster identifier of the cluster to grant access to.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether to force the revoke action. If true, the Redshift-managed VPC endpoints associated with the endpoint authorization are also deleted. Default value is `false`.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The virtual private cloud (VPC) identifiers to grant access to. If none are specified all VPCs in shared account are allowed.
        #[builder(into, default)]
        pub vpc_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EndpointAuthorizationResult {
        /// The Amazon Web Services account ID to grant access to.
        pub account: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether all VPCs in the grantee account are allowed access to the cluster.
        pub allowed_all_vpcs: pulumi_gestalt_rust::Output<bool>,
        /// The cluster identifier of the cluster to grant access to.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The number of Redshift-managed VPC endpoints created for the authorization.
        pub endpoint_count: pulumi_gestalt_rust::Output<i32>,
        /// Indicates whether to force the revoke action. If true, the Redshift-managed VPC endpoints associated with the endpoint authorization are also deleted. Default value is `false`.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Web Services account ID of the grantee of the cluster.
        pub grantee: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Web Services account ID of the cluster owner.
        pub grantor: pulumi_gestalt_rust::Output<String>,
        /// The virtual private cloud (VPC) identifiers to grant access to. If none are specified all VPCs in shared account are allowed.
        pub vpc_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointAuthorizationArgs,
    ) -> EndpointAuthorizationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_binding_1 = args.account.get_output(context);
        let account_binding = account_binding_1.get_inner();
        let cluster_identifier_binding_1 = args.cluster_identifier.get_output(context);
        let cluster_identifier_binding = cluster_identifier_binding_1.get_inner();
        let force_delete_binding_1 = args.force_delete.get_output(context);
        let force_delete_binding = force_delete_binding_1.get_inner();
        let vpc_ids_binding_1 = args.vpc_ids.get_output(context);
        let vpc_ids_binding = vpc_ids_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/endpointAuthorization:EndpointAuthorization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "account".into(),
                    value: &account_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "vpcIds".into(),
                    value: &vpc_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointAuthorizationResult {
            account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("account"),
            ),
            allowed_all_vpcs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowedAllVpcs"),
            ),
            cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            endpoint_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointCount"),
            ),
            force_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            grantee: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantee"),
            ),
            grantor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("grantor"),
            ),
            vpc_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcIds"),
            ),
        }
    }
}
