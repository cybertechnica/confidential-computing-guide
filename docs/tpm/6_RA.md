---
sidebar_position: 6
---
::: warning
This project is still under development, every feedback and contribution on this project would be helpful :)
:::
# Remote attestation in TPM

The example presented in this chapter relies on the tutorial from the [TPM2 community](https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html) and the explanation is summed up from the TPM2 community's articles. 

The remote attestation example is implemented with Rust, using the [rust-tss-esapi](https://github.com/parallaxsecond/rust-tss-esapi) which is a wrapper around the TSS 2.0 ES API.


## Attestation workflow 

Remote attestation is the mechanism that is built to establish trust with a remote device through attestation. 
Platform anonymity. 

### What to attest 
By definition, remote attestation is a procedure to authenticate the hardware and software configuration of a host/machine. 



### What do we need - TPM provisioning and setup
Before implementation the attestation connection flow, we need to specify which keys, certificates and cryptographic algorithms we are going to use. 

TPMs usually comes with a ***Primary Endorsement Key (PEK)*** and a certificate generated for that key in a X.509 format, ***the EK cert*** (containing the public part of the PEK). 

> See the last chapter [*A tale of keys*](./5_tale_of_keys.md) for more information about how the keys are handled by the TPM and how are they securely used. 





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

### Defining the structure and functions that we need

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
fn setup_key_algP384() -> Result<KeyAlgorithm> {
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





## References 
- [https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html](https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html)
- [https://lpc.events/event/4/contributions/295/attachments/374/608/What_does_Remote_Attestation_buy_you_.pdf](https://lpc.events/event/4/contributions/295/attachments/374/608/What_does_Remote_Attestation_buy_you_.pdf)