pub fn get_with_retries<const RETRIES: u8>(
    url: &String,
) -> Result<ureq::http::Response<ureq::Body>, ()> {
    for _ in 0..RETRIES {
        if let Ok(resp) = ureq::get(url).call() {
            return Ok(resp);
        }
    }

    Err(())
}
