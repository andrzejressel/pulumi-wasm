/// Resource for managing an AWS IdentityStore Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let this = group::create(
///         "this",
///         GroupArgs::builder()
///             .description("Example description")
///             .display_name("Example group")
///             .identity_store_id("${example.identityStoreIds[0]}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Identity Store Group using the combination `identity_store_id/group_id`. For example:
///
/// ```sh
/// $ pulumi import aws:identitystore/group:Group example d-9c6705e95c/b8a1c340-8031-7071-a2fb-7dc540320c30
/// ```
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// A string containing the description of the group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A string containing the name of the group. This value is commonly displayed when the group is referenced.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The globally unique identifier for the identity store.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// A string containing the description of the group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A string containing the name of the group. This value is commonly displayed when the group is referenced.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// A list of external IDs that contains the identifiers issued to this resource by an external identity provider. See External IDs below.
        pub external_ids: pulumi_wasm_rust::Output<
            Vec<super::super::types::identitystore::GroupExternalId>,
        >,
        /// The identifier of the newly created group in the identity store.
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// The globally unique identifier for the identity store.
        ///
        /// The following arguments are optional:
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GroupArgs) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let identity_store_id_binding = args.identity_store_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:identitystore/group:Group".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "externalIds".into(),
                },
                register_interface::ResultField {
                    name: "groupId".into(),
                },
                register_interface::ResultField {
                    name: "identityStoreId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            external_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIds").unwrap(),
            ),
            group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupId").unwrap(),
            ),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityStoreId").unwrap(),
            ),
        }
    }
}