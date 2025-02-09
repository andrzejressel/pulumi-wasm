/// Creates a new Google SQL User on a Google SQL User Instance. For more information, see the [official documentation](https://cloud.google.com/sql/), or the [JSON API](https://cloud.google.com/sql/docs/admin-api/v1beta4/users).
///
///
///
/// ## Example Usage
///
/// Example creating a SQL User.
///
/// ```yaml
/// resources:
///   dbNameSuffix:
///     type: random:RandomId
///     name: db_name_suffix
///     properties:
///       byteLength: 4
///   main:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: main-instance-${dbNameSuffix.hex}
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-f1-micro
///   users:
///     type: gcp:sql:User
///     properties:
///       name: me
///       instance: ${main.name}
///       host: me.com
///       password: changeme
/// ```
///
/// Example using [Cloud SQL IAM database authentication](https://cloud.google.com/sql/docs/mysql/authentication).
///
/// ```yaml
/// resources:
///   dbNameSuffix:
///     type: random:RandomId
///     name: db_name_suffix
///     properties:
///       byteLength: 4
///   main:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: main-instance-${dbNameSuffix.hex}
///       databaseVersion: POSTGRES_15
///       settings:
///         tier: db-f1-micro
///         databaseFlags:
///           - name: cloudsql.iam_authentication
///             value: on
///   iamUser:
///     type: gcp:sql:User
///     name: iam_user
///     properties:
///       name: me@example.com
///       instance: ${main.name}
///       type: CLOUD_IAM_USER
///   iamServiceAccountUser:
///     type: gcp:sql:User
///     name: iam_service_account_user
///     properties:
///       name:
///         fn::invoke:
///           function: std:trimsuffix
///           arguments:
///             input: ${serviceAccount.email}
///             suffix: .gserviceaccount.com
///           return: result
///       instance: ${main.name}
///       type: CLOUD_IAM_SERVICE_ACCOUNT
/// ```
///
/// Example using [Cloud SQL IAM Group authentication](https://cloud.google.com/sql/docs/mysql/iam-authentication#iam-group-auth).
///
/// ```yaml
/// resources:
///   dbNameSuffix:
///     type: random:RandomId
///     name: db_name_suffix
///     properties:
///       byteLength: 4
///   main:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: main-instance-${dbNameSuffix.hex}
///       databaseVersion: MYSQL_8_0
///       settings:
///         tier: db-f1-micro
///         databaseFlags:
///           - name: cloudsql_iam_authentication
///             value: on
///   iamGroupUser:
///     type: gcp:sql:User
///     name: iam_group_user
///     properties:
///       name: iam_group@example.com
///       instance: ${main.name}
///       type: CLOUD_IAM_GROUP
/// ```
///
/// ## Import
///
/// SQL users for MySQL databases can be imported using the `project`, `instance`, `host` and `name`, e.g.
///
/// * `{{project_id}}/{{instance}}/{{host}}/{{name}}`
///
/// SQL users for PostgreSQL databases can be imported using the `project`, `instance` and `name`, e.g.
///
/// * `{{project_id}}/{{instance}}/{{name}}`
///
/// When using the `pulumi import` command, NAME_HERE can be imported using one of the formats above. For example:
///
/// MySQL database
///
/// ```sh
/// $ pulumi import gcp:sql/user:User default {{project_id}}/{{instance}}/{{host}}/{{name}}
/// ```
///
/// PostgreSQL database
///
/// ```sh
/// $ pulumi import gcp:sql/user:User default {{project_id}}/{{instance}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The deletion policy for the user.
        /// Setting `ABANDON` allows the resource to be abandoned rather than deleted. This is useful
        /// for Postgres, where users cannot be deleted from the API if they have been granted SQL roles.
        ///
        /// Possible values are: `ABANDON`.
        ///
        /// - - -
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The host the user can connect from. This is only supported
        /// for BUILT_IN users in MySQL instances. Don't set this field for PostgreSQL and SQL Server instances.
        /// Can be an IP address. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub host: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud SQL instance. Changing this
        /// forces a new resource to be created.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the user. Changing this forces a new resource
        /// to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password for the user. Can be updated. For Postgres
        /// instances this is a Required field, unless type is set to either CLOUD_IAM_USER
        /// or CLOUD_IAM_SERVICE_ACCOUNT. Don't set this field for CLOUD_IAM_USER
        /// and CLOUD_IAM_SERVICE_ACCOUNT user types for any Cloud SQL instance.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub password_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sql::UserPasswordPolicy>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user type. It determines the method to authenticate the
        /// user during login. The default is the database's built-in user type. Flags
        /// include "BUILT_IN", "CLOUD_IAM_USER", "CLOUD_IAM_SERVICE_ACCOUNT", "CLOUD_IAM_GROUP",
        /// "CLOUD_IAM_GROUP_USER" and "CLOUD_IAM_GROUP_SERVICE_ACCOUNT" for
        /// [Postgres](https://cloud.google.com/sql/docs/postgres/admin-api/rest/v1beta4/users#sqlusertype)
        /// and [MySQL](https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1beta4/users#sqlusertype).
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// The deletion policy for the user.
        /// Setting `ABANDON` allows the resource to be abandoned rather than deleted. This is useful
        /// for Postgres, where users cannot be deleted from the API if they have been granted SQL roles.
        ///
        /// Possible values are: `ABANDON`.
        ///
        /// - - -
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The host the user can connect from. This is only supported
        /// for BUILT_IN users in MySQL instances. Don't set this field for PostgreSQL and SQL Server instances.
        /// Can be an IP address. Changing this forces a new resource to be created.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cloud SQL instance. Changing this
        /// forces a new resource to be created.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// The name of the user. Changing this forces a new resource
        /// to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password for the user. Can be updated. For Postgres
        /// instances this is a Required field, unless type is set to either CLOUD_IAM_USER
        /// or CLOUD_IAM_SERVICE_ACCOUNT. Don't set this field for CLOUD_IAM_USER
        /// and CLOUD_IAM_SERVICE_ACCOUNT user types for any Cloud SQL instance.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        pub password_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::sql::UserPasswordPolicy>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        pub sql_server_user_details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::sql::UserSqlServerUserDetail>,
        >,
        /// The user type. It determines the method to authenticate the
        /// user during login. The default is the database's built-in user type. Flags
        /// include "BUILT_IN", "CLOUD_IAM_USER", "CLOUD_IAM_SERVICE_ACCOUNT", "CLOUD_IAM_GROUP",
        /// "CLOUD_IAM_GROUP_USER" and "CLOUD_IAM_GROUP_SERVICE_ACCOUNT" for
        /// [Postgres](https://cloud.google.com/sql/docs/postgres/admin-api/rest/v1beta4/users#sqlusertype)
        /// and [MySQL](https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1beta4/users#sqlusertype).
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
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
        let deletion_policy_binding_1 = args.deletion_policy.get_output(context);
        let deletion_policy_binding = deletion_policy_binding_1.get_inner();
        let host_binding_1 = args.host.get_output(context);
        let host_binding = host_binding_1.get_inner();
        let instance_binding_1 = args.instance.get_output(context);
        let instance_binding = instance_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let password_binding_1 = args.password.get_output(context);
        let password_binding = password_binding_1.get_inner();
        let password_policy_binding_1 = args.password_policy.get_output(context);
        let password_policy_binding = password_policy_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:sql/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "passwordPolicy".into(),
                    value: &password_policy_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserResult {
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            password_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("passwordPolicy"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            sql_server_user_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sqlServerUserDetails"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
