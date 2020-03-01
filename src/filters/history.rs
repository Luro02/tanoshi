pub mod history {
    use crate::handlers::auth::auth as auth_handler;
    use crate::handlers::history::history as history_handler;
    use crate::history::history::History;
    use crate::history::HistoryChapter;
    use sled::Db;
    use warp::Filter;

    pub(crate) fn history(
        history: History,
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get_history(history.clone(), db.clone()).or(add_history(history, db))
    }

    fn get_history(
        history: History,
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "history" / "source" / String / "manga" / String)
            .and(warp::get())
            .and(auth_handler::validate())
            .and(with_history(history))
            .and(with_db(db))
            .and_then(history_handler::get_history)
    }

    fn add_history(
        history: History,
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "history" / "source" / String / "manga" / String)
            .and(warp::post())
            .and(auth_handler::validate())
            .and(json_body())
            .and(with_history(history))
            .and(with_db(db))
            .and_then(history_handler::add_history)
    }

    fn with_history(
        history: History,
    ) -> impl Filter<Extract = (History,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || history.clone())
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn json_body() -> impl Filter<Extract = (HistoryChapter,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
