/// ## Example Usage
///
/// ### Kms Key Handle Basic
///
///
/// ```yaml
/// resources:
///   # Create Folder in GCP Organization
///   autokmsFolder:
///     type: gcp:organizations:Folder
///     name: autokms_folder
///     properties:
///       displayName: my-folder
///       parent: organizations/123456789
///       deletionProtection: false
///   # Create the key project
///   keyProject:
///     type: gcp:organizations:Project
///     name: key_project
///     properties:
///       projectId: key-proj
///       name: key-proj
///       folderId: ${autokmsFolder.folderId}
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///     options:
///       dependsOn:
///         - ${autokmsFolder}
///   # Create the resource project
///   resourceProject:
///     type: gcp:organizations:Project
///     name: resource_project
///     properties:
///       projectId: res-proj
///       name: res-proj
///       folderId: ${autokmsFolder.folderId}
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///     options:
///       dependsOn:
///         - ${autokmsFolder}
///   # Enable the Cloud KMS API
///   kmsApiService:
///     type: gcp:projects:Service
///     name: kms_api_service
///     properties:
///       service: cloudkms.googleapis.com
///       project: ${keyProject.projectId}
///       disableOnDestroy: false
///       disableDependentServices: true
///     options:
///       dependsOn:
///         - ${keyProject}
///   # Wait delay after enabling APIs
///   waitEnableServiceApi:
///     type: time:sleep
///     name: wait_enable_service_api
///     properties:
///       createDuration: 30s
///     options:
///       dependsOn:
///         - ${kmsApiService}
///   #Create KMS Service Agent
///   kmsServiceAgent:
///     type: gcp:projects:ServiceIdentity
///     name: kms_service_agent
///     properties:
///       service: cloudkms.googleapis.com
///       project: ${keyProject.number}
///     options:
///       dependsOn:
///         - ${waitEnableServiceApi}
///   # Wait delay after creating service agent.
///   waitServiceAgent:
///     type: time:sleep
///     name: wait_service_agent
///     properties:
///       createDuration: 10s
///     options:
///       dependsOn:
///         - ${kmsServiceAgent}
///   #Grant the KMS Service Agent the Cloud KMS Admin role
///   autokeyProjectAdmin:
///     type: gcp:projects:IAMMember
///     name: autokey_project_admin
///     properties:
///       project: ${keyProject.projectId}
///       role: roles/cloudkms.admin
///       member: serviceAccount:service-${keyProject.number}@gcp-sa-cloudkms.iam.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${waitServiceAgent}
///   # Wait delay after granting IAM permissions
///   waitSrvAccPermissions:
///     type: time:sleep
///     name: wait_srv_acc_permissions
///     properties:
///       createDuration: 10s
///     options:
///       dependsOn:
///         - ${autokeyProjectAdmin}
///   autokeyConfig:
///     type: gcp:kms:AutokeyConfig
///     name: autokey_config
///     properties:
///       folder: ${autokmsFolder.folderId}
///       keyProject: projects/${keyProject.projectId}
///     options:
///       dependsOn:
///         - ${waitSrvAccPermissions}
///   # Wait delay for autokey config to take effect
///   waitAutokeyConfig:
///     type: time:sleep
///     name: wait_autokey_config
///     properties:
///       createDuration: 10s
///     options:
///       dependsOn:
///         - ${autokeyConfig}
///   example-keyhandle:
///     type: gcp:kms:KeyHandle
///     properties:
///       project: ${resourceProject.projectId}
///       name: tf-test-key-handle
///       location: global
///       resourceTypeSelector: storage.googleapis.com/Bucket
///     options:
///       dependsOn:
///         - ${waitAutokeyConfig}
/// ```
///
/// ## Import
///
/// KeyHandle can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/keyHandles/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, KeyHandle can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/keyHandle:KeyHandle default projects/{{project}}/locations/{{location}}/keyHandles/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/keyHandle:KeyHandle default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/keyHandle:KeyHandle default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_handle {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyHandleArgs {
        /// The location for the KeyHandle.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name for the KeyHandle.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Selector of the resource type where we want to protect resources.
        /// For example, `storage.googleapis.com/Bucket`.
        #[builder(into)]
        pub resource_type_selector: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeyHandleResult {
        /// A reference to a Cloud KMS CryptoKey that can be used for CMEK in the requested
        /// product/project/location, for example
        /// `projects/1/locations/us-east1/keyRings/foo/cryptoKeys/bar-ffffff`
        pub kms_key: pulumi_gestalt_rust::Output<String>,
        /// The location for the KeyHandle.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the KeyHandle.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Selector of the resource type where we want to protect resources.
        /// For example, `storage.googleapis.com/Bucket`.
        pub resource_type_selector: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KeyHandleArgs,
    ) -> KeyHandleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let resource_type_selector_binding_1 = args
            .resource_type_selector
            .get_output(context);
        let resource_type_selector_binding = resource_type_selector_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:kms/keyHandle:KeyHandle".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypeSelector".into(),
                    value: &resource_type_selector_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyHandleResult {
            kms_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKey"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            resource_type_selector: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTypeSelector"),
            ),
        }
    }
}
