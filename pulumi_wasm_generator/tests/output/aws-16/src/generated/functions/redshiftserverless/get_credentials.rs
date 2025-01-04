pub mod get_credentials {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCredentialsArgs {
        /// The name of the database to get temporary authorization to log on to.
        #[builder(into, default)]
        pub db_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of seconds until the returned temporary password expires. The minimum is 900 seconds, and the maximum is 3600 seconds.
        #[builder(into, default)]
        pub duration_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the workgroup associated with the database.
        #[builder(into)]
        pub workgroup_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCredentialsResult {
        pub db_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Temporary password that authorizes the user name returned by `db_user` to log on to the database `db_name`.
        pub db_password: pulumi_wasm_rust::Output<String>,
        /// A database user name that is authorized to log on to the database `db_name` using the password `db_password` . If the specified `db_user` exists in the database, the new user name has the same database privileges as the user named in `db_user` . By default, the user is added to PUBLIC. the user doesn't exist in the database.
        pub db_user: pulumi_wasm_rust::Output<String>,
        pub duration_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Date and time the password in `db_password` expires.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub workgroup_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCredentialsArgs) -> GetCredentialsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_name_binding = args.db_name.get_inner();
        let duration_seconds_binding = args.duration_seconds.get_inner();
        let workgroup_name_binding = args.workgroup_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshiftserverless/getCredentials:getCredentials".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbName".into(),
                    value: &db_name_binding,
                },
                register_interface::ObjectField {
                    name: "durationSeconds".into(),
                    value: &duration_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "workgroupName".into(),
                    value: &workgroup_name_binding,
                },
            ]),
            results: Vec::from([
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
        GetCredentialsResult {
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
            workgroup_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workgroupName").unwrap(),
            ),
        }
    }
}
