/// Manages an AppStream Fleet Stack association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder()
///             .compute_capacity(
///                 FleetComputeCapacity::builder().desiredInstances(1).build_struct(),
///             )
///             .image_name("Amazon-AppStream2-Sample-Image-03-11-2023")
///             .instance_type("stream.standard.small")
///             .name("NAME")
///             .build_struct(),
///     );
///     let exampleFleetStackAssociation = fleet_stack_association::create(
///         "exampleFleetStackAssociation",
///         FleetStackAssociationArgs::builder()
///             .fleet_name("${example.name}")
///             .stack_name("${exampleStack.name}")
///             .build_struct(),
///     );
///     let exampleStack = stack::create(
///         "exampleStack",
///         StackArgs::builder().name("STACK NAME").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppStream Stack Fleet Association using the `fleet_name` and `stack_name` separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appstream/fleetStackAssociation:FleetStackAssociation example fleetName/stackName
/// ```
pub mod fleet_stack_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetStackAssociationArgs {
        /// Name of the fleet.
        #[builder(into)]
        pub fleet_name: pulumi_wasm_rust::Output<String>,
        /// Name of the stack.
        #[builder(into)]
        pub stack_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FleetStackAssociationResult {
        /// Name of the fleet.
        pub fleet_name: pulumi_wasm_rust::Output<String>,
        /// Name of the stack.
        pub stack_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FleetStackAssociationArgs,
    ) -> FleetStackAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let fleet_name_binding = args.fleet_name.get_inner();
        let stack_name_binding = args.stack_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/fleetStackAssociation:FleetStackAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fleetName".into(),
                    value: &fleet_name_binding,
                },
                register_interface::ObjectField {
                    name: "stackName".into(),
                    value: &stack_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "fleetName".into(),
                },
                register_interface::ResultField {
                    name: "stackName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FleetStackAssociationResult {
            fleet_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetName").unwrap(),
            ),
            stack_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackName").unwrap(),
            ),
        }
    }
}