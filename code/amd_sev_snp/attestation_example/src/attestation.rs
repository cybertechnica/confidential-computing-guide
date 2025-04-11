/// To be ran in the Guest VM 
/// 
use sev::{
    firmware::guest::*, 
    certs::snp::{Chain, ca}
};
use openssl::{
    ecdsa::EcdsaSig,
    sha::{sha512, Sha384},
};
use anyhow::{Ok, Result};

/// Attestation report request generation
/// 
/// Args:
///     nonce ([u8; 64]):
///     vmpl (Option<u32>):
/// 
/// Returns: 
///     AttestationReport as a Result
pub fn generate_attestation(nonce: [u8; 64], vmpl: Option<u32>) -> Result<AttestationReport> {    
    let mut fw: Firmware = Firmware::open()?;

    let attestation_report = AttestationReport::from_bytes(
        &fw.get_report(None, Some(nonce), vmpl)?
    )?;

    Ok(attestation_report)
}


pub enum ProductName {
    Naples,
    Rome,
    Milan, 
    Genoa,
    Turin
}
/// Validating the RoT for verifying the signature chain 
///     - The AMD Root Key (ARK) - Self-signed
///     - The AMD Signing Key (ASK) - Signed by the ARK
///     - Versioned Chip Endorsement Key (VCEK) - Signed by the ASK
pub fn verify_certificate_chain() {

}
