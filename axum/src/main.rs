use axum::{
    body::Body,
    extract::{Path, Query},
    http::header,
    response::IntoResponse,
    routing::{get, post},
    serve, Json, Router,
};
use axum_template::{engine::Engine, RenderHtml};
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;

#[derive(Serialize, Deserialize)]
struct Ping {
    name: String,
}

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

type AppEngine = Engine<Environment<'static>>;

#[derive(Serialize)]
pub struct PageContext {
    name: String,
}

async fn root() -> &'static str {
    "hi"
}

async fn mirror(Json(payload): Json<Ping>) -> Json<Ping> {
    Json(payload)
}

async fn params(Path(id): Path<u32>, Query(query): Query<QueryParams>) -> impl IntoResponse {
    (
        [("x-powered-by", "benchmark")],
        format!("{} {}", id, query.name),
    )
}

async fn ely() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        Body::from_stream(ReaderStream::new(
            tokio::fs::File::open("public/ely.png").await.unwrap(),
        )),
    )
}

async fn page(engine: AppEngine, Query(query): Query<QueryParams>) -> impl IntoResponse {
    let context = PageContext { name: query.name };

    RenderHtml("page.html", engine, context)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut jinja = Environment::new();
    jinja
        .add_template("page.html", include_str!("../templates/page.html"))
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/id/:id", get(params))
        .route("/json", post(mirror))
        .route("/ely.png", get(ely))
        .route("/page.html", get(page))
        .with_state(Engine::from(jinja));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
