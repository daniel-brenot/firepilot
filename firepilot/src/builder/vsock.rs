use std::path::PathBuf;

use crate::builder::{assert_not_none, Builder, BuilderError};
use firepilot_models::models::Vsock;

#[derive(Debug)]
pub struct VsockBuilder {
    pub guest_cid: Option<i32>,
    pub path_on_host: Option<PathBuf>,
}

impl VsockBuilder {
    pub fn new() -> VsockBuilder {
        VsockBuilder {
            guest_cid: None,
            path_on_host: None
        }
    }

    pub fn with_guest_cid(mut self, guest_cid: i32) -> VsockBuilder {
        self.guest_cid = Some(guest_cid);
        self
    }

    pub fn with_path_on_host(mut self, path_on_host: PathBuf) -> VsockBuilder {
        self.path_on_host = Some(path_on_host);
        self
    }
}

impl Builder<Vsock> for VsockBuilder {
    fn try_build(self) -> Result<Vsock, BuilderError> {
        assert_not_none(stringify!(self.drive_id), &self.guest_cid)?;
        assert_not_none(stringify!(self.path_on_host), &self.path_on_host)?;
        Ok(Vsock {
            guest_cid: self.guest_cid.unwrap(),
            uds_path: self.path_on_host
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
            vsock_id: None
        })
    }
}