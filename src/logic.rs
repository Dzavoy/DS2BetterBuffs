use memory_box::{ModuleContext, LocalPtr};

const GAME_MANAGER_IMP: [Option<u8>; 17] = [
    Some(0x48),
    Some(0x8B),
    Some(0x05),
    None,
    None,
    None,
    None,
    Some(0x48),
    Some(0x8B),
    Some(0x58),
    Some(0x38),
    Some(0x48),
    Some(0x85),
    Some(0xDB),
    Some(0x74),
    None,
    Some(0xF6),
];

pub fn apply() -> Option<()> {
    let param_start: LocalPtr = ModuleContext::current()?
        .pattern_scan(&GAME_MANAGER_IMP)?
        .rip_relative(3, 7)?
        .deref()?
        .chain()
        .offset(0x18)?
        .deref()?
        .offset(0x310)?
        .deref()?
        .offset(0xD8)?
        .deref()?
        .offset(0x1C8)?
        .offset(0x60C)?
        .finish();

    const BUFFS: [isize; 7] = [0x254, 0x274, 0x294, 0x2B4, 0x2D4, 0x2F4, 0x3B4];

    for &off in &BUFFS {
        param_start.offset(off)?.write_f32_protected(0.0)?;
    }

    Some(())
}
