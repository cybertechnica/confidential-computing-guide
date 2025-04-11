# SEV-SNP internals

::: info 
The information detailed here is about SEV-SNP internals and how it works. 
It is based on the information done by researchers (referenced below).
We do not have the detailed internals from AMD (which means the source code and docs). 

Still this gives an more precise information of what we might expect. 
::: 

## AMD Secure Processor 
The ASP (also known as the Platform Secure Processor or PSP) is an ARM processor that runs independently from the main platform. 

*Why does this exists really?*

The ASP manages multiple security sensitive features (hence the important of it being seperated from the rest of the platform) :
- Secure Boot : Authentication of the initial BIOS boot code. 
- Manages the SEV-SNP guests life cycle : by managing the memory encryption keys.




## references
- [*AMD platform Security Processor*](https://en.wikipedia.org/wiki/AMD_Platform_Security_Processor)