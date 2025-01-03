/// Catalogs are top-level containers for Databases and Tables.
///
///
/// To get more information about Catalog, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/biglake/rest/v1/projects.locations.catalogs)
/// * How-to Guides
///     * [Manage open source metadata with BigLake Metastore](https://cloud.google.com/bigquery/docs/manage-open-source-metadata#create_catalogs)
///
/// ## Example Usage
///
/// ### Bigquery Biglake Catalog
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = catalog::create(
///         "default",
///         CatalogArgs::builder().location("US").name("my_catalog").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Catalog can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/catalogs/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Catalog can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:biglake/catalog:Catalog default projects/{{project}}/locations/{{location}}/catalogs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:biglake/catalog:Catalog default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:biglake/catalog:Catalog default {{location}}/{{name}}
/// ```
///
pub mod catalog {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogArgs {
        /// The geographic location where the Catalog should reside.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Catalog. Format:
        /// projects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CatalogResult {
        /// Output only. The creation time of the catalog. A timestamp in RFC3339 UTC
        /// "Zulu" format, with nanosecond resolution and up to nine fractional
        /// digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The deletion time of the catalog. Only set after the catalog
        /// is deleted. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
        /// resolution and up to nine fractional digits.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when this catalog is considered expired. Only set
        /// after the catalog is deleted. Only set after the catalog is deleted.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits.
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// The geographic location where the Catalog should reside.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Catalog. Format:
        /// projects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. The last modification time of the catalog. A timestamp in
        /// RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CatalogArgs) -> CatalogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:biglake/catalog:Catalog".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CatalogResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
