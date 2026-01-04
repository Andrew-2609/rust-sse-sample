mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::{
    application::use_cases::{
        metric::{DynMetricRepository, MetricUseCaseImpl},
        metric_reading::{DynMetricReadingRepository, MetricReadingUseCaseImpl},
    },
    infrastructure::repositories::{
        in_memory_metric::InMemoryMetricRepository,
        in_memory_metric_reading::InMemoryMetricReadingRepository,
    },
    presentation::controllers::{
        metric::{DynMetricUseCase, create_metric, get_all_metrics, get_metric_by_id},
        metric_reading::{DynMetricReadingUseCase, create_metric_reading},
    },
};

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let metric_repository: DynMetricRepository = Arc::new(InMemoryMetricRepository::default());
    let metric_use_case: DynMetricUseCase =
        Arc::new(MetricUseCaseImpl::new(metric_repository.clone()));

    let metric_reading_repository: DynMetricReadingRepository =
        Arc::new(InMemoryMetricReadingRepository::default());
    let metric_reading_use_case: DynMetricReadingUseCase = Arc::new(MetricReadingUseCaseImpl::new(
        metric_repository,
        metric_reading_repository,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(metric_use_case.clone()))
            .app_data(web::Data::new(metric_reading_use_case.clone()))
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
            .route("", web::get().to(get_all_metrics))
            .route("{id}", web::get().to(get_metric_by_id))
            // Metric Readings
            .service(web::scope("/readings").route("", web::post().to(create_metric_reading))),
    );
}
