/// Manages a Service Catalog Tag Option Resource Association.
///
/// > **Tip:** A "resource" is either a Service Catalog portfolio or product.
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
///     let example = tag_option_resource_association::create(
///         "example",
///         TagOptionResourceAssociationArgs::builder()
///             .resource_id("prod-dnigbtea24ste")
///             .tag_option_id("tag-pjtvyakdlyo3m")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_tag_option_resource_association` using the tag option ID and resource ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/tagOptionResourceAssociation:TagOptionResourceAssociation example tag-pjtvyakdlyo3m:prod-dnigbtea24ste
/// ```
pub mod tag_option_resource_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagOptionResourceAssociationArgs {
        /// Resource identifier.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tag Option identifier.
        #[builder(into)]
        pub tag_option_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagOptionResourceAssociationResult {
        /// ARN of the resource.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// Creation time of the resource.
        pub resource_created_time: pulumi_wasm_rust::Output<String>,
        /// Description of the resource.
        pub resource_description: pulumi_wasm_rust::Output<String>,
        /// Resource identifier.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// Description of the resource.
        pub resource_name: pulumi_wasm_rust::Output<String>,
        /// Tag Option identifier.
        pub tag_option_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TagOptionResourceAssociationArgs,
    ) -> TagOptionResourceAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let tag_option_id_binding = args.tag_option_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/tagOptionResourceAssociation:TagOptionResourceAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "tagOptionId".into(),
                    value: &tag_option_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceCreatedTime".into(),
                },
                register_interface::ResultField {
                    name: "resourceDescription".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceName".into(),
                },
                register_interface::ResultField {
                    name: "tagOptionId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagOptionResourceAssociationResult {
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            resource_created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceCreatedTime").unwrap(),
            ),
            resource_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceDescription").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            resource_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceName").unwrap(),
            ),
            tag_option_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagOptionId").unwrap(),
            ),
        }
    }
}
