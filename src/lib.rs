#![deny(clippy::unwrap_used)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::nursery)]
#![allow(clippy::needless_return)]

pub mod post;
pub mod protokoll;

pub use protokoll::{
    ProtokollTemplate,
    events::Event,
    person::{Abmeldung, Person, PersonWithAbmeldung},
    sitzung::{Antrag, Sitzung, SitzungKind, Top, TopKind},
};
