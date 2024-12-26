pub struct GetUserResult {
    /// The user's email address.
    pub email: pulumi_wasm_rust::Output<String>,
    /// The user's unique identifier.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The user's username.
    pub username: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke() -> GetUserResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getUser:getUser".into(),
        object: Vec::from([]),
        results: vec![
            register_interface::ResultField { name : "email".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "username".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetUserResult {
        email: into_domain(hashmap.remove("email").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        username: into_domain(hashmap.remove("username").unwrap()),
    }
}
