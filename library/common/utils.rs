#[inline]
pub fn ldb(ss: u8, pp: u8, source: u32) -> u32 {
    (source >> pp) & ((1 << ss) - 1)
}

#[inline]
pub fn dpb(field: u32, ss: u8, pp: u8, background: u32) -> u32 {
    ((field & ((1 << ss) - 1)) << pp) | (background & !(((1 << ss) - 1) << pp))
}

#[inline]
pub fn ceiling(n: u32, d: u32) -> u32 {
    (n + (d - 1)) / d
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Logging
//
///////////////////////////////////////////////////////////////////////////////////////////////////

fn setup_logger(filename: &str) -> Result<(), fern::InitError> {
    fern::Dispatch
        ::new()
        .format(|out, message, record| {
            out.finish(
                format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                )
            )
        })
        .level(log::LevelFilter::Debug)
        // In case we want to echo to stdout
        // .chain(std::io::stdout())
        .chain(fern::log_file(filename)?)
        .apply()?;
    return Ok(());
}

// Close world
pub fn close_world(world: World) {}

#[inline]
pub fn byte_swap_32(b32: u32) -> u32 {
    return ((b32 & 0xff00_0000) >> 24)
        | ((b32 & 0x00ff_0000) >> 8)
        | ((b32 & 0x00_ff00) << 8)
        | ((b32 & 0x0000_00ff) << 24);
}

#[inline]
pub fn unpack_32_to_8(val: u32) -> [u8; 4] {
    let b1 = (0xff00_0000 & val) >> 24;
    let b2 = (0x00ff_0000 & val) >> 16;
    let b3 = (0x0000_ff00 & val) >> 8;
    let b4 = (0x0000_00ff & val) >> 0;

    return [b1 as u8, b2 as u8, b3 as u8, b4 as u8];
}

#[inline]
pub fn pack_8_to_32(bytes: [u8; 4]) -> u32 {
    let v1 = (bytes[0] as u32) << 24;
    let v2 = (bytes[1] as u32) << 16;
    let v3 = (bytes[2] as u32) << 8;
    let v4 = (bytes[3] as u32) << 0;

    return v1 + v2 + v3 + v4;
}
