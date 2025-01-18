/// <!-- Bug: Type and Name are switched -->
/// Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// Driver type for the volume. Defaults to `local`.
        #[builder(into, default)]
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Options specific to the driver.
        #[builder(into, default)]
        pub driver_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
        /// The name of the Docker volume (will be generated if not provided).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// Driver type for the volume. Defaults to `local`.
        pub driver: pulumi_wasm_rust::Output<String>,
        /// Options specific to the driver.
        pub driver_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
        /// The mountpoint of the volume.
        pub mountpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the Docker volume (will be generated if not provided).
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VolumeArgs) -> VolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let driver_binding = args.driver.get_inner();
        let driver_opts_binding = args.driver_opts.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "driver".into(),
                },
                register_interface::ResultField {
                    name: "driverOpts".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "mountpoint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VolumeResult {
            driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driver").unwrap(),
            ),
            driver_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driverOpts").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            mountpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountpoint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
