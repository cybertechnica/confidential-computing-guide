---
sidebar_position: 3
---
::: warning
This project is still under development, every feedback and contribution on this project would be helpful :)
:::
# TPM concepts

## Enrollment and provisioning 
When first used a TPM must be provisioned. Which means, it shoul

## How a TPM identifies itself 

Trust comes from the public-key related to a specific TPM. A digital certificate is created that includes the public part of the key being certified and additional attributes of that key. The certificate is then signed by a certificate authority (CA) key. 

The TPM, in his limited-resource capability, does not create nor consumes X.509 certificates. But it does store some X.509 certificates (according to the TCG). 

two certification processes : 
- The TPM vendor and platform manufacturer may provision the TPM with 

## PCRs 
Platform Configuration Registers (PCRs) are special TPM2 objects that can only be modified or written by hash extension mechanism. The new hash replaces the old one after each extension. PCRs are arranged in banks depending on the hashing algorithm used. Every bank has up to 32 PCRs values available. 

## MakeCredentials 
#### Privacy enablement in TPM

A privacy-related problem that was hard to solve is : providing a means to prove that a key was created and was protected by a TPM without the recipient of that proof knowing which was the creator and protector of the key. 

the Endorsement Key (EK) is used only as a decryption-only key, as opposed to a signing key, it can't be directly used to identify a particular TPM. 

Instead, a protocol is provided to make *Attestation Identity keys* (AIK)s which are pseudo-identity keys for the platform. 

EKs can be used to prove that an AIK originated with a TPM without proving which TPM the AIK originated from.  We can also have an unlimited number of AIKs for different uses, that can be destroyed after creation and use. 

Additionally, a protocol called [Direct Anonymous Attestation (DAA)](https://en.wikipedia.org/wiki/Direct_Anonymous_Attestation) for proving that a key was created by a TPM without providing information as to which TPM created it.  

### TPM 2.0 specifications

#### Enhanced Authorization 

#### Quick Key Loading 

#### Non-Brittle PCRs

### TPM Software Stack (TSS)

Levels of abstraction :
- *Feature API (FAPI)* : Used for most applications, exposes the API for TPM features. 
- *Enhanced System API (ESAPI)* : Needs a deeper knowledge of TPM, and provides session management and support for cryptographic operations. 
- *System API (SAPI)* : Requires much more TPM knowledge. It provides access to all the functionalities but requires a high level of knowledge. 
- *TPM Command Interface (TCTI)* : used for sending commands and receiving responses. 
- *TPM Access Broker (TAB)*
- *Resource Manager (RM)*
- *Device Driver* : Physical transmission of data to and from the TPM. 






#### How PCRs are generated


#### Information necessary for the attestation procedure

#### Remote attestation connection flow
### Security features
### Cryptographic attacks on TPMs

