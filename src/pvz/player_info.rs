
#[repr(C)]
pub struct PlayerInfo {
    _pad_0x0_0x20: [u8; 0x20 - 0x0],
    /// 存档槽位
    pub save_slot: u32,
    _pad_0x24_0x4818: [u8; 0x4818 - 0x24],
}
const _: () = assert!(size_of::<PlayerInfo>() == 0x4818);
