use sqlx::postgres::PgPool;
use warp::{http::Response, Rejection};

use serde_json::json;

use crate::auth::Claims;
use crate::extension::{repository, Extensions};
use tanoshi_lib::extensions::Extension;
use tanoshi_lib::manga::{GetParams, Params, Source, SourceLogin};

use crate::handlers::TransactionReject;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn list_sources(
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let exts = exts.read().await;
    let sources = exts
        .extensions()
        .iter()
        .map(|(key, ext)| {
            info!("source name {}", key.clone());
            ext.info()
        })
        .collect::<Vec<Source>>();
    info!("sources {:?}", sources.clone());

    match repository::insert_sources(sources, db.clone()).await {
        Ok(_) => {}
        Err(e) => return Err(warp::reject()),
    }

    match repository::get_sources(db).await {
        Ok(sources) => Ok(warp::reply::json(&json!(
            {
                "sources": sources,
                "status": "success"
            }
        ))),
        Err(e) => Err(warp::reject::custom(TransactionReject {
            message: e.to_string(),
        })),
    }
}

pub async fn list_mangas(
    source_id: i32,
    claim: Claims,
    source_auth: String,
    param: Params,
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let exts = exts.read().await;
    if let Ok(source) = repository::get_source(source_id, db.clone()).await {
        let mangas = exts
            .get(&source.name)
            .unwrap()
            .get_mangas(&source.url, param, source_auth)
            .unwrap();

        let manga_ids = match repository::insert_mangas(source_id, mangas.clone(), db.clone()).await
        {
            Ok(ids) => ids,
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        };
        match repository::get_mangas(claim.sub, manga_ids, db).await {
            Ok(mangas) => return Ok(warp::reply::json(&mangas)),
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        }
    }
    Err(warp::reject())
}

pub async fn get_manga_info(
    manga_id: i32,
    claim: Claims,
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let exts = exts.read().await;
    if let Ok(manga) = repository::get_manga_detail(manga_id, claim.sub.clone(), db.clone()).await {
        return Ok(warp::reply::json(&manga));
    } else if let Ok(url) = repository::get_manga_url(manga_id, db.clone()).await {
        let source = match repository::get_source_from_manga_id(manga_id, db.clone()).await {
            Ok(source) => source,
            Err(e) => return Err(warp::reject()),
        };

        let manga = exts
            .get(&source.name)
            .unwrap()
            .get_manga_info(&url)
            .unwrap();

        match repository::update_manga_info(manga_id, manga, db.clone()).await {
            Ok(_) => {}
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        }
        match repository::get_manga_detail(manga_id, claim.sub, db).await {
            Ok(res) => return Ok(warp::reply::json(&res)),
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        }
    }
    Err(warp::reject())
}

pub async fn get_chapters(
    manga_id: i32,
    claim: Claims,
    param: GetParams,
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let exts = exts.read().await;
    if !param.refresh.unwrap_or(false) {
        match repository::get_chapters(manga_id, claim.sub.clone(), db.clone()).await {
            Ok(chapter) => return Ok(warp::reply::json(&chapter)),
            Err(_e) => {}
        };
    }

    if let Ok(url) = repository::get_manga_url(manga_id, db.clone()).await {
        let source = match repository::get_source_from_manga_id(manga_id, db.clone()).await {
            Ok(source) => source,
            Err(e) => return Err(warp::reject()),
        };

        let chapter = exts.get(&source.name).unwrap().get_chapters(&url).unwrap();

        match repository::insert_chapters(manga_id, chapter.clone(), db.clone()).await {
            Ok(_) => {}
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        }

        match repository::get_chapters(manga_id, claim.sub, db.clone()).await {
            Ok(chapter) => return Ok(warp::reply::json(&chapter)),
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        };
    }
    Err(warp::reject())
}

pub async fn get_pages(
    chapter_id: i32,
    _param: GetParams,
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let exts = exts.read().await;
    match repository::get_pages(chapter_id, db.clone()).await {
        Ok(pages) => return Ok(warp::reply::json(&pages)),
        Err(_) => {}
    };

    if let Ok(url) = repository::get_chapter_url(chapter_id, db.clone()).await {
        let source = match repository::get_source_from_chapter_id(chapter_id, db.clone()).await {
            Ok(source) => source,
            Err(e) => return Err(warp::reject()),
        };

        let pages = exts.get(&source.name).unwrap().get_pages(&url).unwrap();

        match repository::insert_pages(chapter_id, pages.clone(), db.clone()).await {
            Ok(_) => {}
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        }

        match repository::get_pages(chapter_id, db.clone()).await {
            Ok(pages) => return Ok(warp::reply::json(&pages)),
            Err(e) => {
                return Err(warp::reject::custom(TransactionReject {
                    message: e.to_string(),
                }))
            }
        };
    }
    Err(warp::reject())
}

pub async fn proxy_image(
    page_id: i32,
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let image = match repository::get_image_from_page_id(page_id, db.clone()).await {
        Ok(image) => image,
        Err(_) => return Err(warp::reject()),
    };

    let exts = exts.read().await;
    let bytes = exts
        .get(&image.source_name)
        .unwrap()
        .get_page(image.clone())
        .unwrap();

    let path = std::path::PathBuf::from(image.path).join(image.file_name);
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    let resp = Response::builder()
        .header("Content-Type", mime.as_ref())
        .header("Content-Length", bytes.len())
        .body(bytes)
        .unwrap();

    Ok(resp)
}

pub async fn source_login(
    source_id: i32,
    login_info: SourceLogin,
    exts: Arc<RwLock<Extensions>>,
    db: PgPool,
) -> Result<impl warp::Reply, Rejection> {
    let exts = exts.read().await;
    if let Ok(source) = repository::get_source(source_id, db.clone()).await {
        if let Ok(result) = exts.get(&source.name).unwrap().login(login_info) {
            let mut result = result;
            result.source_id = source_id;
            return Ok(warp::reply::json(&result));
        }
    }
    Err(warp::reject())
}
