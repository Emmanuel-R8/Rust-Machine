pub fn EmbCommAreaAlloc(mut nBytes: size_t) -> EmbPtr {
    let mut nWords: size_t = nBytes
        .wrapping_add(::std::mem::size_of::<EmbWord>())
        .wrapping_sub(1)
        .wrapping_div(::std::mem::size_of::<EmbWord>());
    let mut thePtr: EmbPtr = EmbCommAreaAllocPtr;
    if nWords & 1 != 0 {
        nWords = nWords.wrapping_add(1);
    }
    if nWords > EmbCommAreaAllocSize || nBytes <= 0 {
        vpunt(
            b"Couldn't allocate %d words in the embedded communications area\0" as &str,
            nWords,
        );
    }
    EmbCommAreaAllocSize = (EmbCommAreaAllocSize).wrapping_sub(nWords);
    EmbCommAreaAllocPtr = (EmbCommAreaAllocPtr).wrapping_add(nWords) as EmbPtr as EmbPtr;
    return thePtr;
}

pub fn MakeEmbString(mut aString: &str) -> EmbPtr {
    let mut theStringPtr: EmbPtr = 0;
    let mut theString: *mut EmbString = 0 as *mut EmbString;
    let mut nBytes: size_t = if aString.is_null() {
        0
    } else {
        strlen(aString)
    };
    let mut datum: u32 = 0;
    if 0 == nBytes {
        return -(1);
    }
    theStringPtr = EmbCommAreaAlloc((::std::mem::size_of::<EmbString>()).wrapping_add(nBytes));
    theString = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(theStringPtr) as *mut EmbWord as PtrV
        as *mut EmbString;
    (*theString).length = nBytes as EmbWord;
    memcpy(
        &mut (*theString).string as *mut EmbWord as &str,
        aString,
        nBytes,
    );
    return theStringPtr;
}
fn ParseVersionNumber(
    mut versionString: &str,
    mut majorVersion: *mut u32,
    mut minorVersion: *mut u32,
) {
    let mut start: &str = "";
    let mut end: &str = "";
    let mut major: u32 = 0;
    let mut minor: u32 = -(1);
    *minorVersion = -(1);
    *majorVersion = *minorVersion;
    start = versionString;
    major = strtoul(start, &mut end, 10);
    if start == end {
        return;
    }
    if *end != 0 {
        if *end == '.' as i32 {
            start = end.offset(1);
            minor = strtoul(start, &mut end, 0);
            if start == end || *end != 0 {
                return;
            }
        } else {
            return;
        }
    }
    *majorVersion = major;
    *minorVersion = minor;
}

pub fn InitializeLifeSupport(mut config: *mut VLMConfig) {
    let mut osfName: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    let mut worldPathname: [libc::c_char; 262] = [0; 262];
    let mut loginName: &str = "";
    let mut identifier: &str = "";
    let mut major: u32 = 0;
    let mut minor: u32 = 0;
    EnsureVirtualAddressRange(
        0xfffe0000,
        (64 + 64).wrapping_add((*config).commAreaSize),
        false,
    );
    BootCommAreaPtr = MapVirtualAddressData(0xfffe0000) as *mut BootCommArea;
    BootDataAreaPtr = MapVirtualAddressData(0xfffe0040) as *mut BootDataArea;
    EmbCommAreaPtr = MapVirtualAddressData(0xfffe0080) as *mut EmbCommArea;
    VirtualMemoryWriteBlockConstant(0xfffe0000, MakeLispObj(0, 0xfffe0000), 64 + 64, 1);
    let mut lispDatum: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum.parts.data.u = 0xfffe0080;
    lispDatum.parts.tag = 25 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((0).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum,
    );
    let mut lispDatum_0: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_0.parts.data.u = SystemTypeVLM;
    lispDatum_0.parts.tag = 8 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((4).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum_0,
    );
    let mut lispDatum_1: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_1.parts.data.u = 0xf8000100;
    lispDatum_1.parts.tag = 25 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((8).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum_1,
    );
    let mut lispDatum_2: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_2.parts.data.u = 0xf00;
    lispDatum_2.parts.tag = 8 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((12).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum_2,
    );
    let mut lispDatum_3: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_3.parts.data.u = 0xfffe0040;
    lispDatum_3.parts.tag = 25 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((28).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum_3,
    );
    let mut lispDatum_4: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_4.parts.data.u = (0xfffe0040)
        .wrapping_add((0).wrapping_div(::std::mem::size_of::<EmbWord>()))
        .wrapping_add(46);
    lispDatum_4.parts.tag = 25 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((20).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum_4,
    );
    let mut lispDatum_5: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_5.parts.data.u = (0xfffe0040)
        .wrapping_add((4).wrapping_div(::std::mem::size_of::<EmbWord>()))
        .wrapping_add(46);
    lispDatum_5.parts.tag = 25 as Tag;
    VirtualMemoryWrite(
        (0xfffe0000).wrapping_add((24).wrapping_div(::std::mem::size_of::<EmbWord>())),
        &mut lispDatum_5,
    );
    EnsureVirtualAddressRange(0xf8041000, 256, false);
    VirtualMemoryWriteBlockConstant(0xf8041000, MakeLispObj(0, 0xf8041000), 256, 1);
    FEPCommAreaPtr = MapVirtualAddressData(0xf8041000) as *mut FEPCommArea;
    EnsureVirtualAddressRange(0xf8041100, 256, false);
    VirtualMemoryWriteBlockConstant(0xf8041100, MakeLispObj(0, 0xf8041100), 256, 1);
    SystemCommAreaPtr = MapVirtualAddressData(0xf8041100) as *mut SystemCommArea;
    VirtualMemoryWriteBlockConstant(0xfffe0080, MakeLispObj(8, 0), (*config).commAreaSize, 0);
    identifier = b"EMBD\0" as &str;
    (*EmbCommAreaPtr).identifier = *(identifier as *mut EmbWord);
    (*EmbCommAreaPtr).version = 1;
    (*EmbCommAreaPtr).system_type = SystemTypeVLM;
    (*EmbCommAreaPtr).number_of_slots =
        (&mut (*EmbCommAreaPtr).pad0 as *mut EmbWord - EmbCommAreaPtr)
            .wrapping_div(::std::mem::size_of::<EmbWord>()) as EmbWord;
    (*EmbCommAreaPtr).comm_memory_size = (*config).commAreaSize as EmbWord;
    let ref mut fresh0 = (*EmbCommAreaPtr).generaVersion;
    (*fresh0).set_major(9);
    let ref mut fresh1 = (*EmbCommAreaPtr).generaVersion;
    (*fresh1).set_minor(0);
    if uname(&mut osfName) < 0 {
        let ref mut fresh2 = (*EmbCommAreaPtr).osfVersion;
        (*fresh2).set_majorRelease(0);
    } else {
        let ref mut fresh3 = (*EmbCommAreaPtr).osfVersion;
        (*fresh3).set_testReleaseP(0);
        if *(*__ctype_b_loc()).offset(osfName.release[0]) & _ISdigit as libc::c_ushort != 0 {
            ParseVersionNumber((osfName.release).as_mut_ptr(), &mut major, &mut minor);
        } else {
            let ref mut fresh4 = (*EmbCommAreaPtr).osfVersion;
            (*fresh4).set_testReleaseP((osfName.release[0] != 'V' as i32));
            ParseVersionNumber(
                &mut *(osfName.release).as_mut_ptr().offset(1),
                &mut major,
                &mut minor,
            );
        }
        let ref mut fresh5 = (*EmbCommAreaPtr).osfVersion;
        (*fresh5).set_majorRelease(major);
        let ref mut fresh6 = (*EmbCommAreaPtr).osfVersion;
        (*fresh6).set_minorRelease(minor);
        ParseVersionNumber((osfName.version).as_mut_ptr(), &mut major, &mut minor);
        let ref mut fresh7 = (*EmbCommAreaPtr).osfVersion;
        (*fresh7).set_majorRevision(major);
        let ref mut fresh8 = (*EmbCommAreaPtr).osfVersion;
        (*fresh8).set_minorRevision(minor);
    }
    (*EmbCommAreaPtr).channel_table = -(1);
    (*EmbCommAreaPtr).consoleChannel = -(1);
    (*EmbCommAreaPtr).cold_load_channel = -(1);
    (*EmbCommAreaPtr).command_channel = -(1);
    (*EmbCommAreaPtr).clock_signal = -(1);
    let ref mut fresh9 = (*EmbCommAreaPtr).slaveTrigger;
    *fresh9 = 0 as u64;
    InitializeSignalHandlers();
    if pthread_key_create(&mut mainThread, None) != 0 {
        vpunt(b"Unable to establish per-thread data.\0" as &str);
    }
    pthread_setspecific(mainThread, 1);
    if atexit(Some(TerminateLifeSupport as fn() -> ())) != 0 {
        vpunt(b"Unable to establish cleanup handler for Life Support\0" as &str);
    }
    SetupThreadAttrs(
        b"polling\0" as &str,
        0,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        &mut (*EmbCommAreaPtr).pollThreadAttrsSetup,
    );
    SetupThreadAttrs(
        b"output\0" as &str,
        2,
        &mut (*EmbCommAreaPtr).outputThreadAttrs,
        &mut (*EmbCommAreaPtr).outputThreadAttrsSetup,
    );
    SetupThreadAttrs(
        b"input\0" as &str,
        3,
        &mut (*EmbCommAreaPtr).inputThreadAttrs,
        &mut (*EmbCommAreaPtr).inputThreadAttrsSetup,
    );
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).signalLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support signal lock\0" as &str);
    }
    (*EmbCommAreaPtr).signalLockSetup = true;
    if pthread_cond_init(
        &mut (*EmbCommAreaPtr).signalSignal,
        0 as *const pthread_condattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support signal signal\0" as &str);
    }
    (*EmbCommAreaPtr).signalSignalSetup = true;
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support signal lock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    if pthread_create(
        &mut (*EmbCommAreaPtr).pollingThread,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        ::std::mem::transmute::<Option<fn(pthread_addr_t) -> ()>, pthread_startroutine_t>(Some(
            IvoryLifePolling as fn(pthread_addr_t) -> (),
        )),
        0,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support polling thread\0" as &str);
    }
    (*EmbCommAreaPtr).pollingThreadSetup = true;
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).clockLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support clock lock\0" as &str);
    }
    (*EmbCommAreaPtr).clockLockSetup = true;
    if pthread_cond_init(
        &mut (*EmbCommAreaPtr).clockSignal,
        0 as *const pthread_condattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support clock signal\0" as &str);
    }
    (*EmbCommAreaPtr).clockSignalSetup = true;
    if pthread_create(
        &mut (*EmbCommAreaPtr).clockThread,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        ::std::mem::transmute::<Option<fn(pthread_addr_t) -> ()>, pthread_startroutine_t>(Some(
            IntervalTimerDriver as fn(pthread_addr_t) -> (),
        )),
        0,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support interval timer thread\0" as &str);
    }
    (*EmbCommAreaPtr).clockThreadSetup = true;
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).XLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the Life Support X library lock\0" as &str);
    }
    (*EmbCommAreaPtr).XLockSetup = true;
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).wakeupLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the VLM wakeup lock\0" as &str);
    }
    (*EmbCommAreaPtr).wakeupLockSetup = true;
    if pthread_cond_init(
        &mut (*EmbCommAreaPtr).wakeupSignal,
        0 as *const pthread_condattr_t,
    ) != 0
    {
        vpunt(b"Unable to create the VLM wakeup signal\0" as &str);
    }
    (*EmbCommAreaPtr).wakeupSignalSetup = true;
    EmbCommAreaAllocPtr = (::std::mem::size_of::<EmbCommArea>())
        .wrapping_div(::std::mem::size_of::<EmbWord>()) as EmbPtr;
    EmbCommAreaAllocSize = ((*EmbCommAreaPtr).comm_memory_size - EmbCommAreaAllocPtr);
    if (*config).worldPath[0] != 0 {
        sprintf(
            worldPathname.as_mut_ptr(),
            b"HOST:%s\0",
            ((*config).worldPath).as_mut_ptr(),
        );
    } else {
        worldPathname[0] = 0;
    }
    (*EmbCommAreaPtr).worldPathname = MakeEmbString(worldPathname.as_mut_ptr());
    loginName = getlogin();
    if !loginName.is_null() {
        (*EmbCommAreaPtr).unixLoginName = MakeEmbString(loginName);
    } else {
        (*EmbCommAreaPtr).unixLoginName = -(1);
    }
    (*EmbCommAreaPtr).unixUID = getuid();
    (*EmbCommAreaPtr).unixGID = getgid();
    InitializeColdLoadChannel(config);
    InitializeConsoleChannel(config);
    InitializeMessageChannels(config);
    InitializeNetworkChannels(config);
    (*EmbCommAreaPtr).host_buffer_start = EmbCommAreaAllocPtr;
    (*EmbCommAreaPtr).host_buffer_size = (*config).hostBufferSpace as EmbWord;
    (*EmbCommAreaPtr).fep_buffer_start = EmbCommAreaAllocPtr + (*EmbCommAreaPtr).host_buffer_size;
    (*EmbCommAreaPtr).fep_buffer_size = 512;
    (*EmbCommAreaPtr).guest_buffer_start = EmbCommAreaAllocPtr
        + (*EmbCommAreaPtr).host_buffer_size
        + (*EmbCommAreaPtr).fep_buffer_size;
    (*EmbCommAreaPtr).guest_buffer_size = EmbCommAreaAllocSize
        .wrapping_sub((*EmbCommAreaPtr).host_buffer_size)
        .wrapping_sub((*EmbCommAreaPtr).fep_buffer_size)
        as EmbWord;
    if ((*EmbCommAreaPtr).guest_buffer_size) < (*config).guestBufferSpace {
        vpunt(

            b"Couldn't allocate %d words for guest buffers in the communcations area; only %d words are available.\0"
                  as&str,
            (*config).guestBufferSpace,
            (*EmbCommAreaPtr).guest_buffer_size,
        );
    }
    (*EmbCommAreaPtr).useSignalLocks = true;
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
            b"Unable to unlock the Life Support signal lock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
}

pub fn TerminateLifeSupport() {
    let mut killSleep: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut exit_code: *mut libc::c_void = 0;
    if (pthread_getspecific(mainThread)).is_null() {
        return;
    }
    TerminateSignalHandlers();
    TerminateColdLoadChannel();
    TerminateConsoleChannel();
    TerminateDiskChannels();
    TerminateMessageChannels();
    exit(1);
}
fn SetupThreadAttrs(
    mut thread_class: &str,
    mut priorityBoost: u32,
    mut threadAttrs: *mut u64,
    mut threadAttrsSetup: *mut Boole,
) {
    let mut stackSize: size_t = 0;
    let mut priority: u32 = 0;
    if pthread_attr_init(threadAttrs) != 0 {
        vpunt(
            b"Unable to create attributes for Life Support %s threads\0" as &str,
            thread_class,
        );
    }
    *threadAttrsSetup = true;
    pthread_attr_getstacksize(threadAttrs, &mut stackSize);
    if pthread_attr_setstacksize(threadAttrs, (4).wrapping_mul(stackSize)) != 0 {
        vpunt(
            b"Unable to set stack size attribute for Life Support %s threads to %d bytes\0" as &str,
            thread_class,
            (4).wrapping_mul(stackSize),
        );
    }
}
