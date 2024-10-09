use anyhow::Result;
use arc_swap::ArcSwap;
use platform_fs::{
    common::file_system_service::CreateOptions,
    disk::file_system_service::AbstractDiskFileSystemService,
};
use platform_user_profile::user_profile_service::UserProfile;
use platform_user_profile::user_profile_service::UserProfileService as PlatformUserProfileService;
use std::path::PathBuf;
use std::sync::Arc;

pub struct UserProfileService {
    default_profile: Arc<UserProfile>,
    current_profile: ArcSwap<UserProfile>,

    fs_service: Arc<dyn AbstractDiskFileSystemService>,
}

impl UserProfileService {
    pub async fn new(
        home_dir: PathBuf,
        fs_service: Arc<dyn AbstractDiskFileSystemService>,
    ) -> Result<Self> {
        // TODO: use dirs crate
        // https://crates.io/crates/dirs

        let config_dir = home_dir.join(".config").join("moss");

        let settings_resource = config_dir
            .join("user")
            .join("default")
            .join("settings.json");

        if fs_service.file_exists(&settings_resource).await {
            if let Some(parent) = settings_resource.parent() {
                fs_service.create_dir(&parent.to_path_buf()).await?;
            }

            fs_service
                .create_file(
                    &settings_resource,
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        let default_profile = Arc::new(UserProfile {
            home: home_dir,
            settings_resource,
        });

        Ok(Self {
            default_profile: Arc::clone(&default_profile),
            current_profile: ArcSwap::new(Arc::clone(&default_profile)),
            fs_service,
        })
    }
}

#[async_trait]
impl<'a> PlatformUserProfileService<'a> for UserProfileService {
    fn default_profile(&'a self) -> &'a UserProfile {
        &self.default_profile
    }

    async fn create_profile(&self, _home: &PathBuf) -> Result<UserProfile> {
        todo!()
    }

    async fn delete_profile(&self) -> Result<()> {
        todo!()
    }

    async fn cleanup(&self) -> Result<()> {
        Ok(self
            .fs_service
            .truncate_file(&self.current_profile.load().settings_resource)
            .await?)
    }
}
