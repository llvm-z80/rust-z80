use crate::spec::{
    Arch, Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "sm83-nintendo-none-elf".into(),
        metadata: TargetMetadata {
            description: Some("SM83 / Game Boy (bare-metal, ELF, SDCC ABI)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 16,
        data_layout: "e-m:o-p:16:8-i16:8-i32:8-i64:8-i128:8-f32:8-f64:8-n8:16".into(),
        arch: Arch::Sm83,
        options: TargetOptions {
            c_int_width: 16,
            exe_suffix: ".elf".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("ld.lld".into()),
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
            ..Default::default()
        },
    }
}
