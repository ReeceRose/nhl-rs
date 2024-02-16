use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub(crate) async fn get<T>(url: String) -> Result<T, u16>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status().as_u16());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap().as_u16());
            }
            return Err(StatusCode::BAD_REQUEST.as_u16());
        }
    }

    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST.as_u16())
        }
    }
}
