#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(
	missing_copy_implementations,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	variant_size_differences
)]
#![forbid(unsafe_code)]

use std::fmt::{self, Display, Formatter};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

#[cfg(feature = "with-actix")]
mod actix;
#[cfg(feature = "with-serde")]
mod serde;

/// The whole point.
///
/// Has variants for TCP and Unix socket addresses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BindableAddr {
	/// The address of a Unix socket (a path).
	Unix(PathBuf),
	/// The address of a TCP socket (an IP).
	Tcp(SocketAddr),
}

/// Reasons why parsing a [`BindableAddr`] from a string can fail.
#[derive(Debug, thiserror::Error)]
#[allow(variant_size_differences)]
pub enum FromStrError {
	/// The protocol (the portion before the `://`) is unrecognized.
	///
	/// `tcp` and `unix` are the valid protocols.
	#[error("unknown protocol {0:?}")]
	UnknownProtocol(String),
	/// For a TCP address, the IP was invalid.
	#[error("could not parse socket address: {0}")]
	SocketAddr(<SocketAddr as FromStr>::Err),
}

impl FromStr for BindableAddr {
	type Err = FromStrError;

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

/// Allows binding an HTTP server to a [`BindableAddr`].
///
/// Implementations of this trait are gated behind features based on the HTTP library you want to use.
pub trait BindBindableExt {
	/// The result of binding the server to the address.
	type Return;

	/// Bind the HTTP server to the provided address.
	fn bind_bindable(self, addr: &BindableAddr) -> Self::Return;
}
