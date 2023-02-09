# Signatrust
Signatrust provides a secure, unified and high throughput solution for signing linux packages&binaries in Rust.
# Background
There are many cases when we need signing packages&binaries for a linux distro, usually PGP is used for rpm package, 
ISO checksum, AppImage and repository metadata, while X509 certificate covers the case of kernel module and EFI, 
also, there are lots of mature projects & scripts are already been used in the community. But most of them are only singing 
binaries that usually be used in the CI CD environments and the security & management of private key itself is not related.
We noticed there are several projects aiming to fix some of the challenges:
1. [OBS sign](https://github.com/openSUSE/obs-sign): obs-sign is developed by openSUSE and widely used in the linux distro 
   packaging system including [OBS](https://build.opensuse.org/) and [COPR](https://copr.fedorainfracloud.org/). It
   provides a server&client solution for massive signing tasks in a production environment. But it's hard to replicate the
   instances to increase throughput, also there are some security & management concerns since pgp is located on server disk.
2. [sbsigntools](https://github.com/phrack/sbsigntools) this is a fork version of official sbsigntools which can store
    certificates & key in AWS CloudHSM and targets for UEFI signing.
3. other tools.

# Features
Signatrust, stands for `Signature + Trust` is a rust project that can provided a unified solution for all the challenges:
1. **User friendly key management**: 
2. **E2E security design**: 
3. **High throughput**:
4. **Complete binary support**:

# Architecture

# Comparison

# Quick Start Guide

# Contribute
