use crate::spec::{
    Arch, Cc, LinkSelfContainedDefault, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target,
    TargetMetadata, TargetOptions, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "z80-unknown-none-elf".into(),
        metadata: TargetMetadata {
            description: Some("Z80 (bare-metal, ELF, SDCC ABI)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 16,
        data_layout: "e-m:o-p:16:8-i16:8-i32:8-i64:8-i128:8-f32:8-f64:8-ve-n8:16".into(),
        arch: Arch::Z80,
        options: TargetOptions {
            cpu: "z80".into(),
            c_int_width: 16,
            exe_suffix: ".elf".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("ld.lld".into()),
            link_self_contained: LinkSelfContainedDefault::True,
            late_link_args: TargetOptions::link_args(
                LinkerFlavor::Gnu(Cc::No, Lld::No),
                &["-lz80_rt"],
            ),
            max_atomic_width: Some(0),
            atomic_cas: false,
            panic_strategy: PanicStrategy::ImmediateAbort,
            relocation_model: RelocModel::Static,
            default_codegen_units: Some(1),
            trap_unreachable: false,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            generate_arange_section: false,
            features: "+inline-i16-runtime".into(),
            // The same LLVM defaults the clang driver passes for these targets.
            llvm_args: cvs![
                "-two-entry-phi-node-folding-threshold=0",
                "-enable-load-in-loop-pre=false"
            ],
            ..Default::default()
        },
    }
}
