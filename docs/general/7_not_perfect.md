---
sidebar_position: 7
---


# Privacy and security concerns

Many people raised several concerns about confidential computing and trusted computing. We can categorize these concerns into two parts :
- ethical concerns : Where using TPMs and confidential computing will result in a DRM system that could limit the freedom that we have on a device. 
- security concerns : In the way it is implemented, might be difficult to recover from if critical vulnerability that exposes the secrets is found. 

## Ethical and privacy concerns 

One first thing to understand is that there is a difference between confidentiality and privacy. In information systems, TEEs can guarantee the confidentiality of runtime execution. However, privacy IS NOT something that TEEs protect us from. It is even the contrary. In TPMs for example, knowing the Endorsement Key of a secure chip could reveal information about which chip it is. 

The ethical and privacy concerns are valid. Numerous discussions on the freedom of using you own hardware according to what you want is rightfull. For example, Intel ME has been accused of containing a backdoor and is insecure due to multiple vulnerabilities since 2017. This suggestion is mainly strenghten [by the fact that the code isn't publicly accessible for researchers](https://blog.thenewoil.org/the-not-so-scary-truth-behind-intel-me) (which is by the way a valid point) and the fact that the Intel ME chip is always up, can not be disabled and has access to everything inside the computer (network, memory, keyboard, display, etc...) - yes, this is really suspicious indeed. However, the main concern is about management (as indicated by the name Intel *"Management Engine"*). The goal of this chip might have been a DRM type of control, which isn't by any means less concerning.

So the main question remains : *is it possible to trust you own computer ? Or do you have a full control over it ?* 

Well, the quick answer is no, and that might be because of the remote attestation. 
Richard Stallman (*just the guy who created the GNU project*) actually wrote an [essay back in 2002](https://www.gnu.org/philosophy/can-you-trust.en.html) basically predicting this issue. Unfortunately, he wasn't totally wrong. The example is, as he noted in his update, Google Play Integrity API (formerly known as Google SafetyNet) which verifies that the operating system you are running is a Google official version. Hence, if you're using another OS such as [GrapheneOS](https://grapheneos.org/articles/attestation-compatibility-guide), [LineageOS](https://lineageos.org/PlayIntegrity/) or others, some bank apps (and Netflix apparently) will not run on other operating systems other than an official Google android one. 

Not great for those who do not want to use Google apps (with the whole constant tracking) anymore or wants to have full control of their hardware. 

[Gabriel Sieben](https://gabrielsieben.tech/2022/07/29/remote-assertion-is-coming-back-how-much-freedom-will-it-take/) wrote a better article explaining these predicaments and what could lead to. 

*What should be done?*

Remote attestation is not all that bad. If used correctly. 
There is many use-cases where we can find useful purposes as a protective mechanism. From malware and bootkits detection to phishing protection. It should just not be used as a mean to control and restrict people from using the software they want. 

One way to achieve that is through transparency. 


## Security concerns

A popular myth that people (I hope only marketers) might promote these days, is running everything in confidential environments will suffice at having a military-grade, most awesome, secure environment (or whatever). 

**They could not be more wrong**. 


***On one hand***, while vulnerabilities on low level component and hardware is surely rare (*Should we talk about Spectre and Meltdown?*) and might be difficult to exploit, they still exist. This doesn't mean that using TEEs doesn't add any security, but if a critical vulnerability isn't handled and patched correctly, the secure layers added won't be of any help. Having the best impenetrable door with the all windows open and the house is on the first floor won't add any protection from entering. 

A simple example is intel SGX. This one is a 6-stars elite champion in terms of vulnerabilities. We can cite [SmashEx](https://jasonyu1996.github.io/SmashEx/), [AEPic](https://aepicleak.com/) Leak, [Downfall](https://downfall.page/) and others ([you can find a list here](https://sgx.fail/)). 

*Should we then stop using SGX ?* **Not Necessarily**. Intel updates the microcode and adds mitigations each time a vulnerability is disclosed. The whole issue then is to update the SGX platforms running which might be more difficult to deal with. Some SGX developers trade security over usability which is wrong but comprehensible due the troubles that you have to go through to install BIOS updates (and sometimes having to reprovision the SGX CPU keys). *It is not a package update that could be automatable or quickly fixed.* 

***One the other hand***, the objective of having a minimal Trusted Computing Base (TCB) might be overlooked. And this property is what makes the TEEs interesting to use. 
While TEEs can help reduce the TCB by isolating the code running inside from the host, we have to keep in mind that the code running inside should also be meticulously investigated and tested. 

For instance, if you have an arbitrary code execution on a server running in a TEE, it will not matter that you are using the next-gen TEE, because the issue will be the code running inside. 

Thus, reducing the TCB should also imply reducing the software stack you are running. Otherwise, it will be the same as running a normal OS directly. 

One use-case that is highly discussed at the time of writing this document, is protecting AI and ML Model weights by putting them inside a TEE by leveraging, for example, AMD SEV-SNP combined with Nvidia H100. It could be a legitimate answer to protect the intellectual property from a internal/external threats. However, to be able to run workloads on it, other services must be implemented, and the more we add services to confidential platforms, the more it adds to the attack vector and thus of a service being compromised. This possess a real threat to the confidential system because its sole purpose is to be to run isolated and inacessible workloads. 

*If an attacker could inject commands inside the confidential environment and succeeds, the confidential system wouldn't be that confidential now, would it?* 





Some shadowy subjects has also to be taken into account. For example, other connections has to be made, especially between GPUs. Nvidia is trying to tackle the issue by adding attestation between GPUs and HPC networks through NvLink and InfiniBand but it isn't still enough documented to understand what is really happening. The overall question is, how to scale attestation in GPUs easily and make it work seamlessly.  

There is still a lot of research that has to be done when it comes to exploring RDMA (Remote Direct Memory Access) where the computation bypasses the CPUs for more efficiency but could be critical for the data sent as that there might be no encryption mechanisms.

Some research about this : [https://github.com/spcl/redmark/blob/master/paper/redmark.pdf](https://github.com/spcl/redmark/blob/master/paper/redmark.pdf) and [https://www.usenix.org/conference/atc20/presentation/taranov](https://www.usenix.org/conference/atc20/presentation/taranov)




