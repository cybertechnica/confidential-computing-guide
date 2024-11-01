---
sidebar_position: 5
---

# Uses-cases and applications

Where all these use-cases collides is removing trust from the host system to a particular application. 

## Confidential AI 
In the era of AI boom, it becames primordial to be able to protect the AI workloads in every stage, which basically includes, training, fine-tuning and inference. 

Confidential computing can resolve this issue by running all the AI workload in a Confidential environment. 

Another complexity that can be resolved by confidential computing is [decentralized AI](https://medium.com/secret-network-ecosystem-and-technology/why-confidential-computing-is-inevitable-for-decentralized-ai-42d489ab028a), if we manage to run enclaves efficiently in a distributed manner that said. 


## Key management services

Key management services 
## Identity and device authentication
Another usage for remote attesstation could be device authentication before user authentication.

Authentication to a remote machine usually utilizes an authentication protocol (Public-key authentication, Kerberos, LDAP, OAuth2 or others). Usually these types of authentication protocols depends heavily on only verifying the user's identity. Thus, if an attacker gets his hand on the user credentials, the server might give access to the resources without any more checks.

Adding device authentication could help resolve this issue. Maybe even for lateral movement inside the same network. Each device will have to send a report for the attestation on top of the authentication mechanism. That way, the server could verify the attestation and establish a remote attestation communication channel before establishing the user authentication.




