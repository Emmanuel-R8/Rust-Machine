// #![allow(non_camel)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unused_variables)]
extern crate num;
extern crate num_derive;

pub mod rust_library {
    pub mod library;
}


pub mod library  {
    pub mod emulator {
        pub mod config;
        pub mod disassembly;
        pub mod instructions {
            pub mod common;

        }
    }
}

mod ui {
    pub mod termui;
}

//
// CRATE FILES
//
//
// STANDARD IMPORTS
//
use std::fs::File;

//// EXTERNAL IMPORTS
//
use anyhow::{Context, Result};
use simplelog::{Config, LevelFilter, WriteLogger};

//
// LOCAL IMPORTS
//
use crate::library;
use emulator::{config::VLMConfig, emulator::GlobalContext};
use emulator::instructions::def_build_set::build_instruction_vec_map;

use ui::termui::{restore_terminal, run_app, setup_terminal, AppUI};


//
//
pub fn main() -> Result<()> {
    ///////////////////////////////////////////////////////////////////////////
    //
    // Global state
    //
    let mut ctx: GlobalContext = GlobalContext::new();
    let instruction_vec_map = build_instruction_vec_map();

    ctx.cpu.initialise();

    let mut args: Vec<String> = Vec::new();
    for arg in ::std::env::args() {
        args.push(arg);
    }

    // Set up log files
    let ivory_page_log = WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("ivoryPageLog.log").unwrap(),
    );
    let run_log = WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("run.log").unwrap(),
    );

    let world_image_size: usize = 0;
    let world_image_mb: usize = 0;
    let message: &str = "";
    let reason: u32 = 0;

    let config = VLMConfig::default();
    let enable_ids_p = config.enable_ids;
    let trace_p = config.tracing.trace_post;

    // let TestFunction = config.testFunction;

    // initialize_ivory_processor(map_virtual_address_data(0), map_virtual_address_tag(0));
    // initialize_life_support(&mut config);

    // if pthread_key_create(&mut mainThread, None) != 0 {
    //     vpunt("Unable to establish per-thread data.");
    // }

    // let worldImageSize = load_world(&mut config);
    // load_vlm_debugger(&mut config);
    // let world_image_MB = (5 * worldImageSize + 1024 * 1024 - 1) / (1024 * 1024);

    // if world_image_MB > config.virtualMemory {
    //     vpunt(
    //         "World file {} won't fit within the requested virtual memory {}",
    //         config.worldPath,
    //         config.virtualMemory,
    //     );
    // }
    // if 2 * world_image_MB > config.virtualMemory {
    //     vwarn(
    //         "Only {} of virtual memory unused after loading world file {}",
    //         config.virtualMemory - world_image_MB,
    //         config.worldPath,
    //     );
    // }
    // VirtualMemoryWrite(
    //     (ADDRESS_NIL)
    //         .wrapping_div(::std::mem::size_of::<EmbWord>())
    //         .wrapping_add(enableSysoutAtColdBoot),
    //     if EnableIDS == true {
    //         ADDRESS_T as *mut LispObj
    //     } else {
    //         ADDRESS_NIL as *mut LispObj
    //     },
    // );

    // EmbCommAreaPtr.virtualMemorySize = (config.virtualMemory)
    //     .wrapping_mul(1024)
    //     .wrapping_mul(1024)
    //     .wrapping_add(4)
    //     .wrapping_div(5) as EmbWord;

    // (*EmbCommAreaPtr).worldImageSize = worldImageSize as EmbWord;

    // while Runningp() != 0 {
    //     reason = InstructionSequencer();
    //     if reason != 0 {
    //         match reason {
    //             1 => {
    //                 message = "Unimplemented instruction";
    //             }
    //             2 => {
    //                 message = "";
    //             }
    //             3 => {
    //                 message = "";
    //             }
    //             4 => {
    //                 message = "Stack overflow while not in emulator mode";
    //             }
    //             5 => {
    //                 message = "Illegal trap vector contents";
    //             }
    //             _ => {
    //                 message = "Halted for unknown reason";
    //             }
    //         }
    //         if !message.is_null() {
    //             vwarn(
    //                 "{} at PC {}",
    //                 message,
    //                 processor.pc.whole >> 1,
    //                 if processor.pc.whole & 1 != 0 {
    //                     "Odd"
    //                 } else {
    //                     "Even"
    //                 },
    //             );
    //         }
    //     }
    //     if 2 == reason {
    //         break;
    //     }
    // }

    ///////////////////////////////////////////////////////////////////////////
    //
    // UI terminal
    //

    // setup terminal
    let mut terminal_ui = setup_terminal()?;

    // create app and run it
    let app = AppUI::new();
    run_app(&mut terminal_ui, app).context("app loop failed")?;

    // restore terminal
    restore_terminal(&mut terminal_ui)?;

    return Ok(());
}
