use crate::IfpaClient;
use crate::IfpaError;
use crate::models::common::TimePeriod;
use crate::models::director::*;

impl IfpaClient {
    pub async fn get_director(&self, id: i64) -> Result<Director, IfpaError> {
        let req = self.request(&format!("/director/{id}"));
        self.send(req).await
    }

    pub async fn get_director_tournaments(
        &self,
        id: i64,
        time_period: TimePeriod,
    ) -> Result<DirectorTournamentsResponse, IfpaError> {
        let req = self.request(&format!("/director/{id}/tournaments/{time_period}"));
        self.send(req).await
    }

    pub async fn get_country_directors(&self) -> Result<CountryDirectorsResponse, IfpaError> {
        let req = self.request("/director/country");
        self.send(req).await
    }

    pub async fn search_directors(
        &self,
        params: &DirectorSearchParams,
    ) -> Result<DirectorSearchResponse, IfpaError> {
        let req = self.request("/director/search").query(params);
        self.send(req).await
    }
}
