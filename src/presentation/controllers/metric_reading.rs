use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};

use crate::{
    domain::use_cases::metric_reading::MetricReadingUseCase,
    presentation::{
        dtos::metric_reading::CreateMetricReadingRequestDTO, errors::PresentationError,
    },
};

pub type DynMetricReadingUseCase = Arc<dyn MetricReadingUseCase + Send + Sync>;

pub async fn create_metric_reading(
    service: web::Data<DynMetricReadingUseCase>,
    payload: web::Json<CreateMetricReadingRequestDTO>,
) -> Result<impl Responder, PresentationError> {
    payload.validate()?;
    let metric_reading = service.create_metric_reading(payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(metric_reading))
}
