//! This is made to generate code for the cabidge vm.
//! There will be a cabidge language all this is designed for but you can have fun with this too :3
//! You can find me @lizzie@brain.worm.pink on (activitypub) fedi or lz@worm.pink on xmpp
//! This language has no strict evaluation order.
//! Probably be besgt to treat it like it is lazy.
//! Monads will probably be needed to contain side effects to ensure evaluation.
//!
//! The language defined here is designed to be modular
//! and prioritize simplicity > size > speed.
//! The vm itself will prioritize speed but that is a different crate.
//! 
//! This crate here outlines a cps that was really the result of trying to make function calls and block jumps the same in SSA.

pub mod func;
pub mod gen;
pub mod module;
