/// Manages an EC2 Availability Zone Group, such as updating its opt-in status.
///
/// > **NOTE:** This is an advanced resource. The provider will automatically assume management of the EC2 Availability Zone Group without import and perform no actions on removal from configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = availability_zone_group::create(
///         "example",
///         AvailabilityZoneGroupArgs::builder()
///             .group_name("us-west-2-lax-1")
///             .opt_in_status("opted-in")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 Availability Zone Groups using the group name. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/availabilityZoneGroup:AvailabilityZoneGroup example us-west-2-lax-1
/// ```
pub mod availability_zone_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AvailabilityZoneGroupArgs {
        /// Name of the Availability Zone Group.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to enable or disable Availability Zone Group. Valid values: `opted-in` or `not-opted-in`.
        #[builder(into)]
        pub opt_in_status: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AvailabilityZoneGroupResult {
        /// Name of the Availability Zone Group.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether to enable or disable Availability Zone Group. Valid values: `opted-in` or `not-opted-in`.
        pub opt_in_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AvailabilityZoneGroupArgs,
    ) -> AvailabilityZoneGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_name_binding = args.group_name.get_inner();
        let opt_in_status_binding = args.opt_in_status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/availabilityZoneGroup:AvailabilityZoneGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "optInStatus".into(),
                    value: &opt_in_status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "optInStatus".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AvailabilityZoneGroupResult {
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            opt_in_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optInStatus").unwrap(),
            ),
        }
    }
}