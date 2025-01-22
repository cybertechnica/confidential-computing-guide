---
sidebar_position: 2
---

# Try it, you already have it 

Nowadays, pretty much every laptop and server is equipped with a TPM device (or a T2 secure enclave for Apple devices). 

And even if it's not available, we can still be able to emulate a TPM using [swtpm](https://github.com/stefanberger/swtpm). 

## Using a hardware TPM 


## Emulating a TPM

### Installing `libtpms`

```bash 
git clone https://github.com/stefanberger/libtpms

cd libtpms
# from https://github.com/stefanberger/libtpms/blob/master/INSTALL
./autogen.sh --with-tpm2 --with-openssl --prefix=/usr
make
make check
sudo make install
``` 

> Sometimes, you'll need to set up the PKG_CONFIG_PATH environment variable
```bash 
$ export PKG_CONFIG_PATH=/usr/lib/pkgconfig/
```

### Installing and using swtpm

More documentation available on the repo `https://github.com/stefanberger/swtpm/wiki`. 

### Manufacturing a simulated TPM
Because we are emulating the TPM, we need to simulate the manufacturing of the TPM so that we test properly the emulated device. 

Following the swtpm wiki, we can remanufacture the TPM by generating key and creating a self-signed EK certificate. 

We start by setting up the TPM state where secrets will be saved : 
```bash
$ sudo mkdir /tmp/mytpm2 && sudo chown tss:root /tmp/mytpm2
```
> This also shows the security precautions that should be taken. In fact, in a real TPM, the state is not accessible. In a software implementation however, it is directly accessible via /tmp/mytpm2 in this case. 

We can then remanufacture the TPM using the following command :
```bash
$ sudo swtpm_setup --tpmstate /tmp/mytpm2 --create-ek-cert --create-platform-cert --tpm2
```
 
It should give us a result like the following :
```bash
Starting vTPM manufacturing as root:root @ Tue 21 Jan 2025 08:20:12 PM CET
TPM is listening on Unix socket.
Successfully created RSA 2048 EK with handle 0x81010001.
  Invoking /usr/bin/swtpm_localca --type ek --ek e3c53b41b8eabffeb1fd871deb13fe32c5f54cf7a54342e7560ded9cbc5ada1f8efd8ddb0599d10f20aa753edc2e848c8091347bc248d7a9a28b7812fe0419d2ebc5e77e8380e5b95e1ee7a2f5a496230a3e9f7b383e618b8eaab6afe48fe8d57ed526cf89259fd5dda9c9d5e9ca11a69d2f3aadf93bfbf308beed6c36492e55998f74ad67202a7f97dc347895ee1f29e26fa71c4a34800f26c20daad4f310b47c8b56f0381d3ef14b3444a9fe1060b143ace4589d17d3d4929b49cf59527885cc67d12e9cb8a66f6098eaa1f28a60298a9b0b83225311561836180d288e29b7c09eab6c03d3d7ab52bed1a5eb72a34f7bb19747108c8fac980e503613037e27 --dir /tmp/swtpm_setup.certs.M4V8Z2 --tpm-spec-family 2.0 --tpm-spec-level 0 --tpm-spec-revision 164 --tpm-manufacturer id:00001014 --tpm-model swtpm --tpm-version id:20191023 --tpm2 --configfile /etc/swtpm-localca.conf --optsfile /etc/swtpm-localca.options
swtpm_localca: Successfully created EK certificate locally.
  Invoking /usr/bin/swtpm_localca --type platform --ek e3c53b41b8eabffeb1fd871deb13fe32c5f54cf7a54342e7560ded9cbc5ada1f8efd8ddb0599d10f20aa753edc2e848c8091347bc248d7a9a28b7812fe0419d2ebc5e77e8380e5b95e1ee7a2f5a496230a3e9f7b383e618b8eaab6afe48fe8d57ed526cf89259fd5dda9c9d5e9ca11a69d2f3aadf93bfbf308beed6c36492e55998f74ad67202a7f97dc347895ee1f29e26fa71c4a34800f26c20daad4f310b47c8b56f0381d3ef14b3444a9fe1060b143ace4589d17d3d4929b49cf59527885cc67d12e9cb8a66f6098eaa1f28a60298a9b0b83225311561836180d288e29b7c09eab6c03d3d7ab52bed1a5eb72a34f7bb19747108c8fac980e503613037e27 --dir /tmp/swtpm_setup.certs.M4V8Z2 --tpm-spec-family 2.0 --tpm-spec-level 0 --tpm-spec-revision 164 --tpm-manufacturer id:00001014 --tpm-model swtpm --tpm-version id:20191023 --tpm2 --configfile /etc/swtpm-localca.conf --optsfile /etc/swtpm-localca.options
swtpm_localca: Successfully created platform certificate locally.
Successfully created NVRAM area 0x1c00002 for RSA 2048 EK certificate.
Successfully created NVRAM area 0x1c08000 for platform certificate.
Successfully created ECC EK with handle 0x81010016.
  Invoking /usr/bin/swtpm_localca --type ek --ek x=26f0025537504ffb74bcbaf77a323e26e3f06cd5fb07d93b4c952a86e50470921c558046385107b02ff7a2a7b52642bd,y=d6100f4d453d8d515e9955bde20eb8793ccb6ba981ab6de68d22ac0199d3e263a3c74a628bd5b0cb3972269468f77606,id=secp384r1 --dir /tmp/swtpm_setup.certs.M4V8Z2 --tpm-spec-family 2.0 --tpm-spec-level 0 --tpm-spec-revision 164 --tpm-manufacturer id:00001014 --tpm-model swtpm --tpm-version id:20191023 --tpm2 --configfile /etc/swtpm-localca.conf --optsfile /etc/swtpm-localca.options
swtpm_localca: Successfully created EK certificate locally.
Successfully created NVRAM area 0x1c00016 for ECC EK certificate.
Successfully activated PCR banks sha256 among sha1,sha256,sha384,sha512.
Successfully authored TPM state.
Ending vTPM manufacturing @ Tue 21 Jan 2025 08:20:12 PM CET
``` 

### Setting up the TPM device

The TPM interface is accessible through multiple ways. 
To set up the emulated TPM we can use a listener to interact with it. 
We start by specifying the environment variable for tpm2-tools:
```bash
export TPM2TOOLS_TCTI="swtpm:port=2321"
```

and we start the listener : 
```bash
swtpm socket --tpmstate dir=/tmp/mytpm2 --tpm2 --server type=tcp,port=2321 --ctrl type=tcp,port=2322 --flags not-need-init,startup-clear
```

We can start testing the virtual TPM by opening a new terminal and trying to get the available certificates :
```bash
sudo tpm2_getcap handles-nv-index
```

### Security notes
On a virtual TPM, the PCRs, responsible for software and firmware measurements, are not extended. This is normal because the TPM starts after the OS is ran. 


## Running with Qemu 

# Sum up

We now know how to start using a TPM. Which is a great skill ! The next chapters are more theoretical, but will be important to understand how to make use of the TPM. 


# References
[https://ejaaskel.dev/yocto-emulation-setting-up-qemu-with-tpm/](https://ejaaskel.dev/yocto-emulation-setting-up-qemu-with-tpm/)

[https://link.springer.com/book/10.1007/978-1-4302-6584-9](https://link.springer.com/book/10.1007/978-1-4302-6584-9)

[https://stackoverflow.com/questions/71220170/how-to-install-start-using-swtpm-on-linux](https://stackoverflow.com/questions/71220170/how-to-install-start-using-swtpm-on-linux)