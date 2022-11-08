#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

pub type EmbWord = i32;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;

pub static mut Trace: bool = false;

pub static mut EnableIDS: bool = false;

pub static mut TestFunction: bool = false;
static mut mainThread: pthread_key_t = 0;
 fn MaybeTerminateVLM(mut signal: u32) {
    let mut answer: &str =  "" ;
    let mut answerSize: size_t = 0;
    let mut answerSize_p: *mut size_t = &mut answerSize;
    let mut nRead: ssize_t = 0;
    if (pthread_getspecific(mainThread)).is_null() {
        return;
    }
    if (*EmbCommAreaPtr).guestStatus > StartedGuestStatus  {
        if RunningGuestStatus  == (*EmbCommAreaPtr).guestStatus {
            fprintf(
                stderr,
                b"\nLisp is running!\n\n\0"  ,
            );
        } else {
            fprintf(
                stderr,
                b"\nLisp was running!\n\n\0"  ,
            );
        }
        fprintf(
            stderr,
            b"If you exit, the current state of Lisp will be lost.\n\0"
                ,
        );
        fprintf(
            stderr,
            b"All information in its memory image (e.g., any modified editor\n\0"
                 ,
        );
        fprintf(
            stderr,
            b"buffers) will be irretrievably lost.  Further, Lisp will abandon\n\0"
                 ,
        );
        fprintf(
            stderr,
            b"any tasks it is performing for its clients.\n\n\0"
                ,
        );
        fprintf(
            stderr,
            b"Do you still wish to exit?  (yes or no) \0"
                ,
        );
        fflush(stderr);
        loop {
            nRead = getline(&mut answer, answerSize_p, stdin);
            if nRead < 0   {
                vpunt(

                    b"Unexpected EOF on standard input\0"
                         as&str,
                );
            }
            *answer
                .offset(
                    (nRead - 1  ),
                ) = '\0' as i32 ;
            if 0
                == strcmp(answer, b"yes\0"  )
            {
                break;
            }
            if 0
                == strcmp(answer, b"no\0"  )
            {
                return
            } else {
                fprintf(
                    stderr,
                    b"Please answer 'yes' or 'no'.  \0"
                        ,
                );
                fflush(stderr);
            }
        }
    }
    TerminateLifeSupport();
    _exit(0);
}

unsafe fn main_0(
    mut argc: u32,
    mut argv: *mut&str,
) -> u32 {
    let mut config: VLMConfig = VLMConfig {
        enableSpy: 0,
        tracing: TraceConfig {
            traceP: 0,
            tracePOST: 0,
            bufferSize: 0,
            startPC: 0,
            stopPC: 0,
            outputFile:
        },
        commAreaSize: 0,
        hostBufferSpace: 0,
        guestBufferSpace: 0,
        vlmDebuggerPath: [0; 257],
        worldPath: [0; 257],
        worldSearchPath:
        enableIDS: 0,
        virtualMemory: 0,
        coldLoadXParams: XParams {
            xpHostName:
            xpHostAddress: 0,
            xpDisplay: 0,
            xpScreen: 0,
            xpInitialState: 0,
            xpGeometry:
            xpForegroundColor:
            xpBackgroundColor:
            xpBorderColor:
            xpBorderWidth: 0,
        },
        generaXParams: XParams {
            xpHostName:
            xpHostAddress: 0,
            xpDisplay: 0,
            xpScreen: 0,
            xpInitialState: 0,
            xpGeometry:
            xpForegroundColor:
            xpBackgroundColor:
            xpBorderColor:
            xpBorderWidth: 0,
        },
        diagnosticIPAddress: in_addr { s_addr: 0 },
        interfaces: [NetworkInterface {
            present: 0,
            device: [0; 257],
            myProtocol: 0,
            myAddress: in_addr { s_addr: 0 },
            myOptions: [0; 257],
            anotherAddress: 0 as *mut NetworkInterface,
        }; 8],
        testFunction: 0,
    };
    let mut sigAction: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut worldImageSize: isize = 0;
    let mut worldImageMB: isize = 0;
    let mut message: &str =  "" ;
    let mut reason: u32 = 0;
    BuildConfiguration(&mut config, argc, argv);
    EnableIDS = config.enableIDS;
    TestFunction = config.testFunction;
    Trace = config.tracing.tracePOST;
    InitializeIvoryProcessor(
        MapVirtualAddressData(0 ),
        MapVirtualAddressTag(0 ),
    );
    InitializeLifeSupport(&mut config);
    if pthread_key_create(&mut mainThread, None) != 0 {
        vpunt(

            b"Unable to establish per-thread data.\0"
                as&str,
        );
    }
    pthread_setspecific(mainThread, 1  );
    sigAction
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        Option::<fn(u32) -> ()>,
        sa_handler_t,
    >(Some(MaybeTerminateVLM as fn(u32) -> ()));
    sigemptyset(&mut sigAction.sa_mask);
    sigAction.sa_flags = 0;
    if sigaction(2, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(

            b"Unable to establish SIGINT handler.\0"
                as&str,
        );
    }
    if sigaction(15, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(

            b"Unable to establish SIGTERM handler.\0"
                as&str,
        );
    }
    if sigaction(1, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(

            b"Unable to establish SIGHUP handler.\0"
                as&str,
        );
    }
    if sigaction(3, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(

            b"Unable to establish SIGQUIT handler.\0"
                as&str,
        );
    }
    worldImageSize = LoadWorld(&mut config);
    LoadVLMDebugger(&mut config);
    worldImageMB = (5)
        .wrapping_mul(worldImageSize)
        .wrapping_add((1024  * 1024))
        .wrapping_sub(1)
        .wrapping_div((1024  * 1024));
    if worldImageMB > config.virtualMemory {
        vpunt(

            b"World file %s won't fit within the requested virtual memory (%dMB)\0"
                  as&str,
            (config.worldPath).as_mut_ptr(),
            config.virtualMemory,
        );
    }
    if (2).wrapping_mul(worldImageMB)
        > config.virtualMemory
    {
        vwarn(

            b"Only %dMB of virtual memory unused after loading world file %s\n\0"
                  as&str,
            (config.virtualMemory).wrapping_sub(worldImageMB),
            (config.worldPath).as_mut_ptr(),
        );
    }
    VirtualMemoryWrite(
        (0xf8041100 )
            .wrapping_div(::std::mem::size_of::<EmbWord>())
            .wrapping_add(
                enableSysoutAtColdBoot  ,
            ),
        if EnableIDS  != 0 {
            ADDRESS_T  as *mut LispObj
        } else {
            ADDRESS_NIL  as *mut LispObj
        },
    );
    (*EmbCommAreaPtr)
        .virtualMemorySize = (config.virtualMemory)
        .wrapping_mul(1024)
        .wrapping_mul(1024)
        .wrapping_add(4)
        .wrapping_div(5) as EmbWord;
    (*EmbCommAreaPtr).worldImageSize = worldImageSize as EmbWord;
    while Runningp() != 0 {
        reason = InstructionSequencer();
        if reason != 0 {
            match reason {
                1 => {
                    message = b"Unimplemented instruction\0"
                         as&str;
                }
                2 => {
                    message =  "" ;
                }
                3 => {
                    message =  "" ;
                }
                4 => {
                    message = b"Stack overflow while not in emulator mode\0"
                         as&str;
                }
                5 => {
                    message = b"Illegal trap vector contents\0"
                         as&str;
                }
                _ => {
                    message = b"Halted for unknown reason\0"
                         as&str;
                }
            }
            if !message.is_null() {
                vwarn(

                    b"%s at PC %016x (%s)\0"
                        as&str,
                    message,
                    (*processor).pc.whole >> 1,
                    if (*processor).pc.whole & 1 != 0 {
                        b"Odd\0"
                    } else {
                        b"Even\0"
                    },
                );
            }
        }
        if 2  == reason {
            break;
        }
    }
    exit(0);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1),
                args.as_mut_ptr() as *mut&str,
            ) as i32,
        )
    }
}
