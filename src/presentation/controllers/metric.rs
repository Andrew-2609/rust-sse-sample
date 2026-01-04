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
) -> Result<impl Responder, PresentationError> {
    payload.validate()?;
    let metric = service.create_metric(payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(metric))
}

pub async fn get_metric_by_id(
    service: web::Data<DynMetricUseCase>,
    path: Path<String>,
) -> Result<impl Responder, PresentationError> {
    let id = match uuid::Uuid::try_parse(path.into_inner().as_str()) {
        Err(_) => return Err(PresentationError::BadRequest("id is invalid".to_string())),
        Ok(value) => value,
    };

    let result = service.get_metric_by_id(id).await?;

    let Some(metric) = result else {
        let msg = format!("metric {} not found", id);
        return Err(PresentationError::NotFound(msg));
    };

    Ok(HttpResponse::Ok().json(metric))
}

pub async fn get_all_metrics(
    service: web::Data<DynMetricUseCase>,
) -> Result<impl Responder, PresentationError> {
    let result = service.get_all_metrics().await?;
    Ok(HttpResponse::Ok().json(result))
}
