pub mod awards;
pub mod conferences;
pub(crate) mod http;

use awards::AwardsClient;
use conferences::ConferencesClient;

pub struct Client {
    pub awards: AwardsClient,
    pub confrences: ConferencesClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            awards: AwardsClient::new(),
            confrences: ConferencesClient::new(),
        }
    }
}
