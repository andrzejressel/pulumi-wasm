/// Deploys a revision of a sharedflow.
///
///
/// To get more information about SharedflowDeployment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.sharedflows.revisions.deployments)
/// * How-to Guides
///     * [sharedflows.revisions.deployments](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.sharedflows.revisions.deployments)
///
/// ## Import
///
/// SharedflowDeployment can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/sharedflows/{{sharedflow_id}}/revisions/{{revision}}/deployments/{{name}}`
///
/// * `{{org_id}}/{{environment}}/{{sharedflow_id}}/{{revision}}/{{name}}`
///
/// When using the `pulumi import` command, SharedflowDeployment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/sharedflowDeployment:SharedflowDeployment default organizations/{{org_id}}/environments/{{environment}}/sharedflows/{{sharedflow_id}}/revisions/{{revision}}/deployments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/sharedflowDeployment:SharedflowDeployment default {{org_id}}/{{environment}}/{{sharedflow_id}}/{{revision}}/{{name}}
/// ```
///
pub mod sharedflow_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedflowDeploymentArgs {
        /// The resource ID of the environment.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Apigee Organization associated with the Sharedflow
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Revision of the Sharedflow to be deployed.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub revision: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The service account represents the identity of the deployed proxy, and determines what permissions it has. The format must be {ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Id of the Sharedflow to be deployed.
        #[builder(into)]
        pub sharedflow_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedflowDeploymentResult {
        /// The resource ID of the environment.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization associated with the Sharedflow
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Revision of the Sharedflow to be deployed.
        ///
        ///
        /// - - -
        pub revision: pulumi_gestalt_rust::Output<String>,
        /// The service account represents the identity of the deployed proxy, and determines what permissions it has. The format must be {ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com.
        pub service_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// Id of the Sharedflow to be deployed.
        pub sharedflow_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SharedflowDeploymentArgs,
    ) -> SharedflowDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let environment_binding = args.environment.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let revision_binding = args.revision.get_output(context).get_inner();
        let service_account_binding = args
            .service_account
            .get_output(context)
            .get_inner();
        let sharedflow_id_binding = args.sharedflow_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/sharedflowDeployment:SharedflowDeployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "revision".into(),
                    value: &revision_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "sharedflowId".into(),
                    value: &sharedflow_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SharedflowDeploymentResult {
            environment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            org_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("orgId"),
            ),
            revision: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revision"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            sharedflow_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedflowId"),
            ),
        }
    }
}
