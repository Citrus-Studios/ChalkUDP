// TODO: Documentation
use std::{
	error::Error,
	fmt::Display,
	sync::{Arc, RwLock},
};
use tokio::net::{TcpListener, TcpStream};

mod connecting;
mod messaging;

// SECTION: TcpClientError

// TODO: Add Documentation
#[derive(Debug)]
pub enum TcpClientError {
	FailedToWriteStreams,
	FailedToWriteListeners,
	UnboundStream,
	UnboundListener,
}

impl Display for TcpClientError {
	// TODO: Add Error Messages
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Error for TcpClientError {}

// !SECTION
// SECTION: TcpClient

// TODO: Documentation
pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}
impl TcpClient {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			inner: Arc::new(InnerTcpClient::new()),
		}
	}
}

// !SECTION
// SECTION: InnerTcpClient

// TODO: Documentation
pub(crate) struct InnerTcpClient {
	streams: RwLock<Option<TcpStream>>,
	listeners: RwLock<Option<TcpListener>>,
}

impl InnerTcpClient {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			streams: RwLock::new(None),
			listeners: RwLock::new(None),
		}
	}
}

// !SECTION
