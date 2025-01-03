/// Provides an alias for a KMS customer master key. AWS Console enforces 1-to-1 mapping between aliases & keys,
/// but API (hence this provider too) allows you to create as many aliases as
/// the [account limits](http://docs.aws.amazon.com/kms/latest/developerguide/limits.html) allow you.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let a = key::create("a", KeyArgs::builder().build_struct());
///     let aAlias = alias::create(
///         "aAlias",
///         AliasArgs::builder()
///             .name("alias/my-key-alias")
///             .target_key_id("${a.keyId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS aliases using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/alias:Alias a alias/my-key-alias
/// ```
pub mod alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// The display name of the alias. The name must start with the word "alias" followed by a forward slash (alias/)
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates an unique alias beginning with the specified prefix.
        /// The name must start with the word "alias" followed by a forward slash (alias/).  Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier for the key for which the alias is for, can be either an ARN or key_id.
        #[builder(into)]
        pub target_key_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// The Amazon Resource Name (ARN) of the key alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The display name of the alias. The name must start with the word "alias" followed by a forward slash (alias/)
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates an unique alias beginning with the specified prefix.
        /// The name must start with the word "alias" followed by a forward slash (alias/).  Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the target key identifier.
        pub target_key_arn: pulumi_wasm_rust::Output<String>,
        /// Identifier for the key for which the alias is for, can be either an ARN or key_id.
        pub target_key_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AliasArgs) -> AliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let target_key_id_binding = args.target_key_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/alias:Alias".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "targetKeyId".into(),
                    value: &target_key_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "targetKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "targetKeyId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            target_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetKeyArn").unwrap(),
            ),
            target_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetKeyId").unwrap(),
            ),
        }
    }
}
