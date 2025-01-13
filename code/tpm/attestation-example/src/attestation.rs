use tss_esapi::{
    Context,
    interface_types::{
        algorithm::{HashingAlgorithm, PublicAlgorithm, SignatureSchemeAlgorithm},
        ecc::EccCurve,
        reserved_handles::Hierarchy,
        session_handles::PolicySession,
    },
    structures::{PcrSelectionListBuilder, PcrSlot, HashScheme, Public, CreateKeyResult},
    abstraction::{
        pcr::{PcrData},
        ak::{createak, load_ak}, 
        ek::{create_ek_public_from_default_template, retrieve_ek_pubcert},
        AsymmetricAlgorithmSelection,
    }, 
    handles::NvIndexHandle

};

struct KeyAlgorithm {
    key_alg: AsymmetricAlgorithmSelection, 
    hash_alg: HashingAlgorithm, 
    sign_alg: SignatureSchemeAlgorithm, 
    sign_scheme: SignatureScheme
}

/// Setup Key Algorithm to be used to for the Endorsement Key and the AIK 
/// ECDSA P384 as the Asymmetric Algo 
/// and SHA384 as the hashing function
fn setup_key_algP384() -> Result<KeyAlgo> {
    KeyAlgorithm {
        key_alg: AsymmetricAlgorithmSelection::Ecc(EccCurve::NistP384), 
        hash_alg: HashingAlgorithm::Sha384, 
        sign_alg: SignatureSchemeAlgorithm::EcDsa, 
        sign_scheme: SignatureScheme::EcDsa {
            scheme: HashScheme::new(hash_alg),
        }
    }
}

fn get_ek_handle(context: &mut Context, key_algo: KeyAlgorithm) -> Result<NvIndexHandle> {
    // We generate the default template to be used
    let ek_template = create_ek_public_from_default_template(key_algo.key_alg, None).unwrap();
    // We fetch the EK handle with our default template
    let ek_handle = context.execute_with_nullauth_session(|ctx| {
        ctx.create_primary(Hierarchy::Endorsement, ek_template, None, None, None, None)
    })
    .expect("Failed to load ek_template")
    .key_handle;

    Ok(key_handle)
}

fn get_ek_pub(context: &mut Context, ek_handle: NvIndexHandle, key_algo: KeyAlgorithm) -> Result<Public> {

    // Retrieving the public part from the EK 
    let (ek_public, _name, _qualified_name) = context
        .read_public(ek_handle)
        .expect("Failed to read ek_public");
    
    Ok(ek_public)
}

/// Deriving the Attestation Identity Key for RA purposes 
fn create_ak(context: &mut Context, ek_handle: NvIndexHandle, key_algo: KeyAlgorithm) -> Result<CreateKeyResult> {
    Ok(
        create_ak(context, ek_handle, key_algo.hash_alg, key_algo.key_alg, key_algo.sign_alg, None, None)
    )
}

fn get_pcrs_bank(context: Context) -> Result<PcrData> {

}