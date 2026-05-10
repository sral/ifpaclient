use crate::IfpaClient;
use crate::IfpaError;
use crate::models::stats::*;

impl IfpaClient {
    pub async fn get_stats_country_players(
        &self,
        params: &StatsRankTypeParams,
    ) -> Result<CountryPlayersResponse, IfpaError> {
        let req = self.request("/stats/country_players").query(params);
        self.send(req).await
    }

    pub async fn get_stats_state_players(
        &self,
        params: &StatsRankTypeParams,
    ) -> Result<StatePlayersResponse, IfpaError> {
        let req = self.request("/stats/state_players").query(params);
        self.send(req).await
    }

    pub async fn get_stats_state_tournaments(
        &self,
        params: &StatsRankTypeParams,
    ) -> Result<StateTournamentsResponse, IfpaError> {
        let req = self.request("/stats/state_tournaments").query(params);
        self.send(req).await
    }

    pub async fn get_stats_events_by_year(
        &self,
        params: &StatsCountryParams,
    ) -> Result<EventsByYearResponse, IfpaError> {
        let req = self.request("/stats/events_by_year").query(params);
        self.send(req).await
    }

    pub async fn get_stats_players_by_year(&self) -> Result<PlayersByYearResponse, IfpaError> {
        let req = self.request("/stats/players_by_year");
        self.send(req).await
    }

    pub async fn get_stats_largest_tournaments(
        &self,
        params: &StatsCountryParams,
    ) -> Result<LargestTournamentsResponse, IfpaError> {
        let req = self.request("/stats/largest_tournaments").query(params);
        self.send(req).await
    }

    pub async fn get_stats_lucrative_tournaments(
        &self,
        params: &LucrativeTournamentsParams,
    ) -> Result<LucrativeTournamentsResponse, IfpaError> {
        let req = self.request("/stats/lucrative_tournaments").query(params);
        self.send(req).await
    }

    pub async fn get_stats_points_given_period(
        &self,
        params: &PeriodParams,
    ) -> Result<PointsGivenPeriodResponse, IfpaError> {
        let req = self.request("/stats/points_given_period").query(params);
        self.send(req).await
    }

    pub async fn get_stats_events_attended_period(
        &self,
        params: &PeriodParams,
    ) -> Result<EventsAttendedPeriodResponse, IfpaError> {
        let req = self.request("/stats/events_attended_period").query(params);
        self.send(req).await
    }

    pub async fn get_stats_overall(
        &self,
        params: &OverallStatsParams,
    ) -> Result<OverallStatsResponse, IfpaError> {
        let req = self.request("/stats/overall").query(params);
        self.send(req).await
    }
}
