# Signatrust
[![RepoSize](https://img.shields.io/github/repo-size/TommyLike/signatrust)](https://github.com/volcano-sh/volcano)
[![Clippy check](https://github.com/TommyLike/signatrust/actions/workflows/build.yml/badge.svg)](https://github.com/TommyLike/signatrust/actions/workflows/build.yml)

Signatrust offers a highly secure and efficient solution for signing Linux packages and binaries using Rust. Our unified
platform ensures streamlined operations and a high throughput for all signing requests.

# Background

Signing packages and binaries for a Linux distribution is essential in many use cases. Typically, PGP is used for RPM
packages, ISO checksums, AppImages, and repository metadata. X509 certificates, on the other hand, are used to cover the
cases of kernel modules and EFI. While there are several projects and scripts already in use within the community, 
they are often limited to CI/CD environments, and the management and security of private keys are not always covered.

We have observed several projects aiming to address these challenges.
1. [**OBS sign**](https://github.com/openSUSE/obs-sign): Developed by openSUSE, obs-sign is a widely used Linux distro
   packaging system, including [OBS](https://build.opensuse.org/) and [COPR](https://copr.fedorainfracloud.org/). The
   solution provides a comprehensive server-client model for massive signing tasks in a production environment. 
   However, one of the challenges faced by the system is the difficulty in replicating instances to increase throughput.
   Additionally, the system is also plagued by security and management concerns, as PGP is located on the server disk directly.
2. [**sbsigntools**](https://github.com/phrack/sbsigntools) This is a fork version of official sbsigntools which can store
    certificates & key in AWS CloudHSM and targets for UEFI signing.
3. other tools.

# Features

**Signatrust**, stands for `Signature + Trust + Rust` is a rust project that can provide a unified solution for all the challenges:
 
1. **E2E security design**: Our end-to-end security design prioritizes the protection of sensitive data, such as keys and
   certificates, by transparently encrypting them with external KMS providers, like CloudHSM or Huawei KMS, before storing them in the
   database. Additionally, we have eliminated the need to transfer private keys to the client for local sign operations,
   opting instead to deliver content to the sign server and perform signature calculations directly in memory. Furthermore,
   all memory keys are zeroed out when dropped to protect against leaks to swap and core dump. Currently, mutual TLS is required
   for communication between the client and server, with future upgrades planned to integrate with the SPIFF&SPIRE ecosystem.

2. **High throughput**: To ensure high throughput, we have split the control server and data server and made it easy to
   replicate the data server. We have also made several performance enhancements, such as utilizing gRPC stream, client
   round-robin, memory cache, and async tasks to increase single-instance performance.

3. **Complete binaries support**:
   1. RPM/SRPM signature.
   2. Detached PGP signature including ISO checksum and repo metadata.
   3. Kernel module signature.
   4. EFI(todo).
   5. Container Image(todo).
   6. WSL Image(todo).
   7. AppImage(todo).

4. **User-friendly key management**: Signatrust provides a standalone UI for key management and can be easily integrated with
   external account system via OIDC protocol. Administrators can generate & import & export & delete keys through the interface.

# System Context
![System Context](./docs/images/System%20Context.png)
# Performance

# Quick Start Guide

# Contribute
