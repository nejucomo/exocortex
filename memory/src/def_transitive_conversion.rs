macro_rules! def_transitive_conversion {
    ( TryInto: $a:ty => $b:ty => $c:ty ) => {
        impl TryFrom<$a> for $c {
            type Error = derive_more::TryIntoError<$a>;

            fn try_from(v: $a) -> Result<Self, Self::Error> {
                v.try_into().and_then(|v2: $b| {
                    v2.try_into().map_err(|e: derive_more::TryIntoError<$b>| {
                        derive_more::TryIntoError::new(e.input.into(), "FIXME", "FIXME")
                    })
                })
            }
        }
    };

    ( From: $a:ty => $b:ty => $c:ty ) => {
        impl From<$a> for $c {
            fn from(v: $a) -> $c {
                Self::from(<$b>::from(v))
            }
        }
    };
}

pub(crate) use def_transitive_conversion;
