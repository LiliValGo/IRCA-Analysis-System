# IRCA Data Input and Risk Analysis System
This project is a command-line application designed for recording, analyzing, and assessing risk levels based on user-provided data entries and geolocation information. It collects various parameters and scores, storing them in an in-memory DuckDB database for immediate analysis and risk assessment.

The application calculates an overall risk status based on sample data, determining qualitative levels like "No Risk" or "High Risk" and assigns a corresponding risk status. Monthly results are stored in a separate DuckDB file-based database for long-term tracking, allowing users to review past entries and calculated risk levels in a comprehensive report.

## Features
  * **Data Entry and Management:** Allows users to input various parameters, storing them in an in-memory DuckDB database.
  * **Risk Analysis Calculation:** Based on data provided, the application calculates and assigns a risk status to each entry.
  * **Automated Location Detection:**  Utilizes the IP info API to gather the user’s geographic location dynamically.
  * **Persistent Data Storage:** Saves monthly risk assessments in a DuckDB file-based database, enabling long-term data tracking.
  * **Comprehensive Reporting:** Outputs saved data entries, allowing users to review each entry with calculated risk levels and statuses.

## Technical Requirements
  * **Rust:** This project is built using Rust, so Rust must be installed.
  * **DuckDB:** Provides an in-memory database and file-based storage.
  * **reqwest:** Used for making asynchronous HTTP requests to retrieve location data.
To install these dependencies, add the following to ```Cargo.toml```:
```[dependencies]
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
duckdb = "0.4.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] } 
```

## Installation
1. Clone the repository:
   
   ```git clone https://github.com/yourusername/irca-risk-analysis.git```
2. Install dependencies:
   
   ```cargo build```
3. Run the application

   ```cargo run```
