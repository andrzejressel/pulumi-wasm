/// ## Example Usage
///
/// ### Firebase Extentions Instance Resize Image
///
///
/// ```yaml
/// resources:
///   images:
///     type: gcp:storage:Bucket
///     properties:
///       project: my-project-name
///       name: bucket-id
///       location: US
///       uniformBucketLevelAccess: true # Delete all objects when the bucket is deleted
///       forceDestroy: true
///   resizeImage:
///     type: gcp:firebase:ExtensionsInstance
///     name: resize_image
///     properties:
///       project: my-project-name
///       instanceId: storage-resize-images
///       config:
///         extensionRef: firebase/storage-resize-images
///         extensionVersion: 0.2.2
///         params:
///           DELETE_ORIGINAL_FILE: false
///           MAKE_PUBLIC: false
///           IMAGE_TYPE: false
///           IS_ANIMATED: true
///           FUNCTION_MEMORY: 1024
///           DO_BACKFILL: false
///           IMG_SIZES: 200x200
///           IMG_BUCKET: ${images.name}
///         systemParams:
///           firebaseextensions.v1beta.function/location: ""
///           firebaseextensions.v1beta.function/maxInstances: 3000
///           firebaseextensions.v1beta.function/minInstances: 0
///           firebaseextensions.v1beta.function/vpcConnectorEgressSettings: VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED
///         allowedEventTypes:
///           - firebase.extensions.storage-resize-images.v1.onCompletion
///         eventarcChannel: projects/my-project-name/locations//channels/firebase
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{instance_id}}`
///
/// * `{{project}}/{{instance_id}}`
///
/// * `{{instance_id}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/extensionsInstance:ExtensionsInstance default projects/{{project}}/instances/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/extensionsInstance:ExtensionsInstance default {{project}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/extensionsInstance:ExtensionsInstance default {{instance_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod extensions_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionsInstanceArgs {
        /// The current Config of the Extension Instance.
        /// Structure is documented below.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::firebase::ExtensionsInstanceConfig,
        >,
        /// The ID to use for the Extension Instance, which will become the final
        /// component of the instance's name.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ExtensionsInstanceResult {
        /// The current Config of the Extension Instance.
        /// Structure is documented below.
        pub config: pulumi_gestalt_rust::Output<
            super::super::types::firebase::ExtensionsInstanceConfig,
        >,
        /// The time at which the Extension Instance was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// If this Instance has `state: ERRORED`, the error messages
        /// will be found here.
        /// Structure is documented below.
        pub error_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::firebase::ExtensionsInstanceErrorStatus>,
        >,
        /// A weak etag that is computed by the server based on other configuration
        /// values and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The ID to use for the Extension Instance, which will become the final
        /// component of the instance's name.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the last operation that acted on this Extension
        /// Instance
        pub last_operation_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the last operation that acted on the Extension Instance.
        pub last_operation_type: pulumi_gestalt_rust::Output<String>,
        /// The fully-qualified resource name of the Extension Instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Data set by the extension instance at runtime.
        /// Structure is documented below.
        pub runtime_datas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::firebase::ExtensionsInstanceRuntimeData>,
        >,
        /// The email of the service account to be used at runtime by compute resources
        /// created for the operation of the Extension instance.
        pub service_account_email: pulumi_gestalt_rust::Output<String>,
        /// The processing state of the extension instance.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The time at which the Extension Instance was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExtensionsInstanceArgs,
    ) -> ExtensionsInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_binding = args.config.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebase/extensionsInstance:ExtensionsInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExtensionsInstanceResult {
            config: o.get_field("config"),
            create_time: o.get_field("createTime"),
            error_statuses: o.get_field("errorStatuses"),
            etag: o.get_field("etag"),
            instance_id: o.get_field("instanceId"),
            last_operation_name: o.get_field("lastOperationName"),
            last_operation_type: o.get_field("lastOperationType"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            runtime_datas: o.get_field("runtimeDatas"),
            service_account_email: o.get_field("serviceAccountEmail"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
