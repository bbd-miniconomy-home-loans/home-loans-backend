use aide::axum::ApiRouter;
use aide::openapi::{Info, OpenApi};
use axum::{Extension, middleware, Router};
use axum::extract::{MatchedPath, State};
use axum::http::Request;
use axum_typed_routing::{route, TypedApiRouter, TypedRouter};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::AppState;
use crate::web::mw_auth::mw_auth;
use crate::web::mw_request_stamp::mw_request_stamp_resolver;
use crate::web::mw_response_mapper::mw_response_mapper;
use crate::web::routes_docs::{api_docs, docs_routes};
use crate::web::routes_home_loan::{apply_request_handler, get_loan_status_request_handler};

pub fn init_router(state: AppState) -> Router {
	let mut api = OpenApi {
		info: Info {
			description: Some("Home loans api spec".to_string()),
			..Info::default()
		},
		..OpenApi::default()
	};


	ApiRouter::new()
		.nest("/admin", internal_routes())
		.nest("/api", api_routes())
		.layer(middleware::map_response(mw_response_mapper))
		.layer(middleware::from_fn(mw_request_stamp_resolver))
		.nest_api_service("/docs", docs_routes())
		.finish_api_with(&mut api, api_docs)
		.layer(Extension(api))
		.typed_route(health)
		.layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
			let matched_path = request
				.extensions()
				.get::<MatchedPath>()
				.map(MatchedPath::as_str);
			info_span!("http_request",method = ?request.method(),matched_path,)
		})
		)
		.with_state(state)
}

fn internal_routes() -> ApiRouter<AppState> {
	ApiRouter::new()
		.typed_api_route_with(apply_request_handler, |p| p.security_requirement("oauth"))
		.typed_api_route_with(get_loan_status_request_handler, |p| p.security_requirement("oauth"))
	// Middleware with auth
	// 	.layer(middleware::from_fn(custom_mw_auth))
}

fn api_routes() -> ApiRouter<AppState> {
	ApiRouter::new()
		.typed_api_route_with(apply_request_handler, |p| p.security_requirement("keys"))
		.typed_api_route_with(get_loan_status_request_handler, |p| p.security_requirement("keys"))
		.layer(middleware::from_fn(mw_auth))
}

#[route(GET "/")]
pub async fn health(
	State(_state): State<AppState>,
) -> String {
	"<OK>".to_string()
}