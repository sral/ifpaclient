use crate::IfpaClient;
use crate::IfpaError;
use crate::models::common::{PaginationParams, RankType, WomenTournamentType};
use crate::models::rankings::*;

impl IfpaClient {
    pub async fn get_rankings_wppr(
        &self,
        params: &PaginationParams,
    ) -> Result<WpprRankingsResponse, IfpaError> {
        let req = self.request("/rankings/wppr").query(params);
        self.send(req).await
    }

    pub async fn get_rankings_youth(
        &self,
        params: &PaginationParams,
    ) -> Result<YouthRankingsResponse, IfpaError> {
        let req = self.request("/rankings/youth").query(params);
        self.send(req).await
    }

    pub async fn get_rankings_virtual(
        &self,
        params: &PaginationParams,
    ) -> Result<VirtualRankingsResponse, IfpaError> {
        let req = self.request("/rankings/virtual").query(params);
        self.send(req).await
    }

    pub async fn get_rankings_women(
        &self,
        tournament_type: WomenTournamentType,
        params: &PaginationParams,
    ) -> Result<WomenRankingsResponse, IfpaError> {
        let req = self
            .request(&format!("/rankings/women/{tournament_type}"))
            .query(params);
        self.send(req).await
    }

    pub async fn get_rankings_pro(
        &self,
        ranking_system: RankType,
    ) -> Result<ProRankingsResponse, IfpaError> {
        let req = self.request(&format!("/rankings/pro/{ranking_system}"));
        self.send(req).await
    }

    pub async fn get_rankings_country(
        &self,
        params: &CountryRankingsParams,
    ) -> Result<CountryRankingsResponse, IfpaError> {
        let req = self.request("/rankings/country").query(params);
        self.send(req).await
    }

    pub async fn get_rankings_country_list(&self) -> Result<CountryListResponse, IfpaError> {
        let req = self.request("/rankings/country_list");
        self.send(req).await
    }

    pub async fn get_custom_rankings_list(&self) -> Result<CustomRankingsListResponse, IfpaError> {
        let req = self.request("/rankings/custom/list");
        self.send(req).await
    }

    pub async fn get_custom_rankings(
        &self,
        id: i64,
        params: &CustomRankingsParams,
    ) -> Result<CustomRankingsResponse, IfpaError> {
        let req = self
            .request(&format!("/rankings/custom/{id}"))
            .query(params);
        self.send(req).await
    }
}
