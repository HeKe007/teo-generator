use teo_runtime::config::admin::Admin;
use teo_runtime::namespace::Namespace;
use teo_result::Result;
use serde::Deserialize;
use crate::utils::file::FileUtil;

static FILE_ADDRESS: &'static str = "https://raw.githubusercontent.com/teocloud/teo-admin-dev/main/";
static FILE_JSON: &'static str = ".generator/data/fileList.json";

#[derive(Deserialize)]
struct FileList {
    generated: Vec<String>,
    extended: Vec<String>,
}

pub async fn generate(main_namespace: &Namespace, admin: &Admin) -> Result<()> {
    let dest_dir = std::env::current_dir()?.join(admin.dest.as_str());
    let file_util = FileUtil::new(dest_dir.clone());
    file_util.ensure_root_directory().await?;
    let file_list = reqwest::get(FILE_ADDRESS.to_owned() + FILE_JSON)
        .await?
        .json::<FileList>()
        .await?;
    for extended_file in &file_list.extended {
        let file_location = dest_dir.join(extended_file);
        if !file_location.exists() {
            create_file_from_remote_source(extended_file, &file_util).await?;
        }
    }
    for generated_file in &file_list.generated {
        create_file_from_remote_source(generated_file, &file_util).await?;
    }
    Ok(())
}

async fn create_file_from_remote_source(location: &str, file_util: &FileUtil) -> Result<()> {
    let content = reqwest::get(FILE_ADDRESS.to_owned() + location)
        .await?
        .text()
        .await?;
    file_util.ensure_directory_and_generate_file(location, content).await
}