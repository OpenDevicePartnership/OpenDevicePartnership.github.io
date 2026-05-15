//! Components for the redesigned ODP website.
//!
//! The module tree is intentionally flat: each file groups closely
//! related primitives (layout, typography, controls, media, ...) so
//! a contributor can find a component without spelunking through a
//! deep folder hierarchy. The `repo_view` module is the lone
//! third-party-integration outlier and stays at the top level.

pub mod cards;
pub mod controls;
pub mod layout;
pub mod media;
pub mod nav;
pub mod project_layout;
pub mod repo_view;
pub mod theme;
pub mod typography;
