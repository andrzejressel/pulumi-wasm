#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_credentials {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCredentialsArgs {
        /// The name of the database to get temporary authorization to log on to.
        #[builder(into, default)]
        pub db_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of seconds until the returned temporary password expires. The minimum is 900 seconds, and the maximum is 3600 seconds.
        #[builder(into, default)]
        pub duration_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the workgroup associated with the database.
        #[builder(into)]
        pub workgroup_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCredentialsResult {
        pub db_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Temporary password that authorizes the user name returned by `db_user` to log on to the database `db_name`.
        pub db_password: pulumi_gestalt_rust::Output<String>,
        /// A database user name that is authorized to log on to the database `db_name` using the password `db_password` . If the specified `db_user` exists in the database, the new user name has the same database privileges as the user named in `db_user` . By default, the user is added to PUBLIC. the user doesn't exist in the database.
        pub db_user: pulumi_gestalt_rust::Output<String>,
        pub duration_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Date and time the password in `db_password` expires.
        pub expiration: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub workgroup_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCredentialsArgs,
    ) -> GetCredentialsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let db_name_binding_1 = args.db_name.get_output(context);
        let db_name_binding = db_name_binding_1.get_inner();
        let duration_seconds_binding_1 = args.duration_seconds.get_output(context);
        let duration_seconds_binding = duration_seconds_binding_1.get_inner();
        let workgroup_name_binding_1 = args.workgroup_name.get_output(context);
        let workgroup_name_binding = workgroup_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshiftserverless/getCredentials:getCredentials".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCredentialsResult {
            db_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbName"),
            ),
            db_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbPassword"),
            ),
            db_user: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbUser"),
            ),
            duration_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("durationSeconds"),
            ),
            expiration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiration"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            workgroup_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workgroupName"),
            ),
        }
    }
}
