use super::TimestampMillis;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Timestamped<T> {
    pub value: T,
    pub timestamp: TimestampMillis,
}

impl<T> Timestamped<T> {
    pub fn new(now: TimestampMillis, value: T) -> Timestamped<T> {
        Timestamped {
            value,
            timestamp: now,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Timestamped<U> {
        Timestamped {
            value: f(self.value),
            timestamp: self.timestamp,
        }
    }

    pub fn map_value<U, F: FnOnce(T) -> U>(self, f: F) -> U {
        f(self.value)
    }
}

impl<T: Default> Default for Timestamped<T> {
    fn default() -> Self {
        Timestamped {
            value: T::default(),
            timestamp: TimestampMillis::default(),
        }
    }
}

impl<T> Deref for Timestamped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Taken from macro expansion
impl<T: CandidType> CandidType for Timestamped<T> {
    #[allow(deprecated)]
    fn _ty() -> ::candid::types::Type {
        ::candid::types::TypeInner::Record(vec![
            ::candid::types::Field {
                id: ::candid::types::Label::Named("value".to_string()).into(),
                ty: <T as ::candid::types::CandidType>::ty(),
            },
            ::candid::types::Field {
                id: ::candid::types::Label::Named("timestamp".to_string()).into(),
                ty: <TimestampMillis as ::candid::types::CandidType>::ty(),
            },
        ])
        .into()
    }

    fn idl_serialize<__S>(&self, __serializer: __S) -> ::std::result::Result<(), __S::Error>
    where
        __S: ::candid::types::Serializer,
    {
        let mut ser = __serializer.serialize_struct()?;
        ::candid::types::Compound::serialize_element(&mut ser, &self.value)?;
        ::candid::types::Compound::serialize_element(&mut ser, &self.timestamp)?;
        Ok(())
    }
}
