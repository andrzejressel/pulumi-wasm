#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InteractiveQueryClusterMetastores {
    /// An `ambari` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "ambari")]
    pub r#ambari: Box<Option<super::super::types::hdinsight::InteractiveQueryClusterMetastoresAmbari>>,
    /// A `hive` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "hive")]
    pub r#hive: Box<Option<super::super::types::hdinsight::InteractiveQueryClusterMetastoresHive>>,
    /// An `oozie` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "oozie")]
    pub r#oozie: Box<Option<super::super::types::hdinsight::InteractiveQueryClusterMetastoresOozie>>,
}
