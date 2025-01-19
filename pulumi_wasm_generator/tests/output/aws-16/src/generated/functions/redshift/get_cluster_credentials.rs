pub mod get_cluster_credentials {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterCredentialsArgs {
        /// Create a database user with the name specified for the user named in `db_user` if one does not exist.
        #[builder(into, default)]
        pub auto_create: pulumi_wasm_rust::Output<Option<bool>>,
        /// Unique identifier of the cluster that contains the database for which your are requesting credentials.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// List of the names of existing database groups that the user named in `db_user` will join for the current session, in addition to any group memberships for an existing user. If not specified, a new user is added only to `PUBLIC`.
        #[builder(into, default)]
        pub db_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of a database that DbUser is authorized to log on to. If `db_name` is not specified, `db_user` can log on to any existing database.
        #[builder(into, default)]
        pub db_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of a database user. If a user name matching `db_user` exists in the database, the temporary user credentials have the same permissions as the  existing user. If `db_user` doesn't exist in the database and `auto_create` is `True`, a new user is created using the value for `db_user` with `PUBLIC` permissions.  If a database user matching the value for `db_user` doesn't exist and `not` is `False`, then the command succeeds but the connection attempt will fail because the user doesn't exist in the database.
        #[builder(into)]
        pub db_user: pulumi_wasm_rust::Output<String>,
        /// The number of seconds until the returned temporary password expires. Valid values are between `900` and `3600`. Default value is `900`.
        #[builder(into, default)]
        pub duration_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetClusterCredentialsResult {
        pub auto_create: pulumi_wasm_rust::Output<Option<bool>>,
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        pub db_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub db_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Temporary password that authorizes the user name returned by `db_user` to log on to the database `db_name`.
        pub db_password: pulumi_wasm_rust::Output<String>,
        pub db_user: pulumi_wasm_rust::Output<String>,
        pub duration_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Date and time the password in `db_password` expires.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetClusterCredentialsArgs) -> GetClusterCredentialsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_create_binding = args.auto_create.get_inner();
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let db_groups_binding = args.db_groups.get_inner();
        let db_name_binding = args.db_name.get_inner();
        let db_user_binding = args.db_user.get_inner();
        let duration_seconds_binding = args.duration_seconds.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshift/getClusterCredentials:getClusterCredentials".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoCreate".into(),
                    value: &auto_create_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbGroups".into(),
                    value: &db_groups_binding,
                },
                register_interface::ObjectField {
                    name: "dbName".into(),
                    value: &db_name_binding,
                },
                register_interface::ObjectField {
                    name: "dbUser".into(),
                    value: &db_user_binding,
                },
                register_interface::ObjectField {
                    name: "durationSeconds".into(),
                    value: &duration_seconds_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoCreate".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbGroups".into(),
                },
                register_interface::ResultField {
                    name: "dbName".into(),
                },
                register_interface::ResultField {
                    name: "dbPassword".into(),
                },
                register_interface::ResultField {
                    name: "dbUser".into(),
                },
                register_interface::ResultField {
                    name: "durationSeconds".into(),
                },
                register_interface::ResultField {
                    name: "expiration".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterCredentialsResult {
            auto_create: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoCreate").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            db_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbGroups").unwrap(),
            ),
            db_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbName").unwrap(),
            ),
            db_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbPassword").unwrap(),
            ),
            db_user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbUser").unwrap(),
            ),
            duration_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("durationSeconds").unwrap(),
            ),
            expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiration").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
