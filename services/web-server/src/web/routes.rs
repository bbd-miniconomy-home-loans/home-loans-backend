use aide::axum::ApiRouter;
use aide::openapi::{Info, OpenApi};
use axum::{Extension, middleware, Router};
use axum_typed_routing::TypedApiRouter;

use crate::AppState;
use crate::web::mw_auth::mw_auth;
use crate::web::mw_request_stamp::mw_request_stamp_resolver;
use crate::web::mw_response_mapper::mw_response_mapper;
use crate::web::routes_docs::{api_docs, docs_routes};
use crate::web::routes_home_loan::home_loan_request_handler;

pub fn init_router(state: AppState) -> Router {
	let mut api = OpenApi {
		info: Info {
			description: Some("Home loans api spec".to_string()),
			..Info::default()
		},
		..OpenApi::default()
	};


	ApiRouter::new()
		.nest("/api", api_routes())
		.layer(middleware::map_response(mw_response_mapper))
		.layer(middleware::from_fn(mw_auth))
		.layer(middleware::from_fn(mw_request_stamp_resolver))
		.nest_api_service("/docs", docs_routes())
		.finish_api_with(&mut api, api_docs)
		.layer(Extension(api))
		.with_state(state)
}

fn api_routes() -> ApiRouter<AppState> {
	ApiRouter::new()
		.typed_api_route(home_loan_request_handler)
}