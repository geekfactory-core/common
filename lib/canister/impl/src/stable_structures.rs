use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};

#[derive(Default)]
pub struct CBor<T>(pub T)
where
    T: serde::Serialize + serde::de::DeserializeOwned;

impl<T> std::ops::Deref for CBor<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Storable for CBor<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self.0, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(ciborium::de::from_reader(bytes.as_ref()).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;

    fn into_bytes(self) -> Vec<u8> {
        self.to_bytes().into_owned()
    }
}
