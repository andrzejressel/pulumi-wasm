/// Resource for managing an AWS Service Quotas Template Association.
///
/// > Only the management account of an organization can associate Service Quota templates, and this must be done from the `us-east-1` region.
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
///     let example = template_association::create(
///         "example",
///         TemplateAssociationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Quotas Template Association using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:servicequotas/templateAssociation:TemplateAssociation example 123456789012
/// ```
pub mod template_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateAssociationArgs {
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TemplateAssociationResult {
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Association status. Creating this resource will result in an `ASSOCIATED` status, and quota increase requests in the template are automatically applied to new AWS accounts in the organization.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TemplateAssociationArgs,
    ) -> TemplateAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicequotas/templateAssociation:TemplateAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "skipDestroy".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TemplateAssociationResult {
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
