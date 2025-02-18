pub fn calculate_longest_streak(days: &[serde_json::Value]) -> i64 {
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

pub fn calculate_current_streak(days: &[serde_json::Value]) -> i64 {
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
