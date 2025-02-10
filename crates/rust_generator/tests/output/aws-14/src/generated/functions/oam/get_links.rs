#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_links {
    #[allow(dead_code)]
    pub struct GetLinksResult {
        /// Set of ARN of the Links.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetLinksResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:oam/getLinks:getLinks".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetLinksResult {
            arns: o.get_field("arns"),
            id: o.get_field("id"),
        }
    }
}
