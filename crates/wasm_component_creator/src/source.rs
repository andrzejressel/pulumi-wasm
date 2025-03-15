use anyhow::*;
use async_trait::async_trait;
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

#[async_trait]
pub trait WasmComponentSource {
    async fn get(&self, version: &str, debug: bool) -> Result<Vec<u8>>;
}

pub struct GithubWasmComponentSource;

#[async_trait]
impl WasmComponentSource for GithubWasmComponentSource {
    async fn get(&self, version: &str, debug: bool) -> Result<Vec<u8>> {
        let profile = if debug { "debug" } else { "release" };

        let wasm_location = BaseDirs::new()
            .context("Unable to get user directories")?
            .cache_dir()
            .join("pulumi-gestalt")
            .join(format!("pulumi-gestalt-{version}-{profile}.wasm"));

        let url = format!(
            "https://github.com/andrzejressel/pulumi-gestalt/releases/download/v{version}/pulumi_gestalt-{profile}.wasm"
        );

        download_file_and_cache(wasm_location, &url)
            .await
            .context(format!(
            "Cannot download pulumi-gestalt in version {version} with profile {profile}. Url: [{url}]"
        ))
    }
}

async fn download_file_and_cache(cache: PathBuf, url: &String) -> Result<Vec<u8>> {
    if !cache.exists() {
        fs::create_dir_all(cache.parent().unwrap())?;

        let response = reqwest::get(url).await?;

        let status = response.status();

        let bytes = response
            .bytes()
            .await
            .context("Failed to download pulumi_gestalt")?;

        if !status.is_success() {
            bail!(
                "Cannot download - message from server: {}",
                String::from_utf8(bytes.to_vec()).unwrap()
            );
        }

        tokio::fs::write(&cache, &bytes)
            .await
            .context("Failed to write pulumi_gestalt file")?;
        Ok(bytes.to_vec())
    } else {
        fs::read(&cache).context(format!("Cannot read file: {}", cache.to_str().unwrap()))
    }
}

pub struct FileSource(PathBuf);

impl FileSource {
    pub fn new(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }
}

#[async_trait]
impl WasmComponentSource for FileSource {
    async fn get(&self, _version: &str, _debug: bool) -> Result<Vec<u8>> {
        Ok(fs::read(&self.0)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod pulumi_gestalt_source {
        use super::*;

        #[tokio::test]
        async fn should_download_existing_pulumi_gestalt() -> Result<()> {
            let source = GithubWasmComponentSource {};
            let res = source.get("0.0.1", false).await?;
            assert!(!res.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn should_download_existing_debug_pulumi_gestalt() -> Result<()> {
            let source = GithubWasmComponentSource {};
            let res = source.get("0.0.1", true).await?;
            assert!(!res.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn should_fail_on_noexisting_version() -> Result<()> {
            let source = GithubWasmComponentSource {};
            let err = source
                .get("0.0.0-nonexistent-version", false)
                .await
                .expect_err("Expected error");
            assert_eq!(
                err.to_string(),
                "Cannot download pulumi-gestalt in version 0.0.0-nonexistent-version with profile release. Url: [https://github.com/andrzejressel/pulumi-gestalt/releases/download/v0.0.0-nonexistent-version/pulumi_gestalt-release.wasm]"
            );
            Ok(())
        }
    }
}
