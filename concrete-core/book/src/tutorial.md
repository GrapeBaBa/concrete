# Tutorial: how to create your own backend?

Everything's been made easy for anyone to create their own backend in `concrete-core`!
Implementations targeting specific hardware are thus easy to plug with Concrete. The main steps to
create your backend are:

1. [Add an optional feature in the Cargo.toml](#add-an-optional-feature)
2. [Build the structure for the new backend](#build-the-structure-for-the-new-backend)
3. [Implement some entity and engine traits of your choice](#implement-the-entity-and-engine-traits-of-your-choice)

Let's see how to do this in more details. For the sake of this tutorial, we're going to be adding a
GPU backend to `concrete-core`, but it could be any other hardware.

Before following any of the steps shown in this tutorial, you actually have to create a crate that
exposes some hardware-accelerated functions you want to use in `concrete-core`. For example, your
Rust crate could be actually wrapping some C/C++ code. It's the example we consider in this
tutorial: let's consider you've created a crate `fhe-gpu` that exposes some functions:

- `get_number_of_gpus`
- `drop`
- `malloc`
- `copy_to_gpu`

What we want to do is to pass some data from Rust to some C/C++ functions: we're going to do it
using void pointers (with some metadata). This is definitely not the only way to do this, maybe not
the best: we're open to any suggestion for improvement, do not hesitate to contact us!

## Add an optional feature

So the first step is to configure `concrete-core`'s manifest to recognize your backend, and be able
to optionally activate it.

Open `concrete-core`'s `Cargo.toml` file and edit the following section:

```asm
[features]
default = ["backend_core"]
doc = []
backend_core = []
slow-csprng = ["concrete-csprng/slow"]
multithread = ["rayon", "concrete-csprng/multithread"]
```

Add this line at the end of it:

```asm
backend_gpu = []
```

Now, you'll be able to:

```
cargo build -p concrete-core --release --features=backend_gpu
```

which will build `concrete-core` with the features `backend_core` and `backend_gpu`.

## Build the structure for the new backend

### Create some new directories

Now we're going to build the structure for the new backend. First, create some empty directories:

```
mkdir /path/to/concrete-core/src/backends/gpu
mkdir /path/to/concrete-core/src/backends/gpu/implementation
mkdir /path/to/concrete-core/src/backends/gpu/implementation/engines
mkdir /path/to/concrete-core/src/backends/gpu/implementation/entities
mkdir /path/to/concrete-core/src/backends/gpu/private
```

The `private` module is where you'll be putting the code you don't want to expose in the backend
itself. Edit `concrete-core/src/backends/mod.rs` to add the following lines:

```asm
#[cfg(feature = "backend_gpu")]
pub mod gpu;
```

Edit also the prelude (`concrete-core/src/prelude.rs`) to add these lines:

```asm
#[cfg(feature = "backend_gpu")]
pub use super::backends::gpu::engines::*;
#[cfg(feature = "backend_gpu")]
pub use super::backends::gpu::entities::*;
```

With this in the prelude, it'll be possible for the user to import all they need with just one line:

```asm
use concrete_core::prelude::*;
```

### Create new modules

Start with `concrete-core/src/backends/gpu/mod.rs`, which should contain the following:

```asm
//! A module containing the GPU backend implementation.
//!
//! This module contains GPU implementations of some functions of the concrete specification.

#[doc(hidden)]
pub mod private;

pub(crate) mod implementation;

pub use implementation::{engines, entities};
```

Then, `concrete-core/src/backends/gpu/implementation/mod.rs` should contain:

```asm
pub mod engines;
pub mod entities;
```

Create also two empty modules for engines and entities
at `concrete-core/src/backends/gpu/implementation/engines/mod.rs`
and `concrete-core/src/backends/gpu/implementation/entities/mod.rs`

## Implement the entity and engine traits of your choice

### Entities

Start by implementing the entities you'll be using. For example, let's imagine we want to be
manipulating LWE ciphertext vectors on the GPU. We need to create a new
file: `concrete-core/src/backends/gpu/implementation/entities/lwe_ciphertext_vector.rs`
Modify the entity module file, `concrete-core/src/backends/gpu/implementation/entities/mod.rs`, to
actually link it to the rest of the sources:

```asm
//! A module containing all the [entities](crate::specification::entities) exposed by the GPU
//! backend.

mod lwe_ciphertext_vector;

pub use lwe_ciphertext_vector::*;
```

Now, let's implement that entity. What we want is to implement a `GpuLweCiphertextVector32` entity
for the `LweCiphertextVectorEntity` trait from the specification.

First, in the private module we're going to implement a structure that's going to be wrapped by
this `GpuLweCiphertextVector32`. That structure is going to contain a void pointer that will
eventually be allocated on the GPU, and some metadata. For example, you can create a new `lwe.rs`
file in the `private` module, containing:

```asm
// Fields with `d_` are data in the GPU
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GpuLweList<T: UnsignedInteger> {
    // Pointer to GPU data
    pub(crate) d_ptr: *mut c_void,
    // Number of ciphertexts in the array
    pub(crate) lwe_ciphertext_count: LweCiphertextCount,
    // Lwe dimension
    pub(crate) lwe_dimension: LweDimension,
    // Field to hold type T
    pub(crate) _phantom: PhantomData<T>,
}

impl<T: UnsignedInteger> GpuLweList<T> {
    pub(crate) fn lwe_ciphertext_count(&self) -> LweCiphertextCount {
        self.lwe_ciphertext_count
    }

    pub(crate) fn lwe_dimension(&self) -> LweDimension {
        self.lwe_dimension
    }

    /// Returns a mut pointer to the GPU data on a chosen GPU
    #[allow(dead_code)]
    pub(crate) unsafe fn get_ptr(&self) -> GpuLweCiphertextVectorPointer {
        self.d_ptr
    }
}
```

Do not forget to modify the `concrete-core/src/backends/gpu/private/mod.rs` file to add:

```asm
pub mod lwe;
```

Now, we can actually implement the entity trait:

```asm
use std::fmt::Debug;

use concrete_commons::parameters::{LweCiphertextCount, LweDimension};

use crate::backends::cuda::private::crypto::lwe::list::CudaLweList;
use crate::specification::entities::markers::{BinaryKeyDistribution, LweCiphertextVectorKind};
use crate::specification::entities::{AbstractEntity, LweCiphertextVectorEntity};

/// A structure representing a vector of LWE ciphertexts with 32 bits of precision on the GPU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuLweCiphertextVector32(pub(crate) GpuLweList<u32>);

impl AbstractEntity for GpuLweCiphertextVector32 {
    type Kind = LweCiphertextVectorKind;
}

impl LweCiphertextVectorEntity for GpuLweCiphertextVector32 {
    type KeyDistribution = BinaryKeyDistribution;

    fn lwe_dimension(&self) -> LweDimension {
        self.0.lwe_dimension()
    }

    fn lwe_ciphertext_count(&self) -> LweCiphertextCount {
        self.0.lwe_ciphertext_count()
    }
}
```

You can do this for all the entity traits you need in your backend.

### Engines

Now we have some entities, let's actually do something with them. For the GPU backend example, we
first have to allocate data on the GPU and copy the LWE ciphertext vector from the CPU to the GPU.

First, let's create the main engine
in `concrete-core/src/backends/gpu/implementation/engines/mod.rs`:

Let's imagine you created a `fhe-gpu` crate that exposes a function to get the number of GPUs
available on the machine, `get_number_of_gpus`. We'll create a `GpuEngine` only in the case where
that function actually finds at least one GPU. Otherwise, an error will be returned: this example
also shows you how to define error cases and their display to the user.

```asm
use crate::prelude::sealed::AbstractEngineSeal;
use crate::prelude::AbstractEngine;
use std::error::Error;
use std::fmt::{Display, Formatter};

use fhe_gpu::get_number_of_gpus;

#[derive(Debug)]
pub enum GpuError {
DeviceNotFound,
}
impl Display for GpuError {
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
        GpuError::DeviceNotFound => {
            write!(f, "No GPU detected on the machine.")
        }
    }
}
}
impl Error for GpuError {}

/// The main engine exposed by the GPU backend.
///
#[derive(Debug, Clone)]
pub struct GpuEngine {}

impl AbstractEngineSeal for GpuEngine {}

impl AbstractEngine for GpuEngine {
type EngineError = GpuError;

fn new() -> Result<Self, Self::EngineError> {
    let number_of_gpus = unsafe { get_number_of_gpus() as usize };
    if number_of_gpus == 0 {
        Err(GpuError::DeviceNotFound)
    } else {
        Ok(GpuEngine {})
    }
}
}

mod destruction;
mod lwe_ciphertext_vector_conversion;
```

As you see at the bottom of the previous code block, we're going to implement two engine traits: one
to copy our LWE ciphertext vector from the CPU to the GPU, and one to destroy data on the GPU.
Create the file `concrete-core/src/backends/gpu/implementation/engines/destruction.rs`
and `concrete-core/src/backends/gpu/implementation/engines/lwe_ciphertext_vector_conversion.rs`.
