use crate::plugins::{render_html, Plugin};
use async_trait::async_trait;
use liquid::model::Value;
use liquid::{ParserBuilder, ValueView};
use liquid_core::{
    Display_filter, Filter, FilterReflection, ParseFilter, Runtime,
};
use reqwest::Client;
use serde_json::json;
use std::fmt::Debug;

pub struct GithubCommitGraphPlugin;
#[async_trait]
impl Plugin for GithubCommitGraphPlugin {
    async fn render(&self) -> anyhow::Result<String> {
        let username = "Lukas-Heiligenbrunner";
        let token = "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";
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
            "variables": { "userName": username }
        });

        let resp = client
            .post("https://api.github.com/graphql")
            .bearer_auth(token)
            .header("User-Agent", "byos_rs")
            .body(serde_json::to_string(&body)?)
            .send()
            .await?;

        let text = resp.text().await?;
        let data: serde_json::Value = serde_json::from_str(&text)?;
        let contributions =
            data["data"]["user"]["contributionsCollection"]["contributionCalendar"].clone();

        let total_contributions = contributions["totalContributions"].as_i64().unwrap_or(0);
        let commits = contributions["weeks"].as_array().unwrap_or(&vec![]).clone();

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
            .unwrap_or(0);
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

        let template = parser.parse(raw_template())?;

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
            "instance_name": "BYOS_rs",
        });

        let output = template.render(&globals)?;
        Ok(render_html(output))
    }
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "git_commit_grayscale",
    description = "Limits a number to a maximum value.",
    parsed(GitCommitGreyscaleFilter)
)]
pub struct GitCommitGreyscale;

#[derive(Debug, Default, Display_filter)]
#[name = "git_commit_grayscale"]
struct GitCommitGreyscaleFilter;

impl Filter for GitCommitGreyscaleFilter {
    fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> liquid_core::Result<Value> {
        let count = input.as_scalar().and_then(|s| s.to_integer()).unwrap_or(0);
        let shade = match count {
            0 => "bg-white",
            1 => "bg--gray-7",
            2 => "bg--gray-6",
            3 => "bg--gray-5",
            4 => "bg--gray-4",
            5 => "bg--gray-3",
            6 => "bg--gray-2",
            7 => "bg--gray-1",
            _ => "bg-black",
        };
        Ok(Value::Scalar(shade.to_string().into()))
    }
}

fn calculate_longest_streak(days: &[serde_json::Value]) -> i64 {
    let mut longest = 0;
    let mut current = 0;
    for day in days {
        if day["contributionCount"].as_i64().unwrap_or(0) > 0 {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    longest
}

fn calculate_current_streak(days: &[serde_json::Value]) -> i64 {
    let mut streak = 0;
    for day in days.iter().rev() {
        if day["contributionCount"].as_i64().unwrap_or(0) > 0 {
            streak += 1;
        } else {
            break;
        }
    }
    streak
}

fn raw_template() -> &'static str {
    r#"
<div class="view view--full">
  <div class="layout layout--col gap--space-between">
    <div class="grid grid--cols-2">
      <div class="item">
        <div class="meta"></div>
        <div class="content">
          <span class="value value--xxxlarge">{{ contributions.total }}</span>
          <span class="label">Contributions in last year</span>
        </div>
      </div>
      <div class="flex flex--col gap--medium">
        <div class="grid grid--cols-2">
          <div class="item">
            <div class="meta"></div>
            <div class="content">
              <span class="value">{{ stats.longest_streak }}</span>
              <span class="label">Longest streak</span>
            </div>
          </div>
          <div class="item">
            <div class="meta"></div>
            <div class="content">
              <span class="value">{{ stats.current_streak }}</span>
              <span class="label">Current streak</span>
            </div>
          </div>
        </div>

        <div class="w-full b-h-gray-5"></div>

        <div class="grid grid--cols-2">
          <div class="item">
            <div class="meta"></div>
            <div class="content">
              <span class="value">{{ stats.max_contributions }}</span>
              <span class="label">Most in a day</span>
            </div>
          </div>
          <div class="item">
            <div class="meta"></div>
            <div class="content">
              <span class="value">{{ stats.average_contributions }}</span>
              <span class="label">Average per day</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="w-full b-h-gray-5"></div>

    <div id="github_commit_graph">
      {% for week_of_commits in contributions.commits %}
        {% for day in week_of_commits.contributionDays %}
          <span class="day {{ day.contributionCount | git_commit_grayscale }}"></span>
        {% endfor %}
      {% endfor %}
    </div>
  </div>

  <div class="title_bar">
    <img class="image" src="{{ base_url }}/images/plugins/github--render.svg" />
    <span class="title">GitHub</span>
    <span class="instance">{{ instance_name }}</span>
  </div>
</div>

<style>
  #github_commit_graph {
    width: 755px;
    height: 182px;
    overflow: hidden;

    column-count: auto;
    column-fill: auto;
    column-width: 14px;
    column-gap: 0px;
  }

  #github_commit_graph .day {
    width: 11px;
    height: 23px;
    float: left;
    border-radius: 4px;
    margin: 0px 0px 3px 0px;

    break-inside: avoid-column;
  }

  .view--quadrant #github_commit_graph {
    width: 318px;
    height: 70px;
    column-width: 6px;
    margin-top: 0;
    padding-left: 0;
    margin-left: 0px;
  }

  .view--quadrant #github_commit_graph .day {
    width: 5px;
    height: 9px;
    border-radius: 2px;
    margin: 0 0 1px 0;
  }

  .view--half_vertical #github_commit_graph {
    width: 318px;
    height: 252px;
    column-width: 6px;
    margin-top: 0;
    padding-left: 0;
    margin-left: 0px;
  }

  .view--half_vertical #github_commit_graph .day {
    width: 5px;
    height: 34px;
    margin: 0 0 2px 0px;
  }

  .view--half_horizontal #github_commit_graph {
    width: 589px;
    height: 175px;
    column-width: 11px;
  }

  .view--half_horizontal #github_commit_graph .day {
    width: 9px;
    height: 22px;
  }
</style>
"#
}
