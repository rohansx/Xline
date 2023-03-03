use super::ExecuteError;

/// The Stable Storage Api
pub trait StorageApi: Send + Sync + 'static + std::fmt::Debug {
    /// Insert key-value pair into the underlying storage engine
    ///
    /// # Errors
    ///
    /// if error occurs in storage, return `Err(error)`
    fn insert<K, V>(&self, table: &str, key: K, value: V, sync: bool) -> Result<(), ExecuteError>
    where
        K: Into<Vec<u8>> + std::fmt::Debug,
        V: Into<Vec<u8>> + std::fmt::Debug;

    /// Get values by keys from storage
    ///
    /// # Errors
    ///
    /// if error occurs in storage, return `Err(error)`
    fn get_values<K>(&self, table: &str, keys: &[K]) -> Result<Vec<Vec<u8>>, ExecuteError>
    where
        K: AsRef<[u8]> + std::fmt::Debug;

    /// Delete key-value pair from storage
    ///
    /// # Errors
    ///
    /// if error occurs in storage, return `Err(error)`
    fn delete<K>(&self, table: &str, key: K, sync: bool) -> Result<(), ExecuteError>
    where
        K: AsRef<[u8]> + std::fmt::Debug;
}