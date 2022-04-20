use super::{BindBindableExt, BindableAddr};

use actix_http::{Request, Response};
use actix_service::{IntoServiceFactory, Service, ServiceFactory};
use actix_web::body::MessageBody;
use actix_web::dev::AppConfig;
use actix_web::{Error, HttpServer};

impl<F, I, S, B> BindBindableExt for HttpServer<F, I, S, B>
where
	F: Fn() -> I + Send + Clone + 'static,
	I: IntoServiceFactory<S, Request>,

	S: ServiceFactory<Request, Config = AppConfig> + 'static,
	S::Error: Into<Error> + 'static,
	S::InitError: std::fmt::Debug,
	S::Response: Into<Response<B>> + 'static,
	<S::Service as Service<Request>>::Future: 'static,
	S::Service: 'static,

	B: MessageBody + 'static,
{
	type Return = std::io::Result<Self>;

	fn bind_bindable(self, addr: &BindableAddr) -> Self::Return {
		match addr {
			BindableAddr::Tcp(addr) => self.bind(addr),
			BindableAddr::Unix(path) => self.bind_uds(path),
		}
	}
}
