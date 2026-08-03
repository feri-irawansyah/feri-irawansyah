use actix_web::{HttpResponse, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/cv.pdf", web::get().to(serve_pdf))
        .route("/cv/preview/{page}", web::get().to(serve_svg));
}

async fn serve_pdf(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    match typests::cv::cv_pdf() {
        Ok(bytes) => {
            let disposition = if query.get("dl").is_some() {
                "attachment; filename=\"feri-irawansyah-cv.pdf\""
            } else {
                "inline; filename=\"feri-irawansyah-cv.pdf\""
            };
            HttpResponse::Ok()
                .content_type("application/pdf")
                .append_header(("Content-Disposition", disposition))
                .append_header(("Cache-Control", "public, max-age=3600"))
                .body(bytes.to_vec())
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("CV render error: {e}")),
    }
}

async fn serve_svg(path: web::Path<usize>) -> HttpResponse {
    let page = path.into_inner();
    match typests::cv::cv_svg(page) {
        Ok(svg) => HttpResponse::Ok()
            .content_type("image/svg+xml")
            .append_header(("Cache-Control", "public, max-age=3600"))
            .body(svg.to_owned()),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("CV render error: {e}")),
    }
}
