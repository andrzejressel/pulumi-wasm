/// Provides a resource to manage AWS Data Exchange Revisions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = revision::create(
///         "example",
///         RevisionArgs::builder()
///             .data_set_id("${exampleAwsDataexchangeDataSet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataExchange Revisions using their `data-set-id:revision-id`. For example:
///
/// ```sh
/// $ pulumi import aws:dataexchange/revision:Revision example 4fa784c7-ccb4-4dbf-ba4f-02198320daa1:4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
pub mod revision {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RevisionArgs {
        /// An optional comment about the revision.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The dataset id.
        #[builder(into)]
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RevisionResult {
        /// The Amazon Resource Name of this data set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// An optional comment about the revision.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The dataset id.
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// The Id of the revision.
        pub revision_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RevisionArgs) -> RevisionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_inner();
        let data_set_id_binding = args.data_set_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dataexchange/revision:Revision".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "dataSetId".into(),
                    value: &data_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "dataSetId".into(),
                },
                register_interface::ResultField {
                    name: "revisionId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RevisionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            data_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSetId").unwrap(),
            ),
            revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
