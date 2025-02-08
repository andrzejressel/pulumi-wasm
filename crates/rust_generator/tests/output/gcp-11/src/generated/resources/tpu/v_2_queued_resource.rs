/// ## Example Usage
///
/// ### Tpu V2 Queued Resource Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let qr = v_2_queued_resource::create(
///         "qr",
///         V2QueuedResourceArgs::builder()
///             .name("test-qr")
///             .project("my-project-name")
///             .tpu(
///                 V2QueuedResourceTpu::builder()
///                     .nodeSpecs(
///                         vec![
///                             V2QueuedResourceTpuNodeSpec::builder()
///                             .node(V2QueuedResourceTpuNodeSpecNode::builder()
///                             .acceleratorType("v2-8")
///                             .description("Text description of the TPU.")
///                             .runtimeVersion("tpu-vm-tf-2.13.0").build_struct())
///                             .nodeId("test-tpu")
///                             .parent("projects/my-project-name/locations/us-central1-c")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .zone("us-central1-c")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// QueuedResource can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{zone}}/queuedResources/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, QueuedResource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tpu/v2QueuedResource:V2QueuedResource default projects/{{project}}/locations/{{zone}}/queuedResources/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/v2QueuedResource:V2QueuedResource default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/v2QueuedResource:V2QueuedResource default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/v2QueuedResource:V2QueuedResource default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod v_2_queued_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2QueuedResourceArgs {
        /// The immutable name of the Queued Resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines a TPU resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tpu: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::V2QueuedResourceTpu>,
        >,
        /// The GCP location for the Queued Resource. If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct V2QueuedResourceResult {
        /// The immutable name of the Queued Resource.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Defines a TPU resource.
        /// Structure is documented below.
        pub tpu: pulumi_gestalt_rust::Output<
            Option<super::super::types::tpu::V2QueuedResourceTpu>,
        >,
        /// The GCP location for the Queued Resource. If it is not provided, the provider zone is used.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: V2QueuedResourceArgs,
    ) -> V2QueuedResourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let tpu_binding = args.tpu.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tpu/v2QueuedResource:V2QueuedResource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "tpu".into(),
                    value: &tpu_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2QueuedResourceResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            tpu: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tpu")),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
