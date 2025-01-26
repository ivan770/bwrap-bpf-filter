/*
 * Adapted from https://github.com/flatpak/flatpak
 *
 * Copyright © 2014-2019 Red Hat, Inc
 * Copyright © 2024 GNOME Foundation, Inc.
 * Copyright © 2025 Ivan Leshchenko
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors:
 *       Alexander Larsson <alexl@redhat.com>
 *       Hubert Figuière <hub@figuiere.net>
 *       Ivan Leshchenko <ivan@ivan770.me>
 */

use std::{fs::File, path::PathBuf};

use clap::{Parser, ValueEnum};
use libseccomp::{scmp_cmp, ScmpAction, ScmpArch, ScmpFilterContext, ScmpSyscall};

macro_rules! syscall {
    ($name:ident, $arch:expr) => {
        ScmpSyscall::from_name_by_arch_rewrite(stringify!($name), $arch)?
    };
}

#[derive(Clone, ValueEnum)]
enum Arch {
    X86_64,
}

impl From<Arch> for ScmpArch {
    fn from(arch: Arch) -> ScmpArch {
        match arch {
            Arch::X86_64 => ScmpArch::X8664,
        }
    }
}

/// Simple syscall filter BPF generator.
#[derive(Parser)]
#[command(about)]
struct Command {
    /// Output architecture.
    target_arch: Arch,

    /// Output path.
    output: PathBuf,

    /// Allow nested sandboxing.
    ///
    /// This flag is usually enabled with applications that attempt to
    /// initialize their own sandbox within a Bubblewrap environment.
    #[arg(long, verbatim_doc_comment)]
    nested_sandboxing: bool,
}

fn main() -> anyhow::Result<()> {
    let command = Command::parse();

    let mut output_file = File::create(command.output)?;

    let mut ctx = ScmpFilterContext::new_filter(ScmpAction::Allow)?;
    let arch = command.target_arch.into();
    ctx.add_arch(arch)?;

    // From https://github.com/flatpak/flatpak/blob/main/common/flatpak-run.c
    ctx.add_rule(ScmpAction::Errno(1), syscall!(syslog, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(uselib, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(acct, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(quotactl, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(add_key, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(keyctl, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(request_key, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(move_pages, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(mbind, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(get_mempolicy, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(set_mempolicy, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(migrate_pages, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(mount, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(umount, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(umount2, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(pivot_root, arch))?;

    if !command.nested_sandboxing {
        ctx.add_rule(ScmpAction::Errno(1), syscall!(chroot, arch))?;
        ctx.add_rule(ScmpAction::Errno(1), syscall!(unshare, arch))?;
        ctx.add_rule(ScmpAction::Errno(1), syscall!(setns, arch))?;

        ctx.add_rule_conditional(
            ScmpAction::Errno(1),
            syscall!(clone, arch),
            &[scmp_cmp!($arg0 & (libc::CLONE_NEWUSER as u64) == libc::CLONE_NEWUSER as u64)],
        )?;
    }

    ctx.add_rule_conditional(
        ScmpAction::Errno(1),
        syscall!(ioctl, arch),
        &[scmp_cmp!($arg1 & 0xFFFFFFFF == libc::TIOCSTI)],
    )?;

    ctx.add_rule_conditional(
        ScmpAction::Errno(1),
        syscall!(ioctl, arch),
        &[scmp_cmp!($arg1 & 0xFFFFFFFF == libc::TIOCLINUX)],
    )?;

    ctx.add_rule(ScmpAction::Errno(38), syscall!(clone3, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(open_tree, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(move_mount, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(fsopen, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(fsconfig, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(fsmount, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(fspick, arch))?;
    ctx.add_rule(ScmpAction::Errno(38), syscall!(mount_setattr, arch))?;

    ctx.add_rule(ScmpAction::Errno(1), syscall!(perf_event_open, arch))?;
    ctx.add_rule(ScmpAction::Errno(1), syscall!(ptrace, arch))?;

    ctx.add_rule_conditional(
        ScmpAction::Errno(1),
        syscall!(personality, arch),
        &[scmp_cmp!($arg0 != 0x0)],
    )?;

    ctx.export_bpf(&mut output_file)?;

    Ok(())
}
