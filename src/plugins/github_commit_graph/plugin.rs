use crate::plugins::github_commit_graph::commit_grayscale_filter::GitCommitGreyscale;
use crate::plugins::github_commit_graph::utils::{
    calculate_current_streak, calculate_longest_streak,
};
use crate::plugins::plugin::Plugin;
use crate::plugins::utils::html_body;
use crate::renderer::bmp_renderer::BmpRenderer;
use async_trait::async_trait;
use liquid::ParserBuilder;
use logcall::logcall;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Clone, Debug, Deserialize)]
pub struct GithubCommitGraphPlugin {
    pub(crate) username: String,
    pub(crate) api_key: String,
}

#[async_trait]
impl Plugin for GithubCommitGraphPlugin {
    #[logcall(err = "warn")]
    async fn template(&self) -> anyhow::Result<String> {
        let client = Client::new();

        let query = r#"
            query($userName:String!) {
                user(login: $userName) {
                    contributionsCollection {
                        contributionCalendar {
                            totalContributions
                            weeks {
                                contributionDays {
                                    contributionCount
                                    date
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let body = json!({
            "query": query,
            "variables": { "userName": self.username }
        });

        let resp = client
            .post("https://api.github.com/graphql")
            .bearer_auth(self.api_key.clone())
            .header("User-Agent", "byos_rs")
            .body(serde_json::to_string(&body)?)
            .send()
            .await?;

        let text = resp.text().await?;
        let data: serde_json::Value = serde_json::from_str(&text)?;
        let contributions =
            data["data"]["user"]["contributionsCollection"]["contributionCalendar"].clone();

        let total_contributions = contributions["totalContributions"]
            .as_i64()
            .ok_or(anyhow::anyhow!("No contributions found"))?;
        let commits = contributions["weeks"]
            .as_array()
            .ok_or(anyhow::anyhow!("No contributions found"))?;

        let mut days: Vec<_> = commits
            .iter()
            .flat_map(|week| {
                week["contributionDays"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .clone()
            })
            .collect();

        days.sort_by_key(|day| {
            day["date"]
                .as_str()
                .map_or("".to_string(), |v| v.to_string())
        });

        let max_contributions = days
            .iter()
            .map(|d| d["contributionCount"].as_i64().unwrap_or(0))
            .max()
            .ok_or(anyhow::anyhow!("No contributions found"))?;
        let average_contributions = (days
            .iter()
            .map(|d| d["contributionCount"].as_i64().unwrap_or(0))
            .sum::<i64>() as f64
            / days.len() as f64)
            .round();
        let longest_streak = calculate_longest_streak(&days);
        let current_streak = calculate_current_streak(&days);

        let parser = ParserBuilder::with_stdlib()
            .filter(GitCommitGreyscale)
            .build()?;

        let template = include_str!("github_commit_graph.liquid");
        let template = parser.parse(template)?;

        let globals = liquid::object!({
            "contributions": {
                "total": total_contributions,
                "commits": commits,
            },
            "stats": {
                "longest_streak": longest_streak,
                "current_streak": current_streak,
                "max_contributions": max_contributions,
                "average_contributions": average_contributions,
            },
            "base_url": "https://example.com",
            "instance_name": format!("@{}", self.username),
        });

        let output = template.render(&globals)?;
        Ok(html_body(output))
    }

    #[logcall(err = "warn")]
    async fn render(
        &self,
        template: String,
        bmp_renderer: &BmpRenderer,
    ) -> anyhow::Result<Vec<u8>> {
        bmp_renderer.render_html(template)
    }
}
