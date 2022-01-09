use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage};
use serde::{de::DeserializeOwned, Serialize};

pub mod address_store {
    use std::marker::PhantomData;

    use cosmwasm_std::{CanonicalAddr, ReadonlyStorage, StdResult, Storage};
    use serde::{de::DeserializeOwned, Serialize};

    use super::{AsTargetRef, DynamicKey, DynamicLoad, DynamicSave, Store};

    pub trait SaveForAddress<T> {
        fn save_for_address(&self, addr: CanonicalAddr) -> Operation<'_, T, Save<'_, T>>;
    }

    pub trait LoadForAddress<T> {
        fn load_for_address(addr: CanonicalAddr) -> Operation<'static, T, Load>;
    }

    pub trait ExecuteSaveWithStorage {
        fn execute_with_storage<S: Storage>(self, storage: &mut S) -> StdResult<()>;
    }

    pub trait ExecuteLoadWithStorage<T> {
        fn execute_with_storage<S: ReadonlyStorage>(self, storage: &S) -> StdResult<T>;
    }

    pub struct Save<'a, T>(&'a T);

    pub struct Load;

    pub struct Operation<'a, T, Op> {
        _t: PhantomData<&'a T>,
        op: Op,
        address: CanonicalAddr,
    }

    impl<'a, T, Op> DynamicKey for Operation<'a, T, Op> {
        fn key(&self) -> &[u8] {
            self.address.as_slice()
        }
    }

    impl<'a, T, Op> Store<T> for Operation<'a, T, Op> {}

    impl<'a, T> AsTargetRef<T> for Operation<'a, T, Save<'a, T>> {
        fn as_target(&self) -> &T {
            self.op.0
        }
    }

    impl<T> SaveForAddress<T> for T
    where
        T: Serialize + DeserializeOwned,
    {
        fn save_for_address(&self, addr: CanonicalAddr) -> Operation<'_, Self, Save<'_, T>> {
            Operation {
                _t: PhantomData,
                op: Save(self),
                address: addr,
            }
        }
    }

    impl<T> LoadForAddress<T> for T
    where
        T: Serialize + DeserializeOwned,
    {
        fn load_for_address(addr: CanonicalAddr) -> Operation<'static, Self, Load> {
            Operation {
                _t: PhantomData,
                op: Load,
                address: addr,
            }
        }
    }

    impl<'a, T> ExecuteSaveWithStorage for Operation<'a, T, Save<'a, T>>
    where
        T: Serialize + DeserializeOwned,
    {
        fn execute_with_storage<S: Storage>(self, storage: &mut S) -> StdResult<()> {
            self.save(storage)
        }
    }

    impl<T> ExecuteLoadWithStorage<T> for Operation<'static, T, Load>
    where
        T: Serialize + DeserializeOwned,
    {
        fn execute_with_storage<S: ReadonlyStorage>(self, storage: &S) -> StdResult<T> {
            self.load(storage)
        }
    }
}

// TODO: Can this be done without introducing our own AsRef type trait?
pub trait AsTargetRef<Target> {
    fn as_target(&self) -> &Target;
}

pub trait Store<Target = Self> {}

pub trait StaticKey {
    fn key() -> &'static [u8];
}

pub trait DynamicKey {
    fn key(&self) -> &[u8];
}

pub trait StaticSave<Target> {
    fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()>;
}

pub trait DynamicSave<Target> {
    fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()>;
}

pub trait StaticLoad<T> {
    fn load<S: ReadonlyStorage>(storage: &S) -> StdResult<T>;
}

pub trait DynamicLoad<T> {
    fn load<S: ReadonlyStorage>(&self, storage: &S) -> StdResult<T>;
}

pub trait StaticUpdate<Target> {
    fn update<S: Storage, F: FnOnce(&mut Target)>(f: F, storage: &mut S) -> StdResult<()>;
}

fn ser_bin_data<T: Serialize>(obj: &T) -> StdResult<Vec<u8>> {
    bincode2::serialize(&obj).map_err(|e| StdError::serialize_err(std::any::type_name::<T>(), e))
}

fn deser_bin_data<T: DeserializeOwned>(data: &[u8]) -> StdResult<T> {
    bincode2::deserialize::<T>(data)
        .map_err(|e| StdError::serialize_err(std::any::type_name::<T>(), e))
}

fn key_not_found(key: &[u8]) -> StdError {
    let key_lossy = String::from_utf8_lossy(key);
    let msg = format!("Key '{}' not found in storage", key_lossy);
    StdError::not_found(msg)
}

pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], data: &T) -> StdResult<()> {
    let bin_data = ser_bin_data(data)?;
    storage.set(key, &bin_data);
    Ok(())
}

pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    storage
        .get(key)
        .ok_or_else(|| key_not_found(key))
        .and_then(|bytes| deser_bin_data(&bytes))
}

impl<T, Target> StaticSave<Target> for T
where
    T: Store<Target> + StaticKey + AsTargetRef<Target>,
    Target: Serialize,
{
    fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        save(storage, T::key(), self.as_target())
    }
}

impl<T, Target> DynamicSave<Target> for T
where
    T: Store<Target> + DynamicKey + AsTargetRef<Target>,
    Target: Serialize,
{
    fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        save(storage, self.key(), self.as_target())
    }
}

impl<T, Target> StaticLoad<Target> for T
where
    T: Store<Target> + StaticKey,
    Target: DeserializeOwned,
{
    fn load<S: ReadonlyStorage>(storage: &S) -> StdResult<Target> {
        load(storage, T::key())
    }
}

impl<T, Target> DynamicLoad<Target> for T
where
    T: Store<Target> + DynamicKey,
    Target: DeserializeOwned,
{
    fn load<S: ReadonlyStorage>(&self, storage: &S) -> StdResult<Target> {
        load(storage, self.key())
    }
}

impl<T> StaticUpdate<T> for T
where
    T: Store + StaticKey + Serialize + DeserializeOwned,
{
    fn update<S: Storage + ReadonlyStorage, F: FnOnce(&mut T)>(
        f: F,
        storage: &mut S,
    ) -> StdResult<()> {
        let mut target = T::load(storage)?;
        f(&mut target);
        target.save(storage)
    }
}

impl<T> Store for T where T: Serialize + DeserializeOwned {}

impl<T> AsTargetRef<T> for T
where
    T: Store,
{
    fn as_target(&self) -> &T {
        self
    }
}
