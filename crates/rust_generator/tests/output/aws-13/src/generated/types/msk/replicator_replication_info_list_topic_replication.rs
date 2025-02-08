#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ReplicatorReplicationInfoListTopicReplication {
    /// Whether to periodically configure remote topic ACLs to match their corresponding upstream topics.
    #[builder(into, default)]
    #[serde(rename = "copyAccessControlListsForTopics")]
    pub r#copy_access_control_lists_for_topics: Box<Option<bool>>,
    /// Whether to periodically configure remote topics to match their corresponding upstream topics.
    #[builder(into, default)]
    #[serde(rename = "copyTopicConfigurations")]
    pub r#copy_topic_configurations: Box<Option<bool>>,
    /// Whether to periodically check for new topics and partitions.
    #[builder(into, default)]
    #[serde(rename = "detectAndCopyNewTopics")]
    pub r#detect_and_copy_new_topics: Box<Option<bool>>,
    /// Configuration for specifying the position in the topics to start replicating from.
    #[builder(into, default)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: Box<Option<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationStartingPosition>>,
    #[builder(into, default)]
    #[serde(rename = "topicNameConfiguration")]
    pub r#topic_name_configuration: Box<Option<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration>>,
    /// List of regular expression patterns indicating the topics that should not be replica.
    #[builder(into, default)]
    #[serde(rename = "topicsToExcludes")]
    pub r#topics_to_excludes: Box<Option<Vec<String>>>,
    /// List of regular expression patterns indicating the topics to copy.
    #[builder(into)]
    #[serde(rename = "topicsToReplicates")]
    pub r#topics_to_replicates: Box<Vec<String>>,
}
