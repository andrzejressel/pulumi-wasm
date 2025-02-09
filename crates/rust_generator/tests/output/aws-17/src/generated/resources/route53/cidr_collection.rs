/// Provides a Route53 CIDR collection resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cidr_collection::create(
///         "example",
///         CidrCollectionArgs::builder().name("collection-1").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CIDR collections using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/cidrCollection:CidrCollection example 9ac32814-3e67-0932-6048-8d779cc6f511
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cidr_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CidrCollectionArgs {
        /// Unique name for the CIDR collection.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CidrCollectionResult {
        /// The Amazon Resource Name (ARN) of the CIDR collection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the CIDR collection.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The lastest version of the CIDR collection.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CidrCollectionArgs,
    ) -> CidrCollectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/cidrCollection:CidrCollection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CidrCollectionResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            version: o.get_field("version"),
        }
    }
}
