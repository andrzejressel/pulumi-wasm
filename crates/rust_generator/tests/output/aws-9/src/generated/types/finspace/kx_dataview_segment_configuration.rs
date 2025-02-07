#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxDataviewSegmentConfiguration {
    /// The database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume.
    #[builder(into)]
    #[serde(rename = "dbPaths")]
    pub r#db_paths: Box<Vec<String>>,
    /// Enables on-demand caching on the selected database path when a particular file or a column of the database is accessed. When on demand caching is **True**, dataviews perform minimal loading of files on the filesystem as needed. When it is set to **False**, everything is cached. The default value is **False**.
    #[builder(into, default)]
    #[serde(rename = "onDemand")]
    pub r#on_demand: Box<Option<bool>>,
    /// The name of the volume that you want to attach to a dataview. This volume must be in the same availability zone as the dataview that you are attaching to.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<String>,
}
