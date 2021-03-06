use chrono::prelude::*;

#[derive(Debug)]
pub struct SunInfo {
    sunrise: DateTime<FixedOffset>,
    sunset: DateTime<FixedOffset>,
}

impl SunInfo {
    pub fn from_api(lat: String, lng: String) -> Result<SunInfo, &'static str> {
        let url = format!(
            "https://api.sunrise-sunset.org/json?lat={}&lng={}&formatted=0",
            lat, lng
        );

        match reqwest::blocking::get(&url) {
            Ok(response) => match response.json::<serde_json::Value>() {
                Ok(j) => {
                    let sunrise = match DateTime::parse_from_rfc3339(
                        j["results"]["sunrise"].as_str().unwrap(),
                    ) {
                        Ok(s) => s,
                        Err(_e) => return Err("unable to parse sunrise")?,
                    };

                    let sunset = match DateTime::parse_from_rfc3339(
                        j["results"]["sunset"].as_str().unwrap(),
                    ) {
                        Ok(s) => s,
                        Err(_e) => return Err("unable to parse sunset")?,
                    };

                    Ok(SunInfo { sunrise, sunset })
                }
                Err(_e) => Err("unable to parse sun information")?,
            },
            Err(_e) => Err("unable to get sun information")?,
        }
    }

    pub fn sunup(&self) -> &DateTime<FixedOffset> {
        &self.sunrise
    }

    pub fn sundown(&self) -> &DateTime<FixedOffset> {
        &self.sunset
    }
}
