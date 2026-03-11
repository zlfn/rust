use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Z80 Z80InlineAsmRegClass {
        reg,
        reg16,
    }
}

impl Z80InlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! { _: I8; },
            Self::reg16 => types! { _: I16; },
        }
    }
}

def_regs! {
    Z80 Z80InlineAsmReg Z80InlineAsmRegClass {
        a: reg = ["a"],
        b: reg = ["b"],
        c: reg = ["c"],
        d: reg = ["d"],
        e: reg = ["e"],
        h: reg = ["h"],
        l: reg = ["l"],
        bc: reg16 = ["bc"],
        de: reg16 = ["de"],
        hl: reg16 = ["hl"],

        #error = ["sp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
        #error = ["ix"] =>
            "the index register IX is reserved for the frame pointer",
        #error = ["iy"] =>
            "the index register IY is reserved",
    }
}

impl Z80InlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }

    pub fn overlapping_regs(self, mut cb: impl FnMut(Self)) {
        cb(self);
        // 16-bit register pairs overlap with their 8-bit halves.
        match self {
            Self::bc => { cb(Self::b); cb(Self::c); }
            Self::de => { cb(Self::d); cb(Self::e); }
            Self::hl => { cb(Self::h); cb(Self::l); }
            Self::b | Self::c => cb(Self::bc),
            Self::d | Self::e => cb(Self::de),
            Self::h | Self::l => cb(Self::hl),
            _ => {}
        }
    }
}
