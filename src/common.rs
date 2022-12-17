use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;



pub type CameraId = String;
pub type StreamId = String;
pub type CameraList = Vec<Camera>;
pub type CameraMap = HashMap<CameraId, Camera>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Camera {
	pub id: CameraId,
	pub name: String,
	pub username: Option<String>,
	pub password: Option<String>,
	pub streams: HashMap<StreamId, Stream>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Stream {
	pub source_url: Url,
	pub recast_url: Option<Url>,
	pub width: u32,
	pub height: u32,
}

/// Slimmed-down version of Camera with only basic info
/// Used in the REST API when listing many cameras
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct BasicCameraInfo {
	pub id: CameraId,
	pub name: String,
}

/// Allows a BasicCameraInfo to be created from a Camera
impl From<Camera> for BasicCameraInfo {
	fn from(cam: Camera) -> Self {
		BasicCameraInfo { id: cam.id, name: cam.name }
	}
}
