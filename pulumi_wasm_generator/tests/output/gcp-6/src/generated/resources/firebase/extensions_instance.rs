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
pub mod extensions_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionsInstanceArgs {
        /// The current Config of the Extension Instance.
        /// Structure is documented below.
        #[builder(into)]
        pub config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::firebase::ExtensionsInstanceConfig,
        >,
        /// The ID to use for the Extension Instance, which will become the final
        /// component of the instance's name.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ExtensionsInstanceResult {
        /// The current Config of the Extension Instance.
        /// Structure is documented below.
        pub config: pulumi_wasm_rust::Output<
            super::super::types::firebase::ExtensionsInstanceConfig,
        >,
        /// The time at which the Extension Instance was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// If this Instance has `state: ERRORED`, the error messages
        /// will be found here.
        /// Structure is documented below.
        pub error_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::firebase::ExtensionsInstanceErrorStatus>,
        >,
        /// A weak etag that is computed by the server based on other configuration
        /// values and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The ID to use for the Extension Instance, which will become the final
        /// component of the instance's name.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The name of the last operation that acted on this Extension
        /// Instance
        pub last_operation_name: pulumi_wasm_rust::Output<String>,
        /// The type of the last operation that acted on the Extension Instance.
        pub last_operation_type: pulumi_wasm_rust::Output<String>,
        /// The fully-qualified resource name of the Extension Instance.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Data set by the extension instance at runtime.
        /// Structure is documented below.
        pub runtime_datas: pulumi_wasm_rust::Output<
            Vec<super::super::types::firebase::ExtensionsInstanceRuntimeData>,
        >,
        /// The email of the service account to be used at runtime by compute resources
        /// created for the operation of the Extension instance.
        pub service_account_email: pulumi_wasm_rust::Output<String>,
        /// The processing state of the extension instance.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The time at which the Extension Instance was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExtensionsInstanceArgs,
    ) -> ExtensionsInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_binding = args.config.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/extensionsInstance:ExtensionsInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "config".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "errorStatuses".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "lastOperationName".into(),
                },
                register_interface::ResultField {
                    name: "lastOperationType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "runtimeDatas".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountEmail".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExtensionsInstanceResult {
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            error_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorStatuses").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            last_operation_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastOperationName").unwrap(),
            ),
            last_operation_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastOperationType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            runtime_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtimeDatas").unwrap(),
            ),
            service_account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountEmail").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
