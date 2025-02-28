# Advanced TPM : understanding the TPM low-level commands and features


## sessions 

## authorizations

## TPM command/response structures

TPM command is a protected capability that is performed by the TPM. 

it has five components :
- The command header that includes :
    - the size of the command
    - the command code 
    - a tag indicating if the Authorization Area is present
