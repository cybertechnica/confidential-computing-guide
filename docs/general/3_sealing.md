::: warning Under-development 
This project is still under development, any [feedback and contribution](https://github.com/cybertechnica/confidential-computing-guide/issues) on this project would be helpful :)
:::
# Sealing

Sealing is the way of securely store a state on the secure environment for later use. 


## Encrypting a disk

::: info
> This is a high-level explanation and not a very detailed one, but gives an idea how some tools like bitlocker works. 
::: 
Data protection is usually done by disk encryption. In Linux for example, this is usually done using cryptsetup+LUKS. LUKS specifically uses a passphrase by default to derive keys to encrypt the disk. 

However using a passphrase is not practical for different reasons : 
- Lack of entropy.  
- The need to remember the passphrase. 

So instead of a passphrase, we can use a secure processor to encrypt the data, such as a TPM. By using the TPM, we guarantee the encryption of the disk with a key derived from a location only accessible under certain circumstances. ***And this is what Bitlocker does.*** 

## Sealing a secure enclave 

When using a TEE, we might want to save the state of the enclave (or CVM) that we are using, or launch a VM that is in an encrypted state.

Different ways are implemented to do that, depending on the TEE that is used. 
