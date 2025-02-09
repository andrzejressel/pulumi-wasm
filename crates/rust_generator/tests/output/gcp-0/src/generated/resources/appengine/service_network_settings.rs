/// A NetworkSettings resource is a container for ingress settings for a version or service.
///
///
/// To get more information about ServiceNetworkSettings, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services)
///
/// ## Example Usage
///
/// ### App Engine Service Network Settings
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: appengine-static-content
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: hello-world.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/hello-world.zip
///   internalapp:
///     type: gcp:appengine:StandardAppVersion
///     properties:
///       versionId: v1
///       service: internalapp
///       deleteServiceOnDestroy: true
///       runtime: nodejs20
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///   internalappServiceNetworkSettings:
///     type: gcp:appengine:ServiceNetworkSettings
///     name: internalapp
///     properties:
///       service: ${internalapp.service}
///       networkSettings:
///         ingressTrafficAllowed: INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY
/// ```
///
/// ## Import
///
/// ServiceNetworkSettings can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/services/{{service}}`
///
/// * `{{project}}/{{service}}`
///
/// * `{{service}}`
///
/// When using the `pulumi import` command, ServiceNetworkSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings default apps/{{project}}/services/{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings default {{project}}/{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings default {{service}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_network_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceNetworkSettingsArgs {
        /// Ingress settings for this service. Will apply to all versions.
        /// Structure is documented below.
        #[builder(into)]
        pub network_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appengine::ServiceNetworkSettingsNetworkSettings,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the service these settings apply to.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceNetworkSettingsResult {
        /// Ingress settings for this service. Will apply to all versions.
        /// Structure is documented below.
        pub network_settings: pulumi_gestalt_rust::Output<
            super::super::types::appengine::ServiceNetworkSettingsNetworkSettings,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The name of the service these settings apply to.
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceNetworkSettingsArgs,
    ) -> ServiceNetworkSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let network_settings_binding = args.network_settings.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/serviceNetworkSettings:ServiceNetworkSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkSettings".into(),
                    value: network_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceNetworkSettingsResult {
            network_settings: o.get_field("networkSettings"),
            project: o.get_field("project"),
            service: o.get_field("service"),
        }
    }
}
