use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tokio::time::Instant;
use tracing::level_filters::LevelFilter;
use tracing::{debug, info, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_attributes::instrument;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};

#[derive(Deserialize, Debug)]
struct Params {
    name: String,
    age: Option<u8>, // 可选参数
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志订阅者
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::TRACE);

    // 获取当前项目根目录
    let mut log_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    log_dir.push("logs"); // 追加 logs 子目录

    // 如果目录不存在则创建
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)?;
    }

    fn daily_with_custom_format(log_dir: impl Into<PathBuf>) -> RollingFileAppender {
        return RollingFileAppender::builder()
            .rotation(Rotation::DAILY) // rotate log files once every hour
            .filename_prefix("myapp") // log file names will be prefixed with `myapp.`
            .filename_suffix("log") // log file names will be suffixed with `.log`
            .build(log_dir.into()) // try to build an appender that stores log files in `/var/log`
            .expect("initializing rolling file appender failed");
    }

    // let file_appender = tracing_appender::rolling::daily(log_dir, "app.log");
    let file_appender = daily_with_custom_format(log_dir);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file = fmt::Layer::new()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(query_handler));

    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn query_handler(Query(params): Query<Params>) -> String {
    let start = Instant::now();
    let msg = format!(
        "Name: {}, Age: {}",
        params.name,
        params.age.unwrap_or_default()
    );
    debug!("{}", msg);
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    let elapsed_s = start.elapsed().as_secs_f32();
    warn!(
        app.task = elapsed_s,
        "query_handler is took {:.2} seconds", elapsed_s
    );
    return msg;
}
