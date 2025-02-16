::: warning Under-development 
This project is still under development, any [feedback and contribution](https://github.com/cybertechnica/confidential-computing-guide/issues) on this project would be helpful :)
:::
# Remote attestation in TPM

::: info Information
The example presented in this chapter relies on the tutorial from the [TPM2 community](https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html) and the explanation is summed up from the TPM2 community's articles. 

The remote attestation example is implemented with Rust, using the [rust-tss-esapi](https://github.com/parallaxsecond/rust-tss-esapi) which is a wrapper around the TSS 2.0 ES API.
:::

## Attestation workflow 

Remote attestation is the mechanism that is built to establish trust with a remote device through attestation. 


### What to attest 
By definition, remote attestation is a procedure to authenticate the hardware and software configuration of a host/machine. 


### What do we need - TPM provisioning and setup
Before implementation the attestation connection flow, we need to specify which keys, certificates and cryptographic algorithms we are going to use. 

TPMs usually comes with a ***Primary Endorsement Key (PEK)*** and a certificate generated for that key in a X.509 format, ***the EK cert*** (containing the public part of the PEK). 

> See the last chapter [*A tale of keys*](./5_tale_of_keys.md) for more information about how the keys are handled by the TPM and how are they securely used. 

One particular issue is that the EK is not a signing key. So we need another key to sign the quote that contains all the information about . This key is called the *Attestation Key* (or *Attestation Identity Key*) and is the one used to sign the quote and verify the remote attestation of the device. 

> The EK is not for signing for privacy reasons. The AK is an ephemeral key, while the EK can identify a device which is a risk for platform anonymity. 






### Protocol
Before a system can be attested the owner needs to obtain the public key of the TPM Vendor EK and OEM generated Attestation Key along with the appropriate certificates. 


```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

## Implementation
We'll be implementing the remote attestation using Rust. To do so, we'll be using the Rust wrapper from parallaxsecond [rust-tss-esapi](https://github.com/parallaxsecond/rust-tss-esapi) to communicate with the TPM. 

### Structures and operations functions definitions

#### Key algorithm structure 
We start by defining the asymmetric and hashing algorithm that we'll need for later : 
- `SHA384` as the hashing algorithm. 
- ECDSA NistP384 as the asymmetric algorithm. 
- ECDSA as the signing algorithm. 

We can define a structure containing all of this information and set it up with the precedent system :
```Rust
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
```

These structures will be used across the whole process of key creation, which means for the EK and the AK. 

#### Retrieving the necessary keys 
The first steps are to generate the EK (if not achieved yet), and to retrieve the EK handle for later use with the AK. Using the `tss_esapi` crate we define `get_ek_handle` function that returns a EK handle from a default template and null auth session: 
```Rust
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
``` 

We also define a function that returns the AK for a later use. This function needs the EK handle to generate another set of keys : 

```Rust
fn create_ak_alg(context: &mut Context, ek_handle: &KeyHandle, key_algo: &KeyAlgorithm) -> Result<CreateKeyResult> {
    Ok(create_ak(context, *ek_handle, key_algo.hash_alg, key_algo.key_alg, key_algo.sign_alg, None, None)?)
}
```

### Attestation Report : generating and signing the quote 

To be able to verify a platform, we need a document or a report that contains information about the authenticity and integrity of the platform. 

To do so, we need to measure the state of both the software and hardware stack. The Platform Configuration Registers (PCRs) do this stack by measuring each component starting with the firmware and UEFI. 

More details in [TPM features.](./4_tpm_features.md)

#### Retrieving the PCRs 
In pratice, after booting up a device, a trusted software component starts to measure each component from the UEFI/firmware and above. It is the first software that runs and is called the *root-of-trust for measurements*.  

::: info Quick details on the trusted component
The trusted component that starts pre-boot and measures all the component is a secure component that provides cryptographic functions that achieve these measurements. 
The core RoT (the truted component) has to follow certain requirements such as the code must be immutable (Read-only Memory). An example of the requirements can be found in [this document page 23](https://globalplatform.org/wp-content/uploads/2018/07/GP_RoT_Definitions_and_Requirements_v1.1_PublicRelease-2018-06-28.pdf).
:::




## References 
- [https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html](https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html)
- [https://lpc.events/event/4/contributions/295/attachments/374/608/What_does_Remote_Attestation_buy_you_.pdf](https://lpc.events/event/4/contributions/295/attachments/374/608/What_does_Remote_Attestation_buy_you_.pdf)