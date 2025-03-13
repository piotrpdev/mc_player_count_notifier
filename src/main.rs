use std::{env, thread, time::Duration};

use minecraft_query::server_status;

const DEFAULT_MC_PORT: u16 = 25565;
const DEFAULT_CHECK_INTERVAL: Duration = Duration::from_secs(10);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = env::var("MC_HOSTNAME").map_err(|_| "MC_HOSTNAME env var not set")?;
    let port = if let Ok(port_string) = env::var("MC_PORT") {
        port_string.parse::<u16>().map_err(|_| "Failed to parse MC_PORT env var as u16")?
    } else {
        println!("MC_PORT env var not set, using default port: {DEFAULT_MC_PORT}");
        DEFAULT_MC_PORT
    };
    let webhook_url = env::var("WEBHOOK_URL").map_err(|_| "WEBHOOK_URL env var not set")?;
    let status_check_interval = if let Ok(interval_string) = env::var("CHECK_INTERVAL") {
        Duration::from_secs(interval_string.parse::<u64>().map_err(|_| "Failed to parse CHECK_INTERVAL env var as u64")?)
    } else {
        println!(
            "CHECK_INTERVAL not set, using default interval: {}s",
            DEFAULT_CHECK_INTERVAL.as_secs()
        );
        DEFAULT_CHECK_INTERVAL
    };

    
    let mut old_players_name_list: Vec<String> = vec![];
    
    println!("Starting...");
    
    let mut first_loop_pass = true;
    loop {
        if !first_loop_pass {
            thread::sleep(status_check_interval);
        }
        first_loop_pass = false;

        let server_status = match server_status(&hostname, port) {
            Ok(status) => status,
            Err(e) => {
                eprintln!("Failed to get server status: {e}");
                continue;
            }
        };

        let mut new_players_name_list = server_status
            .players
            .sample
            .iter()
            .map(|player| player.name.clone())
            .collect::<Vec<String>>();
        new_players_name_list.sort_by_key(|a| a.to_lowercase());

        // https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#impl-PartialOrd%3CVec%3CT,+A2%3E%3E-for-Vec%3CT,+A1%3E
        // https://doc.rust-lang.org/std/cmp/trait.Ord.html#lexicographical-comparison
        if new_players_name_list == old_players_name_list {
            continue;
        }

        old_players_name_list = new_players_name_list;

        let player_list_string = if old_players_name_list.is_empty() {
            "Server empty.".to_owned()
        } else {
            format!("All players: `{}`", old_players_name_list.join(", "))
        };

        println!("{player_list_string}");

        let response = match minreq::post(&webhook_url)
            .with_header("Content-Type", "application/json")
            .with_body(format!("{{\"content\": \"{player_list_string}\"}}"))
            .send()
        {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to send webhook: {e}");
                continue;
            }
        };

        if !response.status_code.to_string().starts_with('2') {
            let response_string = response.as_str().unwrap_or("Failed to parse as string");
            eprintln!(
                "Webhook responded with non-2xx status code: {0}, body: {1}",
                response.status_code, response_string
            );
        }
    }
}
