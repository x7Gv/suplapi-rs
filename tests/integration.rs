extern crate suplapi;

#[cfg(feature = "http-client")]
mod tests {
    use suplapi::SuplAPI;
    use suplapi::http;


    fn s() -> SuplAPI<http::default::Client> {
        SuplAPI::default()
    }

    #[tokio::test]
    async fn playlist() {
        let suplapi = s();
        let res = suplapi.playlist(70, 20, None).await.unwrap();
        assert!(res.items.len() == 20);
    }
}
