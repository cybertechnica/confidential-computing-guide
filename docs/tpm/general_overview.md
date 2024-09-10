---
sidebar_position: 2
---

# General overview and security features

## Security concepts
#### Public-key certificate

Trust comes from the public-key related to a specific TPM. A digital certificate is created that includes the public part of the key being certified and additional attributes of that key. The certificate is then signed by a certificate authority (CA) key. 

The TPM, in his limited-resource capability, does not create nor consumes X.509 certificates. But it does store some X.509 certificates (according to the TCG). 

two certification processes : 
- The TPM vendor and platform manufacturer may provision the TPM with 

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


### A tale of keys 
In this section we are going to talk about the different keys. The ones that can be used, the ones that must be protected, and the ones that are saved. 
#### Key provisioning 
Two-types of provisioning for the TPM root-of-trust : 
- Attestation and identities -> initial 
- part of the device onboarding and deployment -> local 

A TPM pre-provisioned with only an EK may provision "Initial" keys at different stages. 
TPMs supports provisioning that happens over different stages. 

***Primary endorsement key (PEK) :*** 
TPMs vendors provision the TPM device with a Primary Endorsement Key (PEK) and generate a certificate for that key (EK cert) in X.509 format at the time of manufacture. 
The EK cert contains the public part of the PEK And other information such as TPM manufacturer name, part model, part version, key id, etc. 
This information is used to uniquely identify the TPM and if the device OEM (Original Equipment Manufacturer)  securely attaches a TPM to the device it can be used as a device identifier. 
The EK certificate may be stored in the TPM Non Volatile Memory (NVM), where it can be made available to client software or directly from the OEM's website. 
The EK certificate is signed (issued) by the TPM vendor's CA and verifiers are assumed to trust the vendor's root certificate. 
The private of the EK is expected to never be exposed outside of the TPM. 

***Endorsement Keys (EKs) :***
Since TPM 2.0, we can have more than one Endorsement Key (EK). The algorithm flexibility allows the TPM to create EKs of any type asymmetric algorithm implemented in the TPM. 

The properties of the Endorsement Key are expressed by its public area template. Creating two EK with the same template will result in the same key. The EK and its credential may be considered **privacy-sensitive** if the private part of the EK is used in a cryptographic protocol. the EK **Should not be** used as a **signing key** for privacy-sensitive purposes.  

[https://trustedcomputinggroup.org/wp-content/uploads/TCG_IWG_Credential_Profile_EK_V2.1_R13.pdf](https://trustedcomputinggroup.org/wp-content/uploads/TCG_IWG_Credential_Profile_EK_V2.1_R13.pdf)

***Primary Attestation Key (PAK) :***

***Initial Attestation Key (IAK) :***
The OEM may also generate Initial Attestation Key (IAK) and IAK certificate

***Primary Storage Key (PSK) :***

### TPM Remote attestation
[https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html](https://tpm2-software.github.io/2020/06/12/Remote-Attestation-With-tpm2-tools.html)
Remote attestation is the mechanism that is built to establish trust with a remote device through attestation. 
Platform anonymity. 
#### Attestation Procedure
Before a system can be attested the owner needs to obtain the public key of the TPM Vendor EK and OEM generated Attestation Key along with the appropriate certificates. 

#### How PCRs are generated
Platform Configuration Registers (PCRs) are special TPM2 objects that can only be modified or written by hash extension mechanism. The new hash replaces the old one after each extension. PCRs are arranged in banks depending on the hashing algorithm used. Every bank has up to 32 PCRs values available. 

#### Information necessary for the attestation procedure

#### Remote attestation connection flow
### Security features
### Cryptographic attacks on TPMs

## NV Indexes

For Non-Volatile indexes. 
In the TPM we can use the non-volatile memory for two different classes of data : 
- Data Structures defined by the TPM 
- Unstructured data defined by a user or a platform specific spec. 

presentation : 
https://static.sched.com/hosted_files/osseu2020/26/Oct_29_Introducing%20TPM%20NV%20Storage%20with%20EA%20Policies%20and%20TSS-FAPI_Andreas%20Fuchs.pdf


## References
https://cloud.google.com/blog/products/identity-security/virtual-trusted-platform-module-for-shielded-vms-security-in-plaintext?hl=en
https://link.springer.com/book/10.1007/978-1-4302-6584-9