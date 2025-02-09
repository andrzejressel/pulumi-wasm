/// Flexible App Version resource to create a new version of flexible GAE Application. Based on Google Compute Engine,
/// the App Engine flexible environment automatically scales your app up and down while also balancing the load.
/// Learn about the differences between the standard environment and the flexible environment
/// at https://cloud.google.com/appengine/docs/the-appengine-environments.
///
/// > **Note:** The App Engine flexible environment service account uses the member ID `service-[YOUR_PROJECT_NUMBER]@gae-api-prod.google.com.iam.gserviceaccount.com`
/// It should have the App Engine Flexible Environment Service Agent role, which will be applied when the `appengineflex.googleapis.com` service is enabled.
///
///
/// To get more information about FlexibleAppVersion, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/appengine/docs/flexible)
///
/// ## Example Usage
///
/// ### App Engine Flexible App Version
///
///
/// ```yaml
/// resources:
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: appeng-flex
///       projectId: appeng-flex
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   app:
///     type: gcp:appengine:Application
///     properties:
///       project: ${myProject.projectId}
///       locationId: us-central
///   service:
///     type: gcp:projects:Service
///     properties:
///       project: ${myProject.projectId}
///       service: appengineflex.googleapis.com
///       disableDependentServices: false
///   customServiceAccount:
///     type: gcp:serviceaccount:Account
///     name: custom_service_account
///     properties:
///       project: ${service.project}
///       accountId: my-account
///       displayName: Custom Service Account
///   gaeApi:
///     type: gcp:projects:IAMMember
///     name: gae_api
///     properties:
///       project: ${service.project}
///       role: roles/compute.networkUser
///       member: serviceAccount:${customServiceAccount.email}
///   logsWriter:
///     type: gcp:projects:IAMMember
///     name: logs_writer
///     properties:
///       project: ${service.project}
///       role: roles/logging.logWriter
///       member: serviceAccount:${customServiceAccount.email}
///   storageViewer:
///     type: gcp:projects:IAMMember
///     name: storage_viewer
///     properties:
///       project: ${service.project}
///       role: roles/storage.objectViewer
///       member: serviceAccount:${customServiceAccount.email}
///   myappV1:
///     type: gcp:appengine:FlexibleAppVersion
///     name: myapp_v1
///     properties:
///       versionId: v1
///       project: ${gaeApi.project}
///       service: default
///       runtime: nodejs
///       flexibleRuntimeSettings:
///         operatingSystem: ubuntu22
///         runtimeVersion: '20'
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       livenessCheck:
///         path: /
///       readinessCheck:
///         path: /
///       envVariables:
///         port: '8080'
///       handlers:
///         - urlRegex: .*\/my-path\/*
///           securityLevel: SECURE_ALWAYS
///           login: LOGIN_REQUIRED
///           authFailAction: AUTH_FAIL_ACTION_REDIRECT
///           staticFiles:
///             path: my-other-path
///             uploadPathRegex: .*\/my-path\/*
///       automaticScaling:
///         coolDownPeriod: 120s
///         cpuUtilization:
///           targetUtilization: 0.5
///       noopOnDestroy: true
///       serviceAccount: ${customServiceAccount.email}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       project: ${myProject.projectId}
///       name: appengine-static-content
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: hello-world.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/hello-world.zip
/// ```
///
/// ## Import
///
/// FlexibleAppVersion can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/services/{{service}}/versions/{{version_id}}`
///
/// * `{{project}}/{{service}}/{{version_id}}`
///
/// * `{{service}}/{{version_id}}`
///
/// When using the `pulumi import` command, FlexibleAppVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/flexibleAppVersion:FlexibleAppVersion default apps/{{project}}/services/{{service}}/versions/{{version_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/flexibleAppVersion:FlexibleAppVersion default {{project}}/{{service}}/{{version_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/flexibleAppVersion:FlexibleAppVersion default {{service}}/{{version_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flexible_app_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleAppVersionArgs {
        /// Serving configuration for Google Cloud Endpoints.
        #[builder(into, default)]
        pub api_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionApiConfig>,
        >,
        /// Automatic scaling is based on request rate, response latencies, and other application metrics.
        #[builder(into, default)]
        pub automatic_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionAutomaticScaling>,
        >,
        /// Metadata settings that are supplied to this version to enable beta runtime features.
        #[builder(into, default)]
        pub beta_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding
        /// StaticFilesHandler does not specify its own expiration time.
        #[builder(into, default)]
        pub default_expiration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set to 'true', the service will be deleted if it is the last version.
        #[builder(into, default)]
        pub delete_service_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Code and application artifacts that make up this version.
        #[builder(into, default)]
        pub deployment: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionDeployment>,
        >,
        /// Code and application artifacts that make up this version.
        #[builder(into, default)]
        pub endpoints_api_service: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionEndpointsApiService>,
        >,
        /// The entrypoint for the application.
        #[builder(into, default)]
        pub entrypoint: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionEntrypoint>,
        >,
        #[builder(into, default)]
        pub env_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Runtime settings for App Engine flexible environment.
        #[builder(into, default)]
        pub flexible_runtime_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appengine::FlexibleAppVersionFlexibleRuntimeSettings,
            >,
        >,
        /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the
        /// request and other request handlers are not attempted.
        #[builder(into, default)]
        pub handlers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appengine::FlexibleAppVersionHandler>>,
        >,
        /// A list of the types of messages that this application is able to receive. Possible values: ["INBOUND_SERVICE_MAIL",
        /// "INBOUND_SERVICE_MAIL_BOUNCE", "INBOUND_SERVICE_XMPP_ERROR", "INBOUND_SERVICE_XMPP_MESSAGE",
        /// "INBOUND_SERVICE_XMPP_SUBSCRIBE", "INBOUND_SERVICE_XMPP_PRESENCE", "INBOUND_SERVICE_CHANNEL_PRESENCE",
        /// "INBOUND_SERVICE_WARMUP"]
        #[builder(into, default)]
        pub inbound_services: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Instance class that is used to run this version. Valid values are AutomaticScaling: F1, F2, F4, F4_1G ManualScaling: B1,
        /// B2, B4, B8, B4_1G Defaults to F1 for AutomaticScaling and B1 for ManualScaling.
        #[builder(into, default)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances.
        /// Structure is documented below.
        #[builder(into)]
        pub liveness_check: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appengine::FlexibleAppVersionLivenessCheck,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        #[builder(into, default)]
        pub manual_scaling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionManualScaling>,
        >,
        /// Extra network settings
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionNetwork>,
        >,
        /// Files that match this pattern will not be built into this version. Only applicable for Go runtimes.
        #[builder(into, default)]
        pub nobuild_files_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set to 'true', the application version will not be deleted.
        #[builder(into, default)]
        pub noop_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation.
        /// Structure is documented below.
        #[builder(into)]
        pub readiness_check: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appengine::FlexibleAppVersionReadinessCheck,
        >,
        /// Machine resources for a version.
        #[builder(into, default)]
        pub resources: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionResources>,
        >,
        /// Desired runtime. Example python27.
        #[builder(into)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at
        /// 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\ Substitute '<language>' with 'python',
        /// 'java', 'php', 'ruby', 'go' or 'nodejs'.
        #[builder(into, default)]
        pub runtime_api_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The channel of the runtime to use. Only available for some runtimes.
        #[builder(into, default)]
        pub runtime_channel: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The path or name of the app's main executable.
        #[builder(into, default)]
        pub runtime_main_executable_path: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// AppEngine service resource. Can contain numbers, letters, and hyphens.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default
        /// if this field is neither provided in app.yaml file nor through CLI flag.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.
        /// Default value: "SERVING" Possible values: ["SERVING", "STOPPED"]
        #[builder(into, default)]
        pub serving_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Relative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters,
        /// numbers, or hyphens. Reserved names,"default", "latest", and any name with the prefix "ah-".
        #[builder(into, default)]
        pub version_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables VPC connectivity for standard apps.
        #[builder(into, default)]
        pub vpc_access_connector: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::FlexibleAppVersionVpcAccessConnector>,
        >,
    }
    #[allow(dead_code)]
    pub struct FlexibleAppVersionResult {
        /// Serving configuration for Google Cloud Endpoints.
        pub api_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionApiConfig>,
        >,
        /// Automatic scaling is based on request rate, response latencies, and other application metrics.
        pub automatic_scaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionAutomaticScaling>,
        >,
        /// Metadata settings that are supplied to this version to enable beta runtime features.
        pub beta_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding
        /// StaticFilesHandler does not specify its own expiration time.
        pub default_expiration: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set to 'true', the service will be deleted if it is the last version.
        pub delete_service_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Code and application artifacts that make up this version.
        pub deployment: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionDeployment>,
        >,
        /// Code and application artifacts that make up this version.
        pub endpoints_api_service: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionEndpointsApiService>,
        >,
        /// The entrypoint for the application.
        pub entrypoint: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionEntrypoint>,
        >,
        pub env_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Runtime settings for App Engine flexible environment.
        pub flexible_runtime_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appengine::FlexibleAppVersionFlexibleRuntimeSettings,
            >,
        >,
        /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the
        /// request and other request handlers are not attempted.
        pub handlers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appengine::FlexibleAppVersionHandler>,
        >,
        /// A list of the types of messages that this application is able to receive. Possible values: ["INBOUND_SERVICE_MAIL",
        /// "INBOUND_SERVICE_MAIL_BOUNCE", "INBOUND_SERVICE_XMPP_ERROR", "INBOUND_SERVICE_XMPP_MESSAGE",
        /// "INBOUND_SERVICE_XMPP_SUBSCRIBE", "INBOUND_SERVICE_XMPP_PRESENCE", "INBOUND_SERVICE_CHANNEL_PRESENCE",
        /// "INBOUND_SERVICE_WARMUP"]
        pub inbound_services: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Instance class that is used to run this version. Valid values are AutomaticScaling: F1, F2, F4, F4_1G ManualScaling: B1,
        /// B2, B4, B8, B4_1G Defaults to F1 for AutomaticScaling and B1 for ManualScaling.
        pub instance_class: pulumi_gestalt_rust::Output<Option<String>>,
        /// Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances.
        /// Structure is documented below.
        pub liveness_check: pulumi_gestalt_rust::Output<
            super::super::types::appengine::FlexibleAppVersionLivenessCheck,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        pub manual_scaling: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionManualScaling>,
        >,
        /// Full path to the Version resource in the API. Example, "v1".
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Extra network settings
        pub network: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionNetwork>,
        >,
        /// Files that match this pattern will not be built into this version. Only applicable for Go runtimes.
        pub nobuild_files_regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set to 'true', the application version will not be deleted.
        pub noop_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation.
        /// Structure is documented below.
        pub readiness_check: pulumi_gestalt_rust::Output<
            super::super::types::appengine::FlexibleAppVersionReadinessCheck,
        >,
        /// Machine resources for a version.
        pub resources: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionResources>,
        >,
        /// Desired runtime. Example python27.
        pub runtime: pulumi_gestalt_rust::Output<String>,
        /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at
        /// 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\ Substitute '<language>' with 'python',
        /// 'java', 'php', 'ruby', 'go' or 'nodejs'.
        pub runtime_api_version: pulumi_gestalt_rust::Output<String>,
        /// The channel of the runtime to use. Only available for some runtimes.
        pub runtime_channel: pulumi_gestalt_rust::Output<Option<String>>,
        /// The path or name of the app's main executable.
        pub runtime_main_executable_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// AppEngine service resource. Can contain numbers, letters, and hyphens.
        pub service: pulumi_gestalt_rust::Output<String>,
        /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default
        /// if this field is neither provided in app.yaml file nor through CLI flag.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.
        /// Default value: "SERVING" Possible values: ["SERVING", "STOPPED"]
        pub serving_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Relative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters,
        /// numbers, or hyphens. Reserved names,"default", "latest", and any name with the prefix "ah-".
        pub version_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enables VPC connectivity for standard apps.
        pub vpc_access_connector: pulumi_gestalt_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionVpcAccessConnector>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FlexibleAppVersionArgs,
    ) -> FlexibleAppVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_config_binding_1 = args.api_config.get_output(context);
        let api_config_binding = api_config_binding_1.get_inner();
        let automatic_scaling_binding_1 = args.automatic_scaling.get_output(context);
        let automatic_scaling_binding = automatic_scaling_binding_1.get_inner();
        let beta_settings_binding_1 = args.beta_settings.get_output(context);
        let beta_settings_binding = beta_settings_binding_1.get_inner();
        let default_expiration_binding_1 = args.default_expiration.get_output(context);
        let default_expiration_binding = default_expiration_binding_1.get_inner();
        let delete_service_on_destroy_binding_1 = args
            .delete_service_on_destroy
            .get_output(context);
        let delete_service_on_destroy_binding = delete_service_on_destroy_binding_1
            .get_inner();
        let deployment_binding_1 = args.deployment.get_output(context);
        let deployment_binding = deployment_binding_1.get_inner();
        let endpoints_api_service_binding_1 = args
            .endpoints_api_service
            .get_output(context);
        let endpoints_api_service_binding = endpoints_api_service_binding_1.get_inner();
        let entrypoint_binding_1 = args.entrypoint.get_output(context);
        let entrypoint_binding = entrypoint_binding_1.get_inner();
        let env_variables_binding_1 = args.env_variables.get_output(context);
        let env_variables_binding = env_variables_binding_1.get_inner();
        let flexible_runtime_settings_binding_1 = args
            .flexible_runtime_settings
            .get_output(context);
        let flexible_runtime_settings_binding = flexible_runtime_settings_binding_1
            .get_inner();
        let handlers_binding_1 = args.handlers.get_output(context);
        let handlers_binding = handlers_binding_1.get_inner();
        let inbound_services_binding_1 = args.inbound_services.get_output(context);
        let inbound_services_binding = inbound_services_binding_1.get_inner();
        let instance_class_binding_1 = args.instance_class.get_output(context);
        let instance_class_binding = instance_class_binding_1.get_inner();
        let liveness_check_binding_1 = args.liveness_check.get_output(context);
        let liveness_check_binding = liveness_check_binding_1.get_inner();
        let manual_scaling_binding_1 = args.manual_scaling.get_output(context);
        let manual_scaling_binding = manual_scaling_binding_1.get_inner();
        let network_binding_1 = args.network.get_output(context);
        let network_binding = network_binding_1.get_inner();
        let nobuild_files_regex_binding_1 = args.nobuild_files_regex.get_output(context);
        let nobuild_files_regex_binding = nobuild_files_regex_binding_1.get_inner();
        let noop_on_destroy_binding_1 = args.noop_on_destroy.get_output(context);
        let noop_on_destroy_binding = noop_on_destroy_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let readiness_check_binding_1 = args.readiness_check.get_output(context);
        let readiness_check_binding = readiness_check_binding_1.get_inner();
        let resources_binding_1 = args.resources.get_output(context);
        let resources_binding = resources_binding_1.get_inner();
        let runtime_binding_1 = args.runtime.get_output(context);
        let runtime_binding = runtime_binding_1.get_inner();
        let runtime_api_version_binding_1 = args.runtime_api_version.get_output(context);
        let runtime_api_version_binding = runtime_api_version_binding_1.get_inner();
        let runtime_channel_binding_1 = args.runtime_channel.get_output(context);
        let runtime_channel_binding = runtime_channel_binding_1.get_inner();
        let runtime_main_executable_path_binding_1 = args
            .runtime_main_executable_path
            .get_output(context);
        let runtime_main_executable_path_binding = runtime_main_executable_path_binding_1
            .get_inner();
        let service_binding_1 = args.service.get_output(context);
        let service_binding = service_binding_1.get_inner();
        let service_account_binding_1 = args.service_account.get_output(context);
        let service_account_binding = service_account_binding_1.get_inner();
        let serving_status_binding_1 = args.serving_status.get_output(context);
        let serving_status_binding = serving_status_binding_1.get_inner();
        let version_id_binding_1 = args.version_id.get_output(context);
        let version_id_binding = version_id_binding_1.get_inner();
        let vpc_access_connector_binding_1 = args
            .vpc_access_connector
            .get_output(context);
        let vpc_access_connector_binding = vpc_access_connector_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:appengine/flexibleAppVersion:FlexibleAppVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiConfig".into(),
                    value: &api_config_binding,
                },
                register_interface::ObjectField {
                    name: "automaticScaling".into(),
                    value: &automatic_scaling_binding,
                },
                register_interface::ObjectField {
                    name: "betaSettings".into(),
                    value: &beta_settings_binding,
                },
                register_interface::ObjectField {
                    name: "defaultExpiration".into(),
                    value: &default_expiration_binding,
                },
                register_interface::ObjectField {
                    name: "deleteServiceOnDestroy".into(),
                    value: &delete_service_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "deployment".into(),
                    value: &deployment_binding,
                },
                register_interface::ObjectField {
                    name: "endpointsApiService".into(),
                    value: &endpoints_api_service_binding,
                },
                register_interface::ObjectField {
                    name: "entrypoint".into(),
                    value: &entrypoint_binding,
                },
                register_interface::ObjectField {
                    name: "envVariables".into(),
                    value: &env_variables_binding,
                },
                register_interface::ObjectField {
                    name: "flexibleRuntimeSettings".into(),
                    value: &flexible_runtime_settings_binding,
                },
                register_interface::ObjectField {
                    name: "handlers".into(),
                    value: &handlers_binding,
                },
                register_interface::ObjectField {
                    name: "inboundServices".into(),
                    value: &inbound_services_binding,
                },
                register_interface::ObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "livenessCheck".into(),
                    value: &liveness_check_binding,
                },
                register_interface::ObjectField {
                    name: "manualScaling".into(),
                    value: &manual_scaling_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "nobuildFilesRegex".into(),
                    value: &nobuild_files_regex_binding,
                },
                register_interface::ObjectField {
                    name: "noopOnDestroy".into(),
                    value: &noop_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "readinessCheck".into(),
                    value: &readiness_check_binding,
                },
                register_interface::ObjectField {
                    name: "resources".into(),
                    value: &resources_binding,
                },
                register_interface::ObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeApiVersion".into(),
                    value: &runtime_api_version_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeChannel".into(),
                    value: &runtime_channel_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeMainExecutablePath".into(),
                    value: &runtime_main_executable_path_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "servingStatus".into(),
                    value: &serving_status_binding,
                },
                register_interface::ObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcAccessConnector".into(),
                    value: &vpc_access_connector_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FlexibleAppVersionResult {
            api_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiConfig"),
            ),
            automatic_scaling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticScaling"),
            ),
            beta_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("betaSettings"),
            ),
            default_expiration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultExpiration"),
            ),
            delete_service_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteServiceOnDestroy"),
            ),
            deployment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deployment"),
            ),
            endpoints_api_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointsApiService"),
            ),
            entrypoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entrypoint"),
            ),
            env_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("envVariables"),
            ),
            flexible_runtime_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("flexibleRuntimeSettings"),
            ),
            handlers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("handlers"),
            ),
            inbound_services: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inboundServices"),
            ),
            instance_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceClass"),
            ),
            liveness_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("livenessCheck"),
            ),
            manual_scaling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manualScaling"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            nobuild_files_regex: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nobuildFilesRegex"),
            ),
            noop_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("noopOnDestroy"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            readiness_check: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readinessCheck"),
            ),
            resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resources"),
            ),
            runtime: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtime"),
            ),
            runtime_api_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeApiVersion"),
            ),
            runtime_channel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeChannel"),
            ),
            runtime_main_executable_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeMainExecutablePath"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            serving_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servingStatus"),
            ),
            version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
            vpc_access_connector: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcAccessConnector"),
            ),
        }
    }
}
