#[cfg(test)]
mod tests {
    #[test]
    fn uuidtest() {
      use uuid::Uuid;
      assert_eq!(Uuid::new_v4().to_string().is_empty(), false);
    }

    #[test]
    fn datetest() {
      use chrono::prelude::*;
      assert_eq!(Utc::now().to_string().is_empty(), false);
      let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28).unwrap().and_hms_nano_opt(12, 0, 9, 1).unwrap().and_local_timezone(Utc).unwrap();
      assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
    }

    #[test]
    fn envset() {
       use std::env;
       use uuid::Uuid;
       let txid = Uuid::new_v4().to_string();
       env::set_var("txid", &txid);
    }

    #[actix_web::test]
    async fn test_index() {
        async fn handler(req: HttpRequest) -> HttpResponse {
            if let Some(_hdr) = req.headers().get(header::CONTENT_TYPE) {
                HttpResponse::Ok().into()
            } else {
                HttpResponse::BadRequest().into()
            }
        }

        use actix_web::{test, HttpRequest, HttpResponse};
        use actix_web::http::{header, StatusCode};

        let req = test::TestRequest::default()
            .insert_header(header::ContentType::plaintext())
            .to_http_request();

        let resp = handler(req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = test::TestRequest::default().to_http_request();
        let resp = handler(req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
