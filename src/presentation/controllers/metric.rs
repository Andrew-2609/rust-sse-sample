use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder,
    web::{self, Path},
};

use crate::{
    domain::use_cases::metric::MetricUseCase,
    presentation::{dtos::metric::CreateMetricRequestDTO, errors::PresentationError},
};

pub type DynMetricUseCase = Arc<dyn MetricUseCase + Send + Sync>;

pub async fn create_metric(
    service: web::Data<DynMetricUseCase>,
    payload: web::Json<CreateMetricRequestDTO>,
) -> impl Responder {
    if let Err(err) = payload.validate() {
        return HttpResponse::BadRequest().json(err);
    }

    match service.create_metric(payload.into_inner()).await {
        Err(err) => HttpResponse::InternalServerError().json(PresentationError::from(err)),
        Ok(metric) => HttpResponse::Ok().json(metric),
    }
}

pub async fn get_metric_by_id(
    service: web::Data<DynMetricUseCase>,
    path: Path<String>,
) -> impl Responder {
    let id = match uuid::Uuid::try_parse(path.into_inner().as_str()) {
        Err(_) => return HttpResponse::BadRequest().json("id is invalid"),
        Ok(value) => value,
    };

    match service.get_metric_by_id(id).await {
        Err(err) => HttpResponse::InternalServerError().json(PresentationError::from(err)),
        Ok(result) => {
            let Some(metric) = result else {
                let msg = format!("metric {} not found", id);
                return HttpResponse::NotFound().json(PresentationError::NotFound(msg));
            };

            HttpResponse::Ok().json(metric)
        }
    }
}

pub async fn get_all_metrics(service: web::Data<DynMetricUseCase>) -> impl Responder {
    match service.get_all_metrics().await {
        Err(err) => HttpResponse::InternalServerError().json(PresentationError::from(err)),
        Ok(result) => HttpResponse::Ok().json(result),
    }
}
