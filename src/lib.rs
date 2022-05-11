#![deny(warnings)]
#![forbid(unsafe_code)]

use std::fmt::{self, Display, Formatter};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(feature = "with-actix")]
mod actix;
#[cfg(feature = "with-serde")]
mod serde;
#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BindableAddr {
	Unix(PathBuf),
	Tcp(SocketAddr),
}

#[derive(Debug, thiserror::Error)]
pub enum BindableAddrFromStrError {
	#[error("unknown protocol {0:?}")]
	UnknownProtocol(String),
	#[error("could not parse socket address: {0}")]
	SocketAddr(<SocketAddr as FromStr>::Err),
}

impl FromStr for BindableAddr {
	type Err = BindableAddrFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (protocol, inner) = s.split_once("://").unwrap_or(("tcp", s));
		match protocol {
			"unix" => Ok(Self::Unix(PathBuf::from(inner))),
			"tcp" => SocketAddr::from_str(inner)
				.map_err(Self::Err::SocketAddr)
				.map(Self::Tcp),
			unknown => Err(Self::Err::UnknownProtocol(unknown.to_owned())),
		}
	}
}

impl Display for BindableAddr {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Tcp(inner) => write!(f, "tcp://{}", inner),
			Self::Unix(inner) => write!(f, "unix://{}", inner.display()),
		}
	}
}

pub trait BindBindableExt {
	type Return;

	fn bind_bindable(self, addr: &BindableAddr) -> Self::Return;
}
