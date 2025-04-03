# Remote attestation in AMD SEV-SNP 

AMD SEV-SNP has two configurations for remote attestation. Standard and extended. 
- Standard configuration will request the certificate chain and VCEK certificate directly from AMD's KDS. 
- In extended configuration you will have to configure the host to store the ASK, ARK and VCEK (on the PSP?). Then we can request those certificates from the guest via the PSP. 



## Trusted Computing Base (TCB) in AMD SEV-SNP
The TCB in AMD SEV-SNP confidential VMs is restricted to the AMD hardware and firmware, and the Confidential VM itself. 

## Attestation flow 

## Example of a standard attestation in AMD SEV-SNP

### quick overview on the software used

The interface used to communicate with the AMD SEV-SNP hardware is from the [virTEE](https://virtee.io/) project. 

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