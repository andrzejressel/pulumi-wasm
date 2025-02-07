#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryRemoteRepositoryConfigCommonRepository {
    /// One of:
    /// a. Artifact Registry Repository resource, e.g. `projects/UPSTREAM_PROJECT_ID/locations/REGION/repositories/UPSTREAM_REPOSITORY`
    /// b. URI to the registry, e.g. `"https://registry-1.docker.io"`
    /// c. URI to Artifact Registry Repository, e.g. `"https://REGION-docker.pkg.dev/UPSTREAM_PROJECT_ID/UPSTREAM_REPOSITORY"`
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
