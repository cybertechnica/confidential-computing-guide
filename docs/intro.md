---
sidebar_position: 1
---

# A first introduction to Confidential Computing 

## What is this all about? 

Data has to be protected in three different stages. When it's not used (at rest), when in transit (in transport) and when processed (in use). 

While we had a way to protect the data in transit for a while now with TLS and end-to-end encryption, but also data storage encryption (mostly using a KMS or the TPM-based application such as bitlocker), securing runtime execution was never a simple process. 

Protecting runtime execution is always a hustle. It's a combination of code coverage, dynamic analysis, virtualization and encryption. 
In a normal architecture, when we own the hardware we are working on (or trust it for that matter), it is acceptable to employ the well established protections, by hardening known operating systems, applying isolation and so on.
However, with other use-cases, from multi-party computation to cloud computing (if we suppose that there is a trust constraint that is not satisfied), is it crucial to be able to know exactly what is inside the remote server, that it is isolated and running the right computation that we expect. 

In a nutshell, Confidential Computing is a [Privacy-enhancing Technology](https://en.wikipedia.org/wiki/Privacy-enhancing_technologies) for protecting data in use. Combined with added network and storage encryption, it also protects data at rest (through Sealing) and data in transit (TLS connection attested with Remote attestation). 


## Trusted Computing
Trusted computing

## Trusted Execution Environments

