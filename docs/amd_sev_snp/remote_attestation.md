# Remote attestation in AMD SEV-SNP 


## Trusted Computing Base (TCB) in AMD SEV-SNP
The TCB in AMD SEV-SNP confidential VMs is restricted to the AMD hardware and firmware, and the Confidential VM itself. 

## Example of a standard attestation in AMD SEV-SNP

### Guest VM 

Supply a nonce : 
```rust
// This is a 512 bits array that is signed and included in the attestation report. 
// Its purpose could be to add a hash of a public key that would later used or other important 
// data that needs to be certified by the attestation report 
let user_data: [u8; 64];    
```



## references 
- [https://www.amd.com/content/dam/amd/en/documents/developer/lss-snp-attestation.pdf](https://www.amd.com/content/dam/amd/en/documents/developer/lss-snp-attestation.pdf)
- [https://www.amd.com/content/dam/amd/en/documents/developer/58217-epyc-9004-ug-platform-attestation-using-virtee-snp.pdf](https://www.amd.com/content/dam/amd/en/documents/developer/58217-epyc-9004-ug-platform-attestation-using-virtee-snp.pdf)
- [https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/specifications/57230.pdf](https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/specifications/57230.pdf)