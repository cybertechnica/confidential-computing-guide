::: warning Under-development 
This project is still under development, any [feedback and contribution](https://github.com/cybertechnica/confidential-computing-guide/issues) on this project would be helpful :)
:::
# A tale of keys 
 
In this section we are going to talk about the different keys. The ones that can be used, the ones that must be protected, and the ones that are saved. 

## Key hierarchies

### before keys there is primary seeds 

TPMs does not store the root keys directly. Instead it uses secret values called primary seeds (large random numbers) that are persistent in the TPMs through all states (reboots...). And from those seeds we can deterministically regenerate the keys to identify the TPM for the same policies in place. 

::: info
A primary seed actually generates Primary Objects by also providing the parameters of the object to be created. Using these parameters and the primary seeds, we provide it to a Key Derivation Function to generate the hierarchy keys. 
:::

There is four different seeds that are associated with the hierarchies available : 
- Endorsement : Generates the Endorsement key and is the basis for Root of Trust for Reporting's identity. This hierarchy is basically the hierarchy for TPM identity. 
- Storage/Owner : 
- Platform : Used by the platform firmware and should not be available for user-end software (such as the OS and the applications). 
- Null 


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


[https://security.stackexchange.com/questions/235148/whats-the-difference-between-the-endorsement-key-and-the-attestation-identity-k](https://security.stackexchange.com/questions/235148/whats-the-difference-between-the-endorsement-key-and-the-attestation-identity-k)