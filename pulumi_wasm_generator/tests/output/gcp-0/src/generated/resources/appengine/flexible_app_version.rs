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
pub mod flexible_app_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleAppVersionArgs {
        /// Serving configuration for Google Cloud Endpoints.
        #[builder(into, default)]
        pub api_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionApiConfig>,
        >,
        /// Automatic scaling is based on request rate, response latencies, and other application metrics.
        #[builder(into, default)]
        pub automatic_scaling: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionAutomaticScaling>,
        >,
        /// Metadata settings that are supplied to this version to enable beta runtime features.
        #[builder(into, default)]
        pub beta_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding
        /// StaticFilesHandler does not specify its own expiration time.
        #[builder(into, default)]
        pub default_expiration: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to 'true', the service will be deleted if it is the last version.
        #[builder(into, default)]
        pub delete_service_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Code and application artifacts that make up this version.
        #[builder(into, default)]
        pub deployment: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionDeployment>,
        >,
        /// Code and application artifacts that make up this version.
        #[builder(into, default)]
        pub endpoints_api_service: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionEndpointsApiService>,
        >,
        /// The entrypoint for the application.
        #[builder(into, default)]
        pub entrypoint: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionEntrypoint>,
        >,
        #[builder(into, default)]
        pub env_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Runtime settings for App Engine flexible environment.
        #[builder(into, default)]
        pub flexible_runtime_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appengine::FlexibleAppVersionFlexibleRuntimeSettings,
            >,
        >,
        /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the
        /// request and other request handlers are not attempted.
        #[builder(into, default)]
        pub handlers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appengine::FlexibleAppVersionHandler>>,
        >,
        /// A list of the types of messages that this application is able to receive. Possible values: ["INBOUND_SERVICE_MAIL",
        /// "INBOUND_SERVICE_MAIL_BOUNCE", "INBOUND_SERVICE_XMPP_ERROR", "INBOUND_SERVICE_XMPP_MESSAGE",
        /// "INBOUND_SERVICE_XMPP_SUBSCRIBE", "INBOUND_SERVICE_XMPP_PRESENCE", "INBOUND_SERVICE_CHANNEL_PRESENCE",
        /// "INBOUND_SERVICE_WARMUP"]
        #[builder(into, default)]
        pub inbound_services: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Instance class that is used to run this version. Valid values are AutomaticScaling: F1, F2, F4, F4_1G ManualScaling: B1,
        /// B2, B4, B8, B4_1G Defaults to F1 for AutomaticScaling and B1 for ManualScaling.
        #[builder(into, default)]
        pub instance_class: pulumi_wasm_rust::Output<Option<String>>,
        /// Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances.
        /// Structure is documented below.
        #[builder(into)]
        pub liveness_check: pulumi_wasm_rust::Output<
            super::super::types::appengine::FlexibleAppVersionLivenessCheck,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        #[builder(into, default)]
        pub manual_scaling: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionManualScaling>,
        >,
        /// Extra network settings
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionNetwork>,
        >,
        /// Files that match this pattern will not be built into this version. Only applicable for Go runtimes.
        #[builder(into, default)]
        pub nobuild_files_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to 'true', the application version will not be deleted.
        #[builder(into, default)]
        pub noop_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation.
        /// Structure is documented below.
        #[builder(into)]
        pub readiness_check: pulumi_wasm_rust::Output<
            super::super::types::appengine::FlexibleAppVersionReadinessCheck,
        >,
        /// Machine resources for a version.
        #[builder(into, default)]
        pub resources: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionResources>,
        >,
        /// Desired runtime. Example python27.
        #[builder(into)]
        pub runtime: pulumi_wasm_rust::Output<String>,
        /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at
        /// 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\ Substitute '<language>' with 'python',
        /// 'java', 'php', 'ruby', 'go' or 'nodejs'.
        #[builder(into, default)]
        pub runtime_api_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The channel of the runtime to use. Only available for some runtimes.
        #[builder(into, default)]
        pub runtime_channel: pulumi_wasm_rust::Output<Option<String>>,
        /// The path or name of the app's main executable.
        #[builder(into, default)]
        pub runtime_main_executable_path: pulumi_wasm_rust::Output<Option<String>>,
        /// AppEngine service resource. Can contain numbers, letters, and hyphens.
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
        /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default
        /// if this field is neither provided in app.yaml file nor through CLI flag.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.
        /// Default value: "SERVING" Possible values: ["SERVING", "STOPPED"]
        #[builder(into, default)]
        pub serving_status: pulumi_wasm_rust::Output<Option<String>>,
        /// Relative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters,
        /// numbers, or hyphens. Reserved names,"default", "latest", and any name with the prefix "ah-".
        #[builder(into, default)]
        pub version_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables VPC connectivity for standard apps.
        #[builder(into, default)]
        pub vpc_access_connector: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionVpcAccessConnector>,
        >,
    }
    #[allow(dead_code)]
    pub struct FlexibleAppVersionResult {
        /// Serving configuration for Google Cloud Endpoints.
        pub api_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionApiConfig>,
        >,
        /// Automatic scaling is based on request rate, response latencies, and other application metrics.
        pub automatic_scaling: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionAutomaticScaling>,
        >,
        /// Metadata settings that are supplied to this version to enable beta runtime features.
        pub beta_settings: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding
        /// StaticFilesHandler does not specify its own expiration time.
        pub default_expiration: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to 'true', the service will be deleted if it is the last version.
        pub delete_service_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Code and application artifacts that make up this version.
        pub deployment: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionDeployment>,
        >,
        /// Code and application artifacts that make up this version.
        pub endpoints_api_service: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionEndpointsApiService>,
        >,
        /// The entrypoint for the application.
        pub entrypoint: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionEntrypoint>,
        >,
        pub env_variables: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Runtime settings for App Engine flexible environment.
        pub flexible_runtime_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appengine::FlexibleAppVersionFlexibleRuntimeSettings,
            >,
        >,
        /// An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the
        /// request and other request handlers are not attempted.
        pub handlers: pulumi_wasm_rust::Output<
            Vec<super::super::types::appengine::FlexibleAppVersionHandler>,
        >,
        /// A list of the types of messages that this application is able to receive. Possible values: ["INBOUND_SERVICE_MAIL",
        /// "INBOUND_SERVICE_MAIL_BOUNCE", "INBOUND_SERVICE_XMPP_ERROR", "INBOUND_SERVICE_XMPP_MESSAGE",
        /// "INBOUND_SERVICE_XMPP_SUBSCRIBE", "INBOUND_SERVICE_XMPP_PRESENCE", "INBOUND_SERVICE_CHANNEL_PRESENCE",
        /// "INBOUND_SERVICE_WARMUP"]
        pub inbound_services: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Instance class that is used to run this version. Valid values are AutomaticScaling: F1, F2, F4, F4_1G ManualScaling: B1,
        /// B2, B4, B8, B4_1G Defaults to F1 for AutomaticScaling and B1 for ManualScaling.
        pub instance_class: pulumi_wasm_rust::Output<Option<String>>,
        /// Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances.
        /// Structure is documented below.
        pub liveness_check: pulumi_wasm_rust::Output<
            super::super::types::appengine::FlexibleAppVersionLivenessCheck,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        pub manual_scaling: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionManualScaling>,
        >,
        /// Full path to the Version resource in the API. Example, "v1".
        pub name: pulumi_wasm_rust::Output<String>,
        /// Extra network settings
        pub network: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionNetwork>,
        >,
        /// Files that match this pattern will not be built into this version. Only applicable for Go runtimes.
        pub nobuild_files_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to 'true', the application version will not be deleted.
        pub noop_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation.
        /// Structure is documented below.
        pub readiness_check: pulumi_wasm_rust::Output<
            super::super::types::appengine::FlexibleAppVersionReadinessCheck,
        >,
        /// Machine resources for a version.
        pub resources: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionResources>,
        >,
        /// Desired runtime. Example python27.
        pub runtime: pulumi_wasm_rust::Output<String>,
        /// The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at
        /// 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\ Substitute '<language>' with 'python',
        /// 'java', 'php', 'ruby', 'go' or 'nodejs'.
        pub runtime_api_version: pulumi_wasm_rust::Output<String>,
        /// The channel of the runtime to use. Only available for some runtimes.
        pub runtime_channel: pulumi_wasm_rust::Output<Option<String>>,
        /// The path or name of the app's main executable.
        pub runtime_main_executable_path: pulumi_wasm_rust::Output<Option<String>>,
        /// AppEngine service resource. Can contain numbers, letters, and hyphens.
        pub service: pulumi_wasm_rust::Output<String>,
        /// The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default
        /// if this field is neither provided in app.yaml file nor through CLI flag.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.
        /// Default value: "SERVING" Possible values: ["SERVING", "STOPPED"]
        pub serving_status: pulumi_wasm_rust::Output<Option<String>>,
        /// Relative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters,
        /// numbers, or hyphens. Reserved names,"default", "latest", and any name with the prefix "ah-".
        pub version_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables VPC connectivity for standard apps.
        pub vpc_access_connector: pulumi_wasm_rust::Output<
            Option<super::super::types::appengine::FlexibleAppVersionVpcAccessConnector>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FlexibleAppVersionArgs) -> FlexibleAppVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_config_binding = args.api_config.get_inner();
        let automatic_scaling_binding = args.automatic_scaling.get_inner();
        let beta_settings_binding = args.beta_settings.get_inner();
        let default_expiration_binding = args.default_expiration.get_inner();
        let delete_service_on_destroy_binding = args
            .delete_service_on_destroy
            .get_inner();
        let deployment_binding = args.deployment.get_inner();
        let endpoints_api_service_binding = args.endpoints_api_service.get_inner();
        let entrypoint_binding = args.entrypoint.get_inner();
        let env_variables_binding = args.env_variables.get_inner();
        let flexible_runtime_settings_binding = args
            .flexible_runtime_settings
            .get_inner();
        let handlers_binding = args.handlers.get_inner();
        let inbound_services_binding = args.inbound_services.get_inner();
        let instance_class_binding = args.instance_class.get_inner();
        let liveness_check_binding = args.liveness_check.get_inner();
        let manual_scaling_binding = args.manual_scaling.get_inner();
        let network_binding = args.network.get_inner();
        let nobuild_files_regex_binding = args.nobuild_files_regex.get_inner();
        let noop_on_destroy_binding = args.noop_on_destroy.get_inner();
        let project_binding = args.project.get_inner();
        let readiness_check_binding = args.readiness_check.get_inner();
        let resources_binding = args.resources.get_inner();
        let runtime_binding = args.runtime.get_inner();
        let runtime_api_version_binding = args.runtime_api_version.get_inner();
        let runtime_channel_binding = args.runtime_channel.get_inner();
        let runtime_main_executable_path_binding = args
            .runtime_main_executable_path
            .get_inner();
        let service_binding = args.service.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let serving_status_binding = args.serving_status.get_inner();
        let version_id_binding = args.version_id.get_inner();
        let vpc_access_connector_binding = args.vpc_access_connector.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiConfig".into(),
                },
                register_interface::ResultField {
                    name: "automaticScaling".into(),
                },
                register_interface::ResultField {
                    name: "betaSettings".into(),
                },
                register_interface::ResultField {
                    name: "defaultExpiration".into(),
                },
                register_interface::ResultField {
                    name: "deleteServiceOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "deployment".into(),
                },
                register_interface::ResultField {
                    name: "endpointsApiService".into(),
                },
                register_interface::ResultField {
                    name: "entrypoint".into(),
                },
                register_interface::ResultField {
                    name: "envVariables".into(),
                },
                register_interface::ResultField {
                    name: "flexibleRuntimeSettings".into(),
                },
                register_interface::ResultField {
                    name: "handlers".into(),
                },
                register_interface::ResultField {
                    name: "inboundServices".into(),
                },
                register_interface::ResultField {
                    name: "instanceClass".into(),
                },
                register_interface::ResultField {
                    name: "livenessCheck".into(),
                },
                register_interface::ResultField {
                    name: "manualScaling".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "nobuildFilesRegex".into(),
                },
                register_interface::ResultField {
                    name: "noopOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "readinessCheck".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "runtimeApiVersion".into(),
                },
                register_interface::ResultField {
                    name: "runtimeChannel".into(),
                },
                register_interface::ResultField {
                    name: "runtimeMainExecutablePath".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "servingStatus".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
                register_interface::ResultField {
                    name: "vpcAccessConnector".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlexibleAppVersionResult {
            api_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiConfig").unwrap(),
            ),
            automatic_scaling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticScaling").unwrap(),
            ),
            beta_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("betaSettings").unwrap(),
            ),
            default_expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultExpiration").unwrap(),
            ),
            delete_service_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteServiceOnDestroy").unwrap(),
            ),
            deployment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deployment").unwrap(),
            ),
            endpoints_api_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointsApiService").unwrap(),
            ),
            entrypoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entrypoint").unwrap(),
            ),
            env_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envVariables").unwrap(),
            ),
            flexible_runtime_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flexibleRuntimeSettings").unwrap(),
            ),
            handlers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("handlers").unwrap(),
            ),
            inbound_services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundServices").unwrap(),
            ),
            instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceClass").unwrap(),
            ),
            liveness_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("livenessCheck").unwrap(),
            ),
            manual_scaling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manualScaling").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            nobuild_files_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nobuildFilesRegex").unwrap(),
            ),
            noop_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noopOnDestroy").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            readiness_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readinessCheck").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            runtime_api_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeApiVersion").unwrap(),
            ),
            runtime_channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeChannel").unwrap(),
            ),
            runtime_main_executable_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeMainExecutablePath").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            serving_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servingStatus").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
            vpc_access_connector: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcAccessConnector").unwrap(),
            ),
        }
    }
}
