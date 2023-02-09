# Signatrust
[![RepoSize](https://img.shields.io/github/repo-size/TommyLike/signatrust)](https://github.com/volcano-sh/volcano)
[![Clippy check](https://github.com/TommyLike/signatrust/actions/workflows/build.yml/badge.svg)](https://github.com/TommyLike/signatrust/actions/workflows/build.yml)

Signatrust provides a secure, unified and high throughput solution for signing linux packages&binaries in Rust.
# Background
There are many cases when we need signing packages&binaries for a linux distro, usually PGP is used for rpm package, 
ISO checksum, AppImage and repository metadata, while X509 certificate covers the case of kernel module and EFI, 
also, there are lots of mature projects & scripts are already been used in the community. But most of them are usually 
used in the CI CD environments and the security & management of private key is not covered.
We noticed there are several projects aiming to fix some challenges:
1. [OBS sign](https://github.com/openSUSE/obs-sign): obs-sign is developed by openSUSE and widely used in the linux distro 
   packaging system including [OBS](https://build.opensuse.org/) and [COPR](https://copr.fedorainfracloud.org/). It
   provides a server&client solution for massive signing tasks in a production environment. But it's hard to replicate the
   instances to increase throughput, also there are some security & management concerns since pgp is located on server disk.
2. [sbsigntools](https://github.com/phrack/sbsigntools) this is a fork version of official sbsigntools which can store
    certificates & key in AWS CloudHSM and targets for UEFI signing.
3. other tools.

# Features
**Signatrust**, stands for `Signature + Trust` is a rust project that can provide a unified solution for all the challenges:
 
1. **E2E security design**: First of all, all the sensitive data (key and certificates) will be encrypted transparently with 
   external KMS provider such as CloudHSM before saving into database, second, instead of transferring private key to client
   and perform a local sign operation, all the content will be delivered to sign server and signature calculated in the memory
   directly, last, mutual-tls is required for communication between client and server for now, and can be upgraded to integrated
   with SPIFF&SPIRE system.
2. **High throughput**: The control server and data server are split and the data server can be easily replicated, additional
   improvements including gRPC stream, client round-robin, memory cache and async tasks are used to increase single instance 
   performance.
3. **Complete binary support**:
   1. RPM/SRPM signature.
   2. Detached PGP signature including ISO checksum and repo metadata.
   3. Kernel module signature.
   4. EFI.
   5. Container Image.
   6. WSL Image.
   7. AppImage.
4. **User-friendly key management**: Signatrust provides a standalone UI for key management and can be easily integrated with
   external account system via OIDC protocol. Administrators can generate & import & export & delete keys through the interface.

# Architecture
![System Context](./docs/images/System%20Context.png)
# Performance

# Quick Start Guide

# Contribute
