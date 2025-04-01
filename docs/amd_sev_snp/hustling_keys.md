## A bunch of keys 

ARK (AMD Root Key)  : RSA key that acts as the root certificate authority for VCEK certificates.  
ASK (AMD SEV Key)   : The intermediate certificate authority key that signs VCEK certificates. Specific to each product (and thus each processor).
VCEK (Versioned Chip Endorsement Key)   : The certificate related to this key is the leaf certificate. 

