#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTriggerBuildArtifact {
    /// A list of images to be pushed upon the successful completion of all build steps.
    /// 
    /// The images will be pushed using the builder service account's credentials.
    /// 
    /// The digests of the pushed images will be stored in the Build resource's results field.
    /// 
    /// If any of the images fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "images")]
    pub r#images: Box<Vec<String>>,
    /// A Maven artifact to upload to Artifact Registry upon successful completion of all build steps.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "mavenArtifacts")]
    pub r#maven_artifacts: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactMavenArtifact>>,
    /// Npm package to upload to Artifact Registry upon successful completion of all build steps.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "npmPackages")]
    pub r#npm_packages: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactNpmPackage>>,
    /// A list of objects to be uploaded to Cloud Storage upon successful completion of all build steps.
    /// 
    /// Files in the workspace matching specified paths globs will be uploaded to the
    /// Cloud Storage location using the builder service account's credentials.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "objects")]
    pub r#objects: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactObject>>,
    /// Python package to upload to Artifact Registry upon successful completion of all build steps. A package can encapsulate multiple objects to be uploaded to a single repository.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "pythonPackages")]
    pub r#python_packages: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactPythonPackage>>,
}
