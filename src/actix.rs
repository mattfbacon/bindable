use super::{BindBindableExt, BindableAddr};

impl BindBindableExt for actix_web::HttpServer<_, _, _, _> {
	type Return = std::io::Result<Self>;

	fn bind_bindable(self, addr: &BindableAddr) -> Self::Return {
		match addr {
			BindableAddr::Tcp(addr) => self.bind(addr),
			BindableAddr::Unix(path) => self.bind_uds(path),
		}
	}
}
