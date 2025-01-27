---
sidebar_position: 2
---
::: warning
This project is still under development, every feedback and contribution on this project would be helpful :)
:::
# Attestation

As explained before, attestation is one of the core aspects of Confidential Computing. 

In this section we are going to detail the theory behind while explaining the security protocols and communications that comes into place. 

> For more detailed and practical use-cases for each TEE implementation, see the following chapters depending on each TEE. 

## Local Attestation vs Remote Attestation

An Enclave or a Confidential VM has two ways of attesting itself. 

Local attestation can be used to attest an enclave locally on the same machine. This can be used to attest an enclave against another one on the same machine.  

Remote attestation is used to attest a enclave remotely. A simple example is to verify a machine from a remote user. 

<!-- ## Types of remote Attestation implementations -->

