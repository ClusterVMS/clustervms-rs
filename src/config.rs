use figment::{Figment, providers::{Format, Toml}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::common::{Camera, CameraId};


const DEFAULT_CONFIG_FILE: &str = "/etc/clustervms/clustervms.toml";

fn base_url_default() -> String {
	"http://clustervms.localdomain".to_owned()
}


#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ClusterVmsConfig {
	// base URL of the ClusterVMS system, e.g. "http://clustervms.localdomain"
	#[serde(default = "base_url_default")]
	pub base_url: String,

	#[serde(default)]
	pub cameras: HashMap<CameraId, Camera>,
}

#[derive(Clone)]
pub struct ConfigManager {
	config: ClusterVmsConfig,
}

impl ConfigManager {
	pub fn new() -> Self {
		ConfigManager {
			config: ClusterVmsConfig::default(),
		}
	}

	pub fn read_default_config_files(&mut self) -> figment::error::Result<()> {
		self.read_config(vec!(DEFAULT_CONFIG_FILE))
	}

	pub fn read_config(&mut self, filenames: Vec<&str>) -> figment::error::Result<()> {
		let mut figment = Figment::new();
		for filename in filenames {
			figment = figment.merge(Toml::file(filename));
		}
		self.config = figment.extract()?;
		
		Ok(())
	}

	pub fn write_config(&self) -> anyhow::Result<()> {
		let toml_string = toml::to_string(&self.config)?;

		fs::write(DEFAULT_CONFIG_FILE, toml_string)?;
		Ok(())
	}
	pub fn get_config(&self) -> &ClusterVmsConfig {
		&self.config
	}

	pub fn get_config_mut(&mut self) -> &mut ClusterVmsConfig {
		&mut self.config
	}
}
