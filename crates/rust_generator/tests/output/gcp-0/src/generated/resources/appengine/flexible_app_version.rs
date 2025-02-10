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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleAppVersionArgs,
    ) -> FlexibleAppVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_config_binding = args.api_config.get_output(context);
        let automatic_scaling_binding = args.automatic_scaling.get_output(context);
        let beta_settings_binding = args.beta_settings.get_output(context);
        let default_expiration_binding = args.default_expiration.get_output(context);
        let delete_service_on_destroy_binding = args
            .delete_service_on_destroy
            .get_output(context);
        let deployment_binding = args.deployment.get_output(context);
        let endpoints_api_service_binding = args
            .endpoints_api_service
            .get_output(context);
        let entrypoint_binding = args.entrypoint.get_output(context);
        let env_variables_binding = args.env_variables.get_output(context);
        let flexible_runtime_settings_binding = args
            .flexible_runtime_settings
            .get_output(context);
        let handlers_binding = args.handlers.get_output(context);
        let inbound_services_binding = args.inbound_services.get_output(context);
        let instance_class_binding = args.instance_class.get_output(context);
        let liveness_check_binding = args.liveness_check.get_output(context);
        let manual_scaling_binding = args.manual_scaling.get_output(context);
        let network_binding = args.network.get_output(context);
        let nobuild_files_regex_binding = args.nobuild_files_regex.get_output(context);
        let noop_on_destroy_binding = args.noop_on_destroy.get_output(context);
        let project_binding = args.project.get_output(context);
        let readiness_check_binding = args.readiness_check.get_output(context);
        let resources_binding = args.resources.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let runtime_api_version_binding = args.runtime_api_version.get_output(context);
        let runtime_channel_binding = args.runtime_channel.get_output(context);
        let runtime_main_executable_path_binding = args
            .runtime_main_executable_path
            .get_output(context);
        let service_binding = args.service.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let serving_status_binding = args.serving_status.get_output(context);
        let version_id_binding = args.version_id.get_output(context);
        let vpc_access_connector_binding = args.vpc_access_connector.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/flexibleAppVersion:FlexibleAppVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConfig".into(),
                    value: api_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticScaling".into(),
                    value: automatic_scaling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "betaSettings".into(),
                    value: beta_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultExpiration".into(),
                    value: default_expiration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteServiceOnDestroy".into(),
                    value: delete_service_on_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deployment".into(),
                    value: deployment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointsApiService".into(),
                    value: endpoints_api_service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entrypoint".into(),
                    value: entrypoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envVariables".into(),
                    value: env_variables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "flexibleRuntimeSettings".into(),
                    value: flexible_runtime_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "handlers".into(),
                    value: handlers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inboundServices".into(),
                    value: inbound_services_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceClass".into(),
                    value: instance_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "livenessCheck".into(),
                    value: liveness_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manualScaling".into(),
                    value: manual_scaling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nobuildFilesRegex".into(),
                    value: nobuild_files_regex_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "noopOnDestroy".into(),
                    value: noop_on_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readinessCheck".into(),
                    value: readiness_check_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resources".into(),
                    value: resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: runtime_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeApiVersion".into(),
                    value: runtime_api_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeChannel".into(),
                    value: runtime_channel_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeMainExecutablePath".into(),
                    value: runtime_main_executable_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: service_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servingStatus".into(),
                    value: serving_status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionId".into(),
                    value: version_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcAccessConnector".into(),
                    value: vpc_access_connector_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlexibleAppVersionResult {
            api_config: o.get_field("apiConfig"),
            automatic_scaling: o.get_field("automaticScaling"),
            beta_settings: o.get_field("betaSettings"),
            default_expiration: o.get_field("defaultExpiration"),
            delete_service_on_destroy: o.get_field("deleteServiceOnDestroy"),
            deployment: o.get_field("deployment"),
            endpoints_api_service: o.get_field("endpointsApiService"),
            entrypoint: o.get_field("entrypoint"),
            env_variables: o.get_field("envVariables"),
            flexible_runtime_settings: o.get_field("flexibleRuntimeSettings"),
            handlers: o.get_field("handlers"),
            inbound_services: o.get_field("inboundServices"),
            instance_class: o.get_field("instanceClass"),
            liveness_check: o.get_field("livenessCheck"),
            manual_scaling: o.get_field("manualScaling"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            nobuild_files_regex: o.get_field("nobuildFilesRegex"),
            noop_on_destroy: o.get_field("noopOnDestroy"),
            project: o.get_field("project"),
            readiness_check: o.get_field("readinessCheck"),
            resources: o.get_field("resources"),
            runtime: o.get_field("runtime"),
            runtime_api_version: o.get_field("runtimeApiVersion"),
            runtime_channel: o.get_field("runtimeChannel"),
            runtime_main_executable_path: o.get_field("runtimeMainExecutablePath"),
            service: o.get_field("service"),
            service_account: o.get_field("serviceAccount"),
            serving_status: o.get_field("servingStatus"),
            version_id: o.get_field("versionId"),
            vpc_access_connector: o.get_field("vpcAccessConnector"),
        }
    }
}
