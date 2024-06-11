use chrono::prelude::*;
use serde::Deserialize;
use duckdb::{Connection, ToSql, Result};
use std::io::{self, Write};

#[derive(Deserialize)]
struct Location {
    city: String,
    region: String,
    country: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Obtener la fecha y hora actuales
    let local: DateTime<Local> = Local::now();
    println!("Fecha y hora actuales: {}", local);

    // Obtener la ubicación utilizando la API de IPinfo
    let response = reqwest::get("https://ipinfo.io/json").await?;
    let location: Location = response.json().await?;
    println!("Ubicación: {}, {}, {}", location.city, location.region, location.country);

    // Conectar a la base de datos de DuckDB en memoria
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE irca_db (
            id INTEGER PRIMARY KEY,
            parameter TEXT,
            result_analysis TEXT,
            expressed_as TEXT,
            risk_score_considered FLOAT, 
            sample_risk_score FLOAT
        )", [],
    )?;

    println!("IRCA DATABASE INPUT SYSTEM");

    loop {
        println!("Do you want to enter another record? (yes/no)");
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input)?;
        if continue_input.trim().to_lowercase() != "yes" {
            break;
        }

        let mut parameter = String::new();
        let mut result_analysis = String::new();
        let mut expressed_as = String::new();
        let mut risk_score_considered_str = String::new();
        let mut sample_risk_score_str = String::new();

        println!("Enter Parameter:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut parameter)?;

        println!("Enter Result Analysis:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut result_analysis)?;

        println!("Enter Expressed As:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut expressed_as)?;

        println!("Enter Risk Score Considered:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut risk_score_considered_str)?;
        let risk_score_considered: f32 = risk_score_considered_str.trim().parse()?;

        println!("Enter Sample Risk Score:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut sample_risk_score_str)?;
        let sample_risk_score: f32 = sample_risk_score_str.trim().parse()?;

        // Obtener el id más alto actual y sumar 1
        let mut stmt = conn.prepare("SELECT COALESCE(MAX(id), 0) + 1 FROM irca_db")?;
        let mut rows = stmt.query([])?;
        let id = if let Some(row) = rows.next()? {
            row.get::<usize, i64>(0)?
        } else {
            1
        };

        // Convertir cadenas a String y tomar referencias a esas cadenas
        let parameter_trimmed = parameter.trim();
        let result_analysis_trimmed = result_analysis.trim();
        let expressed_as_trimmed = expressed_as.trim();

        conn.execute(
            "INSERT INTO irca_db (id, parameter, result_analysis, expressed_as, risk_score_considered, sample_risk_score) VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &id as &dyn ToSql,
                &parameter_trimmed as &dyn ToSql,
                &result_analysis_trimmed as &dyn ToSql,
                &expressed_as_trimmed as &dyn ToSql,
                &risk_score_considered as &dyn ToSql,
                &sample_risk_score as &dyn ToSql,
            ],
        )?;

        println!("Data inserted successfully!");
    }

    // Consulta y muestra los datos guardados
    let mut stmt = conn.prepare("SELECT * FROM irca_db")?;
    let mut rows = stmt.query([])?;

    println!("\nStored Data:");

    let mut total_risk_score_considered: f32 = 0.0;
    let mut total_sample_risk_score: f32 = 0.0;
    let mut count_samples = 0;

    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let parameter: String = row.get(1)?;
        let result_analysis: String = row.get(2)?;
        let expressed_as: String = row.get(3)?;
        let risk_score_considered: f32 = row.get(4)?;
        let sample_risk_score: f32 = row.get(5)?;

        total_risk_score_considered += risk_score_considered;
        total_sample_risk_score += sample_risk_score;
        count_samples += 1;

        println!("ID: {}, Parameter: {}, Result Analysis: {}, Expressed As: {}, Risk Score Considered: {}, Sample Risk Score: {}",
                 id, parameter, result_analysis, expressed_as, risk_score_considered, sample_risk_score);
    }

    let risk_status;
    if total_risk_score_considered != 0.0 {
        let user_input = (total_sample_risk_score / total_risk_score_considered) * 100.0;
        let risk_levels = vec![
            (5.0, "NO RISK"),
            (14.0, "LOW RISK"),
            (35.0, "MEDIUM RISK"),
            (80.0, "HIGH RISK"),
        ];

        risk_status = risk_levels.iter().find(|&&(threshold, _)| user_input <= threshold).map(|&(_, status)| status).unwrap_or("SANITARY UNFEASIBLE");
        println!("Overall Risk Status: {}", risk_status);
        println!("User Input Result: {:.2}", user_input);
    } else {
        println!("No data to calculate risk.");
        return Ok(());
    }

    // Guardar los resultados en la tabla monthly_irca_results
    let conn = Connection::open("irca_results_db.duckdb")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS monthly_irca_results (
            id INTEGER PRIMARY KEY, 
            year INTEGER,
            dpto_code INTEGER,
            dpto_description TEXT, 
            town TEXT,
            month TEXT,
            count_samples INTEGER,
            risk_level FLOAT,
            risk_status TEXT
        )", [],
    )?;

    // Obtener el id más alto actual y sumar 1
    let mut stmt = conn.prepare("SELECT COALESCE(MAX(id), 0) + 1 FROM monthly_irca_results")?;
    let mut rows = stmt.query([])?;
    let result_id = if let Some(row) = rows.next()? {
        row.get::<usize, i64>(0)?
    } else {
        1
    };

    let dpto_code = 12345; // Ejemplo de un código fijo para la región
    let year = local.year();
    let month = local.format("%B").to_string();

    conn.execute(
        "INSERT INTO monthly_irca_results (id, year, dpto_code, dpto_description, town, month, count_samples, risk_level, risk_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        &[
            &result_id as &dyn ToSql,
            &year as &dyn ToSql,
            &dpto_code as &dyn ToSql,
            &location.region as &dyn ToSql,
            &location.city as &dyn ToSql,
            &month as &dyn ToSql,
            &count_samples as &dyn ToSql,
            &(total_sample_risk_score / total_risk_score_considered * 100.0) as &dyn ToSql,
            &risk_status as &dyn ToSql,
        ],
    )?;

    println!("Results stored in the monthly IRCA results database.");

    // Consulta y muestra los datos guardados en la base de datos irca_results_db.duckdb
    let mut stmt = conn.prepare("SELECT * FROM monthly_irca_results")?;
    let mut rows = stmt.query([])?;

    println!("\nMonthly IRCA Results Data:");
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let year: i32 = row.get(1)?;
        let dpto_code: i32 = row.get(2)?;
        let dpto_description: String = row.get(3)?;
        let town: String = row.get(4)?;
        let month: String = row.get(5)?;
        let count_samples: i32 = row.get(6)?;
        let risk_level: f32 = row.get(7)?;
        let risk_status: String = row.get(8)?;

        println!("ID: {}, Year: {}, Dpto Code: {}, Dpto Description: {}, Town: {}, Month: {}, Count Samples: {}, Risk Level: {:.2}, Risk Status: {}",
                 id, year, dpto_code, dpto_description, town, month, count_samples, risk_level, risk_status);
    }

    Ok(())
}







