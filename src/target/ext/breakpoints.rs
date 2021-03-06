//! Add/Remove various kinds of breakpoints.

use crate::arch::Arch;
use crate::target::{Target, TargetResult};

/// The kind of watchpoint that should be set/removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchKind {
    /// Fire when the memory location is written to.
    Write,
    /// Fire when the memory location is read from.
    Read,
    /// Fire when the memory location is written to and/or read from.
    ReadWrite,
}

/// Target Extension - Set/remove Software Breakpoints.
///
/// See [this stackoverflow discussion](https://stackoverflow.com/questions/8878716/what-is-the-difference-between-hardware-and-software-breakpoints)
/// about the differences between hardware and software breakpoints.
///
/// _Recommendation:_ If you're implementing `Target` for an emulator that's
/// using an _interpreted_ CPU (as opposed to a JIT), the simplest way to
/// implement "software" breakpoints would be to check the `PC` value after each
/// CPU cycle.
pub trait SwBreakpoint: Target {
    /// Add a new software breakpoint.
    /// Return `Ok(false)` if the operation could not be completed.
    fn add_sw_breakpoint(&mut self, addr: <Self::Arch as Arch>::Usize) -> TargetResult<bool, Self>;

    /// Remove an existing software breakpoint.
    /// Return `Ok(false)` if the operation could not be completed.
    fn remove_sw_breakpoint(
        &mut self,
        addr: <Self::Arch as Arch>::Usize,
    ) -> TargetResult<bool, Self>;
}

define_ext!(SwBreakpointOps, SwBreakpoint);

/// Target Extension - Set/remove Hardware Breakpoints.
///
/// See [this stackoverflow discussion](https://stackoverflow.com/questions/8878716/what-is-the-difference-between-hardware-and-software-breakpoints)
/// about the differences between hardware and software breakpoints.
///
/// _Recommendation:_ If you're implementing `Target` for an emulator that's
/// using an _interpreted_ CPU (as opposed to a JIT), there shouldn't be any
/// reason to implement this extension (as software breakpoints are likely to be
/// just-as-fast).
pub trait HwBreakpoint: Target {
    /// Add a new hardware breakpoint.
    /// Return `Ok(false)` if the operation could not be completed.
    fn add_hw_breakpoint(&mut self, addr: <Self::Arch as Arch>::Usize) -> TargetResult<bool, Self>;

    /// Remove an existing hardware breakpoint.
    /// Return `Ok(false)` if the operation could not be completed.
    fn remove_hw_breakpoint(
        &mut self,
        addr: <Self::Arch as Arch>::Usize,
    ) -> TargetResult<bool, Self>;
}

define_ext!(HwBreakpointOps, HwBreakpoint);

/// Target Extension - Set/remove Hardware Watchpoints.
///
/// See the [GDB documentation](https://sourceware.org/gdb/current/onlinedocs/gdb/Set-Watchpoints.html)
/// regarding watchpoints for how they're supposed to work.
///
/// _NOTE:_ If this extension isn't implemented, GDB will default to using
/// _software watchpoints_, which tend to be excruciatingly slow (as
/// they are implemented by single-stepping the system, and reading the
/// watched memory location after each step).
pub trait HwWatchpoint: Target {
    /// Add a new hardware watchpoint.
    /// Return `Ok(false)` if the operation could not be completed.
    fn add_hw_watchpoint(
        &mut self,
        addr: <Self::Arch as Arch>::Usize,
        kind: WatchKind,
    ) -> TargetResult<bool, Self>;

    /// Remove an existing hardware watchpoint.
    /// Return `Ok(false)` if the operation could not be completed.
    fn remove_hw_watchpoint(
        &mut self,
        addr: <Self::Arch as Arch>::Usize,
        kind: WatchKind,
    ) -> TargetResult<bool, Self>;
}

define_ext!(HwWatchpointOps, HwWatchpoint);
