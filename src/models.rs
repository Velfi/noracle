use chrono::naive::NaiveDateTime;
use crate::schema::{outcomes, prediction_events, transactions, users};
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Serialize, Queryable, Clone)]
pub struct Outcome {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: NaiveDateTime,
    pub resolution_date: NaiveDateTime,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let outcome_id = match &self.id {
            Some(id) => format!("Outcome #{}", id).to_string(),
            None => "Outcome (No ID)".to_string(),
        };
        let outcome_desc = match &self.description {
            Some(desc) => desc.to_string(),
            None => "(No description)".to_string(),
        };
        let outcome_creation = format!("Created on {}", &self.creation_date);
        let outcome_resolution = format!("and resolving on {}", &self.resolution_date);

        writeln!(
            f,
            "{} -- {}\n{}\n{} {}",
            outcome_id, &self.title, outcome_desc, outcome_creation, outcome_resolution
        )
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "outcomes"]
pub struct NewOutcome {
    pub title: String,
    pub description: Option<String>,
    pub creation_date: NaiveDateTime,
    pub resolution_date: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Clone)]
pub struct PredictionEvent {
    pub id: Option<i32>,
    pub by_user: i32,
    pub for_outcome: i32,
    pub prediction: bool,
    pub creation_date: NaiveDateTime,
}

impl Display for PredictionEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pe_id = match &self.id {
            Some(id) => format!("Prediction #{}", id).to_string(),
            None => "Prediction (No ID)".to_string(),
        };
        let pe_user = format!("made by User #{}", &self.by_user);
        let pe_creation = format!("on {}", &self.creation_date);
        let pe_outcome = format!("Outcome #{}", &self.for_outcome);
        let pe_resolution = match &self.prediction {
            true => "will come to pass".to_string(),
            false => "will not come to pass".to_string(),
        };

        writeln!(
            f,
            "{} -- {} {}\n{} {}",
            pe_id, pe_user, pe_creation, pe_outcome, pe_resolution
        )
    }
}

#[derive(Insertable)]
#[table_name = "prediction_events"]
pub struct NewPredictionEvent {
    pub by_user: i32,
    pub for_outcome: i32,
    pub prediction: bool,
    pub creation_date: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Clone)]
pub struct Transaction {
    pub id: Option<i32>,
    pub date: NaiveDateTime,
    pub amount: i32,
    pub user_id: i32,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pad_amount = 10;
        let trans_id = match &self.id {
            Some(id) => format!("{}", id).to_string(),
            None => "(No ID)".to_string(),
        };
        let trans_amount = match self.amount {
            amt if amt < 0 => left_pad(&format!("({})", amt.wrapping_neg()), pad_amount),
            amt => left_pad(&format!("{} ", amt), pad_amount),
        };

        writeln!(
            f,
            "U{}-T{}-D{}{}",
            &self.user_id,
            trans_id,
            &self.date.date(),
            &trans_amount
        )
    }
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub user_id: i32,
    pub amount: i32,
    pub date: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub display_name: String,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let user_id = match &self.id {
            Some(id) => format!("{}", id).to_string(),
            None => "(No ID)".to_string(),
        };
        let padded_username = pad(&self.username, 10);
        let padded_name = left_pad(&format!("\"{}\"", &self.display_name), 15);

        writeln!(f, "({}) {}  {}", &user_id, &padded_username, &padded_name)
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
}

pub fn left_pad(s: &str, pad: usize) -> String {
    left_pad_char(s, pad, ' ')
}

pub fn left_pad_char(s: &str, pad: usize, padchar: char) -> String {
    let mut out = String::new();

    let len = s.len();
    if pad > len {
        for _ in 0..pad - len {
            out.push(padchar);
        }
    }
    out.push_str(s);

    out
}

pub fn pad(s: &str, pad: usize) -> String {
    pad_char(s, pad, ' ')
}

pub fn pad_char(s: &str, pad: usize, padchar: char) -> String {
    let mut out = String::new();
    out.push_str(s);

    let len = s.len();
    if pad > len {
        for _ in 0..pad - len {
            out.push(padchar);
        }
    }

    out
}
