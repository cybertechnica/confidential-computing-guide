::: warning
This project is still under development, every feedback and contribution on this project would be helpful :)
:::

# Comparing the different trusted Execution Environements 

## The different properties taken into account 

Each TEE from hardware manufacturers might have different properties. This section details what will be compared. 


 
- Isolation technology (confidentiality) : TEEs use different methods that preserves isolation. This property defines the type; either virtualization, hardware isolation or other. 
- Attestation and sealing capabilities (integrity) : Remote attestation and sealing are the backbone of confidential computing, and not all TEEs supports it. 
- Development resources (Ease of use, development and update) : 
- Overhead and performance 
- Limitations 



## Comparison table 

> TPMs are included in this comparison although they are not a TEE. The reason for that is vTPMs are used in Confidential VMs (such as for AMD SEV-SNP) to establish remote attestation. More on this on the AMD SEV-SNP chapter. 

> Another reason, is that we can have a Hypervisor-based TEE using vTPMs. You can find more details on the TPM chapter. 

| Capabilities | Trusted Platform Modules (TPMs) | AMD SEV-SNP | Intel TDX | Intel SGX | Nvidia H100 | ARM CCA | RISC-V Keystone | 
| ----------- | ----------- | ----------- |----------- | ----------- | ----------- | ----------- | ----------- |
| ***Attestation*** | :white_check_mark:  | :white_check_mark: | :white_check_mark: | :white_check_mark:| :white_check_mark:| :white_check_mark: | :white_check_mark:
| ***Sealing*** | :white_check_mark:        |
| ***Isolation Technology*** | :x: | Virtualization | Virtualization | Hardware | :grey_question: | :grey_question: | :grey_question: | 
| ***Overhead and performance*** | 
| ***Security and update complexity*** | 
| ***Development resources*** | 
| ***Use-cases*** |
| ***Limitations*** | 