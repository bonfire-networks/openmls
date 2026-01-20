//! # OpenMLS Default Crypto Provider
//!
//! This is an implementation of the [`OpenMlsProvider`] trait to use with
//! OpenMLS.

pub use openmls_memory_storage::{MemoryStorage, MemoryStorageError};
use openmls_traits::OpenMlsProvider;

mod provider;
pub use provider::*;

mod hmac;

#[derive(Default, Debug)]
#[cfg_attr(feature = "test-utils", derive(Clone))]
pub struct OpenMlsRustCrypto {
    crypto: RustCrypto,
    key_store: MemoryStorage,
}

impl OpenMlsRustCrypto {
    /// Create a new OpenMlsRustCrypto with a custom MemoryStorage.
    pub fn with_storage(storage: MemoryStorage) -> Self {
        Self {
            crypto: RustCrypto::default(),
            key_store: storage,
        }
    }
}

impl OpenMlsProvider for OpenMlsRustCrypto {
    type CryptoProvider = RustCrypto;
    type RandProvider = RustCrypto;
    type StorageProvider = MemoryStorage;

    fn storage(&self) -> &Self::StorageProvider {
        &self.key_store
    }

    fn crypto(&self) -> &Self::CryptoProvider {
        &self.crypto
    }

    fn rand(&self) -> &Self::RandProvider {
        &self.crypto
    }
}
