use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: DateTime<Utc>,
    iat: DateTime<Utc>,
    iss: String,
    nbf: DateTime<Utc>,
    sub: String,
}

impl Claims {
    pub fn new(sub: String) -> Claims {
        let now = Utc::now();

        Claims {
            aud: "gofar".to_string(),
            exp: now + Duration::seconds(dotenv!("JWT_TTL").parse::<i64>().unwrap()),
            iat: now,
            iss: "gofar".to_string(),
            nbf: now,
            sub,
        }
    }
}
