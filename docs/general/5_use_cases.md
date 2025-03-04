::: warning Under-development 
This project is still under development, any [feedback and contribution](https://github.com/cybertechnica/confidential-computing-guide/issues) on this project would be helpful :)
:::
# Uses-cases and applications

> The similarity among all these use-cases is the complexity of removing trust from the host system to a particular application. 

## Confidential AI 
In the era of AI boom, it becames primordial to be able to protect the AI workloads in every stage, which basically includes, training, fine-tuning and inference. 

Confidential computing can resolve this issue by running all the AI workload in a Confidential environment. 

Another complexity that can be resolved by confidential computing is [decentralized AI](https://medium.com/secret-network-ecosystem-and-technology/why-confidential-computing-is-inevitable-for-decentralized-ai-42d489ab028a), if we manage to run enclaves efficiently in a distributed manner that said. 


## Key management services

[Key management services](https://en.wikipedia.org/wiki/Key_management#:~:text=Key%20management%20refers%20to%20management,procedures%2C%20and%20other%20relevant%20protocols.) are important components that manages the numerous keys and certificates in a SaaS product or at an entreprise level. It is thus, one of the most important software to secure. 

TEEs (but also TPMs and HSMs/Hardware Security Modules) help on different KMS operations and necessary management steps such as exchange, storage and use. 

## Secure messaging and end-to-end encryption : Signal use-case
Signal uses SGX for some system used, such as [contact discovery service](https://github.com/signalapp/ContactDiscoveryService-Icelake). 


## Blockchain, smart-contracts and EVMs

Many blockchain companies started using TEEs to protect their users' confidentiality. [Secret Network](https://phala.network/), [Phala](https://phala.network/) and [Oasis](https://oasisprotocol.org/) are just examples among many others. 

## Multi-Party Computation (MPC)
::: info
To be detailed. 
:::

## Secure financial transactions

It appears that also many banking infrasctructure use TEE hardware to secure transaction. They usually use SGX to have a hardware secure enclave to isolate the transaction code that is running. 






