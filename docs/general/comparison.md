::: warning Under-development 
This project is still under development, any [feedback and contribution](https://github.com/cybertechnica/confidential-computing-guide/issues) on this project would be helpful :)
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
| ***Sealing*** | :white_check_mark:        | :white_check_mark: | :white_check_mark: | :white_check_mark:| :grey_question: | :grey_question: | :grey_question:| 
| ***Isolation Technology*** | :x: | Virtualization | Virtualization | Hardware | :grey_question: | :grey_question: | :grey_question: | 
| ***Overhead and performance*** | mostly signing overhead | Overhead when encrypting/Decrypting RAM | Overhead when encrypting/Decrypting RAM | Depending on the workload/SGX is bound by physical performance of available SGX tables. | Crypto functions overhead. | No information | No information
| ***Security and update complexity*** | Depending on the vendor and hardware implementation, software is reviewed continuously. Udpating the software is feasable but not the hardware that comes with it | Continuously reviewed. Firmware udpates are quick. |  Continuously reviewed. Firmware udpates are quick. | Updating following a high-severity vulnerability is quite tricky sometimes, and might be impossible depending on the implementation | Firmware updates are available and quickly implemented (driver update). Software is continuously reviewed. The firmware is new and might still unexplored for security purposes.  | No information | :grey_question: |
| ***Development & resources*** | Lots of features and ressources according to each one. Might take time to understand some concepts. Development and usage is fairly simple (Lot of libraries in different programming languages) | Fairly easy to use. Installation might be tricky. | Fairly easy to use. Installation might be tricky. | There is some difficulty to implement basic functionalities depending on the programming language used. Installation and updates are troublesome. | Ease of use and installation. Lack of detailed ressources. | :grey_question: | :grey_question:
| ***Current use-cases*** | Bitlocker, DRM... | Confidential AI | | Signal Contact discovery service, Crypto exchanges ... | Confidential AI | DRM ? | DRM ? | 
| ***Limitations*** | Small processor that has a lot of features but is slow | | | Scalability | Does not work without a Confidential VM | | | 