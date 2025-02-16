::: warning Under-development 
This project is still under development, any [feedback and contribution](https://github.com/cybertechnica/confidential-computing-guide/issues) on this project would be helpful :)
:::
# Security Features and implementations

In this chapter we will discuss in more detail the security features that are implemented by a confidential computing environment. 
This part presents a first definition for ***attestation*** and ***sealing***, ***how memory and processes are supposed to be protected*** and ***how cryptography is implemented in confidential environments***. 

## Cryptographic operations 
The most important capabilities of confidential computing is to be able to create keys, store them securely, generate certificates and so on. 

But if you are familiar with cryptography, you will know that some conditions has to met to build systems securely. One of them is how randomness is achieved. 

## Isolation
Either physical (**SGX**) or virtualized (**SEV-SNP**), isolation is a crucial feature of confidential computing. 

Usually, when we talk about isolation, we might refer to an isolated environment that can be seperated from the rest of the network and other software. Virtualized environments are an example of isolated environment. But even though, normal virtualized environment still depend on the host/hypervisor. Which also means that the memory is also managed by the hypervisor for example. 

The following table represents the set of the components that could be in place when running multiple applications : 
<div style="margin-left: auto;
            margin-right: auto;
            width: 60%">
<table style="text-align: center;">
    <tr>
        <td colspan="3">Trusted component stack</td>
    </tr>
    <tr>
        <td>Application 1</td>
        <td>Application k</td>
        <td>Application n</td>
    </tr>
    <tr>
        <td>VM 1</td>
        <td colspan="2">VM 2</td>
    </tr>
    <tr>
        <td colspan="3">Hypervisor</td>
    </tr>
    <tr>
        <td colspan="3">Firmware</td>
    </tr>
    <tr>
        <td colspan="3">Physical hardware</td>
    </tr>
</table>
</div>

While each VM (Thus there is no interaction between the different applications on different VMs) is isolated, the hypervisor still have access to both VMs.  

***A confidential computing environment, in the other hand, should also be isolated from the hypervisor and maybe also other low-level components.*** 




## Attestation 

Also known as cryptographic attestation, aims to ensures the integrity and authenticity of data, software or hardware components. Simply put, the attestation is a digital signature for a set of measurements securely stored in hardware that can be validated by a requester. 

See the [attestation part](./2_attestation.md) to know more about the different models of attestation and implementations.



## Sealing 

Sealing is defined as a way to protect persistent and stored data.  






## References 

*NIST, Attestation definition* - [https://csrc.nist.gov/glossary/term/attestation#:~:text=Definitions%3A,and%20the%20set%20of%20measurements](https://csrc.nist.gov/glossary/term/attestation#:~:text=Definitions%3A,and%20the%20set%20of%20measurements).