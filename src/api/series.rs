use crate::IfpaClient;
use crate::IfpaError;
use crate::models::series::*;

impl IfpaClient {
    pub async fn get_series_list(&self) -> Result<SeriesListResponse, IfpaError> {
        let req = self.request("/series/list");
        self.send(req).await
    }

    pub async fn get_series_regions(
        &self,
        series_code: &str,
        params: &SeriesYearParams,
    ) -> Result<SeriesRegionsResponse, IfpaError> {
        let req = self
            .request(&format!("/series/{series_code}/regions"))
            .query(params);
        self.send(req).await
    }

    pub async fn get_series_player_card(
        &self,
        series_code: &str,
        player_id: i64,
        params: &PlayerCardParams,
    ) -> Result<PlayerCardResponse, IfpaError> {
        let req = self
            .request(&format!("/series/{series_code}/player_card/{player_id}"))
            .query(params);
        self.send(req).await
    }

    pub async fn get_series_region_reps(
        &self,
        series_code: &str,
    ) -> Result<RegionRepsResponse, IfpaError> {
        let req = self.request(&format!("/series/{series_code}/region_reps"));
        self.send(req).await
    }

    pub async fn get_series_overall_standings(
        &self,
        series_code: &str,
        params: &SeriesYearParams,
    ) -> Result<OverallStandingsResponse, IfpaError> {
        let req = self
            .request(&format!("/series/{series_code}/overall_standings"))
            .query(params);
        self.send(req).await
    }

    pub async fn get_series_standings(
        &self,
        series_code: &str,
        params: &SeriesRegionParams,
    ) -> Result<StandingsResponse, IfpaError> {
        let req = self
            .request(&format!("/series/{series_code}/standings"))
            .query(params);
        self.send(req).await
    }

    pub async fn get_series_stats(
        &self,
        series_code: &str,
        params: &SeriesRegionParams,
    ) -> Result<SeriesStatsResponse, IfpaError> {
        let req = self
            .request(&format!("/series/{series_code}/stats"))
            .query(params);
        self.send(req).await
    }

    pub async fn get_series_tournaments(
        &self,
        series_code: &str,
        params: &SeriesRegionParams,
    ) -> Result<SeriesTournamentsResponse, IfpaError> {
        let req = self
            .request(&format!("/series/{series_code}/tournaments"))
            .query(params);
        self.send(req).await
    }
}
