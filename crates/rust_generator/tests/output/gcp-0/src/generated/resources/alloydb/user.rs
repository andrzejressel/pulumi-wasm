/// A database user in an AlloyDB cluster.
///
///
/// To get more information about User, see:
///
/// * [API documentation](https://cloud.google.com/alloydb/docs/reference/rest/v1/projects.locations.clusters.users/create)
/// * How-to Guides
///     * [AlloyDB](https://cloud.google.com/alloydb/docs/)
///
/// ## Example Usage
///
/// ### Alloydb User Builtin
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Instance
///     properties:
///       cluster: ${defaultCluster.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   defaultCluster:
///     type: gcp:alloydb:Cluster
///     name: default
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultGoogleComputeNetwork.id}
///       initialUser:
///         password: cluster_secret
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${defaultNetwork.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${defaultNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   user1:
///     type: gcp:alloydb:User
///     properties:
///       cluster: ${defaultCluster.name}
///       userId: user1
///       userType: ALLOYDB_BUILT_IN
///       password: user_secret
///       databaseRoles:
///         - alloydbsuperuser
///     options:
///       dependsOn:
///         - ${default}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Alloydb User Iam
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Instance
///     properties:
///       cluster: ${defaultCluster.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   defaultCluster:
///     type: gcp:alloydb:Cluster
///     name: default
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultNetwork.id}
///       initialUser:
///         password: cluster_secret
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${defaultNetwork.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${defaultNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   user2:
///     type: gcp:alloydb:User
///     properties:
///       cluster: ${defaultCluster.name}
///       userId: user2@foo.com
///       userType: ALLOYDB_IAM_USER
///       databaseRoles:
///         - alloydbiamuser
///     options:
///       dependsOn:
///         - ${default}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// User can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/users/{{user_id}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{user_id}}`
///
/// * `{{location}}/{{cluster}}/{{user_id}}`
///
/// When using the `pulumi import` command, User can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:alloydb/user:User default projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/users/{{user_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/user:User default {{project}}/{{location}}/{{cluster}}/{{user_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/user:User default {{location}}/{{cluster}}/{{user_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Identifies the alloydb cluster. Must be in the format
        /// 'projects/{project}/locations/{location}/clusters/{cluster_id}'
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of database roles this database user has.
        #[builder(into, default)]
        pub database_roles: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Password for this database user.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The database role name of the user.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of this user.
        /// Possible values are: `ALLOYDB_BUILT_IN`, `ALLOYDB_IAM_USER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub user_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Identifies the alloydb cluster. Must be in the format
        /// 'projects/{project}/locations/{location}/clusters/{cluster_id}'
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// List of database roles this database user has.
        pub database_roles: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the resource in the form of projects/{project}/locations/{location}/clusters/{cluster}/users/{user}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Password for this database user.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The database role name of the user.
        pub user_id: pulumi_gestalt_rust::Output<String>,
        /// The type of this user.
        /// Possible values are: `ALLOYDB_BUILT_IN`, `ALLOYDB_IAM_USER`.
        ///
        ///
        /// - - -
        pub user_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let database_roles_binding = args.database_roles.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let user_type_binding = args.user_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:alloydb/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "databaseRoles".into(),
                    value: &database_roles_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
                register_interface::ObjectField {
                    name: "userType".into(),
                    value: &user_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserResult {
            cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            database_roles: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseRoles"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            user_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userId"),
            ),
            user_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userType"),
            ),
        }
    }
}
