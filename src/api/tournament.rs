use crate::IfpaClient;
use crate::IfpaError;
use crate::models::common::LeagueTimePeriod;
use crate::models::tournament::*;

impl IfpaClient {
    pub async fn get_tournament_formats(&self) -> Result<TournamentFormatsResponse, IfpaError> {
        let req = self.request("/tournament/formats");
        self.send(req).await
    }

    pub async fn get_tournament(&self, id: i64) -> Result<Tournament, IfpaError> {
        let req = self.request(&format!("/tournament/{id}"));
        self.send(req).await
    }

    pub async fn get_tournament_related(
        &self,
        id: i64,
    ) -> Result<Option<RelatedTournamentsResponse>, IfpaError> {
        let req = self.request(&format!("/tournament/{id}/related"));
        self.send(req).await
    }

    pub async fn get_tournament_results(
        &self,
        id: i64,
    ) -> Result<TournamentResultsResponse, IfpaError> {
        let req = self.request(&format!("/tournament/{id}/results"));
        self.send(req).await
    }

    pub async fn get_leagues(
        &self,
        time_period: LeagueTimePeriod,
    ) -> Result<LeaguesResponse, IfpaError> {
        let req = self.request(&format!("/tournament/leagues/{time_period}"));
        self.send(req).await
    }

    pub async fn search_tournaments(
        &self,
        params: &TournamentSearchParams,
    ) -> Result<TournamentSearchResponse, IfpaError> {
        let req = self.request("/tournament/search").query(params);
        self.send(req).await
    }
}
