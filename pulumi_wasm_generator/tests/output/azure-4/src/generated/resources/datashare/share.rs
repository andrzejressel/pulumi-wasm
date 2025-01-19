/// Manages a Data Share.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:datashare:Account
///     name: example
///     properties:
///       name: example-dsa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///       tags:
///         foo: bar
///   exampleShare:
///     type: azure:datashare:Share
///     name: example
///     properties:
///       name: example_dss
///       accountId: ${exampleAccount.id}
///       kind: CopyBased
///       description: example desc
///       terms: example terms
///       snapshotSchedule:
///         name: example-ss
///         recurrence: Day
///         startTime: 2020-04-17T04:47:52.9614956Z
/// ```
///
/// ## Import
///
/// Data Shares can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/share:Share example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1
/// ```
///
pub mod share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareArgs {
        /// The ID of the Data Share account in which the Data Share is created. Changing this forces a new Data Share to be created.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The Data Share's description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The kind of the Data Share. Possible values are `CopyBased` and `InPlace`. Changing this forces a new Data Share to be created.
        #[builder(into)]
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Share. Changing this forces a new Data Share to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `snapshot_schedule` block as defined below.
        #[builder(into, default)]
        pub snapshot_schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::datashare::ShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        #[builder(into, default)]
        pub terms: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ShareResult {
        /// The ID of the Data Share account in which the Data Share is created. Changing this forces a new Data Share to be created.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The Data Share's description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The kind of the Data Share. Possible values are `CopyBased` and `InPlace`. Changing this forces a new Data Share to be created.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Share. Changing this forces a new Data Share to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `snapshot_schedule` block as defined below.
        pub snapshot_schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::datashare::ShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        pub terms: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ShareArgs) -> ShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let description_binding = args.description.get_inner();
        let kind_binding = args.kind.get_inner();
        let name_binding = args.name.get_inner();
        let snapshot_schedule_binding = args.snapshot_schedule.get_inner();
        let terms_binding = args.terms.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datashare/share:Share".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotSchedule".into(),
                    value: &snapshot_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "terms".into(),
                    value: &terms_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "snapshotSchedule".into(),
                },
                register_interface::ResultField {
                    name: "terms".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ShareResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            snapshot_schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotSchedule").unwrap(),
            ),
            terms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terms").unwrap(),
            ),
        }
    }
}
