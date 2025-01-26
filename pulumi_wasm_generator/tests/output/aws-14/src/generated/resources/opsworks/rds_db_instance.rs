/// Provides an OpsWorks RDS DB Instance resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod rds_db_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RdsDbInstanceArgs {
        /// A db password
        #[builder(into)]
        pub db_password: pulumi_wasm_rust::InputOrOutput<String>,
        /// A db username
        #[builder(into)]
        pub db_user: pulumi_wasm_rust::InputOrOutput<String>,
        /// The db instance to register for this stack. Changing this will force a new resource.
        #[builder(into)]
        pub rds_db_instance_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The stack to register a db instance for. Changing this will force a new resource.
        #[builder(into)]
        pub stack_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RdsDbInstanceResult {
        /// A db password
        pub db_password: pulumi_wasm_rust::Output<String>,
        /// A db username
        pub db_user: pulumi_wasm_rust::Output<String>,
        /// The db instance to register for this stack. Changing this will force a new resource.
        pub rds_db_instance_arn: pulumi_wasm_rust::Output<String>,
        /// The stack to register a db instance for. Changing this will force a new resource.
        pub stack_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RdsDbInstanceArgs,
    ) -> RdsDbInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_password_binding = args.db_password.get_output(context).get_inner();
        let db_user_binding = args.db_user.get_output(context).get_inner();
        let rds_db_instance_arn_binding = args
            .rds_db_instance_arn
            .get_output(context)
            .get_inner();
        let stack_id_binding = args.stack_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/rdsDbInstance:RdsDbInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbPassword".into(),
                    value: &db_password_binding,
                },
                register_interface::ObjectField {
                    name: "dbUser".into(),
                    value: &db_user_binding,
                },
                register_interface::ObjectField {
                    name: "rdsDbInstanceArn".into(),
                    value: &rds_db_instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dbPassword".into(),
                },
                register_interface::ResultField {
                    name: "dbUser".into(),
                },
                register_interface::ResultField {
                    name: "rdsDbInstanceArn".into(),
                },
                register_interface::ResultField {
                    name: "stackId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RdsDbInstanceResult {
            db_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbPassword").unwrap(),
            ),
            db_user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbUser").unwrap(),
            ),
            rds_db_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rdsDbInstanceArn").unwrap(),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackId").unwrap(),
            ),
        }
    }
}
