#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster_credentials {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterCredentialsArgs {
        /// Create a database user with the name specified for the user named in `db_user` if one does not exist.
        #[builder(into, default)]
        pub auto_create: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Unique identifier of the cluster that contains the database for which your are requesting credentials.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of the names of existing database groups that the user named in `db_user` will join for the current session, in addition to any group memberships for an existing user. If not specified, a new user is added only to `PUBLIC`.
        #[builder(into, default)]
        pub db_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of a database that DbUser is authorized to log on to. If `db_name` is not specified, `db_user` can log on to any existing database.
        #[builder(into, default)]
        pub db_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of a database user. If a user name matching `db_user` exists in the database, the temporary user credentials have the same permissions as the  existing user. If `db_user` doesn't exist in the database and `auto_create` is `True`, a new user is created using the value for `db_user` with `PUBLIC` permissions.  If a database user matching the value for `db_user` doesn't exist and `not` is `False`, then the command succeeds but the connection attempt will fail because the user doesn't exist in the database.
        #[builder(into)]
        pub db_user: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of seconds until the returned temporary password expires. Valid values are between `900` and `3600`. Default value is `900`.
        #[builder(into, default)]
        pub duration_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetClusterCredentialsResult {
        pub auto_create: pulumi_gestalt_rust::Output<Option<bool>>,
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        pub db_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub db_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Temporary password that authorizes the user name returned by `db_user` to log on to the database `db_name`.
        pub db_password: pulumi_gestalt_rust::Output<String>,
        pub db_user: pulumi_gestalt_rust::Output<String>,
        pub duration_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Date and time the password in `db_password` expires.
        pub expiration: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterCredentialsArgs,
    ) -> GetClusterCredentialsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_create_binding = args.auto_create.get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let db_groups_binding = args.db_groups.get_output(context);
        let db_name_binding = args.db_name.get_output(context);
        let db_user_binding = args.db_user.get_output(context);
        let duration_seconds_binding = args.duration_seconds.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:redshift/getClusterCredentials:getClusterCredentials".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoCreate".into(),
                    value: &auto_create_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbGroups".into(),
                    value: &db_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbName".into(),
                    value: &db_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbUser".into(),
                    value: &db_user_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "durationSeconds".into(),
                    value: &duration_seconds_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterCredentialsResult {
            auto_create: o.get_field("autoCreate"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            db_groups: o.get_field("dbGroups"),
            db_name: o.get_field("dbName"),
            db_password: o.get_field("dbPassword"),
            db_user: o.get_field("dbUser"),
            duration_seconds: o.get_field("durationSeconds"),
            expiration: o.get_field("expiration"),
            id: o.get_field("id"),
        }
    }
}
