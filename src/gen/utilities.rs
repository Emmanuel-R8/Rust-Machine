#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XrmHashBucketRec;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut&str,
        _: u32,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: u32) -> !;
    fn getenv(__name: *const libc::c_char) ->&str;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> u32;
    fn sprintf(_:&str, _: *const libc::c_char, _: ...) -> u32;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> u32;
    fn strerror(_: u32) ->&str;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> u32;
    fn u64estcancel();
    fn __errno_location() -> *mut u32;
    fn close(__fd: u32) -> u32;
    fn getcwd(__buf:&str, __size: size_t) ->&str;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_:&str, _: *const libc::c_char) ->&str;
    fn strncpy(
        _:&str,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) ->&str;
    fn strcat(_:&str, _: *const libc::c_char) ->&str;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> u32;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> u32;
    fn strdup(_: *const libc::c_char) ->&str;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) ->&str;
    fn strchr(_: *const libc::c_char, _: u32) ->&str;
    fn open(__file: *const libc::c_char, __oflag: u32, _: ...) -> u32;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn ntohl(__netlong: u32) -> u32;
    fn htonl(__hostlong: u32) -> u32;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn XrmInitialize();
    fn XrmPutStringResource(
        _: *mut XrmDatabase,
        _: *const libc::c_char,
        _: *const libc::c_char,
    );
    fn XrmGetResource(
        _: XrmDatabase,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut&str,
        _: *mut XrmValue,
    ) -> u32;
    fn XrmGetFileDatabase(_: *const libc::c_char) -> XrmDatabase;
    fn XrmMergeDatabases(_: XrmDatabase, _: *mut XrmDatabase);
    fn XrmParseCommand(
        _: *mut XrmDatabase,
        _: XrmOptionDescList,
        _: u32,
        _: *const libc::c_char,
        _: *mut u32,
        _: *mut&str,
    );
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> u32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type u8 = libc::c_uchar;
pub type u32 = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: u32,
    pub _IO_read_ptr:&str,
    pub _IO_read_end:&str,
    pub _IO_read_base:&str,
    pub _IO_write_base:&str,
    pub _IO_write_ptr:&str,
    pub _IO_write_end:&str,
    pub _IO_buf_base:&str,
    pub _IO_buf_end:&str,
    pub _IO_save_base:&str,
    pub _IO_backup_base:&str,
    pub _IO_save_end:&str,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: u32,
    pub _flags2: u32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: u32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type u8 = u8;
pub type u32 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: u32,
    pub tz_dsttime: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name:&str,
    pub h_aliases: *mut&str,
    pub h_addrtype: u32,
    pub h_length: u32,
    pub h_addr_list: *mut&str,
}
pub type XPointer =&str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmValue {
    pub size: libc::c_uint,
    pub addr: XPointer,
}
pub type XrmDatabase = *mut _XrmHashBucketRec;
pub type XrmOptionKind = libc::c_uint;
pub const XrmoptionSkipNArgs: XrmOptionKind = 7;
pub const XrmoptionSkipLine: XrmOptionKind = 6;
pub const XrmoptionSkipArg: XrmOptionKind = 5;
pub const XrmoptionResArg: XrmOptionKind = 4;
pub const XrmoptionSepArg: XrmOptionKind = 3;
pub const XrmoptionStickyArg: XrmOptionKind = 2;
pub const XrmoptionIsArg: XrmOptionKind = 1;
pub const XrmoptionNoArg: XrmOptionKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XrmOptionDescRec {
    pub option:&str,
    pub specifier:&str,
    pub argKind: XrmOptionKind,
    pub value: XPointer,
}
pub type XrmOptionDescList = *mut XrmOptionDescRec;
pub type WindowInitialState = u32;
pub const Normal: WindowInitialState = 1;
pub const Unspecified: WindowInitialState = 0;
pub const Iconic: WindowInitialState = -1;
pub type Boole = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XParams {
    pub xpHostName:&str,
    pub xpHostAddress: libc::c_long,
    pub xpDisplay: u32,
    pub xpScreen: u32,
    pub xpInitialState: u32,
    pub xpGeometry:&str,
    pub xpForegroundColor:&str,
    pub xpBackgroundColor:&str,
    pub xpBorderColor:&str,
    pub xpBorderWidth: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetworkInterface {
    pub present: bool,
    pub device: [libc::c_char; 257],
    pub myProtocol: libc::c_ushort,
    pub myAddress: in_addr,
    pub myOptions: [libc::c_char; 257],
    pub anotherAddress: *mut NetworkInterface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceConfig {
    pub traceP: bool,
    pub tracePOST: bool,
    pub bufferSize: u32,
    pub startPC: libc::c_uint,
    pub stopPC: libc::c_uint,
    pub outputFile:&str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLMConfig {
    pub enableSpy: bool,
    pub tracing: TraceConfig,
    pub commAreaSize: size_t,
    pub hostBufferSpace: size_t,
    pub guestBufferSpace: size_t,
    pub vlmDebuggerPath: [libc::c_char; 257],
    pub worldPath: [libc::c_char; 257],
    pub worldSearchPath:&str,
    pub enableIDS: bool,
    pub virtualMemory: size_t,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: in_addr,
    pub interfaces: [NetworkInterface; 8],
    pub testFunction: bool,
}
static mut CommandName: &str = b"genera\0"
     as&str;
 fn PrintMessage(
    mut section:&str,
    mut format:&str,
    mut arguments: ::std::ffi::VaList,
) -> u32 {
    let mut name: [libc::c_char; 128] = [0; 128];
    if section.is_null() {
        sprintf(
            name.as_mut_ptr(),
            b"%s: \0"  ,
            CommandName,
        );
    } else {
        sprintf(
            name.as_mut_ptr(),
            b"%s (%s): \0"  ,
            CommandName,
            section,
        );
    }
    fprintf(stderr, b"%s\0"  , name.as_mut_ptr());
    if !format.is_null() {
        vfprintf(stderr, format, arguments.as_va_list());
        fprintf(stderr, b"\n\0"  );
    }
    return strlen(name.as_mut_ptr());
}

pub  fn vpunt(
    mut section:&str,
    mut format:&str,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut errmsg: &str =  "" ;
    let mut prefixLength: u32 = 0;
    ap = args.clone();
    prefixLength = PrintMessage(section, format, ap.as_va_list());
    if *__errno_location() != 0 {
        errmsg = strerror(*__errno_location());
        if format.is_null() {
            fprintf(stderr, b"%s\n\0"  , errmsg);
        } else {
            fprintf(
                stderr,
                b"%*s%s\n\0"  ,
                prefixLength,
                b"\0"  ,
                errmsg,
            );
        }
    }
    loop {};
}

pub  fn verror(
    mut section:&str,
    mut format:&str,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut errmsg: &str =  "" ;
    let mut prefixLength: u32 = 0;
    ap = args.clone();
    prefixLength = PrintMessage(section, format, ap.as_va_list());
    if *__errno_location() != 0 {
        errmsg = strerror(*__errno_location());
        if format.is_null() {
            fprintf(stderr, b"%s\n\0"  , errmsg);
        } else {
            fprintf(
                stderr,
                b"%*s%s\n\0"  ,
                prefixLength,
                b"\0"  ,
                errmsg,
            );
        }
    }
}

pub  fn vwarn(
    mut section:&str,
    mut format:&str,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut prefixLength: u32 = 0;
    ap = args.clone();
    PrintMessage(section, format, ap.as_va_list());
}

pub  fn SetCommandName(mut newCommandName:&str) {
    CommandName = strndup(newCommandName, 32);
}

pub  fn BuildXDisplayName(
    mut displayName:&str,
    mut hostName:&str,
    mut display: u32,
    mut screen: u32,
) {
    sprintf(
        displayName,
        b"%s\0"  ,
        if hostName.is_null() {
            b"\0"
        } else {
            hostName
        },
    );
    if display != -(1) || screen != -(1) {
        sprintf(displayName, b"%s:\0"  , displayName);
        if display != -(1) {
            sprintf(
                displayName,
                b"%s%d\0"  ,
                displayName,
                display,
            );
        }
        if screen != -(1) {
            sprintf(
                displayName,
                b"%s.%d\0"  ,
                displayName,
                screen,
            );
        }
    }
}

pub  fn BuildConfiguration(
    mut config: *mut VLMConfig,
    mut argc: u32,
    mut argv: *mut&str,
) {
    let mut options: XrmDatabase = 0 as XrmDatabase;
    let mut homeDir: &str =  "" ;
    let mut workingDir: [libc::c_char; 257] = [0; 257];
    let mut configFile: [libc::c_char; 257] = [0; 257];
    XrmInitialize();
    GetDefaultConfiguration(config, &mut options);
    MaybeReadConfigurationFile(
        config,
        &mut options,
        b"VLM.conf\0"   as&str,
    );
    homeDir = getenv(b"HOME\0"  );
    if !homeDir.is_null() {
        sprintf(
            configFile.as_mut_ptr(),
            b"%s/.VLM\0"  ,
            homeDir,
        );
        MaybeReadConfigurationFile(config, &mut options, configFile.as_mut_ptr());
    }
    if !(getcwd(
        workingDir.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 257]>(),
    ))
        .is_null()
    {
        sprintf(
            configFile.as_mut_ptr(),
            b"%s/.VLM\0"  ,
            workingDir.as_mut_ptr(),
        );
        MaybeReadConfigurationFile(config, &mut options, configFile.as_mut_ptr());
    }
    ProcessCommandArguments(config, &mut options, argc, argv);
    InterpretOptions(config, options);
}
 fn GetDefaultConfiguration(
    mut config: *mut VLMConfig,
    mut options: *mut XrmDatabase,
) {
    let mut display: &str =  "" ;
    let mut worldSearchPath: &str =  "" ;
    let mut i: u32 = 0;
    XrmPutStringResource(
        options,
        b"*spy\0"  ,
        b"no\0"  ,
    );
    XrmPutStringResource(
        options,
        b"*trace\0"  ,
        b"no\0"  ,
    );
    XrmPutStringResource(
        options,
        b"*tracePOST\0"  ,
        b"no\0"  ,
    );
    XrmPutStringResource(
        options,
        b"*testfunction\0"  ,
        b"no\0"  ,
    );
    (*config).commAreaSize = 0x1ff80;
    (*config).hostBufferSpace = 15000;
    (*config).guestBufferSpace = 100000;
    XrmPutStringResource(
        options,
        b"*debugger\0"  ,
        b"/usr/lib/symbolics/VLM_debugger\0"  ,
    );
    i = 0;
    while i < 8  {
        (*config).interfaces[i ].present = false;
        i += 1;
    }
    XrmPutStringResource(
        options,
        b"genera.world\0"  ,
        b"/usr/lib/symbolics/Genera-8-5.vlod\0"  ,
    );
    XrmPutStringResource(
        options,
        b"minima.world\0"  ,
        b"/usr/lib/symbolics/Minima.mlod\0"  ,
    );
    worldSearchPath = getenv(b"WORLDPATH\0"  );
    if !worldSearchPath.is_null() {
        XrmPutStringResource(
            options,
            b"genera.worldSearchPath\0"  ,
            MergeSearchPaths(
                worldSearchPath,
                b"/var/lib/symbolics:/usr/lib/symbolics\0"
                     as&str,
            ),
        );
    } else {
        XrmPutStringResource(
            options,
            b"genera.worldSearchPath\0"  ,
            b"/var/lib/symbolics:/usr/lib/symbolics\0"
                ,
        );
    }
    XrmPutStringResource(
        options,
        b"genera.enableIDS\0"  ,
        b"no\0"  ,
    );
    XrmPutStringResource(
        options,
        b"genera.virtualMemory\0"  ,
        b"200\0"  ,
    );
    display = getenv(b"DISPLAY\0"  );
    if !display.is_null() {
        XrmPutStringResource(
            options,
            b"*display\0"  ,
            display,
        );
    } else {
        XrmPutStringResource(
            options,
            b"*display\0"  ,
            b":0.0\0"  ,
        );
    }
    XrmPutStringResource(
        options,
        b"*coldLoad.iconic\0"  ,
        b"yes\0"  ,
    );
}
 fn MaybeReadConfigurationFile(
    mut config: *mut VLMConfig,
    mut options: *mut XrmDatabase,
    mut pathname:&str,
) {
    let mut fileOptions: XrmDatabase = 0 as *mut _XrmHashBucketRec;
    let mut newSearchPath: [libc::c_char; 4096] = [0; 4096];
    let mut oldSearchPath: [libc::c_char; 4096] = [0; 4096];
    let mut mergedSearchPath: &str =  "" ;
    let mut searchPathOption: [libc::c_char; 128] = [0; 128];
    let mut fd: u32 = 0;
    fd = open(pathname, 0);
    if -(1) == fd {
        if 2  == *__errno_location() {
            *__errno_location() = 0;
            return;
        } else {
            vpunt(

                b"Unable to verify existence of configuration file %s\0"
                     as&str,
                pathname,
            );
        }
    }
    close(fd);
    fileOptions = XrmGetFileDatabase(pathname);
    if fileOptions.is_null() {
        vpunt(

            b"Unable to parse configuration file %s\0"
                 as&str,
            pathname,
        );
    }
    if GetOption(
        fileOptions,
        b"worldSearchPath\0"   as&str,
        b"WorldSearchPath\0"   as&str,
        newSearchPath.as_mut_ptr(),
    ) != 0
    {
        GetOption(
            *options,
            b"worldSearchPath\0"
                as&str,
            b"WorldSearchPath\0"
                as&str,
            oldSearchPath.as_mut_ptr(),
        );
        mergedSearchPath = MergeSearchPaths(
            newSearchPath.as_mut_ptr(),
            oldSearchPath.as_mut_ptr(),
        );
        sprintf(
            searchPathOption.as_mut_ptr(),
            b"%s.worldSearchPath\0"  ,
            CommandName,
        );
        XrmPutStringResource(
            &mut fileOptions,
            searchPathOption.as_mut_ptr(),
            mergedSearchPath,
        );
    }
    XrmMergeDatabases(fileOptions, options);
}
static mut OptionsTable: [XrmOptionDescRec; 33] = [
    {
        let mut init = XrmOptionDescRec {
            option: b"-spy\0"   as&str,
            specifier: b".spy\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-diagnostic\0"
                as&str,
            specifier: b".diagnosticHost\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-testfunction\0"
                as&str,
            specifier: b".testfunction\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-world\0"   as&str,
            specifier: b".world\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-network\0"
                as&str,
            specifier: b".network\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-debugger\0"
                as&str,
            specifier: b".debugger\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-ids\0"   as&str,
            specifier: b".enableIDS\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-vm\0"   as&str,
            specifier: b".virtualMemory\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-display\0"
                as&str,
            specifier: b".main.display\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-geometry\0"
                as&str,
            specifier: b".main.geometry\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-iconic\0"
                as&str,
            specifier: b".main.iconic\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-foreground\0"
                as&str,
            specifier: b".main.foreground\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-fg\0"   as&str,
            specifier: b".main.foreground\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-background\0"
                as&str,
            specifier: b".main.background\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-bg\0"   as&str,
            specifier: b".main.background\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-bordercolor\0"
                as&str,
            specifier: b".main.borderColor\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-bd\0"   as&str,
            specifier: b".main.borderColor\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-borderwidth\0"
                as&str,
            specifier: b".main.borderWidth\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-bw\0"   as&str,
            specifier: b".main.borderWidth\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloaddisplay\0"
                as&str,
            specifier: b".coldLoad.display\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-cld\0"   as&str,
            specifier: b".coldLoad.display\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloadgeometry\0"
                as&str,
            specifier: b".coldLoad.geometry\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-clg\0"   as&str,
            specifier: b".coldLoad.geometry\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloadiconic\0"
                as&str,
            specifier: b".coldLoad.iconic\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-cli\0"   as&str,
            specifier: b".coldLoad.iconic\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloadforeground\0"
                as&str,
            specifier: b".coldLoad.foreground\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-clfg\0"   as&str,
            specifier: b".coldLoad.foreground\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloadbackground\0"
                as&str,
            specifier: b".coldLoad.background\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-clbg\0"   as&str,
            specifier: b".coldLoad.background\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloadbordercolor\0"
                as&str,
            specifier: b".coldLoad.borderColor\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-clbd\0"   as&str,
            specifier: b".coldLoad.borderColor\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-coldloadborderwidth\0"
                as&str,
            specifier: b".coldLoad.borderWidth\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
    {
        let mut init = XrmOptionDescRec {
            option: b"-clbw\0"   as&str,
            specifier: b".coldLoad.borderWidth\0"
                as&str,
            argKind: XrmoptionSepArg,
            value: 0  as XPointer,
        };
        init
    },
];
 fn ProcessCommandArguments(
    mut config: *mut VLMConfig,
    mut options: *mut XrmDatabase,
    mut argc: u32,
    mut argv: *mut&str,
) {
    let mut oldSearchPath: [libc::c_char; 4096] = [0; 4096];
    let mut mergedSearchPath: &str =  "" ;
    let mut searchPathOption: [libc::c_char; 128] = [0; 128];
    let mut argLength: u32 = 0;
    XrmParseCommand(
        options,
        OptionsTable.as_mut_ptr(),
        33  + 0,
        CommandName,
        &mut argc,
        argv,
    );
    while argc > 1  {
        argv = argv.offset(1);
        argc -= 1;
        argLength = strlen(*argv);
        if 0
            == strncmp(
                *argv,
                b"-searchpath\0"  ,
                (if argLength < 7  { 7  } else { argLength })
                  ,
            )
        {
            if argc > 1  {
                argv = argv.offset(1);
                argc -= 1;
                GetOption(
                    *options,
                    b"worldSearchPath\0"
                        as&str,
                    b"WorldSearchPath\0"
                        as&str,
                    oldSearchPath.as_mut_ptr(),
                );
                mergedSearchPath = MergeSearchPaths(*argv, oldSearchPath.as_mut_ptr());
                sprintf(
                    searchPathOption.as_mut_ptr(),
                    b"%s.worldSearchPath\0"  ,
                    CommandName,
                );
                XrmPutStringResource(
                    options,
                    searchPathOption.as_mut_ptr(),
                    mergedSearchPath,
                );
            } else {
                vpunt(

                    b"A list of directory pathnames must follow -searchpath\0"
                          as&str,
                );
            }
        } else {
            vpunt(

                b"Unrecognized option %s\0"
                    as&str,
                *argv,
            );
        }
    }
}
 fn InterpretOptions(
    mut config: *mut VLMConfig,
    mut options: XrmDatabase,
) {
    let mut interface: *mut NetworkInterface = 0 as *mut NetworkInterface;
    let mut value: [libc::c_char; 4096] = [0; 4096];
    let mut hostName: &str =  "" ;
    let mut start: &str =  "" ;
    let mut end: &str =  "" ;
    let mut end2: &str =  "" ;
    let mut hostAddress: libc::c_ulong = 0;
    let mut datum: libc::c_ulong = 0;
    let mut i: u32 = 0;
    GetOption(
        options,
        b"spy\0"   as&str,
        b"Spy\0"   as&str,
        value.as_mut_ptr(),
    );
    if 0
        == strcmp(value.as_mut_ptr(), b"yes\0"  )
    {
        (*config).enableSpy = true;
    } else if 0
        == strcmp(value.as_mut_ptr(), b"no\0"  )
    {
        (*config).enableSpy = false;
    } else {
        vpunt(

            b"Value of spy parameter, %s, is invalid\0"
                 as&str,
            value.as_mut_ptr(),
        );
    }
    GetOption(
        options,
        b"testfunction\0"   as&str,
        b"TestFunction\0"   as&str,
        value.as_mut_ptr(),
    );
    if 0
        == strcmp(value.as_mut_ptr(), b"yes\0"  )
    {
        (*config).testFunction = true;
    } else if 0
        == strcmp(value.as_mut_ptr(), b"no\0"  )
    {
        (*config).testFunction = false;
    } else {
        vpunt(

            b"Value of testfunction parameter, %s, is invalid\0"
                 as&str,
            value.as_mut_ptr(),
        );
    }
    (*config).tracing.traceP = false;
    (*config).tracing.tracePOST = false;
    GetOption(
        options,
        b"world\0"   as&str,
        b"World\0"   as&str,
        value.as_mut_ptr(),
    );
    strcpy(((*config).worldPath).as_mut_ptr(), value.as_mut_ptr());
    InterpretNetworkOptions(config, options);
    GetOption(
        options,
        b"debugger\0"   as&str,
        b"Debugger\0"   as&str,
        value.as_mut_ptr(),
    );
    strcpy(((*config).vlmDebuggerPath).as_mut_ptr(), value.as_mut_ptr());
    GetOption(
        options,
        b"enableIDS\0"   as&str,
        b"EnableIDS\0"   as&str,
        value.as_mut_ptr(),
    );
    if 0
        == strcmp(value.as_mut_ptr(), b"yes\0"  )
    {
        (*config).enableIDS = true;
    } else if 0
        == strcmp(value.as_mut_ptr(), b"no\0"  )
    {
        (*config).enableIDS = false;
    } else {
        vpunt(

            b"Value of enable IDS parameter, %s, is invalid\0"
                 as&str,
            value.as_mut_ptr(),
        );
    }
    GetOption(
        options,
        b"virtualMemory\0"   as&str,
        b"VirtualMemory\0"   as&str,
        value.as_mut_ptr(),
    );
    datum = strtoul(value.as_mut_ptr(), &mut end, 10);
    if *end != 0 {
        vpunt(

            b"Value of virtual memory size parameter, %s, is invalid\0"
                 as&str,
            value.as_mut_ptr(),
        );
    }
    if datum < 125 {
        vpunt(

            b"Minimum virtual memory size is %d megabytes\0"
                 as&str,
            125,
        );
    }
    (*config).virtualMemory = datum;
    GetOption(
        options,
        b"worldSearchPath\0"   as&str,
        b"WorldSearchPath\0"   as&str,
        value.as_mut_ptr(),
    );
    let ref mut fresh0 = (*config).worldSearchPath;
    *fresh0 = strdup(value.as_mut_ptr());
    InterpretXOptions(
        options,
        &mut (*config).generaXParams,
        b"main X console\0"   as&str,
        b"main\0"   as&str,
        b"Main\0"   as&str,
    );
    InterpretXOptions(
        options,
        &mut (*config).coldLoadXParams,
        b"cold load\0"   as&str,
        b"coldLoad\0"   as&str,
        b"ColdLoad\0"   as&str,
    );
    if (*config).enableSpy != 0 {
        if GetOption(
            options,
            b"diagnosticHost\0"   as&str,
            b"DiagnosticHost\0"   as&str,
            value.as_mut_ptr(),
        ) != 0
        {
            if VerifyHostName(
                value.as_mut_ptr(),
                &mut hostName,
                &mut hostAddress,
                false,
            ) != 0
            {
                memcpy(
                    &mut (*config).diagnosticIPAddress.s_addr as *mut in_addr_t
                        as &str ,
                    &mut hostAddress as *mut libc::c_ulong as&str
                        ,
                    ::std::mem::size_of::<in_addr_t>(),
                );
            } else {
                vpunt(

                    b"Unknown diagnostic host %s\0"
                        as&str,
                    value.as_mut_ptr(),
                );
            }
        } else {
            (*config).diagnosticIPAddress.s_addr = 0  as in_addr_t;
            i = 0;
            while i < 8
                && 0
                    == (*config).diagnosticIPAddress.s_addr
            {
                interface = &mut *((*config).interfaces).as_mut_ptr().offset(i )
                    as *mut NetworkInterface;
                while !interface.is_null() && (*interface).present  != 0 {
                    if 0x800  == (*interface).myProtocol  {
                        (*config)
                            .diagnosticIPAddress
                            .s_addr = htonl((*interface).myAddress.s_addr);
                        break;
                    } else {
                        interface = (*interface).anotherAddress;
                    }
                }
                i += 1;
            }
            if 0 == (*config).diagnosticIPAddress.s_addr {
                vpunt(

                    b"You must specify a diagnostic host to use the spy.\0"
                         as&str,
                );
            }
        }
    }
}
 fn InterpretNetworkOptions(
    mut config: *mut VLMConfig,
    mut options: XrmDatabase,
) {
    let mut mainInterface: *mut NetworkInterface = 0 as *mut NetworkInterface;
    let mut interface: *mut NetworkInterface = 0 as *mut NetworkInterface;
    let mut buffer: [libc::c_char; 4096] = [0; 4096];
    let mut value: &str =  "" ;
    let mut deviceName: &str =  "" ;
    let mut hostName: &str =  "" ;
    let mut commaPosition: &str =  "" ;
    let mut colonPosition: &str =  "" ;
    let mut semicolonPosition: &str =  "" ;
    let mut end: &str =  "" ;
    let mut hostAddress: libc::c_ulong = 0;
    let mut i: u32 = 0;
    if GetOption(
        options,
        b"network\0"   as&str,
        b"Network\0"   as&str,
        buffer.as_mut_ptr(),
    ) == 0
    {
        vpunt(

            b"At least one network interface must be defined\0"
                 as&str,
        );
    }
    value = &mut *buffer.as_mut_ptr().offset(0 )
        as&str;
    while !value.is_null() && *value  != 0 {
        commaPosition = strchr(value, ',' as i32);
        if !commaPosition.is_null() {
            *commaPosition = 0  ;
        }
        colonPosition = strchr(value, ':' as i32);
        semicolonPosition = strchr(value, ';' as i32);
        if !colonPosition.is_null() && !semicolonPosition.is_null()
            && semicolonPosition < colonPosition
        {
            vpunt(

                b"Invalid syntax in specification of network interface: %s\0"
                      as&str,
                value,
            );
        }
        if !colonPosition.is_null() {
            *colonPosition = 0  ;
            deviceName = strdup(value);
            value = colonPosition.offset(1 );
        } else {
            deviceName = b"\0"   as&str;
        }
        interface = 0 as *mut NetworkInterface;
        i = 0;
        while i < 8  {
            if (*config).interfaces[i ].present != 0 {
                if 0
                    == strcmp(
                        deviceName,
                        ((*config).interfaces[i ].device).as_mut_ptr(),
                    )
                {
                    mainInterface = &mut *((*config).interfaces)
                        .as_mut_ptr()
                        .offset(i ) as *mut NetworkInterface;
                    interface = mainInterface;
                    while !((*interface).anotherAddress).is_null() {
                        interface = (*interface).anotherAddress;
                    }
                    let ref mut fresh1 = (*interface).anotherAddress;
                    *fresh1 = malloc(
                        ::std::mem::size_of::<NetworkInterface>(),
                    ) as *mut NetworkInterface;
                    if ((*interface).anotherAddress).is_null() {
                        vpunt(

                            b"Unable to allocate space for an additional network address\0"
                                  as&str,
                        );
                    }
                    interface = (*interface).anotherAddress;
                    break;
                } else {
                    i += 1;
                }
            } else {
                mainInterface = &mut *((*config).interfaces)
                    .as_mut_ptr()
                    .offset(i ) as *mut NetworkInterface;
                interface = mainInterface;
                break;
            }
        }
        if interface.is_null() {
            if !commaPosition.is_null() {
                *commaPosition = ',' as i32 ;
            }
            if !colonPosition.is_null() {
                *colonPosition = ':' as i32 ;
            }
            if !semicolonPosition.is_null() {
                *semicolonPosition = ';' as i32 ;
            }
            vpunt(

                b"Too many distinct network interfaces in %s\0"
                     as&str,
                buffer.as_mut_ptr(),
            );
        }
        strcpy(((*interface).device).as_mut_ptr(), deviceName);
        if !semicolonPosition.is_null() {
            *semicolonPosition = 0  ;
        }
        if 0
            == strncmp(
                value,
                b"CHAOS|\0"  ,
                strlen(b"CHAOS|\0"  ),
            )
            || 0
                == strncmp(
                    value,
                    b"chaos|\0"  ,
                    strlen(b"chaos|\0"  ),
                )
        {
            value = value
                .offset(
                    strlen(b"CHAOS|\0"  ) ,
                );
            (*interface).myProtocol = 0x804  as libc::c_ushort;
            hostAddress = strtoul(value, &mut end, 8);
            if *end != 0 {
                if !colonPosition.is_null() {
                    *colonPosition = ':' as i32 ;
                }
                if !semicolonPosition.is_null() {
                    *semicolonPosition = ';' as i32 ;
                }
                vpunt(

                    b"Invalid chaos address in specification of network interface: %s\0"
                          as&str,
                    value,
                );
            } else {
                (*interface).myAddress.s_addr = ntohl(hostAddress );
            }
        } else if 0
            == strncmp(
                value,
                b"INTERNET|\0"  ,
                strlen(b"INTERNET|\0"  ),
            )
            || 0
                == strncmp(
                    value,
                    b"internet|\0"  ,
                    strlen(b"internet|\0"  ),
                )
        {
            value = value
                .offset(
                    strlen(b"INTERNET|\0"  ) ,
                );
            (*interface).myProtocol = 0x800  as libc::c_ushort;
            hostAddress = ntohl(inet_addr(value));
            if hostAddress == ntohl(-(1) ) {
                if !colonPosition.is_null() {
                    *colonPosition = ':' as i32 ;
                }
                if !semicolonPosition.is_null() {
                    *semicolonPosition = ';' as i32 ;
                }
                vpunt(

                    b"Invalid Internet address in specification of network interface: %s\0"
                          as&str,
                    value,
                );
            } else {
                (*interface).myAddress.s_addr = hostAddress as in_addr_t;
            }
        } else {
            (*interface).myProtocol = 0x800  as libc::c_ushort;
            if VerifyHostName(
                value,
                &mut hostName,
                &mut hostAddress,
                true,
            ) != 0
            {
                memcpy(
                    &mut (*interface).myAddress.s_addr as *mut in_addr_t
                        as &str ,
                    &mut hostAddress as *mut libc::c_ulong as&str
                        ,
                    ::std::mem::size_of::<in_addr_t>(),
                );
                (*interface).myAddress.s_addr = ntohl((*interface).myAddress.s_addr);
            } else {
                if !colonPosition.is_null() {
                    *colonPosition = ':' as i32 ;
                }
                if !semicolonPosition.is_null() {
                    *semicolonPosition = ';' as i32 ;
                }
                vpunt(

                    b"Unknown host in specification of network interface: %s\0"
                          as&str,
                    value,
                );
            }
        }
        if !semicolonPosition.is_null() {
            strcpy(
                ((*interface).myOptions).as_mut_ptr(),
                semicolonPosition.offset(1 ),
            );
        } else {
            (*interface)
                .myOptions[0  ] = 0  ;
        }
        let ref mut fresh2 = (*interface).anotherAddress;
        *fresh2 = 0 as *mut NetworkInterface;
        (*interface).present = true;
        value = if !commaPosition.is_null() {
            commaPosition.offset(1 )
        } else {
             ""
        };
    }
}
 fn InterpretXOptions(
    mut options: XrmDatabase,
    mut xParams: *mut XParams,
    mut windowEnglishName:&str,
    mut windowName:&str,
    mut windowClass:&str,
) {
    let mut value: [libc::c_char; 4096] = [0; 4096];
    let mut hostName: &str =  "" ;
    let mut colonPosition: &str =  "" ;
    let mut start: &str =  "" ;
    let mut end: &str =  "" ;
    let mut hostAddress: libc::c_ulong = 0;
    let mut datum: libc::c_ulong = 0;
    GetXOption(
        options,
        windowName,
        windowClass,
        b"display\0"   as&str,
        b"Display\0"   as&str,
        value.as_mut_ptr(),
    );
    colonPosition = strchr(value.as_mut_ptr(), ':' as i32);
    if !colonPosition.is_null() {
        *colonPosition = 0  ;
        if VerifyHostName(
            value.as_mut_ptr(),
            &mut hostName,
            &mut hostAddress,
            false,
        ) != 0
        {
            let ref mut fresh3 = (*xParams).xpHostName;
            *fresh3 = hostName;
            (*xParams).xpHostAddress = hostAddress ;
        } else {
            vpunt(

                b"Unknown host %s specified for display of %s\0"
                     as&str,
                value.as_mut_ptr(),
                windowEnglishName,
            );
        }
        *colonPosition = ':' as i32 ;
        start = colonPosition.offset(1 );
        datum = strtoul(start, &mut end, 10);
        if start != end {
            (*xParams).xpDisplay = datum;
        }
        if *end != 0 {
            if *end  == '.' as i32 {
                start = end.offset(1 );
                datum = strtoul(start, &mut end, 0);
                if start != end {
                    (*xParams).xpScreen = datum;
                }
                if *end != 0 {
                    vpunt(

                        b"Invalid display specification %s for %s\0"
                             as&str,
                        value.as_mut_ptr(),
                        windowEnglishName,
                    );
                }
            } else {
                vpunt(

                    b"Invalid display specification %s for %s\0"
                         as&str,
                    value.as_mut_ptr(),
                    windowEnglishName,
                );
            }
        } else {
            (*xParams).xpScreen = -(1);
        }
    } else {
        if VerifyHostName(
            value.as_mut_ptr(),
            &mut hostName,
            &mut hostAddress,
            false,
        ) != 0
        {
            let ref mut fresh4 = (*xParams).xpHostName;
            *fresh4 = hostName;
            (*xParams).xpHostAddress = hostAddress ;
        } else {
            vpunt(

                b"Unknown host %s specified for display of %s\0"
                     as&str,
                value.as_mut_ptr(),
                windowEnglishName,
            );
        }
        (*xParams).xpDisplay = -(1);
        (*xParams).xpScreen = -(1);
    }
    if GetXOption(
        options,
        windowName,
        windowClass,
        b"iconic\0"   as&str,
        b"Iconic\0"   as&str,
        value.as_mut_ptr(),
    ) != 0
    {
        if 0
            == strcmp(value.as_mut_ptr(), b"yes\0"  )
        {
            (*xParams).xpInitialState = Iconic;
        } else if 0
            == strcmp(value.as_mut_ptr(), b"no\0"  )
        {
            (*xParams).xpInitialState = Normal;
        } else {
            vpunt(

                b"Invalid value, %s, for iconic state of %s\0"
                     as&str,
                value.as_mut_ptr(),
                windowEnglishName,
            );
        }
    } else {
        (*xParams).xpInitialState = Unspecified;
    }
    if GetXOption(
        options,
        windowName,
        windowClass,
        b"geometry\0"   as&str,
        b"Geometry\0"   as&str,
        value.as_mut_ptr(),
    ) != 0
    {
        let ref mut fresh5 = (*xParams).xpGeometry;
        *fresh5 = strdup(value.as_mut_ptr());
    } else {
        let ref mut fresh6 = (*xParams).xpGeometry;
        *fresh6 =  "" ;
    }
    if GetXOption(
        options,
        windowName,
        windowClass,
        b"foreground\0"   as&str,
        b"Foreground\0"   as&str,
        value.as_mut_ptr(),
    ) != 0
    {
        let ref mut fresh7 = (*xParams).xpForegroundColor;
        *fresh7 = strdup(value.as_mut_ptr());
    } else {
        let ref mut fresh8 = (*xParams).xpForegroundColor;
        *fresh8 =  "" ;
    }
    if GetXOption(
        options,
        windowName,
        windowClass,
        b"background\0"   as&str,
        b"Background\0"   as&str,
        value.as_mut_ptr(),
    ) != 0
    {
        let ref mut fresh9 = (*xParams).xpBackgroundColor;
        *fresh9 = strdup(value.as_mut_ptr());
    } else {
        let ref mut fresh10 = (*xParams).xpBackgroundColor;
        *fresh10 = b"white\0"   as&str;
    }
    if GetXOption(
        options,
        windowName,
        windowClass,
        b"borderColor\0"   as&str,
        b"BorderColor\0"   as&str,
        value.as_mut_ptr(),
    ) != 0
    {
        let ref mut fresh11 = (*xParams).xpBorderColor;
        *fresh11 = strdup(value.as_mut_ptr());
    } else {
        let ref mut fresh12 = (*xParams).xpBorderColor;
        *fresh12 =  "" ;
    }
    if GetXOption(
        options,
        windowName,
        windowClass,
        b"borderWidth\0"   as&str,
        b"BorderWidth\0"   as&str,
        value.as_mut_ptr(),
    ) != 0
    {
        datum = strtoul(value.as_mut_ptr(), &mut end, 10);
        if *end != 0 {
            vpunt(

                b"Invalid value, %s, for border width of %s\0"
                     as&str,
                value.as_mut_ptr(),
                windowEnglishName,
            );
        } else {
            (*xParams).xpBorderWidth = datum;
        }
    } else {
        (*xParams).xpBorderWidth = -(1);
    };
}
 fn MergeSearchPaths(
    mut newSearchPath:&str,
    mut oldSearchPath:&str,
) -> &str {
    newSearchPath = strdup(newSearchPath);
    if 0
        == strncmp(
            newSearchPath,
            b"+:\0"  ,
            2,
        )
    {
        newSearchPath = strcat(
            strdup(&mut *newSearchPath.offset(1 )),
            oldSearchPath,
        );
    }
    if 0
        == strncmp(
            newSearchPath
                .offset(strlen(newSearchPath) )
                .offset(-(2 )),
            b":+\0"  ,
            2,
        )
    {
        *newSearchPath
            .offset(
                (strlen(newSearchPath)).wrapping_sub(1)
                    ,
            ) = 0  ;
        newSearchPath = strcat(newSearchPath, oldSearchPath);
    }
    return newSearchPath;
}
 fn GetOption(
    mut options: XrmDatabase,
    mut name:&str,
    mut class:&str,
    mut value:&str,
) -> Boole {
    let mut optionName: [libc::c_char; 128] = [0; 128];
    let mut optionClass: [libc::c_char; 128] = [0; 128];
    let mut valueClass: &str =  "" ;
    let mut dbValue: XrmValue = XrmValue {
        size: 0,
        addr: 0,
    };
    sprintf(
        optionName.as_mut_ptr(),
        b"%s.%s\0"  ,
        CommandName,
        name,
    );
    sprintf(
        optionClass.as_mut_ptr(),
        b"%s.%s\0"  ,
        b"Genera\0"  ,
        class,
    );
    if XrmGetResource(
        options,
        optionName.as_mut_ptr(),
        optionClass.as_mut_ptr(),
        &mut valueClass,
        &mut dbValue,
    ) != 0
    {
        strncpy(
            value,
            dbValue.addr ,
            dbValue.size,
        );
        return true;
    } else {
        return false
    };
}
 fn GetXOption(
    mut options: XrmDatabase,
    mut windowName:&str,
    mut windowClass:&str,
    mut name:&str,
    mut class:&str,
    mut value:&str,
) -> Boole {
    let mut optionName: [libc::c_char; 128] = [0; 128];
    let mut optionClass: [libc::c_char; 128] = [0; 128];
    sprintf(
        optionName.as_mut_ptr(),
        b"%s.%s\0"  ,
        windowName,
        name,
    );
    sprintf(
        optionClass.as_mut_ptr(),
        b"%s.%s\0"  ,
        windowClass,
        class,
    );
    return GetOption(options, optionName.as_mut_ptr(), optionClass.as_mut_ptr(), value);
}
 fn VerifyHostName(
    mut name:&str,
    mut hostName: *mut&str,
    mut hostAddress: *mut libc::c_ulong,
    mut rejectLocalHost: bool,
) -> Boole {
    let mut hp: *mut hostent = 0 as *mut hostent;
    if *name  == '\0' as i32
        || strcmp(name, b"unix\0"  ) == 0
        || strcmp(name, b"localhost\0"  ) == 0
    {
        if rejectLocalHost != 0 {
            return false;
        }
        hp = gethostbyname(b"localhost\0"  );
        if hp.is_null() {
            vpunt(

                b"Unable to determine local host network address\0"
                     as&str,
            );
        }
        *hostAddress = *(*((*hp).h_addr_list).offset(0 )
            as *mut libc::c_ulong);
        *hostName = if *name  == '\0' as i32 {
             ""
        } else {
            strdup(b"localhost\0"  )
        };
    } else {
        hp = gethostbyname(name);
        if !hp.is_null() {
            *hostAddress = *(*((*hp).h_addr_list).offset(0 )
                as *mut libc::c_ulong);
            *hostName = strdup((*hp).h_name);
        } else {
            *hostAddress = ntohl(inet_addr(name));
            if *hostAddress == ntohl(-(1) ) {
                if 11  == *__errno_location() {
                    *__errno_location() = 0;
                }
                return false;
            } else {
                *hostName = strdup(name);
            }
        }
    }
    return true;
}

pub  fn pthread_get_expiration_np(
    mut delta: *const timespec,
    mut abstime: *mut timespec,
) -> u32 {
    let mut status: u32 = 0;
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut obsolete: timezone = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    status = gettimeofday(&mut now, &mut obsolete as *mut timezone );
    if status == 0  {
        (*abstime).tv_sec = now.tv_sec + (*delta).tv_sec;
        (*abstime)
            .tv_nsec = 1000   * now.tv_usec
            + (*delta).tv_nsec;
        while (*abstime).tv_nsec
            > (1000  * 1000  * 1000)

        {
            let ref mut fresh13 = (*abstime).tv_sec;
            *fresh13 += 1  ;
            let ref mut fresh14 = (*abstime).tv_nsec;
            *fresh14
                -= (1000  * 1000  * 1000)
                    ;
        }
    }
    return status;
}

pub  fn pthread_delay_np(
    mut ointerval: *const timespec,
) -> u32 {
    let mut status: u32 = 0;
    let mut interval: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rinterval: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    interval.tv_sec = (*ointerval).tv_sec;
    interval.tv_nsec = (*ointerval).tv_nsec;
    u64estcancel();
    loop {
        status = nanosleep(&mut interval, &mut rinterval);
        if !(status != 0) {
            break;
        }
        if *__errno_location() != 4  {
            break;
        }
        interval.tv_sec = rinterval.tv_sec;
        interval.tv_nsec = rinterval.tv_nsec;
        u64estcancel();
    }
    return status;
}
