/// Provides an OpsWorks RDS DB Instance resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myInstance = rds_db_instance::create(
///         "myInstance",
///         RdsDbInstanceArgs::builder()
///             .db_password("somePass")
///             .db_user("someUser")
///             .rds_db_instance_arn("${myInstanceAwsDbInstance.arn}")
///             .stack_id("${myStack.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rds_db_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RdsDbInstanceArgs {
        /// A db password
        #[builder(into)]
        pub db_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A db username
        #[builder(into)]
        pub db_user: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The db instance to register for this stack. Changing this will force a new resource.
        #[builder(into)]
        pub rds_db_instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The stack to register a db instance for. Changing this will force a new resource.
        #[builder(into)]
        pub stack_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RdsDbInstanceResult {
        /// A db password
        pub db_password: pulumi_gestalt_rust::Output<String>,
        /// A db username
        pub db_user: pulumi_gestalt_rust::Output<String>,
        /// The db instance to register for this stack. Changing this will force a new resource.
        pub rds_db_instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The stack to register a db instance for. Changing this will force a new resource.
        pub stack_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RdsDbInstanceArgs,
    ) -> RdsDbInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_password_binding = args.db_password.get_output(context);
        let db_user_binding = args.db_user.get_output(context);
        let rds_db_instance_arn_binding = args.rds_db_instance_arn.get_output(context);
        let stack_id_binding = args.stack_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opsworks/rdsDbInstance:RdsDbInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbPassword".into(),
                    value: &db_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbUser".into(),
                    value: &db_user_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rdsDbInstanceArn".into(),
                    value: &rds_db_instance_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RdsDbInstanceResult {
            db_password: o.get_field("dbPassword"),
            db_user: o.get_field("dbUser"),
            rds_db_instance_arn: o.get_field("rdsDbInstanceArn"),
            stack_id: o.get_field("stackId"),
        }
    }
}
