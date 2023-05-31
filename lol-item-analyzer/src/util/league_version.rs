const LEAGUE_VERSIONS_URL: &str = "https://ddragon.leagueoflegends.com/api/versions.json";

pub async fn get_all_versions() -> Result<Vec<String>, String> {
    let response = reqwest::get(LEAGUE_VERSIONS_URL)
        .await
        .map_err(|e| format!("Error with response: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Could not get text: {}", e))?;

    serde_json::from_str(response.as_str()).map_err(|e| format!("Error while parsing JSON string: {}", e))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn get_all_versions_works() {
        let result = get_all_versions().await;

        assert!(&result.is_ok(), "Result is error");
        assert_ne!(result.as_ref().unwrap().len(), 0, "Result is empty list");

        println!("{:#?}", result.unwrap());
    }
}
