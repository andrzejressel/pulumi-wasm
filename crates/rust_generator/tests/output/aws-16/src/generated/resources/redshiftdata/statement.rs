/// Executes a Redshift Data Statement.
///
/// ## Example Usage
///
/// ### cluster_identifier
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = statement::create(
///         "example",
///         StatementArgs::builder()
///             .cluster_identifier("${exampleAwsRedshiftCluster.clusterIdentifier}")
///             .database("${exampleAwsRedshiftCluster.databaseName}")
///             .db_user("${exampleAwsRedshiftCluster.masterUsername}")
///             .sql("CREATE GROUP group_name;")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### workgroup_name
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = statement::create(
///         "example",
///         StatementArgs::builder()
///             .database("dev")
///             .sql("CREATE GROUP group_name;")
///             .workgroup_name("${exampleAwsRedshiftserverlessWorkgroup.workgroupName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Data Statements using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftdata/statement:Statement example example
/// ```
pub mod statement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StatementArgs {
        /// The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials.
        #[builder(into, default)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the database.
        #[builder(into)]
        pub database: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The database user name.
        #[builder(into, default)]
        pub db_user: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::redshiftdata::StatementParameter>>,
        >,
        /// The name or ARN of the secret that enables access to the database.
        #[builder(into, default)]
        pub secret_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The SQL statement text to run.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub sql: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SQL statement. You can name the SQL statement when you create it to identify the query.
        #[builder(into, default)]
        pub statement_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A value that indicates whether to send an event to the Amazon EventBridge event bus after the SQL statement runs.
        #[builder(into, default)]
        pub with_event: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The serverless workgroup name. This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.
        #[builder(into, default)]
        pub workgroup_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StatementResult {
        /// The cluster identifier. This parameter is required when connecting to a cluster and authenticating using either Secrets Manager or temporary credentials.
        pub cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the database.
        pub database: pulumi_gestalt_rust::Output<String>,
        /// The database user name.
        pub db_user: pulumi_gestalt_rust::Output<Option<String>>,
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::redshiftdata::StatementParameter>>,
        >,
        /// The name or ARN of the secret that enables access to the database.
        pub secret_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SQL statement text to run.
        ///
        /// The following arguments are optional:
        pub sql: pulumi_gestalt_rust::Output<String>,
        /// The name of the SQL statement. You can name the SQL statement when you create it to identify the query.
        pub statement_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A value that indicates whether to send an event to the Amazon EventBridge event bus after the SQL statement runs.
        pub with_event: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The serverless workgroup name. This parameter is required when connecting to a serverless workgroup and authenticating using either Secrets Manager or temporary credentials.
        pub workgroup_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StatementArgs,
    ) -> StatementResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let db_user_binding = args.db_user.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let secret_arn_binding = args.secret_arn.get_output(context).get_inner();
        let sql_binding = args.sql.get_output(context).get_inner();
        let statement_name_binding = args.statement_name.get_output(context).get_inner();
        let with_event_binding = args.with_event.get_output(context).get_inner();
        let workgroup_name_binding = args.workgroup_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshiftdata/statement:Statement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "dbUser".into(),
                    value: &db_user_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "secretArn".into(),
                    value: &secret_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sql".into(),
                    value: &sql_binding,
                },
                register_interface::ObjectField {
                    name: "statementName".into(),
                    value: &statement_name_binding,
                },
                register_interface::ObjectField {
                    name: "withEvent".into(),
                    value: &with_event_binding,
                },
                register_interface::ObjectField {
                    name: "workgroupName".into(),
                    value: &workgroup_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StatementResult {
            cluster_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            database: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            db_user: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbUser"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            secret_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretArn"),
            ),
            sql: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sql")),
            statement_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statementName"),
            ),
            with_event: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("withEvent"),
            ),
            workgroup_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workgroupName"),
            ),
        }
    }
}
