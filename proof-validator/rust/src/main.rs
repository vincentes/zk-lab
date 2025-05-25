use axum::{
    routing::post, Json, Router
};
use std::{net::SocketAddr, process::Command, time::Duration};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use tower_http::cors::{CorsLayer, Any};

#[derive(Serialize)]
struct VerifyProofResponse {
    valid: bool,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Proof {
    pub a: [String; 2],
    pub b: [[String; 2]; 2],
    pub c: [String; 2],
}

#[derive(Deserialize, Serialize, Debug)]
struct VerifyProofRequest {
    scheme: String,
    curve: String,
    proof: Proof,
    inputs: Vec<String>
}

fn is_valid_hex(s: &str) -> bool {
    s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_proof(proof: &Proof) -> Result<(), String> {
    // Validate a
    for val in &proof.a {
        if !is_valid_hex(val) {
            return Err(format!("Invalid hex value in proof.a: {}", val));
        }
    }

    // Validate b
    for row in &proof.b {
        for val in row {
            if !is_valid_hex(val) {
                return Err(format!("Invalid hex value in proof.b: {}", val));
            }
        }
    }

    // Validate c
    for val in &proof.c {
        if !is_valid_hex(val) {
            return Err(format!("Invalid hex value in proof.c: {}", val));
        }
    }

    Ok(())
}

// Execute snarkjs verification
async fn verify_proof(Json(payload): Json<VerifyProofRequest>) -> Json<VerifyProofResponse> {
    // Validate the proof values
    if let Err(e) = validate_proof(&payload.proof) {
        return Json(VerifyProofResponse {
            valid: false,
            message: format!("Invalid proof format: {}", e),
        });
    }

    let proof = serde_json::to_string(&payload).unwrap();
    std::fs::write("proof.json", &proof).unwrap();


    // Run the verify.js script with a timeout
    let output = match Command::new("node")
        .arg("verify.js")
        .output() {
            Ok(output) => output,
            Err(e) => {
                let error_msg = match e.kind() {
                    ErrorKind::TimedOut => "Verification timed out after 30 seconds".to_string(),
                    _ => format!("Failed to execute verify.js: {}", e),
                };
                return Json(VerifyProofResponse {
                    valid: false,
                    message: error_msg,
                });
            }
        };

    // Check if the process was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Json(VerifyProofResponse {
            valid: false,
            message: format!("Verification failed: {}", error),
        });
    }

    // Parse the output
    let output = match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(e) => {
            return Json(VerifyProofResponse {
                valid: false,
                message: format!("Failed to parse verification output: {}", e),
            });
        }
    };

    // Check if the proof is valid
    let valid = output.contains("✅ Proof is valid!");
    Json(VerifyProofResponse {
        valid,
        message: output,
    })
}

#[tokio::main]
async fn main() {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/verify", post(verify_proof))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await  
    .unwrap();
}

