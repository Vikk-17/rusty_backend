mod error {
    use actix_web:: {HttpResponse, ResponseError};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db(String),
    }

    impl ResponseError for Error {
        fn error_response(&self) -> HttpResponse {
            match self {
                Error:Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}
