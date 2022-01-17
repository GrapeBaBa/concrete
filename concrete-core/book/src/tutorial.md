# Tutorial: how to create your own backend?

Everything's been made easy for anyone to create their own backend in `concrete-core`!
Implementations targetting specific hardware are thus easy to plug with Concrete. The main steps to
create your backend are:

1. [Add an optional feature in the Cargo.toml](#add-an-optional-feature)
2. [Build the structure for the new backend](#build-the-structure-for-the-new-backend)
3. [Implement some entity and engine traits of your choice](#implement-the-entity-and-engine-traits-of-your-choice)

Let's see how to do this in more details. For the sake of this tutorial, we're going to be adding a
GPU backend to `concrete-core`.

## Add an optional feature

So the first step is to configure `concrete-core`'s manifest so as to recognize your backend, and be
able to optionnally activate it.

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

## Implement the entity and engine traits of your choice
