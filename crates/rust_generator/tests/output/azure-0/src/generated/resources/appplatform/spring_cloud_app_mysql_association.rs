/// Associates a Spring Cloud Application with a MySQL Database.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFlexibleDatabase = flexible_database::create(
///         "exampleFlexibleDatabase",
///         FlexibleDatabaseArgs::builder()
///             .charset("utf8")
///             .collation("utf8_unicode_ci")
///             .name("exampledb")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .administrator_login("adminTerraform")
///             .administrator_password("QAZwsx123")
///             .location("${example.location}")
///             .name("example-fsserver")
///             .resource_group_name("${example.name}")
///             .sku_name("B_Standard_B1s")
///             .zone("2")
///             .build_struct(),
///     );
///     let exampleSpringCloudApp = spring_cloud_app::create(
///         "exampleSpringCloudApp",
///         SpringCloudAppArgs::builder()
///             .name("example-springcloudapp")
///             .resource_group_name("${example.name}")
///             .service_name("${exampleSpringCloudService.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudAppMysqlAssociation = spring_cloud_app_mysql_association::create(
///         "exampleSpringCloudAppMysqlAssociation",
///         SpringCloudAppMysqlAssociationArgs::builder()
///             .database_name("${exampleFlexibleDatabase.name}")
///             .mysql_server_id("${exampleFlexibleServer.id}")
///             .name("example-bind")
///             .password("${exampleFlexibleServer.administratorLoginPassword}")
///             .spring_cloud_app_id("${exampleSpringCloudApp.id}")
///             .username("${exampleFlexibleServer.administratorLogin}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-springcloud")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Application MySQL Association can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudAppMysqlAssociation:SpringCloudAppMysqlAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AppPlatform/spring/service1/apps/app1/bindings/bind1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_app_mysql_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAppMysqlAssociationArgs {
        /// Specifies the name of the MySQL Database which the Spring Cloud App should be associated with.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the MySQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub mysql_server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the password which should be used when connecting to the MySQL Database from the Spring Cloud App.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the Spring Cloud Application where this Association is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the username which should be used when connecting to the MySQL Database from the Spring Cloud App.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAppMysqlAssociationResult {
        /// Specifies the name of the MySQL Database which the Spring Cloud App should be associated with.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the MySQL Server. Changing this forces a new resource to be created.
        pub mysql_server_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Spring Cloud Application Association. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the password which should be used when connecting to the MySQL Database from the Spring Cloud App.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Spring Cloud Application where this Association is created. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the username which should be used when connecting to the MySQL Database from the Spring Cloud App.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudAppMysqlAssociationArgs,
    ) -> SpringCloudAppMysqlAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_name_binding = args.database_name.get_output(context);
        let mysql_server_id_binding = args.mysql_server_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let password_binding = args.password.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudAppMysqlAssociation:SpringCloudAppMysqlAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mysqlServerId".into(),
                    value: mysql_server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: spring_cloud_app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: username_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudAppMysqlAssociationResult {
            database_name: o.get_field("databaseName"),
            mysql_server_id: o.get_field("mysqlServerId"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
            username: o.get_field("username"),
        }
    }
}
