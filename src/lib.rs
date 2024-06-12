use reqwest::Proxy;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(TypedBuilder)]
pub struct BrightdataProxyBuilder {
    username: String,
    password: String,
    #[builder(default, setter(strip_option))]
    zone: Option<String>,
}

impl BrightdataProxyBuilder {
    pub fn create_session(&self, session: &str) -> Result<Proxy, reqwest::Error> {
        let mut username = self.username.to_string();

        if let Some(zone) = &self.zone {
            username.push_str("-");
            username.push_str(zone);
        }

        username.push_str("-session-");
        username.push_str(session);

        Ok(Proxy::all("http://brd.superproxy.io:22225")?.basic_auth(&username, &self.password))
    }

    pub fn create_new_session(&self) -> Result<Proxy, reqwest::Error> {
        let session_id = Uuid::new_v4().simple().to_string();
        self.create_session(&session_id)
    }
}
