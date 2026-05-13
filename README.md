# ifpaclient

Async Rust client for the [IFPA Pinball API](https://api.ifpapinball.com/docs/).

Covers all endpoints in the IFPA API v2.1 — players, tournaments, rankings, series, stats, directors, and more.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ifpaclient = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick start

```rust
use ifpaclient::IfpaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = IfpaClient::new("your-api-key");

    // Look up a player
    let response = client.get_player(4).await?;
    let player = &response.player[0];
    println!("{} {}", player.first_name, player.last_name);

    // Search for players
    use ifpaclient::models::player::PlayerSearchParams;

    let results = client.search_players(&PlayerSearchParams {
        name: Some("Sharpe".into()),
        country: Some("US".into()),
        ..Default::default()
    }).await?;

    for player in &results.results {
        println!("#{} - {} {} ({})", player.wppr_rank, player.first_name, player.last_name, player.country_code);
    }

    Ok(())
}
```

## API coverage

All 46 endpoints are implemented. Every method is async and returns `Result<T, IfpaError>`.

### Players

```rust
client.get_player(id).await?;
client.get_players(&[1, 2, 3]).await?;
client.search_players(&PlayerSearchParams { ... }).await?;
client.get_player_results(id, RankingSystem::Main, ResultType::Active).await?;
client.get_player_rank_history(id).await?;
client.get_player_pvp(id).await?;
client.get_player_pvp_comparison(id1, id2).await?;
```

### Tournaments

```rust
use ifpaclient::models::common::LeagueTimePeriod;
use ifpaclient::models::tournament::TournamentSearchParams;

client.get_tournament(id).await?;
client.get_tournament_results(id).await?;
client.get_tournament_related(id).await?;
client.get_tournament_formats().await?;
client.get_leagues(LeagueTimePeriod::Active).await?;
client.search_tournaments(&TournamentSearchParams {
    country: Some("US".into()),
    start_date: Some("2025-01-01".into()),
    end_date: Some("2025-12-31".into()),
    ..Default::default()
}).await?;
```

### Rankings

```rust
use ifpaclient::models::common::*;
use ifpaclient::models::rankings::*;

client.get_rankings_wppr(&PaginationParams { count: Some(10), ..Default::default() }).await?;
client.get_rankings_youth(&PaginationParams::default()).await?;
client.get_rankings_virtual(&PaginationParams::default()).await?;
client.get_rankings_women(WomenTournamentType::Open, &PaginationParams::default()).await?;
client.get_rankings_pro(RankType::Open).await?;
client.get_rankings_country(&CountryRankingsParams { country: "US".into(), ..Default::default() }).await?;
client.get_rankings_country_list().await?;
client.get_custom_rankings_list().await?;
client.get_custom_rankings(id, &CustomRankingsParams::default()).await?;
```

### Series

```rust
use ifpaclient::models::series::*;

client.get_series_list().await?;
client.get_series_regions("NACS", &SeriesYearParams::default()).await?;
client.get_series_standings("NACS", &SeriesRegionParams { region_code: "OH".into(), ..Default::default() }).await?;
client.get_series_overall_standings("NACS", &SeriesYearParams::default()).await?;
client.get_series_stats("NACS", &SeriesRegionParams { region_code: "OH".into(), ..Default::default() }).await?;
client.get_series_tournaments("NACS", &SeriesRegionParams { region_code: "OH".into(), ..Default::default() }).await?;
client.get_series_player_card("NACS", player_id, &PlayerCardParams { region_code: "OH".into(), ..Default::default() }).await?;
client.get_series_region_reps("NACS").await?;
```

### Stats

```rust
use ifpaclient::models::stats::*;

client.get_stats_overall(&OverallStatsParams::default()).await?;
client.get_stats_country_players(&StatsRankTypeParams::default()).await?;
client.get_stats_state_players(&StatsRankTypeParams::default()).await?;
client.get_stats_state_tournaments(&StatsRankTypeParams::default()).await?;
client.get_stats_events_by_year(&StatsCountryParams::default()).await?;
client.get_stats_players_by_year().await?;
client.get_stats_largest_tournaments(&StatsCountryParams::default()).await?;
client.get_stats_lucrative_tournaments(&LucrativeTournamentsParams::default()).await?;
client.get_stats_points_given_period(&PeriodParams {
    start_date: Some("2024-01-01".into()),
    end_date: Some("2024-12-31".into()),
    ..Default::default()
}).await?;
client.get_stats_events_attended_period(&PeriodParams::default()).await?;
```

### Directors

```rust
use ifpaclient::models::common::TimePeriod;
use ifpaclient::models::director::DirectorSearchParams;

client.get_director(id).await?;
client.get_director_tournaments(id, TimePeriod::Past).await?;
client.get_country_directors().await?;
client.search_directors(&DirectorSearchParams { name: Some("Sharpe".into()), ..Default::default() }).await?;
```

### Other

```rust
client.get_countries().await?;
client.get_state_provs().await?;
```

## Error handling

All methods return `Result<T, IfpaError>`. The error type covers both HTTP transport errors and API-level errors (4xx/5xx responses):

```rust
use ifpaclient::IfpaError;

match client.get_player(99999).await {
    Ok(response) => println!("{}", response.player[0].first_name),
    Err(IfpaError::Api { status, message }) => eprintln!("API error {status}: {message}"),
    Err(IfpaError::Http(e)) => eprintln!("Network error: {e}"),
}
```

## Custom base URL

For testing or proxying:

```rust
let client = IfpaClient::new("key").with_base_url("https://my-proxy.example.com");
```

## License

MIT
