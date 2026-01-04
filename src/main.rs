mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::{
    application::use_cases::metric::{DynMetricRepository, MetricUseCaseImpl},
    infrastructure::repositories::in_memory_metric::InMemoryMetricRepository,
    presentation::controllers::metric::{DynMetricUseCase, create_metric, get_metric_by_id},
};

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let metric_repository: DynMetricRepository = Arc::new(InMemoryMetricRepository::default());
    let metric_use_case: DynMetricUseCase = Arc::new(MetricUseCaseImpl::new(metric_repository));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(metric_use_case.clone()))
            .configure(configure)
    })
    .bind(("127.0.0.1", 8089))?
    .run()
    .await
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/metrics")
            .route("", web::post().to(create_metric))
            .route("{id}", web::get().to(get_metric_by_id)),
    );
}
