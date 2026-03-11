// Z80/SM83 calling convention (SDCC __sdcccall)
//
// The actual register assignment (A, HL, DE, etc.) is handled by the
// LLVM backend (Z80CallLowering.cpp). This module only tells rustc
// how to lower aggregates.
//
// For extern "C" (sdcccall(1)):
//   - All aggregates are passed/returned indirectly (sret/byval pointer)
//   - SDCC does not pass structs in registers
//
// For Rust ABI and other ABIs:
//   - Aggregates ≤ 32 bits may be passed directly (LLVM handles register assignment)
//   - Aggregates > 32 bits are passed indirectly

use rustc_abi::TyAbiInterface;
use rustc_abi::CanonAbi;

use crate::callconv::{ArgAbi, FnAbi};

fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>, is_c_abi: bool) {
    if ret.layout.is_aggregate() {
        if is_c_abi || ret.layout.size.bits() > 32 {
            ret.make_indirect();
        }
    }
}

fn classify_arg<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>, is_c_abi: bool)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.layout.pass_indirectly_in_non_rustic_abis(cx) {
        arg.make_indirect();
        return;
    }
    if arg.layout.is_aggregate() {
        if is_c_abi || arg.layout.size.bits() > 32 {
            arg.make_indirect();
        }
    }
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    let is_c_abi = matches!(fn_abi.conv, CanonAbi::C { .. });

    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret, is_c_abi);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(cx, arg, is_c_abi);
    }
}
