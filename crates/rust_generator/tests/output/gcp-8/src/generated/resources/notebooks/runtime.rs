/// A Cloud AI Platform Notebook runtime.
///
///
/// > **Note:** Due to limitations of the Notebooks Runtime API, many fields
/// in this resource do not properly detect drift. These fields will also not
/// appear in state once imported.
///
///
/// To get more information about Runtime, see:
///
/// * [API documentation](https://cloud.google.com/ai-platform/notebooks/docs/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/ai-platform-notebooks)
///
/// ## Example Usage
///
/// ### Notebook Runtime Basic
///
///
/// ```yaml
/// resources:
///   runtime:
///     type: gcp:notebooks:Runtime
///     properties:
///       name: notebooks-runtime
///       location: us-central1
///       accessConfig:
///         accessType: SINGLE_USER
///         runtimeOwner: admin@hashicorptest.com
///       virtualMachine:
///         virtualMachineConfig:
///           machineType: n1-standard-4
///           dataDisk:
///             initializeParams:
///               diskSizeGb: '100'
///               diskType: PD_STANDARD
/// ```
/// ### Notebook Runtime Basic Gpu
///
///
/// ```yaml
/// resources:
///   runtimeGpu:
///     type: gcp:notebooks:Runtime
///     name: runtime_gpu
///     properties:
///       name: notebooks-runtime-gpu
///       location: us-central1
///       accessConfig:
///         accessType: SINGLE_USER
///         runtimeOwner: admin@hashicorptest.com
///       softwareConfig:
///         installGpuDriver: true
///       virtualMachine:
///         virtualMachineConfig:
///           machineType: n1-standard-4
///           dataDisk:
///             initializeParams:
///               diskSizeGb: '100'
///               diskType: PD_STANDARD
///           acceleratorConfig:
///             coreCount: '1'
///             type: NVIDIA_TESLA_V100
/// ```
/// ### Notebook Runtime Basic Container
///
///
/// ```yaml
/// resources:
///   runtimeContainer:
///     type: gcp:notebooks:Runtime
///     name: runtime_container
///     properties:
///       name: notebooks-runtime-container
///       location: us-central1
///       accessConfig:
///         accessType: SINGLE_USER
///         runtimeOwner: admin@hashicorptest.com
///       virtualMachine:
///         virtualMachineConfig:
///           machineType: n1-standard-4
///           dataDisk:
///             initializeParams:
///               diskSizeGb: '100'
///               diskType: PD_STANDARD
///           containerImages:
///             - repository: gcr.io/deeplearning-platform-release/base-cpu
///               tag: latest
///             - repository: gcr.io/deeplearning-platform-release/beam-notebooks
///               tag: latest
/// ```
/// ### Notebook Runtime Kernels
///
///
/// ```yaml
/// resources:
///   runtimeContainer:
///     type: gcp:notebooks:Runtime
///     name: runtime_container
///     properties:
///       name: notebooks-runtime-kernel
///       location: us-central1
///       accessConfig:
///         accessType: SINGLE_USER
///         runtimeOwner: admin@hashicorptest.com
///       softwareConfig:
///         kernels:
///           - repository: gcr.io/deeplearning-platform-release/base-cpu
///             tag: latest
///       virtualMachine:
///         virtualMachineConfig:
///           machineType: n1-standard-4
///           dataDisk:
///             initializeParams:
///               diskSizeGb: '100'
///               diskType: PD_STANDARD
///       labels:
///         k: val
/// ```
/// ### Notebook Runtime Script
///
///
/// ```yaml
/// resources:
///   runtimeContainer:
///     type: gcp:notebooks:Runtime
///     name: runtime_container
///     properties:
///       name: notebooks-runtime-script
///       location: us-central1
///       accessConfig:
///         accessType: SINGLE_USER
///         runtimeOwner: admin@hashicorptest.com
///       softwareConfig:
///         postStartupScriptBehavior: RUN_EVERY_START
///       virtualMachine:
///         virtualMachineConfig:
///           machineType: n1-standard-4
///           dataDisk:
///             initializeParams:
///               diskSizeGb: '100'
///               diskType: PD_STANDARD
///       labels:
///         k: val
/// ```
///
/// ## Import
///
/// Runtime can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/runtimes/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Runtime can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:notebooks/runtime:Runtime default projects/{{project}}/locations/{{location}}/runtimes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:notebooks/runtime:Runtime default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:notebooks/runtime:Runtime default {{location}}/{{name}}
/// ```
///
pub mod runtime {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuntimeArgs {
        /// The config settings for accessing runtime.
        /// Structure is documented below.
        #[builder(into, default)]
        pub access_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::RuntimeAccessConfig>,
        >,
        /// The labels to associate with this runtime. Label **keys** must
        /// contain 1 to 63 characters, and must conform to [RFC 1035]
        /// (https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be
        /// empty, but, if present, must contain 1 to 63 characters, and must
        /// conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No
        /// more than 32 labels can be associated with a cluster.
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
        /// The name specified for the Notebook runtime.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The config settings for software inside the runtime.
        /// Structure is documented below.
        #[builder(into, default)]
        pub software_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::RuntimeSoftwareConfig>,
        >,
        /// Use a Compute Engine VM image to start the managed notebook instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub virtual_machine: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::notebooks::RuntimeVirtualMachine>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuntimeResult {
        /// The config settings for accessing runtime.
        /// Structure is documented below.
        pub access_config: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::RuntimeAccessConfig>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The health state of this runtime. For a list of possible output
        /// values, see `https://cloud.google.com/vertex-ai/docs/workbench/
        /// reference/rest/v1/projects.locations.runtimes#healthstate`.
        pub health_state: pulumi_wasm_rust::Output<String>,
        /// The labels to associate with this runtime. Label **keys** must
        /// contain 1 to 63 characters, and must conform to [RFC 1035]
        /// (https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be
        /// empty, but, if present, must contain 1 to 63 characters, and must
        /// conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No
        /// more than 32 labels can be associated with a cluster.
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
        /// Contains Runtime daemon metrics such as Service status and JupyterLab
        /// status
        /// Structure is documented below.
        pub metrics: pulumi_wasm_rust::Output<
            Vec<super::super::types::notebooks::RuntimeMetric>,
        >,
        /// The name specified for the Notebook runtime.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The config settings for software inside the runtime.
        /// Structure is documented below.
        pub software_config: pulumi_wasm_rust::Output<
            super::super::types::notebooks::RuntimeSoftwareConfig,
        >,
        /// The state of this runtime.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Use a Compute Engine VM image to start the managed notebook instance.
        /// Structure is documented below.
        pub virtual_machine: pulumi_wasm_rust::Output<
            Option<super::super::types::notebooks::RuntimeVirtualMachine>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RuntimeArgs,
    ) -> RuntimeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_config_binding = args.access_config.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let software_config_binding = args
            .software_config
            .get_output(context)
            .get_inner();
        let virtual_machine_binding = args
            .virtual_machine
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:notebooks/runtime:Runtime".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessConfig".into(),
                    value: &access_config_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "softwareConfig".into(),
                    value: &software_config_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachine".into(),
                    value: &virtual_machine_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RuntimeResult {
            access_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessConfig"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            health_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthState"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            metrics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metrics"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            software_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("softwareConfig"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            virtual_machine: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualMachine"),
            ),
        }
    }
}
