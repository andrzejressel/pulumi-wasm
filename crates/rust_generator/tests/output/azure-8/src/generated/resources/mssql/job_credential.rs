/// Manages an Elastic Job Credential.
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
///             .location("northeurope")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .collation("SQL_Latin1_General_CP1_CI_AS")
///             .name("example-db")
///             .server_id("${exampleServer.id}")
///             .sku_name("S1")
///             .build_struct(),
///     );
///     let exampleJobAgent = job_agent::create(
///         "exampleJobAgent",
///         JobAgentArgs::builder()
///             .database_id("${exampleDatabase.id}")
///             .location("${example.location}")
///             .name("example-job-agent")
///             .build_struct(),
///     );
///     let exampleJobCredential = job_credential::create(
///         "exampleJobCredential",
///         JobCredentialArgs::builder()
///             .job_agent_id("${exampleJobAgent.id}")
///             .name("example-credential")
///             .password("MyP4ssw0rd!!!")
///             .username("my-username")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("example-server")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Elastic Job Credentials can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/jobCredential:JobCredential example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Sql/servers/myserver1/jobAgents/myjobagent1/credentials/credential1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job_credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobCredentialArgs {
        /// The ID of the Elastic Job Agent. Changing this forces a new Elastic Job Credential to be created.
        #[builder(into)]
        pub job_agent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Elastic Job Credential. Changing this forces a new Elastic Job Credential to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password part of the credential.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username part of the credential.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct JobCredentialResult {
        /// The ID of the Elastic Job Agent. Changing this forces a new Elastic Job Credential to be created.
        pub job_agent_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Elastic Job Credential. Changing this forces a new Elastic Job Credential to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password part of the credential.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The username part of the credential.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: JobCredentialArgs,
    ) -> JobCredentialResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let job_agent_id_binding = args.job_agent_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/jobCredential:JobCredential".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "jobAgentId".into(),
                    value: &job_agent_id_binding,
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
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        JobCredentialResult {
            job_agent_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("jobAgentId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
