use crate::IfpaClient;
use crate::IfpaError;
use crate::models::common::{RankingSystem, ResultType};
use crate::models::player::*;

impl IfpaClient {
    pub async fn get_player(&self, id: i64) -> Result<PlayerResponse, IfpaError> {
        let req = self.request(&format!("/player/{id}"));
        self.send(req).await
    }

    pub async fn get_players(&self, ids: &[i64]) -> Result<PlayersResponse, IfpaError> {
        let ids_str = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let req = self.request("/player").query(&[("players", &ids_str)]);
        self.send(req).await
    }

    pub async fn get_player_pvp(&self, id: i64) -> Result<PlayerPvpResponse, IfpaError> {
        let req = self.request(&format!("/player/{id}/pvp"));
        self.send(req).await
    }

    pub async fn get_player_pvp_comparison(
        &self,
        id: i64,
        id2: i64,
    ) -> Result<PvpComparisonResponse, IfpaError> {
        let req = self.request(&format!("/player/{id}/pvp/{id2}"));
        self.send(req).await
    }

    pub async fn get_player_rank_history(&self, id: i64) -> Result<RankHistoryResponse, IfpaError> {
        let req = self.request(&format!("/player/{id}/rank_history"));
        self.send(req).await
    }

    pub async fn get_player_results(
        &self,
        id: i64,
        ranking_system: RankingSystem,
        result_type: ResultType,
    ) -> Result<PlayerResultsResponse, IfpaError> {
        let req = self.request(&format!(
            "/player/{id}/results/{ranking_system}/{result_type}"
        ));
        self.send(req).await
    }

    pub async fn search_players(
        &self,
        params: &PlayerSearchParams,
    ) -> Result<PlayerSearchResponse, IfpaError> {
        let req = self.request("/player/search").query(params);
        self.send(req).await
    }
}
