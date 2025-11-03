//! Primary functionality for
//! [RadioMare](https://codeberg.org/r6915ee/radiomare/) clients.
//!
//! This library provides the main functionality behind **RadioMare**.
//! Specifically, it handles assets, especially charts.
//!
//! # Description
//!
//! `radiocore`'s main responsibility is to handle most RadioMare-specific
//! information for both clients *and* additional utilities. What this means
//! is that provides a unified interface for each of its operations.
//!
//! todo

/// A module defining chart-specific data solutions.
///
/// Charts are the primary focus of `radiocore` and act very differently from
/// other solutions.
///
/// They use the following terminology:
///
/// * **Steps**: Steps are the primary unit of measurement for charts, and
///   are typically one fourth of a beat.
/// * **Hooks**: Hooks are assigned a certain step. When that step is reached,
///   then the hook is run, being used in conjunction with the other hooks at
///   that step; this is typically used to create an entity.
/// * **Layers**: Layers are assigned a certain set of hooks and may be
///   activated and deactivated at will.
/// * **Sets**: Typically identified in-game as *difficulties*, sets activate
///   and disable certain layers.
///
/// This system allows reusability, in contrast to other chart systems, and
/// exists to make use of the Entity Component System paradigm.
pub mod chart;
