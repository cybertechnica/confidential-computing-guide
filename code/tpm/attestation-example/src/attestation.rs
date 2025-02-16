use tss_esapi::{
    Context,
    interface_types::{
        algorithm::{AsymmetricAlgorithm, HashingAlgorithm, PublicAlgorithm, SignatureSchemeAlgorithm},
        ecc::EccCurve,
        resource_handles::Hierarchy,
        session_handles::PolicySession,
    },
    structures::{
        PcrSelectionListBuilder, 
        PcrSlot, 
        HashScheme, 
        SignatureScheme, 
        Public, 
        CreateKeyResult
    },
    abstraction::{
        pcr::PcrData,
        ak::{
            create_ak_2 as create_ak, 
            load_ak
            }, 
        ek::{
            create_ek_public_from_default_template_2 as create_ek_public_from_default_template, 
            retrieve_ek_pubcert
            },
        AsymmetricAlgorithmSelection,
    }, 
    handles::KeyHandle
};

use anyhow::Result;

#[derive(Debug)]
struct KeyAlgorithm {
    key_alg: AsymmetricAlgorithmSelection, 
    hash_alg: HashingAlgorithm, 
    sign_alg: SignatureSchemeAlgorithm, 
    sign_scheme: SignatureScheme
}

/// Setup Key Algorithm to be used to for the Endorsement Key and the AIK 
/// ECDSA P384 as the Asymmetric Algo 
/// and SHA384 as the hashing function
fn setup_key_alg_p384() -> Result<KeyAlgorithm> {
    Ok(KeyAlgorithm {
        key_alg: AsymmetricAlgorithmSelection::Ecc(EccCurve::NistP384), 
        hash_alg: HashingAlgorithm::Sha384, 
        sign_alg: SignatureSchemeAlgorithm::EcDsa, 
        sign_scheme: SignatureScheme::EcDsa {
            hash_scheme: HashScheme::new(HashingAlgorithm::Sha384),
        }
    })
}

fn get_ek_handle(context: &mut Context, key_algo: &KeyAlgorithm) -> Result<KeyHandle> {
    // We generate the default template to be used
    let ek_template = create_ek_public_from_default_template(key_algo.key_alg, None).unwrap();
    // We fetch the EK handle with our default template
    let ek_handle = context.execute_with_nullauth_session(|ctx| {
        ctx.create_primary(Hierarchy::Endorsement, ek_template, None, None, None, None)
    })
    .expect("Failed to load ek_template")
    .key_handle;

    Ok(ek_handle)
}

fn get_ek_pub(context: &mut Context, ek_handle: &KeyHandle) -> Result<Public> {
    // Retrieving the public part from the EK 
    let (ek_public, _name, _qualified_name) = context
        .read_public(*ek_handle)
        .expect("Failed to read ek_public");
    Ok(ek_public)
}

/// Deriving the Attestation Identity Key under the Endorsement hierarchy for RA purposes 
fn create_ak_alg(context: &mut Context, ek_handle: &KeyHandle, key_algo: &KeyAlgorithm) -> Result<CreateKeyResult> {
    Ok(create_ak(context, *ek_handle, key_algo.hash_alg, key_algo.key_alg, key_algo.sign_alg, None, None)?)
}

// fn get_pcrs_bank(context: Context) -> Result<PcrData> {

// }

/// Remote attestation Implementation
pub fn run_attestation(context: &mut Context) -> Result<()> {
    // Setting up the Key Algorithm to be used 
    let key_algo = setup_key_alg_p384()?;
    println!("Key algorithm info : {:?}", key_algo);
    // Fetching the EK handle 
    let ek_handle = get_ek_handle(context, &key_algo)?;
    println!("{:?}", ek_handle);
    // // Fetching the EK public part 
    let ek_public = get_ek_pub(context, &ek_handle)?;
    println!("Retrieving the EK public key : {:?}", ek_public);
    // // Creating the Attestation Identity Key 
    let ak_result = create_ak_alg(context, &ek_handle, &key_algo);
    println!("Generating the AK.");
    println!("the AK public key is : {:?}", ak_result.unwrap().out_public.clone());

    // Readign the PCR list and store it for later use
    let pcr_selection_list = PcrSelectionListBuilder::new()
        .with_selection(
            HashingAlgorithm::Sha256, 
            &[PcrSlot::Slot0, PcrSlot::Slot1, PcrSlot::Slot2, PcrSlot::Slot3, PcrSlot::Slot4, PcrSlot::Slot5, PcrSlot::Slot6, PcrSlot::Slot7, PcrSlot::Slot8, PcrSlot::Slot9, PcrSlot::Slot10]
        )
        .build()
        .expect("Failed to build PcrSelectionList.");
    let (update_counter, read_pcr_list, digest_list) = context.pcr_read(pcr_selection_list)
        .expect("Call to pcr_read failed");
    println!("PCR list is : {:?}", digest_list);
    // Establishing the connection and creating the nonce 
    println!("Generating and signing the quote with the AK.");
    println!("Zero nonce used as data for the example.");
    let nonce = vec![0x00; 16];
    Ok(())
}