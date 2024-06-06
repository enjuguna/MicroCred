use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::db::{MongoDB, DID};
use std::sync::Arc;
use paperclip::actix::api_v2_operation;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dids")
            .route("", web::post().to(create_did))
            .route("", web::get().to(get_dids))
            .route("/{id}", web::get().to(get_did))
            .route("/{id}", web::put().to(update_did))
            .route("/{id}", web::delete().to(delete_did))
    );
}

#[api_v2_operation]
async fn create_did(db: web::Data<Arc<MongoDB>>, new_did: web::Json<DID>) -> HttpResponse {
    let did = DID {
        id: new_did.id.clone(),
        credential: new_did.credential.clone(),
    };
    match db.create_did(did).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[api_v2_operation]
async fn get_dids(db: web::Data<Arc<MongoDB>>) -> HttpResponse {
    // Implement this function to return all DIDs
    HttpResponse::Ok().finish()
}

#[api_v2_operation]
async fn get_did(db: web::Data<Arc<MongoDB>>, web::Path(id): web::Path<String>) -> HttpResponse {
    match db.get_did(&id).await {
        Ok(Some(did)) => HttpResponse::Ok().json(did),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[api_v2_operation]
async fn update_did(db: web::Data<Arc<MongoDB>>, web::Path(id): web::Path<String>, new_credential: web::Json<String>) -> HttpResponse {
    match db.update_did(&id, &new_credential).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[api_v2_operation]
async fn delete_did(db: web::Data<Arc<MongoDB>>, web::Path(id): web::Path<String>) -> HttpResponse {
    match db.delete_did(&id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
