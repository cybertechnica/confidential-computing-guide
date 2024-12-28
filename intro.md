---
sidebar_position: 1
---

# A first introduction to Confidential Computing 

## What is this all about? 

Data has to be protected in three different stages. When it's not used (**at rest**), when in transit (**in transport**) and when processed (**in use**). 

While we had a way to protect the data in transit for a while now with TLS and end-to-end encryption, but also data storage encryption (mostly using a KMS or the TPM-based application such as bitlocker), securing runtime execution was never a simple process. 

Protecting runtime execution is always a hustle. It's a combination of code coverage, dynamic analysis, virtualization and encryption. 
In a normal architecture, when we own the hardware we are working on (or trust it for that matter), it is acceptable to employ the well established protections, by hardening known operating systems, applying isolation and so on.
However, with other use-cases, from multi-party computation to cloud computing (if we suppose that there is a trust constraint that is not satisfied), is it crucial to be able to know exactly what is inside the remote server, that it is isolated and running the right computation that we expect. 

In a nutshell, Confidential Computing is a [Privacy-enhancing Technology](https://en.wikipedia.org/wiki/Privacy-enhancing_technologies) for protecting data in use. Combined with added network and storage encryption, it also protects data at rest (through Sealing) and data in transit (TLS connection attested with Remote attestation). 


## Trusted Computing
Before talking about Confidential Computing, let's review trusted computing first. 

Largely initialized and developed by the [Trusting Computing Group](https://trustedcomputinggroup.org/), Truted Platform Modules (famously known as TPMs) are the backbone of trusted computing.

One thing to note is that trusted computing is different from Confidential Computing. 

### TPMs

TPMs can be defined as *secure processors*. While they don't have a isolated and protected environment to launch applications such as TEEs, they can be used to verify the integrity of the platform, encrypt disk, manage secret keys, etc...  

### Why include TPMs in Confidential Computing

Well, for the fact that some TEEs (e.g. AMD SEV-SNP) might use vTPMs (virtual TPMs - explained on the TPMs section) for ***remote attestation and measurements***. 

## Trusted Execution Environments (TEEs)

Trusted Execution Environments (TEEs), sometimes referred to as enclaves, are special environments that physically or virtually isolated, and where the runtime execution is encrypted (which means that the memory layout could be encrypted and not readable by the host even at a higher ring privileges). 


### The notion of Trusted Computing Base (TCB)

The [Truted Computing Base (TCB)](https://en.wikipedia.org/wiki/Trusted_computing_base) defines the components that needs to be defined as trusted. It is a security design requirement. 


As an example, for an application in a normal environment, the TCB includes *the hardware, firmware, hypervisor, operating system, devices, software... and then the application itself*. 

The TCB for TEEs is by design reduced to the minimal possible. SGX for example, the TCB will be the hardware and the enclave itself. 


### Types of TEEs implementations
There is multiple TEEs implementations. 

Intel has two different ones SGX (hardware-based protection) and TDX (Virtualized). 
AMD, only has a virtualized implementation (similar to TDX) called AMD SEV-SNP. 
ARM also has ARM CCA which is related to ARM TrustZone protection. 


There is other TEEs implementations by IBM, RISC-V and others which will not be discussed here (at least for now). 

