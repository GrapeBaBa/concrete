# Introduction

Welcome to the `concrete-core` guide!

This library contains a set of low-level primitives which can be used to implement *Fully
Homomorphically Encrypted* (FHE) programs. In a nutshell, fully homomorphic encryption makes it
possible to perform arbitrary computations over encrypted data. With FHE, you can perform
computations without putting your trust on third-party computation providers.

## Audience

This library is geared towards people who already know their way around FHE. It gives the user
freedom of choice over a breadth of parameters, which can lead to less than 128 bits of security
if chosen incorrectly.

Fortunately, we propose multiple libraries that build on top of `concrete-core` and which
propose a safer API. To see which one best suits your needs, see the
[concrete homepage](https://zama.ai/concrete).

## Quick start
Here is a quick example of how the library can be used to encrypt an integer and decrypt it:

```rust
extern crate concrete_core;
extern crate concrete_commons;

use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::LweDimension;
use concrete_core::prelude::*;

// DISCLAIMER: the parameters used here are only for demo purposes, and are not secure.
let lwe_dimension = LweDimension(630);
// Here a hard-set encoding is applied (shift by 20 bits)
let input = 3_u32 << 20;
let noise = Variance(2_f64.powf(-25.));

let mut engine = CoreEngine::new().unwrap();
let key: LweSecretKey32 = engine.create_lwe_secret_key(lwe_dimension).unwrap();
let plaintext = engine.create_plaintext(&input).unwrap();
let ciphertext = engine.encrypt_lwe_ciphertext(&key, &plaintext, noise).unwrap();

let decrypted_plaintext = engine.decrypt_lwe_ciphertext(&key, &ciphertext).unwrap();

engine.destroy(key).unwrap();
engine.destroy(plaintext).unwrap();
engine.destroy(ciphertext).unwrap();
engine.destroy(decrypted_plaintext).unwrap();
```

## Architecture

`concrete-core` is a modular library which makes it possible to use different backends to
perform FHE operations. Its design revolves around two modules:

+ The [`specification`] module contains a specification (in the form of traits) of the
`concrete` FHE scheme. It describes the FHE objects and operators, which are exposed by the
library.
+ The [`backends`] module contains various backends implementing all or a part of this scheme.
These different backends can be activated by feature flags, each making use of different
hardware or system libraries to make the operations faster.

## Activating backends

The different backends can be activated using the feature flags `backend_*`. The `backend_core`
contains an engine executing operations on a single thread of the cpu. It is activated by
default.

In the next section, we'll show you the steps required for the creation of your own backend!
