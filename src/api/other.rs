use crate::IfpaClient;
use crate::IfpaError;
use crate::models::other::*;

impl IfpaClient {
    pub async fn get_countries(&self) -> Result<CountriesResponse, IfpaError> {
        let req = self.request("/other/countries");
        self.send(req).await
    }

    pub async fn get_state_provs(&self) -> Result<StateProvsResponse, IfpaError> {
        let req = self.request("/other/stateprovs");
        self.send(req).await
    }
}
