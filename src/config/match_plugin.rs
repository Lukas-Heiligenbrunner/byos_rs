use crate::config::types::{Config, Plugin, Schedule};
use anyhow::anyhow;
use chrono::Local;
use log::warn;

fn schedule_active(schedule: &Schedule, today: String, curr_time: String) -> anyhow::Result<bool> {
    if !schedule.days.contains(&today.to_string()) {
        return Ok(false);
    }

    let start_time = schedule.start_time.split(':').collect::<Vec<&str>>();
    let start_hour = start_time
        .first()
        .ok_or(anyhow!("failed to parse time"))?
        .parse::<u8>()?;
    let start_minute = start_time
        .get(1)
        .ok_or(anyhow!("failed to parse time"))?
        .parse::<u8>()?;

    let endtime = schedule.end_time.split(':').collect::<Vec<&str>>();
    let end_hour = endtime
        .first()
        .ok_or(anyhow!("failed to parse time"))?
        .parse::<u8>()?;
    let end_minute = endtime
        .get(1)
        .ok_or(anyhow!("failed to parse time"))?
        .parse::<u8>()?;

    let currtime = curr_time.split(':').collect::<Vec<&str>>();
    let curr_hour = currtime
        .first()
        .ok_or(anyhow!("failed to parse time"))?
        .parse::<u8>()?;
    let curr_minute = currtime
        .get(1)
        .ok_or(anyhow!("failed to parse time"))?
        .parse::<u8>()?;

    if start_hour > curr_hour || curr_hour > end_hour {
        return Ok(false);
    }

    if start_hour == curr_hour && start_minute > curr_minute {
        return Ok(false);
    }

    if end_hour == curr_hour && end_minute <= curr_minute {
        return Ok(false);
    }

    Ok(true)
}

impl Config {
    pub fn match_plugin(&self) -> Plugin {
        // Get the current date and time
        let now = Local::now();

        // Get the current weekday and time
        let today = now.format("%A").to_string(); // Full weekday name
        let curr_time = now.format("%H:%M").to_string(); // Current time in HH:MM format

        let valid_schedules = self
            .schedules
            .iter()
            .filter(|x| {
                schedule_active(x, today.to_string(), curr_time.to_string()).unwrap_or_else(|e| {
                    warn!("{}", e);
                    false
                })
            })
            .collect::<Vec<&Schedule>>();
        match valid_schedules.first() {
            None => self.default_screen.clone(),
            Some(v) => v.plugin.clone(),
        }
    }
}
