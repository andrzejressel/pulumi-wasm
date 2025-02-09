/// Resource for managing an [AWS Mainframe Modernization Deployment.](https://docs.aws.amazon.com/m2/latest/userguide/applications-m2-deploy.html)
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = deployment::create(
///         "test",
///         DeploymentArgs::builder()
///             .application_id("34567890abcdef012345678012")
///             .application_version(1)
///             .environment_id("01234567890abcdef012345678")
///             .start(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Mainframe Modernization Deployment using the `APPLICATION-ID,DEPLOYMENT-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:m2/deployment:Deployment example APPLICATION-ID,DEPLOYMENT-ID
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentArgs {
        /// Application to deploy.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version to application to deploy
        #[builder(into)]
        pub application_version: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Environment to deploy application to.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub force_stop: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Start the application once deployed.
        #[builder(into)]
        pub start: pulumi_gestalt_rust::InputOrOutput<bool>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::m2::DeploymentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentResult {
        /// Application to deploy.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Version to application to deploy
        pub application_version: pulumi_gestalt_rust::Output<i32>,
        pub deployment_id: pulumi_gestalt_rust::Output<String>,
        /// Environment to deploy application to.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        pub force_stop: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Start the application once deployed.
        pub start: pulumi_gestalt_rust::Output<bool>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::m2::DeploymentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DeploymentArgs,
    ) -> DeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding_1 = args.application_id.get_output(context);
        let application_id_binding = application_id_binding_1.get_inner();
        let application_version_binding_1 = args.application_version.get_output(context);
        let application_version_binding = application_version_binding_1.get_inner();
        let environment_id_binding_1 = args.environment_id.get_output(context);
        let environment_id_binding = environment_id_binding_1.get_inner();
        let force_stop_binding_1 = args.force_stop.get_output(context);
        let force_stop_binding = force_stop_binding_1.get_inner();
        let start_binding_1 = args.start.get_output(context);
        let start_binding = start_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:m2/deployment:Deployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "applicationVersion".into(),
                    value: &application_version_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "forceStop".into(),
                    value: &force_stop_binding,
                },
                register_interface::ObjectField {
                    name: "start".into(),
                    value: &start_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeploymentResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            application_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationVersion"),
            ),
            deployment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentId"),
            ),
            environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentId"),
            ),
            force_stop: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceStop"),
            ),
            start: pulumi_gestalt_rust::__private::into_domain(o.extract_field("start")),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
