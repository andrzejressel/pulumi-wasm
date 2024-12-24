#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The description of the teams list.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items of the teams list.
    #[builder(into, default)]
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The items of the teams list that has explicit description.
    #[builder(into, default)]
    pub items_with_descriptions: pulumi_wasm_rust::Output<
        Option<Vec<super::types::TeamsListItemsWithDescription>>,
    >,
    /// Name of the teams list.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}
pub struct TeamsListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The description of the teams list.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items of the teams list.
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The items of the teams list that has explicit description.
    pub items_with_descriptions: pulumi_wasm_rust::Output<
        Option<Vec<super::types::TeamsListItemsWithDescription>>,
    >,
    /// Name of the teams list.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    pub type_: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsListArgs) -> TeamsListResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let description_binding = args.description.get_inner();
    let items_binding = args.items.get_inner();
    let items_with_descriptions_binding = args.items_with_descriptions.get_inner();
    let name_binding = args.name.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/teamsList:TeamsList".into(),
        name: name.to_string(),
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
                name: "items".into(),
                value: &items_binding,
            },
            register_interface::ObjectField {
                name: "itemsWithDescriptions".into(),
                value: &items_with_descriptions_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "items".into() },
            register_interface::ResultField { name : "itemsWithDescriptions".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "type".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    TeamsListResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        items: into_domain(hashmap.remove("items").unwrap()),
        items_with_descriptions: into_domain(
            hashmap.remove("itemsWithDescriptions").unwrap(),
        ),
        name: into_domain(hashmap.remove("name").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
