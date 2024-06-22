use aide::axum::{ApiRouter, IntoApiResponse};
use aide::axum::routing::{get, get_with};
use aide::openapi::OpenApi;
use aide::scalar::Scalar;
use aide::transform::TransformOpenApi;
use axum::{Extension, Json};
use axum::response::IntoResponse;

pub(crate) fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
	api.title("Home loans api spec")
		.summary("The open api spec for the home loans sections of the miniconomy" )
		.security_scheme(
			"OAuth2",
			aide::openapi::SecurityScheme::OAuth2 {
				flows: Default::default(),
				// location: aide::openapi::ApiKeyLocation::Header,
				// name: "X-Auth-Key".into(),
				description: Some("A key that is ignored.".into()),
				extensions: Default::default(),
			},
		)
}


pub fn docs_routes() -> ApiRouter {
	aide::gen::infer_responses(true);
	let router: ApiRouter = ApiRouter::new()
		.api_route_with(
			"/",
			get_with(
				Scalar::new("/docs/private/api.json")
					.with_title("Aide Axum")
					.axum_handler(),
				|op| op.description("This documentation page."),
			),
			|p| p.security_requirement("OAuth2"),
		)
		.route("/private/api.json", get(serve_docs))
		;

	// Afterward, we disable response inference because
	// it might be incorrect for other routes.
	aide::gen::infer_responses(false);

	router
}

async fn serve_docs(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
	Json(api).into_response()
}
