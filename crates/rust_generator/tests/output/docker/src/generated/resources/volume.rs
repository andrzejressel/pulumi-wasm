/// <!-- Bug: Type and Name are switched -->
/// Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sharedVolume = volume::create(
///         "sharedVolume",
///         VolumeArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `volume` as follows
///
/// #!/bin/bash
///
/// docker volume create
///
/// prints the long ID
///
/// 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
///
/// you provide the definition for the resource as follows
///
/// terraform
///
/// resource "docker_volume" "foo" {
///
///   name = "524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d"
///
/// }
///
/// then the import command is as follows
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/volume:Volume foo 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// Driver type for the volume. Defaults to `local`.
        #[builder(into, default)]
        pub driver: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options specific to the driver.
        #[builder(into, default)]
        pub driver_opts: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::VolumeLabel>>,
        >,
        /// The name of the Docker volume (will be generated if not provided).
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// Driver type for the volume. Defaults to `local`.
        pub driver: pulumi_gestalt_rust::Output<String>,
        /// Options specific to the driver.
        pub driver_opts: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-defined key/value metadata
        pub labels: pulumi_gestalt_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
        /// The mountpoint of the volume.
        pub mountpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the Docker volume (will be generated if not provided).
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VolumeArgs,
    ) -> VolumeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let driver_binding_1 = args.driver.get_output(context);
        let driver_binding = driver_binding_1.get_inner();
        let driver_opts_binding_1 = args.driver_opts.get_output(context);
        let driver_opts_binding = driver_opts_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/volume:Volume".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "driver".into(),
                    value: &driver_binding,
                },
                register_interface::ObjectField {
                    name: "driverOpts".into(),
                    value: &driver_opts_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VolumeResult {
            driver: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("driver"),
            ),
            driver_opts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("driverOpts"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            mountpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mountpoint"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
