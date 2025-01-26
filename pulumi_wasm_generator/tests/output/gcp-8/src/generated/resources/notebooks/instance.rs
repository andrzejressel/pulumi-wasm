/// > **Warning:** `google_notebook_instance` is deprecated and will be removed in a future major release. Use `gcp.workbench.Instance` instead.
///
/// A Cloud AI Platform Notebook instance.
///
///
/// > **Note:** Due to limitations of the Notebooks Instance API, many fields
/// in this resource do not properly detect drift. These fields will also not
/// appear in state once imported.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/ai-platform/notebooks/docs/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/ai-platform-notebooks)
///
/// ## Example Usage
///
/// ### Notebook Instance Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .location("us-west1-a")
///             .machine_type("e2-medium")
///             .name("notebooks-instance")
///             .vm_image(
///                 InstanceVmImage::builder()
///                     .imageFamily("tf-latest-cpu")
///                     .project("deeplearning-platform-release")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Notebook Instance Basic Stopped
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .desired_state("STOPPED")
///             .location("us-west1-a")
///             .machine_type("e2-medium")
///             .name("notebooks-instance")
///             .vm_image(
///                 InstanceVmImage::builder()
///                     .imageFamily("tf-latest-cpu")
///                     .project("deeplearning-platform-release")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Notebook Instance Basic Container
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:notebooks:Instance
///     properties:
///       name: notebooks-instance
///       location: us-west1-a
///       machineType: e2-medium
///       metadata:
///         proxy-mode: service_account
///       containerImage:
///         repository: gcr.io/deeplearning-platform-release/base-cpu
///         tag: latest
/// ```
/// ### Notebook Instance Basic Gpu
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:notebooks:Instance
///     properties:
///       name: notebooks-instance
///       location: us-west1-a
///       machineType: n1-standard-1
///       installGpuDriver: true
///       acceleratorConfig:
///         type: NVIDIA_TESLA_T4
///         coreCount: 1
///       vmImage:
///         project: deeplearning-platform-release
///         imageFamily: tf-latest-gpu
/// ```
/// ### Notebook Instance Full
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:notebooks:Instance
///     properties:
///       name: notebooks-instance
///       location: us-central1-a
///       machineType: e2-medium
///       vmImage:
///         project: deeplearning-platform-release
///         imageFamily: tf-latest-cpu
///       instanceOwners:
///         - my@service-account.com
///       serviceAccount: my@service-account.com
///       installGpuDriver: true
///       bootDiskType: PD_SSD
///       bootDiskSizeGb: 110
///       noPublicIp: true
///       noProxyAccess: true
///       network: ${myNetwork.id}
///       subnet: ${mySubnetwork.id}
///       labels:
///         k: val
///       metadata:
///         terraform: 'true'
///       serviceAccountScopes:
///         - https://www.googleapis.com/auth/bigquery
///         - https://www.googleapis.com/auth/devstorage.read_write
///         - https://www.googleapis.com/auth/cloud-platform
///         - https://www.googleapis.com/auth/userinfo.email
///       tags:
///         - foo
///         - bar
///       diskEncryption: CMEK
///       kmsKey: my-crypto-key
///       desiredState: ACTIVE
/// variables:
///   myNetwork:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: default
///   mySubnetwork:
///     fn::invoke:
///       function: gcp:compute:getSubnetwork
///       arguments:
///         name: default
///         region: us-central1
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:notebooks/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:notebooks/instance:Instance default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:notebooks/instance:Instance default {{location}}/{{name}}
/// ```
///
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The hardware accelerator used on this instance. If you use accelerators,
        /// make sure that your configuration has enough vCPUs and memory to support the
        /// machineType you have selected.
        /// Structure is documented below.
        #[builder(into, default)]
        pub accelerator_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::InstanceAcceleratorConfig>,
        >,
        /// The size of the boot disk in GB attached to this instance,
        /// up to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB.
        /// If not specified, this defaults to 100.
        #[builder(into, default)]
        pub boot_disk_size_gb: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Possible disk types for notebook instances.
        /// Possible values are: `DISK_TYPE_UNSPECIFIED`, `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
        #[builder(into, default)]
        pub boot_disk_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Use a container image to start the notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub container_image: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::InstanceContainerImage>,
        >,
        /// Instance creation time
        #[builder(into, default)]
        pub create_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specify a custom Cloud Storage path where the GPU driver is stored.
        /// If not specified, we'll automatically choose from official GPU drivers.
        #[builder(into, default)]
        pub custom_gpu_driver_path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The size of the data disk in GB attached to this instance,
        /// up to a maximum of 64000 GB (64 TB).
        /// You can choose the size of the data disk based on how big your notebooks and data are.
        /// If not specified, this defaults to 100.
        #[builder(into, default)]
        pub data_disk_size_gb: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Possible disk types for notebook instances.
        /// Possible values are: `DISK_TYPE_UNSPECIFIED`, `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
        #[builder(into, default)]
        pub data_disk_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Desired state of the Notebook Instance. Set this field to `ACTIVE` to start the Instance, and `STOPPED` to stop the Instance.
        #[builder(into, default)]
        pub desired_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Disk encryption method used on the boot and data disks, defaults to GMEK.
        /// Possible values are: `DISK_ENCRYPTION_UNSPECIFIED`, `GMEK`, `CMEK`.
        #[builder(into, default)]
        pub disk_encryption: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the end user authorizes Google Cloud to install GPU driver
        /// on this instance. If this field is empty or set to false, the GPU driver
        /// won't be installed. Only applicable to instances with GPUs.
        #[builder(into, default)]
        pub install_gpu_driver: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The list of owners of this instance after creation.
        /// Format: alias@example.com.
        /// Currently supports one owner only.
        /// If not specified, all of the service account users of
        /// your VM instance's service account can use the instance.
        #[builder(into, default)]
        pub instance_owners: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The KMS key used to encrypt the disks, only applicable if diskEncryption is CMEK.
        /// Format: projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}
        #[builder(into, default)]
        pub kms_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels to apply to this instance. These can be later modified by the setLabels method.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// A reference to a machine type which defines VM kind.
        #[builder(into)]
        pub machine_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Custom metadata to apply to this instance.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name specified for the Notebook instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the VPC that this instance is in.
        /// Format: projects/{project_id}/global/networks/{network_id}
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of vNIC driver.
        /// Possible values are: `UNSPECIFIED_NIC_TYPE`, `VIRTIO_NET`, `GVNIC`.
        #[builder(into, default)]
        pub nic_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The notebook instance will not register with the proxy..
        #[builder(into, default)]
        pub no_proxy_access: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// No public IP will be assigned to this instance.
        #[builder(into, default)]
        pub no_public_ip: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If true, the data disk will not be auto deleted when deleting the instance.
        #[builder(into, default)]
        pub no_remove_data_disk: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Path to a Bash script that automatically runs after a
        /// notebook instance fully boots up. The path must be a URL
        /// or Cloud Storage path (gs://path-to-file/file-name).
        #[builder(into, default)]
        pub post_startup_script: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Reservation Affinity for consuming Zonal reservation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub reservation_affinity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::InstanceReservationAffinity>,
        >,
        /// The service account on this instance, giving access to other
        /// Google Cloud services. You can use any service account within
        /// the same project, but you must have the service account user
        /// permission to use the instance. If not specified,
        /// the Compute Engine default service account is used.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. The URIs of service account scopes to be included in Compute Engine instances.
        /// If not specified, the following scopes are defined:
        /// - https://www.googleapis.com/auth/cloud-platform
        /// - https://www.googleapis.com/auth/userinfo.email
        #[builder(into, default)]
        pub service_account_scopes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A set of Shielded Instance options. Check [Images using supported Shielded VM features]
        /// Not all combinations are valid
        /// Structure is documented below.
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::InstanceShieldedInstanceConfig>,
        >,
        /// The name of the subnet that this instance is in.
        /// Format: projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}
        #[builder(into, default)]
        pub subnet: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Compute Engine tags to add to instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Instance update time.
        #[builder(into, default)]
        pub update_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Use a Compute Engine VM image to start the notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vm_image: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::InstanceVmImage>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// The hardware accelerator used on this instance. If you use accelerators,
        /// make sure that your configuration has enough vCPUs and memory to support the
        /// machineType you have selected.
        /// Structure is documented below.
        pub accelerator_config: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::InstanceAcceleratorConfig>,
        >,
        /// The size of the boot disk in GB attached to this instance,
        /// up to a maximum of 64000 GB (64 TB). The minimum recommended value is 100 GB.
        /// If not specified, this defaults to 100.
        pub boot_disk_size_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Possible disk types for notebook instances.
        /// Possible values are: `DISK_TYPE_UNSPECIFIED`, `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
        pub boot_disk_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Use a container image to start the notebook instance.
        /// Structure is documented below.
        pub container_image: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::InstanceContainerImage>,
        >,
        /// Instance creation time
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Specify a custom Cloud Storage path where the GPU driver is stored.
        /// If not specified, we'll automatically choose from official GPU drivers.
        pub custom_gpu_driver_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The size of the data disk in GB attached to this instance,
        /// up to a maximum of 64000 GB (64 TB).
        /// You can choose the size of the data disk based on how big your notebooks and data are.
        /// If not specified, this defaults to 100.
        pub data_disk_size_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Possible disk types for notebook instances.
        /// Possible values are: `DISK_TYPE_UNSPECIFIED`, `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
        pub data_disk_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Desired state of the Notebook Instance. Set this field to `ACTIVE` to start the Instance, and `STOPPED` to stop the Instance.
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// Disk encryption method used on the boot and data disks, defaults to GMEK.
        /// Possible values are: `DISK_ENCRYPTION_UNSPECIFIED`, `GMEK`, `CMEK`.
        pub disk_encryption: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the end user authorizes Google Cloud to install GPU driver
        /// on this instance. If this field is empty or set to false, the GPU driver
        /// won't be installed. Only applicable to instances with GPUs.
        pub install_gpu_driver: pulumi_wasm_rust::Output<Option<bool>>,
        /// The list of owners of this instance after creation.
        /// Format: alias@example.com.
        /// Currently supports one owner only.
        /// If not specified, all of the service account users of
        /// your VM instance's service account can use the instance.
        pub instance_owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The KMS key used to encrypt the disks, only applicable if diskEncryption is CMEK.
        /// Format: projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Labels to apply to this instance. These can be later modified by the setLabels method.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// A reference to a machine type which defines VM kind.
        pub machine_type: pulumi_wasm_rust::Output<String>,
        /// Custom metadata to apply to this instance.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name specified for the Notebook instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the VPC that this instance is in.
        /// Format: projects/{project_id}/global/networks/{network_id}
        pub network: pulumi_wasm_rust::Output<String>,
        /// The type of vNIC driver.
        /// Possible values are: `UNSPECIFIED_NIC_TYPE`, `VIRTIO_NET`, `GVNIC`.
        pub nic_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The notebook instance will not register with the proxy..
        pub no_proxy_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// No public IP will be assigned to this instance.
        pub no_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, the data disk will not be auto deleted when deleting the instance.
        pub no_remove_data_disk: pulumi_wasm_rust::Output<Option<bool>>,
        /// Path to a Bash script that automatically runs after a
        /// notebook instance fully boots up. The path must be a URL
        /// or Cloud Storage path (gs://path-to-file/file-name).
        pub post_startup_script: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The proxy endpoint that is used to access the Jupyter notebook.
        /// Only returned when the resource is in a `PROVISIONED` state. If
        /// needed you can utilize `pulumi up -refresh-only` to await
        /// the population of this value.
        pub proxy_uri: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Reservation Affinity for consuming Zonal reservation.
        /// Structure is documented below.
        pub reservation_affinity: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::InstanceReservationAffinity>,
        >,
        /// The service account on this instance, giving access to other
        /// Google Cloud services. You can use any service account within
        /// the same project, but you must have the service account user
        /// permission to use the instance. If not specified,
        /// the Compute Engine default service account is used.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Optional. The URIs of service account scopes to be included in Compute Engine instances.
        /// If not specified, the following scopes are defined:
        /// - https://www.googleapis.com/auth/cloud-platform
        /// - https://www.googleapis.com/auth/userinfo.email
        pub service_account_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A set of Shielded Instance options. Check [Images using supported Shielded VM features]
        /// Not all combinations are valid
        /// Structure is documented below.
        pub shielded_instance_config: pulumi_wasm_rust::Output<
            super::super::types::notebooks::InstanceShieldedInstanceConfig,
        >,
        /// The state of this instance.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The name of the subnet that this instance is in.
        /// Format: projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}
        pub subnet: pulumi_wasm_rust::Output<String>,
        /// The Compute Engine tags to add to instance.
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        /// Instance update time.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Use a Compute Engine VM image to start the notebook instance.
        /// Structure is documented below.
        pub vm_image: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::InstanceVmImage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accelerator_config_binding = args
            .accelerator_config
            .get_output(context)
            .get_inner();
        let boot_disk_size_gb_binding = args
            .boot_disk_size_gb
            .get_output(context)
            .get_inner();
        let boot_disk_type_binding = args.boot_disk_type.get_output(context).get_inner();
        let container_image_binding = args
            .container_image
            .get_output(context)
            .get_inner();
        let create_time_binding = args.create_time.get_output(context).get_inner();
        let custom_gpu_driver_path_binding = args
            .custom_gpu_driver_path
            .get_output(context)
            .get_inner();
        let data_disk_size_gb_binding = args
            .data_disk_size_gb
            .get_output(context)
            .get_inner();
        let data_disk_type_binding = args.data_disk_type.get_output(context).get_inner();
        let desired_state_binding = args.desired_state.get_output(context).get_inner();
        let disk_encryption_binding = args
            .disk_encryption
            .get_output(context)
            .get_inner();
        let install_gpu_driver_binding = args
            .install_gpu_driver
            .get_output(context)
            .get_inner();
        let instance_owners_binding = args
            .instance_owners
            .get_output(context)
            .get_inner();
        let kms_key_binding = args.kms_key.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let machine_type_binding = args.machine_type.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let nic_type_binding = args.nic_type.get_output(context).get_inner();
        let no_proxy_access_binding = args
            .no_proxy_access
            .get_output(context)
            .get_inner();
        let no_public_ip_binding = args.no_public_ip.get_output(context).get_inner();
        let no_remove_data_disk_binding = args
            .no_remove_data_disk
            .get_output(context)
            .get_inner();
        let post_startup_script_binding = args
            .post_startup_script
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let reservation_affinity_binding = args
            .reservation_affinity
            .get_output(context)
            .get_inner();
        let service_account_binding = args
            .service_account
            .get_output(context)
            .get_inner();
        let service_account_scopes_binding = args
            .service_account_scopes
            .get_output(context)
            .get_inner();
        let shielded_instance_config_binding = args
            .shielded_instance_config
            .get_output(context)
            .get_inner();
        let subnet_binding = args.subnet.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let update_time_binding = args.update_time.get_output(context).get_inner();
        let vm_image_binding = args.vm_image.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:notebooks/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratorConfig".into(),
                    value: &accelerator_config_binding,
                },
                register_interface::ObjectField {
                    name: "bootDiskSizeGb".into(),
                    value: &boot_disk_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "bootDiskType".into(),
                    value: &boot_disk_type_binding,
                },
                register_interface::ObjectField {
                    name: "containerImage".into(),
                    value: &container_image_binding,
                },
                register_interface::ObjectField {
                    name: "createTime".into(),
                    value: &create_time_binding,
                },
                register_interface::ObjectField {
                    name: "customGpuDriverPath".into(),
                    value: &custom_gpu_driver_path_binding,
                },
                register_interface::ObjectField {
                    name: "dataDiskSizeGb".into(),
                    value: &data_disk_size_gb_binding,
                },
                register_interface::ObjectField {
                    name: "dataDiskType".into(),
                    value: &data_disk_type_binding,
                },
                register_interface::ObjectField {
                    name: "desiredState".into(),
                    value: &desired_state_binding,
                },
                register_interface::ObjectField {
                    name: "diskEncryption".into(),
                    value: &disk_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "installGpuDriver".into(),
                    value: &install_gpu_driver_binding,
                },
                register_interface::ObjectField {
                    name: "instanceOwners".into(),
                    value: &instance_owners_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "machineType".into(),
                    value: &machine_type_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "nicType".into(),
                    value: &nic_type_binding,
                },
                register_interface::ObjectField {
                    name: "noProxyAccess".into(),
                    value: &no_proxy_access_binding,
                },
                register_interface::ObjectField {
                    name: "noPublicIp".into(),
                    value: &no_public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "noRemoveDataDisk".into(),
                    value: &no_remove_data_disk_binding,
                },
                register_interface::ObjectField {
                    name: "postStartupScript".into(),
                    value: &post_startup_script_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "reservationAffinity".into(),
                    value: &reservation_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountScopes".into(),
                    value: &service_account_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "shieldedInstanceConfig".into(),
                    value: &shielded_instance_config_binding,
                },
                register_interface::ObjectField {
                    name: "subnet".into(),
                    value: &subnet_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "updateTime".into(),
                    value: &update_time_binding,
                },
                register_interface::ObjectField {
                    name: "vmImage".into(),
                    value: &vm_image_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceleratorConfig".into(),
                },
                register_interface::ResultField {
                    name: "bootDiskSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "bootDiskType".into(),
                },
                register_interface::ResultField {
                    name: "containerImage".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "customGpuDriverPath".into(),
                },
                register_interface::ResultField {
                    name: "dataDiskSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "dataDiskType".into(),
                },
                register_interface::ResultField {
                    name: "desiredState".into(),
                },
                register_interface::ResultField {
                    name: "diskEncryption".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "installGpuDriver".into(),
                },
                register_interface::ResultField {
                    name: "instanceOwners".into(),
                },
                register_interface::ResultField {
                    name: "kmsKey".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "machineType".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "nicType".into(),
                },
                register_interface::ResultField {
                    name: "noProxyAccess".into(),
                },
                register_interface::ResultField {
                    name: "noPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "noRemoveDataDisk".into(),
                },
                register_interface::ResultField {
                    name: "postStartupScript".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "proxyUri".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reservationAffinity".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountScopes".into(),
                },
                register_interface::ResultField {
                    name: "shieldedInstanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnet".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vmImage".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            accelerator_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceleratorConfig").unwrap(),
            ),
            boot_disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDiskSizeGb").unwrap(),
            ),
            boot_disk_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDiskType").unwrap(),
            ),
            container_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerImage").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            custom_gpu_driver_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customGpuDriverPath").unwrap(),
            ),
            data_disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataDiskSizeGb").unwrap(),
            ),
            data_disk_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataDiskType").unwrap(),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredState").unwrap(),
            ),
            disk_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryption").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            install_gpu_driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("installGpuDriver").unwrap(),
            ),
            instance_owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceOwners").unwrap(),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKey").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            machine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineType").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            nic_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nicType").unwrap(),
            ),
            no_proxy_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noProxyAccess").unwrap(),
            ),
            no_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noPublicIp").unwrap(),
            ),
            no_remove_data_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noRemoveDataDisk").unwrap(),
            ),
            post_startup_script: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postStartupScript").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            proxy_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyUri").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reservation_affinity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationAffinity").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            service_account_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountScopes").unwrap(),
            ),
            shielded_instance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shieldedInstanceConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnet").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vm_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmImage").unwrap(),
            ),
        }
    }
}
