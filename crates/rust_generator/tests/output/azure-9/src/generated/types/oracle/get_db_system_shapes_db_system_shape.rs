#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDbSystemShapesDbSystemShape {
    /// The maximum number of CPU cores that can be enabled on the DB system for this shape.
    #[builder(into)]
    #[serde(rename = "availableCoreCount")]
    pub r#available_core_count: Box<i32>,
    /// The maximum number of CPU cores per database node that can be enabled for this shape. Only applicable to the flex Exadata shape, ExaCC Elastic shapes and VM Flex shapes.
    #[builder(into)]
    #[serde(rename = "availableCoreCountPerNode")]
    pub r#available_core_count_per_node: Box<i32>,
    /// The maximum data storage that can be enabled for this shape.
    #[builder(into)]
    #[serde(rename = "availableDataStorageInTbs")]
    pub r#available_data_storage_in_tbs: Box<i32>,
    /// The maximum data storage available per storage server for this shape. Only applicable to ExaCC Elastic shapes.
    #[builder(into)]
    #[serde(rename = "availableDataStoragePerServerInTbs")]
    pub r#available_data_storage_per_server_in_tbs: Box<i32>,
    /// The maximum DB Node storage available per database node for this shape. Only applicable to ExaCC Elastic shapes.
    #[builder(into)]
    #[serde(rename = "availableDbNodePerNodeInGbs")]
    pub r#available_db_node_per_node_in_gbs: Box<i32>,
    /// The maximum DB Node storage that can be enabled for this shape.
    #[builder(into)]
    #[serde(rename = "availableDbNodeStorageInGbs")]
    pub r#available_db_node_storage_in_gbs: Box<i32>,
    /// The maximum memory that can be enabled for this shape.
    #[builder(into)]
    #[serde(rename = "availableMemoryInGbs")]
    pub r#available_memory_in_gbs: Box<i32>,
    /// The maximum memory available per database node for this shape. Only applicable to ExaCC Elastic shapes.
    #[builder(into)]
    #[serde(rename = "availableMemoryPerNodeInGbs")]
    pub r#available_memory_per_node_in_gbs: Box<i32>,
    /// The discrete number by which the CPU core count for this shape can be increased or decreased.
    #[builder(into)]
    #[serde(rename = "coreCountIncrement")]
    pub r#core_count_increment: Box<i32>,
    /// The maximum number of compute servers available for this shape.
    #[builder(into)]
    #[serde(rename = "maximumNodeCount")]
    pub r#maximum_node_count: Box<i32>,
    /// The maximum number of Exadata storage servers available for the Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "maximumStorageCount")]
    pub r#maximum_storage_count: Box<i32>,
    /// The minimum number of CPU cores that can be enabled on the DB system for this shape.
    #[builder(into)]
    #[serde(rename = "minimumCoreCount")]
    pub r#minimum_core_count: Box<i32>,
    /// The minimum number of CPU cores that can be enabled per node for this shape.
    #[builder(into)]
    #[serde(rename = "minimumCoreCountPerNode")]
    pub r#minimum_core_count_per_node: Box<i32>,
    /// The minimum data storage that need be allocated for this shape.
    #[builder(into)]
    #[serde(rename = "minimumDataStorageInTbs")]
    pub r#minimum_data_storage_in_tbs: Box<i32>,
    /// The minimum DB Node storage that need be allocated per node for this shape.
    #[builder(into)]
    #[serde(rename = "minimumDbNodeStoragePerNodeInGbs")]
    pub r#minimum_db_node_storage_per_node_in_gbs: Box<i32>,
    /// The minimum memory that need be allocated per node for this shape.
    #[builder(into)]
    #[serde(rename = "minimumMemoryPerNodeInGbs")]
    pub r#minimum_memory_per_node_in_gbs: Box<i32>,
    /// The minimum number of compute servers available for this shape.
    #[builder(into)]
    #[serde(rename = "minimumNodeCount")]
    pub r#minimum_node_count: Box<i32>,
    /// The minimum number of Exadata storage servers available for the Exadata infrastructure.
    #[builder(into)]
    #[serde(rename = "minimumStorageCount")]
    pub r#minimum_storage_count: Box<i32>,
    /// The runtime minimum number of compute servers available for this shape.
    #[builder(into)]
    #[serde(rename = "runtimeMinimumCoreCount")]
    pub r#runtime_minimum_core_count: Box<i32>,
    /// The family of the shape used for the DB system.
    #[builder(into)]
    #[serde(rename = "shapeFamily")]
    pub r#shape_family: Box<String>,
}
