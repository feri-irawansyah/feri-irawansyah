use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use connectors::supabase::StorageStore;
use futures_util::StreamExt as _;
use modules::auth::Claims;
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/admin/uploads", web::post().to(upload_image));
}

// Keep in sync with `client_max_body_size` in scripts/templates/nginx.conf —
// Nginx would reject anything larger before it ever reaches this check.
const MAX_UPLOAD_BYTES: usize = 2 * 1024 * 1024;

async fn upload_image(
    req: HttpRequest,
    mut payload: Multipart,
    storage: web::Data<Arc<dyn StorageStore>>,
) -> HttpResponse {
    let is_admin = req
        .extensions()
        .get::<Claims>()
        .is_some_and(|c| c.client_category == 1);
    if !is_admin {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
    }

    let mut folder = "misc".to_string();
    let mut filename = String::new();
    let mut content_type = "application/octet-stream".to_string();
    let mut bytes: Vec<u8> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({ "error": e.to_string() }));
            }
        };
        let field_name = field.name().unwrap_or_default().to_string();

        if field_name == "folder" {
            match field.bytes(1024).await {
                Ok(Ok(buf)) => {
                    let value = String::from_utf8_lossy(&buf).trim().to_string();
                    if !value.is_empty() {
                        folder = value;
                    }
                }
                _ => continue,
            }
            continue;
        }

        if field_name == "file" {
            if let Some(ct) = field.content_type() {
                content_type = ct.essence_str().to_string();
            }
            if let Some(fname) = field
                .content_disposition()
                .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
            {
                filename = fname;
            }
            match field.bytes(MAX_UPLOAD_BYTES).await {
                Ok(Ok(buf)) => bytes = buf.to_vec(),
                Ok(Err(e)) => {
                    return HttpResponse::BadRequest()
                        .json(serde_json::json!({ "error": e.to_string() }));
                }
                Err(_) => {
                    return HttpResponse::PayloadTooLarge()
                        .json(serde_json::json!({ "error": "File terlalu besar (maks 2MB)" }));
                }
            }
        }
    }

    if bytes.is_empty() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({ "error": "File kosong atau tidak ditemukan" }));
    }
    if !content_type.starts_with("image/") {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({ "error": "File harus berupa gambar" }));
    }

    let ext = if filename.contains('.') {
        filename
            .rsplit('.')
            .next()
            .filter(|e| e.len() <= 5)
            .unwrap_or("bin")
    } else {
        "bin"
    };
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let path = format!("assets/img/{folder}/{ts}.{ext}");

    match storage.upload(&path, bytes, &content_type).await {
        Ok(url) => HttpResponse::Ok().json(serde_json::json!({ "url": url })),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))
        }
    }
}
