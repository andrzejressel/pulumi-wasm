/// Adds an IoT Thing to an IoT Thing Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = thing_group_membership::create(
///         "example",
///         ThingGroupMembershipArgs::builder()
///             .override_dynamic_group(true)
///             .thing_group_name("example-group")
///             .thing_name("example-thing")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT Thing Group Membership using the thing group name and thing name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/thingGroupMembership:ThingGroupMembership example thing_group_name/thing_name
/// ```
pub mod thing_group_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingGroupMembershipArgs {
        /// Override dynamic thing groups with static thing groups when 10-group limit is reached. If a thing belongs to 10 thing groups, and one or more of those groups are dynamic thing groups, adding a thing to a static group removes the thing from the last dynamic group.
        #[builder(into, default)]
        pub override_dynamic_group: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the group to which you are adding a thing.
        #[builder(into)]
        pub thing_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the thing to add to a group.
        #[builder(into)]
        pub thing_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ThingGroupMembershipResult {
        /// Override dynamic thing groups with static thing groups when 10-group limit is reached. If a thing belongs to 10 thing groups, and one or more of those groups are dynamic thing groups, adding a thing to a static group removes the thing from the last dynamic group.
        pub override_dynamic_group: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the group to which you are adding a thing.
        pub thing_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the thing to add to a group.
        pub thing_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ThingGroupMembershipArgs,
    ) -> ThingGroupMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let override_dynamic_group_binding = args.override_dynamic_group.get_inner();
        let thing_group_name_binding = args.thing_group_name.get_inner();
        let thing_name_binding = args.thing_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/thingGroupMembership:ThingGroupMembership".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "overrideDynamicGroup".into(),
                    value: &override_dynamic_group_binding,
                },
                register_interface::ObjectField {
                    name: "thingGroupName".into(),
                    value: &thing_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "thingName".into(),
                    value: &thing_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "overrideDynamicGroup".into(),
                },
                register_interface::ResultField {
                    name: "thingGroupName".into(),
                },
                register_interface::ResultField {
                    name: "thingName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ThingGroupMembershipResult {
            override_dynamic_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overrideDynamicGroup").unwrap(),
            ),
            thing_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thingGroupName").unwrap(),
            ),
            thing_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thingName").unwrap(),
            ),
        }
    }
}