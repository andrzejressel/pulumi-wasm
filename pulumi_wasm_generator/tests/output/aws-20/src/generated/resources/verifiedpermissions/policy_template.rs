/// Resource for managing an AWS Verified Permissions Policy Template.
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
///     let example = policy_template::create(
///         "example",
///         PolicyTemplateArgs::builder()
///             .policy_store_id("${exampleAwsVerifiedpermissionsPolicyStore.id}")
///             .statement(
///                 "permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy Store using the `policy_store_id:policy_template_id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedpermissions/policyTemplate:PolicyTemplate example policyStoreId:policyTemplateId
/// ```
pub mod policy_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTemplateArgs {
        /// Provides a description for the policy template.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Policy Store.
        #[builder(into)]
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
        /// Defines the content of the statement, written in Cedar policy language.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub statement: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTemplateResult {
        /// The date the Policy Store was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Provides a description for the policy template.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Policy Store.
        pub policy_template_id: pulumi_wasm_rust::Output<String>,
        /// Defines the content of the statement, written in Cedar policy language.
        ///
        /// The following arguments are optional:
        pub statement: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyTemplateArgs) -> PolicyTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let policy_store_id_binding = args.policy_store_id.get_inner();
        let statement_binding = args.statement.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policyTemplate:PolicyTemplate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "policyStoreId".into(),
                    value: &policy_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "statement".into(),
                    value: &statement_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "policyStoreId".into(),
                },
                register_interface::ResultField {
                    name: "policyTemplateId".into(),
                },
                register_interface::ResultField {
                    name: "statement".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyTemplateResult {
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            policy_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyStoreId").unwrap(),
            ),
            policy_template_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyTemplateId").unwrap(),
            ),
            statement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statement").unwrap(),
            ),
        }
    }
}
