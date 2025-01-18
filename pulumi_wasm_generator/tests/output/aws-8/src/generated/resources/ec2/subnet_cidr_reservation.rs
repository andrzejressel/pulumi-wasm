/// Provides a subnet CIDR reservation resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subnet_cidr_reservation::create(
///         "example",
///         SubnetCidrReservationArgs::builder()
///             .cidr_block("10.0.0.16/28")
///             .reservation_type("prefix")
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Existing CIDR reservations using `SUBNET_ID:RESERVATION_ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/subnetCidrReservation:SubnetCidrReservation example subnet-01llsxvsxabqiymcz:scr-4mnvz6wb7otksjcs9
/// ```
pub mod subnet_cidr_reservation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetCidrReservationArgs {
        /// The CIDR block for the reservation.
        #[builder(into)]
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// A brief description of the reservation.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of reservation to create. Valid values: `explicit`, `prefix`
        #[builder(into)]
        pub reservation_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet to create the reservation for.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetCidrReservationResult {
        /// The CIDR block for the reservation.
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// A brief description of the reservation.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the AWS account that owns this CIDR reservation.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The type of reservation to create. Valid values: `explicit`, `prefix`
        pub reservation_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the subnet to create the reservation for.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SubnetCidrReservationArgs,
    ) -> SubnetCidrReservationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_block_binding = args.cidr_block.get_inner();
        let description_binding = args.description.get_inner();
        let reservation_type_binding = args.reservation_type.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/subnetCidrReservation:SubnetCidrReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "reservationType".into(),
                    value: &reservation_type_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "reservationType".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetCidrReservationResult {
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            reservation_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationType").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}
