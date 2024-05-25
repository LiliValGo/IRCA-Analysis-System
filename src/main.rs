use duckdb::{Connection, ToSql};
use std::error::Error;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("irca_db.duckdb")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS irca_db (
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

    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let parameter: String = row.get(1)?;
        let result_analysis: String = row.get(2)?;
        let expressed_as: String = row.get(3)?;
        let risk_score_considered: f32 = row.get(4)?;
        let sample_risk_score: f32 = row.get(5)?;

        total_risk_score_considered += risk_score_considered;
        total_sample_risk_score += sample_risk_score;

        println!("ID: {}, Parameter: {}, Result Analysis: {}, Expressed As: {}, Risk Score Considered: {}, Sample Risk Score: {}",
                 id, parameter, result_analysis, expressed_as, risk_score_considered, sample_risk_score);
    }

    if total_risk_score_considered != 0.0 {
        let user_input = (total_sample_risk_score / total_risk_score_considered) * 100.0;
        let risk_levels = vec![
            (5.0, "NO RISK"),
            (14.0, "LOW RISK"),
            (35.0, "MEDIUM RISK"),
            (80.0, "HIGH RISK"),
        ];

        let mut risk_status = "SANITARY UNFEASIBLE";
        for &(threshold, status) in &risk_levels {
            if user_input <= threshold {
                risk_status = status;
                break;
            }
        }
        println!("Overall Risk Status: {}", risk_status);
        println!("User Input Result: {:.2}", user_input);
    } else {
        println!("No data to calculate risk.");
    }
    
    Ok(())
}







