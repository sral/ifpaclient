use ifpaclient::IfpaClient;
use ifpaclient::models::tournament::TournamentSearchParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("IFPA_API_KEY").expect("set IFPA_API_KEY env var");
    let client = IfpaClient::new(api_key);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let response = client
        .search_tournaments(&TournamentSearchParams {
            country: Some("Sweden".into()),
            start_date: Some(today),
            end_date: Some("2030-12-31".into()),
            ..Default::default()
        })
        .await?;

    let mut tournaments = response.tournaments;
    tournaments.sort_by(|a, b| a.event_start_date.cmp(&b.event_start_date));

    for t in &tournaments {
        let date = t
            .event_start_date
            .split('T')
            .next()
            .unwrap_or(&t.event_start_date);
        println!("{date}: {}", t.tournament_name);
        println!(
            "    Location: {}, {}, {}",
            t.address1, t.city, t.country_name
        );
        println!("    Director: {}", t.director_name);
        println!("    Website: {}", t.website);
        if let Some(reg) = &t.preregistration_date {
            let reg = if reg == "0000-00-00" {
                "Open registration"
            } else {
                reg.as_str()
            };
            println!("    Registration date: {reg}");
        }
    }

    Ok(())
}
