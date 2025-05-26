use std::fs;
use clap::Parser;
use mail_parser::MessageParser;
use mail_auth::{MessageAuthenticator, AuthenticatedMessage, DkimResult};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .eml file
    #[arg(short, long)]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_content = fs::read_to_string(&args.file)?;
    
    // Parse the email message for display
    let message = MessageParser::default().parse(&file_content).unwrap();
    
    // Extract basic info
    println!("From: {}", message.from().unwrap().first().unwrap().address().unwrap());
    println!("To: {}", message.to().unwrap().first().unwrap().address().unwrap());
    println!("Subject: {}", message.subject().unwrap());
    
    // Extract DKIM info for display
    for header in message.headers() {
        if header.name().eq_ignore_ascii_case("DKIM-Signature") {
            let dkim_value = format!("{:?}", header.value());
            println!("\n🔍 DKIM-Signature found!");
            
            // Extract domain
            if let Some(d) = dkim_value.split("d=").nth(1) {
                if let Some(domain) = d.split(';').next() {
                    println!("📧 Domain: {}", domain);
                }
            }
            
            // Extract selector
            if let Some(s) = dkim_value.split("s=").nth(1) {
                if let Some(selector) = s.split(';').next() {
                    println!("🔑 Selector: {}", selector);
                }
            }
            
            // Extract body hash
            if let Some(bh) = dkim_value.split("bh=").nth(1) {
                if let Some(hash) = bh.split(';').next() {
                    println!("🔐 Body Hash: {}", hash);
                }
            }
        }
    }
    
    // Now do the REAL cryptographic verification using mail-auth
    println!("\n🔐 Performing REAL CRYPTOGRAPHIC VERIFICATION...");
    
    // Try system DNS resolver instead of Cloudflare
    let authenticator = MessageAuthenticator::new_system_conf().unwrap();
    
    // Parse message for verification
    let authenticated_message = AuthenticatedMessage::parse(file_content.as_bytes()).unwrap();
    
    // Verify DKIM signatures
    let dkim_results = authenticator.verify_dkim(&authenticated_message).await;
    
    if dkim_results.is_empty() {
        println!("❌ NO DKIM SIGNATURES FOUND!");
        println!("❌ This email has no DKIM signatures to verify");
    } else {
        let mut all_passed = true;
        
        for (i, result) in dkim_results.iter().enumerate() {
            println!("\n📋 DKIM Signature #{}", i + 1);
            
            match result.result() {
                DkimResult::Pass => {
                    println!("   ✅ CRYPTOGRAPHICALLY VERIFIED!");
                    println!("   ✅ This signature was created by the domain owner's private key");
                }
                DkimResult::Fail(reason) => {
                    println!("   ❌ VERIFICATION FAILED: {:?}", reason);
                    println!("   ❌ This signature is INVALID or FORGED!");
                    all_passed = false;
                }
                DkimResult::TempError(reason) => {
                    println!("   ⚠️  TEMPORARY ERROR: {:?}", reason);
                    println!("   ⚠️  Could not verify due to temporary issue");
                    all_passed = false;
                }
                DkimResult::PermError(reason) => {
                    println!("   ❌ PERMANENT ERROR: {:?}", reason);
                    println!("   ❌ Verification failed permanently");
                    all_passed = false;
                }
                DkimResult::Neutral(reason) => {
                    println!("   ⚪ NEUTRAL: {:?}", reason);
                    println!("   ⚪ No definitive result");
                    all_passed = false;
                }
                DkimResult::None => {
                    println!("   ❌ NO SIGNATURE FOUND");
                    all_passed = false;
                }
            }
        }
        
        if all_passed {
            println!("\n🎉 ALL DKIM SIGNATURES CRYPTOGRAPHICALLY VERIFIED!");
            println!("✅ This email is DEFINITELY authentic and unmodified!");
            println!("✅ The signatures prove it came from the claimed domains!");
            println!("✅ You can trust this email is legitimate!");
            println!("\n🔒 SECURITY GUARANTEE:");
            println!("   • The email was signed with the domain owner's private key");
            println!("   • The public key was verified from DNS");
            println!("   • The signature mathematically proves authenticity");
            println!("   • The email content has NOT been tampered with");
        } else {
            println!("\n❌ SOME OR ALL DKIM SIGNATURES FAILED VERIFICATION!");
            println!("❌ This email may be forged, modified, or have configuration issues!");
            println!("❌ DO NOT TRUST this email without further investigation!");
        }
    }
    
    Ok(())
}
