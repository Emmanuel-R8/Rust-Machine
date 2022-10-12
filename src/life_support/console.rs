#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _XErrorThreadInfo;
    pub type _X11XCBPrivate;
    pub type _XtransConnInfo;
    pub type _XkbInfoRec;
    pub type _XIMFilter;
    pub type _XContextDB;
    pub type _XDisplayAtoms;
    pub type _XKeytrans;
    pub type _XLockInfo;
    pub type _XrmHashBucketRec;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XCreateGC(
        _: *mut Display,
        _: Drawable,
        _: libc::c_ulong,
        _: *mut XGCValues,
    ) -> GC;
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: u32,
        _: u32,
        _: libc::c_uint,
        _: libc::c_uint,
        _: u32,
    ) -> u32;
    fn XCloseDisplay(_: *mut Display) -> u32;
    fn XConnectionNumber(_: *mut Display) -> u32;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: u32,
        _: u32,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> u32;
    fn XFlush(_: *mut Display) -> u32;
    fn __errno_location() -> *mut u32;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn htonl(__hostlong: ui32) -> ui32;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> u32;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> u32;
    fn pthread_detach(__th: pthread_t) -> u32;
    fn pthread_self() -> pthread_t;
    fn pthread_cancel(__th: pthread_t) -> u32;
    fn pthread_testcancel();
    fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: u32) -> u32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> u32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> u32;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: u32) -> u32;
    fn read(__fd: u32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: u32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pthread_delay_np(interval: *const timespec) -> u32;
    fn MapVirtualAddressData(vma: isize) -> *mut isize;
    fn EmbQueueTakeWord(q: *mut EmbQueue) -> EmbWord;
    fn EmbCommAreaAlloc(nBytes: size_t) -> EmbPtr;
    fn MakeEmbString(aString:&str) -> EmbPtr;
    fn EmbQueuePutWord(q: *mut EmbQueue, element: EmbWord);
    fn EmbQueueFilled(q: *mut EmbQueue) -> u32;
    fn EmbQueueSpace(q: *mut EmbQueue) -> u32;
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn SignalLater(signal: SignalNumber);
    fn InstallSignalHandler(
        singalHandler: ProcPtrV,
        signalArgument: PtrV,
        inputP: Boole,
    ) -> SignalNumber;
    fn CreateQueue(nElements: u32, elementSize: u32) -> EmbPtr;
    fn ResetIncomingQueue(q: *mut EmbQueue);
    fn ResetOutgoingQueue(q: *mut EmbQueue);
    fn vpunt(section:&str, format:&str, _: ...);
    fn BuildXDisplayName(
        displayName:&str,
        hostName:&str,
        display: u32,
        screen: u32,
    );
}
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __caddr_t =&str;
pub type i32 = i32;
pub type u8 = u8;
pub type ui32 = u32;
pub type u64 = u64;
pub type ssize_t = __ssize_t;
pub type caddr_t = __caddr_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: u32,
    pub __count: libc::c_uint,
    pub __owner: u32,
    pub __nusers: libc::c_uint,
    pub __kind: u32,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer =&str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: u32,
    pub next: *mut _XExtData,
    pub free_private: Option::<fn(*mut _XExtData) -> u32>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExtCodes {
    pub extension: u32,
    pub major_opcode: u32,
    pub first_event: u32,
    pub first_error: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: u32,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub arc_mode: u32,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: u32,
    pub ts_y_origin: u32,
    pub font: Font,
    pub subwindow_mode: u32,
    pub graphics_exposures: u32,
    pub clip_x_origin: u32,
    pub clip_y_origin: u32,
    pub clip_mask: Pixmap,
    pub dash_offset: u32,
    pub dashes: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XGC {
    pub ext_data: *mut XExtData,
    pub gid: GContext,
    pub rects: u32,
    pub dashes: u32,
    pub dirty: libc::c_ulong,
    pub values: XGCValues,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: u32,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: u32,
    pub map_entries: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: u32,
    pub nvisuals: u32,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XDisplay {
    pub ext_data: *mut XExtData,
    pub free_funcs: *mut _XFreeFuncs,
    pub fd: u32,
    pub conn_checker: u32,
    pub proto_major_version: u32,
    pub proto_minor_version: u32,
    pub vendor:&str,
    pub resource_base: XID,
    pub resource_mask: XID,
    pub resource_id: XID,
    pub resource_shift: u32,
    pub resource_alloc: Option::<fn(*mut _XDisplay) -> XID>,
    pub byte_order: u32,
    pub bitmap_unit: u32,
    pub bitmap_pad: u32,
    pub bitmap_bit_order: u32,
    pub nformats: u32,
    pub pixmap_format: *mut ScreenFormat,
    pub vnumber: u32,
    pub release: u32,
    pub head: *mut _XSQEvent,
    pub tail: *mut _XSQEvent,
    pub qlen: u32,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub last_req:&str,
    pub buffer:&str,
    pub bufptr:&str,
    pub bufmax:&str,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub synchandler: Option::<fn(*mut _XDisplay) -> u32>,
    pub display_name:&str,
    pub default_screen: u32,
    pub nscreens: u32,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub min_keycode: u32,
    pub max_keycode: u32,
    pub keysyms: *mut KeySym,
    pub modifiermap: *mut XModifierKeymap,
    pub keysyms_per_keycode: u32,
    pub xdefaults:&str,
    pub scratch_buffer:&str,
    pub scratch_length: libc::c_ulong,
    pub ext_number: u32,
    pub ext_procs: *mut _XExten,
    pub event_vec: [Option::<
        fn(*mut Display, *mut XEvent, *mut xEvent) -> u32,
    >; 128],
    pub wire_vec: [Option::<
        fn(*mut Display, *mut XEvent, *mut xEvent) -> u32,
    >; 128],
    pub lock_meaning: KeySym,
    pub lock: *mut _XLockInfo,
    pub async_handlers: *mut _XInternalAsync,
    pub bigreq_size: libc::c_ulong,
    pub lock_fns: *mut _XLockPtrs,
    pub idlist_alloc: Option::<
        fn(*mut Display, *mut XID, u32) -> (),
    >,
    pub key_bindings: *mut _XKeytrans,
    pub cursor_font: Font,
    pub atoms: *mut _XDisplayAtoms,
    pub mode_switch: libc::c_uint,
    pub num_lock: libc::c_uint,
    pub context_db: *mut _XContextDB,
    pub error_vec: *mut Option::<
        fn(*mut Display, *mut XErrorEvent, *mut xError) -> u32,
    >,
    pub cms: C2RustUnnamed_32,
    pub im_filters: *mut _XIMFilter,
    pub qfree: *mut _XSQEvent,
    pub next_event_serial_num: libc::c_ulong,
    pub flushes: *mut _XExten,
    pub im_fd_info: *mut _XConnectionInfo,
    pub im_fd_length: u32,
    pub conn_watchers: *mut _XConnWatchInfo,
    pub watcher_count: u32,
    pub filedes: XPointer,
    pub savedsynchandler: Option::<fn(*mut Display) -> u32>,
    pub resource_max: XID,
    pub xcmisc_opcode: u32,
    pub xkb_info: *mut _XkbInfoRec,
    pub trans_conn: *mut _XtransConnInfo,
    pub xcb: *mut _X11XCBPrivate,
    pub next_cookie: libc::c_uint,
    pub generic_event_vec: [Option::<
        fn(
            *mut Display,
            *mut XGenericEventCookie,
            *mut xEvent,
        ) -> u32,
    >; 128],
    pub generic_event_copy_vec: [Option::<
        fn(
            *mut Display,
            *mut XGenericEventCookie,
            *mut XGenericEventCookie,
        ) -> u32,
    >; 128],
    pub cookiejar: *mut libc::c_void,
    pub error_threads: *mut _XErrorThreadInfo,
    pub exit_handler: XIOErrorExitHandler,
    pub exit_handler_data: *mut libc::c_void,
}
pub type XIOErrorExitHandler = Option::<
    fn(*mut Display, *mut libc::c_void) -> (),
>;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub extension: u32,
    pub evtype: u32,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
pub type xEvent = _xEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xEvent {
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u: C2RustUnnamed_30,
    pub keyButtonPointer: C2RustUnnamed_29,
    pub enterLeave: C2RustUnnamed_28,
    pub focus: C2RustUnnamed_27,
    pub expose: C2RustUnnamed_26,
    pub graphicsExposure: C2RustUnnamed_25,
    pub noExposure: C2RustUnnamed_24,
    pub visibility: C2RustUnnamed_23,
    pub createNotify: C2RustUnnamed_22,
    pub destroyNotify: C2RustUnnamed_21,
    pub unmapNotify: C2RustUnnamed_20,
    pub mapNotify: C2RustUnnamed_19,
    pub mapRequest: C2RustUnnamed_18,
    pub reparent: C2RustUnnamed_17,
    pub configureNotify: C2RustUnnamed_16,
    pub configureRequest: C2RustUnnamed_15,
    pub gravity: C2RustUnnamed_14,
    pub resizeRequest: C2RustUnnamed_13,
    pub circulate: C2RustUnnamed_12,
    pub property: C2RustUnnamed_11,
    pub selectionClear: C2RustUnnamed_10,
    pub selectionRequest: C2RustUnnamed_9,
    pub selectionNotify: C2RustUnnamed_8,
    pub colormap: C2RustUnnamed_7,
    pub mappingNotify: C2RustUnnamed_6,
    pub clientMessage: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub l: C2RustUnnamed_5,
    pub s: C2RustUnnamed_4,
    pub b: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub type_0: CARD32,
    pub bytes: [INT8; 20],
}
pub type INT8 = libc::c_schar;
pub type CARD32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub type_0: CARD32,
    pub shorts0: INT16,
    pub shorts1: INT16,
    pub shorts2: INT16,
    pub shorts3: INT16,
    pub shorts4: INT16,
    pub shorts5: INT16,
    pub shorts6: INT16,
    pub shorts7: INT16,
    pub shorts8: INT16,
    pub shorts9: INT16,
}
pub type INT16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub type_0: CARD32,
    pub longs0: INT32,
    pub longs1: INT32,
    pub longs2: INT32,
    pub longs3: INT32,
    pub longs4: INT32,
}
pub type INT32 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub pad00: CARD32,
    pub request: CARD8,
    pub firstKeyCode: CARD8,
    pub count: CARD8,
    pub pad1: BYTE,
}
pub type BYTE = CARD8;
pub type CARD8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub colormap: CARD32,
    pub new: BOOL,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
}
pub type BOOL = CARD8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub owner: CARD32,
    pub requestor: CARD32,
    pub selection: CARD32,
    pub target: CARD32,
    pub property: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub atom: CARD32,
    pub time: CARD32,
    pub state: BYTE,
    pub pad1: BYTE,
    pub pad2: CARD16,
}
pub type CARD16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub place: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub width: CARD16,
    pub height: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub sibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub valueMask: CARD16,
    pub pad1: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub aboveSibling: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub parent: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub override_0: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
    pub fromConfigure: BOOL,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub pad00: CARD32,
    pub event: CARD32,
    pub window: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub pad00: CARD32,
    pub parent: CARD32,
    pub window: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub override_0: BOOL,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub state: CARD8,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub minorEvent: CARD16,
    pub majorEvent: BYTE,
    pub bpad: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub pad00: CARD32,
    pub drawable: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub minorEvent: CARD16,
    pub count: CARD16,
    pub majorEvent: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub x: CARD16,
    pub y: CARD16,
    pub width: CARD16,
    pub height: CARD16,
    pub count: CARD16,
    pub pad2: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub pad00: CARD32,
    pub window: CARD32,
    pub mode: BYTE,
    pub pad1: BYTE,
    pub pad2: BYTE,
    pub pad3: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub mode: BYTE,
    pub flags: BYTE,
}
pub type KeyButMask = CARD16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub pad00: CARD32,
    pub time: CARD32,
    pub root: CARD32,
    pub event: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub eventX: INT16,
    pub eventY: INT16,
    pub state: KeyButMask,
    pub sameScreen: BOOL,
    pub pad1: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub type_0: BYTE,
    pub detail: BYTE,
    pub sequenceNumber: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnWatchInfo {
    pub fn_0: XConnectionWatchProc,
    pub client_data: XPointer,
    pub next: *mut _XConnWatchInfo,
}
pub type XConnectionWatchProc = Option::<
    fn(
        *mut Display,
        XPointer,
        u32,
        u32,
        *mut XPointer,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XConnectionInfo {
    pub fd: u32,
    pub read_callback: _XInternalConnectionProc,
    pub call_data: XPointer,
    pub watch_data: *mut XPointer,
    pub next: *mut _XConnectionInfo,
}
pub type _XInternalConnectionProc = Option::<
    fn(*mut Display, u32, XPointer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExten {
    pub next: *mut _XExten,
    pub codes: XExtCodes,
    pub create_GC: CreateGCType,
    pub copy_GC: CopyGCType,
    pub flush_GC: FlushGCType,
    pub free_GC: FreeGCType,
    pub create_Font: CreateFontType,
    pub free_Font: FreeFontType,
    pub close_display: CloseDisplayType,
    pub error: ErrorType,
    pub error_string: ErrorStringType,
    pub name:&str,
    pub error_values: PrintErrorType,
    pub before_flush: BeforeFlushType,
    pub next_flush: *mut _XExten,
}
pub type BeforeFlushType = Option::<
    fn(
        *mut Display,
        *mut XExtCodes,
        *const libc::c_char,
        libc::c_long,
    ) -> (),
>;
pub type PrintErrorType = Option::<
    fn(*mut Display, *mut XErrorEvent, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: u32,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
pub type ErrorStringType = Option::<
    fn(
        *mut Display,
        u32,
        *mut XExtCodes,
       &str,
        u32,
    ) ->&str,
>;
pub type ErrorType = Option::<
    fn(
        *mut Display,
        *mut xError,
        *mut XExtCodes,
        *mut u32,
    ) -> u32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xError {
    pub type_0: BYTE,
    pub errorCode: BYTE,
    pub sequenceNumber: CARD16,
    pub resourceID: CARD32,
    pub minorCode: CARD16,
    pub majorCode: CARD8,
    pub pad1: BYTE,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
pub type CloseDisplayType = Option::<
    fn(*mut Display, *mut XExtCodes) -> u32,
>;
pub type FreeFontType = Option::<
    fn(*mut Display, *mut XFontStruct, *mut XExtCodes) -> u32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: libc::c_uint,
    pub min_char_or_byte2: libc::c_uint,
    pub max_char_or_byte2: libc::c_uint,
    pub min_byte1: libc::c_uint,
    pub max_byte1: libc::c_uint,
    pub all_chars_exist: u32,
    pub default_char: libc::c_uint,
    pub n_properties: u32,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: u32,
    pub descent: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: libc::c_short,
    pub rbearing: libc::c_short,
    pub width: libc::c_short,
    pub ascent: libc::c_short,
    pub descent: libc::c_short,
    pub attributes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: libc::c_ulong,
}
pub type CreateFontType = Option::<
    fn(*mut Display, *mut XFontStruct, *mut XExtCodes) -> u32,
>;
pub type FreeGCType = Option::<
    fn(*mut Display, GC, *mut XExtCodes) -> u32,
>;
pub type FlushGCType = Option::<
    fn(*mut Display, GC, *mut XExtCodes) -> u32,
>;
pub type CopyGCType = Option::<
    fn(*mut Display, GC, *mut XExtCodes) -> u32,
>;
pub type CreateGCType = Option::<
    fn(*mut Display, GC, *mut XExtCodes) -> u32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XSQEvent {
    pub next: *mut _XSQEvent,
    pub event: XEvent,
    pub qserial_num: libc::c_ulong,
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: u32,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub extension: u32,
    pub evtype: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub request: u32,
    pub first_keycode: u32,
    pub count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: u32,
    pub data: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: u32,
    pub state: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub above: Window,
    pub detail: u32,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub width: u32,
    pub height: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: u32,
    pub y: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub above: Window,
    pub override_redirect: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: u32,
    pub y: u32,
    pub override_redirect: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub override_redirect: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub state: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: u32,
    pub minor_code: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub major_code: u32,
    pub minor_code: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub mode: u32,
    pub detail: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: u32,
    pub y: u32,
    pub x_root: u32,
    pub y_root: u32,
    pub mode: u32,
    pub detail: u32,
    pub same_screen: u32,
    pub focus: u32,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: u32,
    pub y: u32,
    pub x_root: u32,
    pub y_root: u32,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: u32,
    pub y: u32,
    pub x_root: u32,
    pub y_root: u32,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: u32,
    pub y: u32,
    pub x_root: u32,
    pub y_root: u32,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub defaultCCCs: XPointer,
    pub clientCmaps: XPointer,
    pub perVisualIntensityMaps: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XLockPtrs {
    pub lock_display: Option::<fn(*mut Display) -> ()>,
    pub unlock_display: Option::<fn(*mut Display) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XInternalAsync {
    pub next: *mut _XInternalAsync,
    pub handler: Option::<
        fn(
            *mut Display,
            *mut xReply,
           &str,
            u32,
            XPointer,
        ) -> u32,
    >,
    pub data: XPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union xReply {
    pub generic: xGenericReply,
    pub geom: xGetGeometryReply,
    pub tree: xQueryTreeReply,
    pub atom: xInternAtomReply,
    pub atomName: xGetAtomNameReply,
    pub property: xGetPropertyReply,
    pub listProperties: xListPropertiesReply,
    pub selection: xGetSelectionOwnerReply,
    pub grabPointer: xGrabPointerReply,
    pub grabKeyboard: xGrabKeyboardReply,
    pub pointer: xQueryPointerReply,
    pub motionEvents: xGetMotionEventsReply,
    pub coords: xTranslateCoordsReply,
    pub inputFocus: xGetInputFocusReply,
    pub textExtents: xQueryTextExtentsReply,
    pub fonts: xListFontsReply,
    pub fontPath: xGetFontPathReply,
    pub image: xGetImageReply,
    pub colormaps: xListInstalledColormapsReply,
    pub allocColor: xAllocColorReply,
    pub allocNamedColor: xAllocNamedColorReply,
    pub colorCells: xAllocColorCellsReply,
    pub colorPlanes: xAllocColorPlanesReply,
    pub colors: xQueryColorsReply,
    pub lookupColor: xLookupColorReply,
    pub bestSize: xQueryBestSizeReply,
    pub extension: xQueryExtensionReply,
    pub extensions: xListExtensionsReply,
    pub setModifierMapping: xSetModifierMappingReply,
    pub getModifierMapping: xGetModifierMappingReply,
    pub setPointerMapping: xSetPointerMappingReply,
    pub getKeyboardMapping: xGetKeyboardMappingReply,
    pub getPointerMapping: xGetPointerMappingReply,
    pub pointerControl: xGetPointerControlReply,
    pub screenSaver: xGetScreenSaverReply,
    pub hosts: xListHostsReply,
    pub error: xError,
    pub event: xEvent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListHostsReply {
    pub type_0: BYTE,
    pub enabled: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nHosts: CARD16,
    pub pad1: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetScreenSaverReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub timeout: CARD16,
    pub interval: CARD16,
    pub preferBlanking: BOOL,
    pub allowExposures: BOOL,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerControlReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub accelNumerator: CARD16,
    pub accelDenominator: CARD16,
    pub threshold: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPointerMappingReply {
    pub type_0: BYTE,
    pub nElts: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetKeyboardMappingReply {
    pub type_0: BYTE,
    pub keySymsPerKeyCode: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
pub type xSetMappingReply = xSetPointerMappingReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xSetPointerMappingReply {
    pub type_0: BYTE,
    pub success: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetModifierMappingReply {
    pub type_0: BYTE,
    pub numKeyPerModifier: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
pub type xSetModifierMappingReply = xSetMappingReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListExtensionsReply {
    pub type_0: BYTE,
    pub nExtensions: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryExtensionReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub present: BOOL,
    pub major_opcode: CARD8,
    pub first_event: CARD8,
    pub first_error: CARD8,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryBestSizeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub width: CARD16,
    pub height: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xLookupColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryColorsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColors: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorPlanesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub pad2: CARD16,
    pub redMask: CARD32,
    pub greenMask: CARD32,
    pub blueMask: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorCellsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPixels: CARD16,
    pub nMasks: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocNamedColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pixel: CARD32,
    pub exactRed: CARD16,
    pub exactGreen: CARD16,
    pub exactBlue: CARD16,
    pub screenRed: CARD16,
    pub screenGreen: CARD16,
    pub screenBlue: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xAllocColorReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub red: CARD16,
    pub green: CARD16,
    pub blue: CARD16,
    pub pad2: CARD16,
    pub pixel: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListInstalledColormapsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nColormaps: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetImageReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub visual: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetFontPathReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nPaths: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListFontsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nFonts: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTextExtentsReply {
    pub type_0: BYTE,
    pub drawDirection: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub fontAscent: INT16,
    pub fontDescent: INT16,
    pub overallAscent: INT16,
    pub overallDescent: INT16,
    pub overallWidth: INT32,
    pub overallLeft: INT32,
    pub overallRight: INT32,
    pub pad: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetInputFocusReply {
    pub type_0: BYTE,
    pub revertTo: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub focus: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xTranslateCoordsReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub child: CARD32,
    pub dstX: INT16,
    pub dstY: INT16,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetMotionEventsReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nEvents: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryPointerReply {
    pub type_0: BYTE,
    pub sameScreen: BOOL,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub child: CARD32,
    pub rootX: INT16,
    pub rootY: INT16,
    pub winX: INT16,
    pub winY: INT16,
    pub mask: CARD16,
    pub pad1: CARD16,
    pub pad: CARD32,
}
pub type xGrabPointerReply = xGrabKeyboardReply;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGrabKeyboardReply {
    pub type_0: BYTE,
    pub status: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetSelectionOwnerReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub owner: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xListPropertiesReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nProperties: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetPropertyReply {
    pub type_0: BYTE,
    pub format: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub propertyType: CARD32,
    pub bytesAfter: CARD32,
    pub nItems: CARD32,
    pub pad1: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetAtomNameReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub nameLength: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
    pub pad7: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xInternAtomReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub atom: CARD32,
    pub pad2: CARD32,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
    pub pad6: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xQueryTreeReply {
    pub type_0: BYTE,
    pub pad1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub parent: CARD32,
    pub nChildren: CARD16,
    pub pad2: CARD16,
    pub pad3: CARD32,
    pub pad4: CARD32,
    pub pad5: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGetGeometryReply {
    pub type_0: BYTE,
    pub depth: CARD8,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub root: CARD32,
    pub x: INT16,
    pub y: INT16,
    pub width: CARD16,
    pub height: CARD16,
    pub borderWidth: CARD16,
    pub pad1: CARD16,
    pub pad2: CARD32,
    pub pad3: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xGenericReply {
    pub type_0: BYTE,
    pub data1: BYTE,
    pub sequenceNumber: CARD16,
    pub length: CARD32,
    pub data00: CARD32,
    pub data01: CARD32,
    pub data02: CARD32,
    pub data03: CARD32,
    pub data04: CARD32,
    pub data05: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: u32,
    pub modifiermap: *mut KeyCode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: u32,
    pub height: u32,
    pub mwidth: u32,
    pub mheight: u32,
    pub ndepths: u32,
    pub depths: *mut Depth,
    pub root_depth: u32,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: u32,
    pub min_maps: u32,
    pub backing_store: u32,
    pub save_unders: u32,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: u32,
    pub bits_per_pixel: u32,
    pub scanline_pad: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XFreeFuncs {
    pub atoms: FreeFuncType,
    pub modifiermap: FreeModmapType,
    pub key_bindings: FreeFuncType,
    pub context_db: FreeFuncType,
    pub defaultCCCs: FreeFuncType,
    pub clientCmaps: FreeFuncType,
    pub intensityMaps: FreeFuncType,
    pub im_filters: FreeFuncType,
    pub xkb: FreeFuncType,
}
pub type FreeFuncType = Option::<fn(*mut Display) -> ()>;
pub type FreeModmapType = Option::<
    fn(*mut XModifierKeymap) -> u32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xConnSetupPrefix {
    pub success: CARD8,
    pub lengthReason: BYTE,
    pub majorVersion: CARD16,
    pub minorVersion: CARD16,
    pub length: CARD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xConnSetup {
    pub release: CARD32,
    pub ridBase: CARD32,
    pub ridMask: CARD32,
    pub motionBufferSize: CARD32,
    pub nbytesVendor: CARD16,
    pub maxRequestSize: CARD16,
    pub numRoots: CARD8,
    pub numFormats: CARD8,
    pub imageByteOrder: CARD8,
    pub bitmapBitOrder: CARD8,
    pub bitmapScanlineUnit: CARD8,
    pub bitmapScanlinePad: CARD8,
    pub minKeyCode: CARD8,
    pub maxKeyCode: CARD8,
    pub pad2: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xPixmapFormat {
    pub depth: CARD8,
    pub bitsPerPixel: CARD8,
    pub scanLinePad: CARD8,
    pub pad1: CARD8,
    pub pad2: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xDepth {
    pub depth: CARD8,
    pub pad1: CARD8,
    pub nVisuals: CARD16,
    pub pad2: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xVisualType {
    pub visualID: CARD32,
    pub class: CARD8,
    pub bitsPerRGB: CARD8,
    pub colormapEntries: CARD16,
    pub redMask: CARD32,
    pub greenMask: CARD32,
    pub blueMask: CARD32,
    pub pad: CARD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xWindowRoot {
    pub windowId: CARD32,
    pub defaultColormap: CARD32,
    pub whitePixel: CARD32,
    pub blackPixel: CARD32,
    pub currentInputMask: CARD32,
    pub pixWidth: CARD16,
    pub pixHeight: CARD16,
    pub mmWidth: CARD16,
    pub mmHeight: CARD16,
    pub minInstalledMaps: CARD16,
    pub maxInstalledMaps: CARD16,
    pub rootVisualID: CARD32,
    pub backingStore: CARD8,
    pub saveUnders: BOOL,
    pub rootDepth: CARD8,
    pub nDepths: CARD8,
}
pub type in_addr_t = ui32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: u32,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut libc::c_void; 4],
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: u32,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type pthread_addr_t = *mut libc::c_void;
pub type pthread_cleanuproutine_t = Option::<
    fn(*mut libc::c_void) -> (),
>;
pub type pthread_startroutine_t = Option::<
    fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type EmbWord = i32;
pub type uEmbWord = ui32;
pub type EmbPtr = EmbWord;
pub type SignalMask = uEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
pub type isize = u64;
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
    pub present: Boole,
    pub device: [libc::c_char; 257],
    pub myProtocol: libc::c_ushort,
    pub myAddress: in_addr,
    pub myOptions: [libc::c_char; 257],
    pub anotherAddress: *mut NetworkInterface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceConfig {
    pub traceP: Boole,
    pub tracePOST: Boole,
    pub bufferSize: u32,
    pub startPC: libc::c_uint,
    pub stopPC: libc::c_uint,
    pub outputFile:&str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLMConfig {
    pub enableSpy: Boole,
    pub tracing: TraceConfig,
    pub commAreaSize: size_t,
    pub hostBufferSpace: size_t,
    pub guestBufferSpace: size_t,
    pub vlmDebuggerPath: [libc::c_char; 257],
    pub worldPath: [libc::c_char; 257],
    pub worldSearchPath:&str,
    pub enableIDS: Boole,
    pub virtualMemory: size_t,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: in_addr,
    pub interfaces: [NetworkInterface; 8],
    pub testFunction: Boole,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalHandler {
    pub handlerThread: pthread_t,
    pub handlerThreadSetup: Boole,
    pub signal: SignalMask,
    pub handlerFunction: ProcPtrV,
    pub handlerArgument: PtrV,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommArea {
    pub identifier: EmbWord,
    pub version: EmbWord,
    pub system_type: EmbWord,
    pub number_of_slots: EmbWord,
    pub comm_memory_size: EmbWord,
    pub generaVersion: C2RustUnnamed_36,
    pub osfVersion: C2RustUnnamed_35,
    pub guest_major_version: EmbWord,
    pub guest_minor_version: EmbWord,
    pub fep_major_version: EmbWord,
    pub fep_minor_version: EmbWord,
    pub guest_buffer_start: EmbPtr,
    pub guest_buffer_size: EmbWord,
    pub host_buffer_start: EmbPtr,
    pub host_buffer_size: EmbWord,
    pub fep_buffer_start: EmbPtr,
    pub fep_buffer_size: EmbWord,
    pub guest_to_host_signals: SignalMask,
    pub live_guest_to_host_signals: SignalMask,
    pub host_to_guest_signals: SignalMask,
    pub live_host_to_guest_signals: SignalMask,
    pub channel_table: EmbPtr,
    pub consoleChannel: EmbPtr,
    pub cold_load_channel: EmbPtr,
    pub command_channel: EmbPtr,
    pub virtualMemorySize: EmbWord,
    pub worldImageSize: EmbWord,
    pub bad_memory_map: EmbPtr,
    pub bad_memory_map_size: EmbWord,
    pub clock_signal: SignalNumber,
    pub clock_interval: EmbWord,
    pub run_lights: EmbWord,
    pub reset_request: EmbWord,
    pub board_serial_number: EmbWord,
    pub board_major_version: EmbWord,
    pub board_minor_version: EmbWord,
    pub spy_command: EmbWord,
    pub spy_status: EmbWord,
    pub stop_request: EmbWord,
    pub fep: C2RustUnnamed_34,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_33,
    pub worldPathname: EmbPtr,
    pub unixLoginName: EmbPtr,
    pub unixUID: uEmbWord,
    pub unixGID: uEmbWord,
    pub pad0: EmbWord,
    pub pad1: [EmbWord; 15],
    pub guestStatus: EmbWord,
    pub pollThreadAttrs: pthread_attr_t,
    pub pollThreadAttrsSetup: Boole,
    pub outputThreadAttrs: pthread_attr_t,
    pub outputThreadAttrsSetup: Boole,
    pub inputThreadAttrs: pthread_attr_t,
    pub inputThreadAttrsSetup: Boole,
    pub useSignalLocks: Boole,
    pub signalHandler: [SignalHandler; 32],
    pub reawaken: SignalMask,
    pub signalLock: pthread_mutex_t,
    pub signalLockSetup: Boole,
    pub signalSignal: pthread_cond_t,
    pub signalSignalSetup: Boole,
    pub pollingThread: pthread_t,
    pub pollTime: libc::c_long,
    pub pollClockTime: libc::c_long,
    pub pollingThreadSetup: Boole,
    pub clockThread: pthread_t,
    pub clockTime: libc::c_long,
    pub clockLock: pthread_mutex_t,
    pub clockLockSetup: Boole,
    pub clockSignal: pthread_cond_t,
    pub clockSignalSetup: Boole,
    pub clockThreadSetup: Boole,
    pub resetRequestCount: EmbWord,
    pub restartApplicationsCount: EmbWord,
    pub inhibitDisk: Boole,
    pub debugLevel: EmbWord,
    pub slaveTrigger: caddr_t,
    pub XLock: pthread_mutex_t,
    pub XLockSetup: Boole,
    pub wakeupLock: pthread_mutex_t,
    pub wakeupLockSetup: Boole,
    pub wakeupSignal: pthread_cond_t,
    pub wakeupSignalSetup: Boole,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbQueue {
    pub element_size: EmbWord,
    pub queue_size: EmbWord,
    pub put_index: EmbWord,
    pub take_index: EmbWord,
    pub signal: SignalNumber,
    pub first_element: [EmbWord; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbChannel {
    pub type_0: EmbWord,
    pub unit: EmbWord,
    pub next: EmbPtr,
}
pub type EmbChannelType = libc::c_uint;
pub const EmbMessageChannelType: EmbChannelType = 8;
pub const EmbHostFileChannelType: EmbChannelType = 7;
pub const EmbColdLoadChannelType: EmbChannelType = 6;
pub const EmbSCSIChannelType: EmbChannelType = 5;
pub const EmbRPCChannelType: EmbChannelType = 4;
pub const EmbNetworkChannelType: EmbChannelType = 3;
pub const EmbConsoleChannelType: EmbChannelType = 2;
pub const EmbDiskChannelType: EmbChannelType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbConsoleRunLights {
    pub windowID: EmbWord,
    pub nLights: EmbWord,
    pub lightWidth: EmbWord,
    pub lightHeight: EmbWord,
    pub firstLightX: EmbWord,
    pub firstLightY: EmbWord,
    pub lightXSpacing: EmbWord,
    pub lightYSpacing: EmbWord,
    pub lightForeground: EmbWord,
    pub lightBackground: EmbWord,
    pub lightPlaneMask: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbConsoleChannel {
    pub type_0: EmbWord,
    pub unit: EmbWord,
    pub next: EmbPtr,
    pub outputRequestQueue: EmbPtr,
    pub outputReplyQueue: EmbPtr,
    pub inputRequestQueue: EmbPtr,
    pub inputReplyQueue: EmbPtr,
    pub hostAddress: EmbWord,
    pub displayNumber: EmbWord,
    pub screenNumber: EmbWord,
    pub initialState: EmbWord,
    pub geometry: EmbPtr,
    pub foregroundColor: EmbPtr,
    pub backgroundColor: EmbPtr,
    pub borderColor: EmbPtr,
    pub borderWidth: EmbPtr,
    pub inputAvailableP: EmbWord,
    pub outputRequestQ: *mut EmbQueue,
    pub outputReplyQ: *mut EmbQueue,
    pub inputRequestQ: *mut EmbQueue,
    pub inputReplyQ: *mut EmbQueue,
    pub drawRunLights: pthread_t,
    pub drawRunLightsSetup: Boole,
    pub hostName:&str,
    pub display: *mut libc::c_void,
    pub fd: u32,
    pub openingState: u32,
    pub nextPixmapFormat: u32,
    pub nextRoot: u32,
    pub nextRootDepth: u32,
    pub nextRootDepthVisual: u32,
    pub rlDisplay: *mut libc::c_void,
    pub runLights: EmbConsoleRunLights,
    pub rlGC: *mut libc::c_void,
    pub lastRunLights: EmbWord,
}
pub type OpeningState = libc::c_uint;
pub const OpeningStateRootDepthVisual: OpeningState = 7;
pub const OpeningStateRootDepth: OpeningState = 6;
pub const OpeningStateRoot: OpeningState = 5;
pub const OpeningStatePixmapFormat: OpeningState = 4;
pub const OpeningStateVendor: OpeningState = 3;
pub const OpeningStateHeader: OpeningState = 2;
pub const OpeningStatePrefix: OpeningState = 1;
pub const OpeningStateNone: OpeningState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbConsoleBuffer {
    pub opcode: EmbWord,
    pub id: EmbWord,
    pub result: EmbWord,
    pub data: [EmbWord; 1],
}
pub type EmbConsoleCommand = libc::c_uint;
pub const EmbConsoleCommandDisableRunLights: EmbConsoleCommand = 8;
pub const EmbConsoleCommandEnableRunLights: EmbConsoleCommand = 7;
pub const EmbConsoleCommandInputWait: EmbConsoleCommand = 6;
pub const EmbConsoleCommandRead: EmbConsoleCommand = 5;
pub const EmbConsoleCommandWrite: EmbConsoleCommand = 4;
pub const EmbConsoleCommandNoOp: EmbConsoleCommand = 3;
pub const EmbConsoleCommandCloseDisplay: EmbConsoleCommand = 2;
pub const EmbConsoleCommandOpenDisplay: EmbConsoleCommand = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbConsoleOpenDisplay {
    pub lastRequestNumber: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbConsoleDataTransfer {
    pub address: EmbWord,
    pub offset: EmbWord,
    pub nBytes: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbConsoleInputWait {
    pub timeout: EmbWord,
    pub availableP: EmbWord,
}
#[no_mangle]
pub  fn InitializeConsoleChannel(mut config: *mut VLMConfig) {
    let mut cp: EmbPtr = EmbCommAreaAlloc(
        ::std::mem::size_of::<EmbConsoleChannel>() as libc::c_ulong,
    );
    let mut p: *mut EmbConsoleChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset(cp as isize) as *mut EmbWord as PtrV as *mut EmbConsoleChannel;
    (*p).type_0 = EmbConsoleChannelType;
    (*p).unit = 0;
    (*p).next = (*EmbCommAreaPtr).channel_table;
    (*EmbCommAreaPtr).channel_table = cp;
    (*EmbCommAreaPtr).consoleChannel = cp;
    (*p)
        .outputRequestQueue = CreateQueue(
        50,
        ::std::mem::size_of::<EmbPtr>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*p).outputRequestQ;
    *fresh0 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).outputRequestQueue as isize) as *mut EmbWord as PtrV
        as *mut EmbQueue;
    (*(*p).outputRequestQ)
        .signal = InstallSignalHandler(
        ::std::mem::transmute::<
            Option::<fn(*mut EmbConsoleChannel) -> ()>,
            ProcPtrV,
        >(Some(ConsoleOutput as fn(*mut EmbConsoleChannel) -> ())),
        p as PtrV,
        0 as usize as Boole,
    );
    (*p)
        .outputReplyQueue = CreateQueue(
        50,
        ::std::mem::size_of::<EmbPtr>() as libc::c_ulong,
    );
    let ref mut fresh1 = (*p).outputReplyQ;
    *fresh1 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).outputReplyQueue as isize) as *mut EmbWord as PtrV as *mut EmbQueue;
    (*p)
        .inputRequestQueue = CreateQueue(
        50,
        ::std::mem::size_of::<EmbPtr>() as libc::c_ulong,
    );
    let ref mut fresh2 = (*p).inputRequestQ;
    *fresh2 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).inputRequestQueue as isize) as *mut EmbWord as PtrV
        as *mut EmbQueue;
    (*(*p).inputRequestQ)
        .signal = InstallSignalHandler(
        ::std::mem::transmute::<
            Option::<fn(*mut EmbConsoleChannel) -> ()>,
            ProcPtrV,
        >(Some(ConsoleInput as fn(*mut EmbConsoleChannel) -> ())),
        p as PtrV,
        1 as usize as Boole,
    );
    (*p)
        .inputReplyQueue = CreateQueue(
        50,
        ::std::mem::size_of::<EmbPtr>() as libc::c_ulong,
    );
    let ref mut fresh3 = (*p).inputReplyQ;
    *fresh3 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).inputReplyQueue as isize) as *mut EmbWord as PtrV as *mut EmbQueue;
    let ref mut fresh4 = (*p).hostName;
    *fresh4 = (*config).generaXParams.xpHostName;
    (*p)
        .hostAddress = htonl((*config).generaXParams.xpHostAddress as ui32)
        as EmbWord;
    (*p).displayNumber = (*config).generaXParams.xpDisplay;
    (*p).screenNumber = (*config).generaXParams.xpScreen;
    (*p).initialState = (*config).generaXParams.xpInitialState;
    (*p).geometry = MakeEmbString((*config).generaXParams.xpGeometry);
    (*p).foregroundColor = MakeEmbString((*config).generaXParams.xpForegroundColor);
    (*p).backgroundColor = MakeEmbString((*config).generaXParams.xpBackgroundColor);
    (*p).borderColor = MakeEmbString((*config).generaXParams.xpBorderColor);
    (*p).borderWidth = (*config).generaXParams.xpBorderWidth;
    let ref mut fresh5 = (*p).display;
    *fresh5 = 0 as *mut libc::c_void;
    (*p).openingState = OpeningStateNone;
    let ref mut fresh6 = (*p).rlDisplay;
    *fresh6 = 0 as *mut libc::c_void;
    if pthread_create(
        &mut (*p).drawRunLights,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        ::std::mem::transmute::<
            Option::<fn(pthread_addr_t) -> ()>,
            pthread_startroutine_t,
        >(Some(DrawRunLights as fn(pthread_addr_t) -> ())),
        p as *mut libc::c_void,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the console channel polling thread\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*p).drawRunLightsSetup = 1 as usize as Boole;
}
#[no_mangle]
pub  fn DoConsoleIO(
    mut consoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) {
    let mut command: *mut EmbConsoleBuffer = pCommand;
    match (*command).opcode {
        1 => {
            (*command).result = OpenDisplay(consoleChannel, command);
        }
        2 => {
            CloseDisplay(consoleChannel);
            (*command).result = 0;
        }
        3 => {
            (*command).result = 0;
        }
        4 => {
            if OpeningStatePrefix as usize == (*consoleChannel).openingState {
                (*command).result = 0;
            } else {
                (*command).result = ConsoleWrite(consoleChannel, command);
            }
        }
        5 => {
            if (*consoleChannel).openingState != OpeningStateNone as usize {
                (*command).result = ProcessConnectionRequest(consoleChannel, command);
            } else {
                (*command).result = ConsoleRead(consoleChannel, command);
            }
        }
        6 => {
            if (*consoleChannel).openingState != OpeningStateNone as usize {
                (*(&mut *((*command).data).as_mut_ptr().offset(0 as usize as isize)
                    as *mut EmbWord as *mut EmbConsoleInputWait))
                    .availableP = 1;
                (*command).result = 0;
            } else {
                (*command).result = ConsoleInputWait(consoleChannel, command);
            }
        }
        7 => {
            EnableRunLights(consoleChannel, command);
            (*command).result = 0;
        }
        8 => {
            DisableRunLights(consoleChannel);
            (*command).result = 0;
        }
        _ => {}
    };
}
 fn ConsoleDriver(
    mut consoleChannel: *mut EmbConsoleChannel,
    mut pRequestQueue: *mut EmbQueue,
    mut pReplyQueue: *mut EmbQueue,
) {
    let mut requestQueue: *mut EmbQueue = pRequestQueue;
    let mut replyQueue: *mut EmbQueue = pReplyQueue;
    let mut command: *mut EmbConsoleBuffer = 0 as *mut EmbConsoleBuffer;
    let mut commandPtr: EmbPtr = 0;
    while EmbQueueFilled(requestQueue) != 0 {
        if 0 as usize == EmbQueueSpace(replyQueue) {
            SignalLater((*requestQueue).signal);
            return;
        }
        commandPtr = EmbQueueTakeWord(requestQueue);
        if commandPtr != 0 {
            command = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(commandPtr as isize)
                as *mut EmbWord as PtrV as *mut EmbConsoleBuffer;
            DoConsoleIO(consoleChannel, command);
            EmbQueuePutWord(replyQueue, commandPtr);
        }
    }
}
 fn ConsoleOutput(mut consoleChannel: *mut EmbConsoleChannel) {
    ConsoleDriver(
        consoleChannel,
        (*consoleChannel).outputRequestQ,
        (*consoleChannel).outputReplyQ,
    );
}
 fn ConsoleInput(mut consoleChannel: *mut EmbConsoleChannel) {
    ConsoleDriver(
        consoleChannel,
        (*consoleChannel).inputRequestQ,
        (*consoleChannel).inputReplyQ,
    );
}
 fn OpenDisplay(
    mut pConsoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) -> usize {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut command: *mut EmbConsoleBuffer = pCommand;
    let mut openDisplay: *mut EmbConsoleOpenDisplay = &mut *((*command).data)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut EmbWord as *mut EmbConsoleOpenDisplay;
    let mut displayName: [libc::c_char; 8192] = [0; 8192];
    let mut result: usize = 0;
    if !((*consoleChannel).display).is_null() {
        return 16;
    }
    BuildXDisplayName(
        displayName.as_mut_ptr(),
        (*consoleChannel).hostName,
        (*consoleChannel).displayNumber,
        (*consoleChannel).screenNumber,
    );
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<fn(*mut pthread_mutex_t) -> u32>,
        pthread_cleanuproutine_t,
    >(
        Some(
            pthread_mutex_unlock
                as fn(*mut pthread_mutex_t) -> u32,
        ),
    );
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
        as *mut pthread_mutex_t as *mut libc::c_void;
    let mut __not_first_call: usize = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
             "" ,
            b"Unable to lock the Life Support XLock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    let ref mut fresh7 = (*consoleChannel).display;
    *fresh7 = XOpenDisplay(displayName.as_mut_ptr()) as *mut libc::c_void;
    if !((*consoleChannel).display).is_null() {
        (*consoleChannel)
            .fd = XConnectionNumber((*consoleChannel).display as *mut Display);
        (*consoleChannel).openingState = OpeningStatePrefix;
        (*openDisplay)
            .lastRequestNumber = (*((*consoleChannel).display as *mut _XDisplay)).request
            as EmbWord;
        result = 0;
    } else {
        result = *__errno_location();
        match result {
            0 => {
                result = 111;
            }
            11 => {
                result = 6;
            }
            _ => {}
        }
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
             "" ,
            b"Unable to unlock the Life Support XLock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    return result;
}
 fn ProcessConnectionRequest(
    mut pConsoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) -> usize {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut command: *mut EmbConsoleBuffer = pCommand;
    let mut dataTransfer: *mut EmbConsoleDataTransfer = &mut *((*command).data)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut EmbWord
        as *mut EmbConsoleDataTransfer;
    let mut display: *mut _XDisplay = (*consoleChannel).display as *mut _XDisplay;
    let mut data: &str =  "" ;
    let mut setupPrefix: xConnSetupPrefix = xConnSetupPrefix {
        success: 0,
        lengthReason: 0,
        majorVersion: 0,
        minorVersion: 0,
        length: 0,
    };
    let mut setup: xConnSetup = xConnSetup {
        release: 0,
        ridBase: 0,
        ridMask: 0,
        motionBufferSize: 0,
        nbytesVendor: 0,
        maxRequestSize: 0,
        numRoots: 0,
        numFormats: 0,
        imageByteOrder: 0,
        bitmapBitOrder: 0,
        bitmapScanlineUnit: 0,
        bitmapScanlinePad: 0,
        minKeyCode: 0,
        maxKeyCode: 0,
        pad2: 0,
    };
    let mut pixmapFormat: xPixmapFormat = xPixmapFormat {
        depth: 0,
        bitsPerPixel: 0,
        scanLinePad: 0,
        pad1: 0,
        pad2: 0,
    };
    let mut screenFormat: *mut ScreenFormat = 0 as *mut ScreenFormat;
    let mut windowRoot: xWindowRoot = xWindowRoot {
        windowId: 0,
        defaultColormap: 0,
        whitePixel: 0,
        blackPixel: 0,
        currentInputMask: 0,
        pixWidth: 0,
        pixHeight: 0,
        mmWidth: 0,
        mmHeight: 0,
        minInstalledMaps: 0,
        maxInstalledMaps: 0,
        rootVisualID: 0,
        backingStore: 0,
        saveUnders: 0,
        rootDepth: 0,
        nDepths: 0,
    };
    let mut screen: *mut Screen = 0 as *mut Screen;
    let mut pDepth: xDepth = xDepth {
        depth: 0,
        pad1: 0,
        nVisuals: 0,
        pad2: 0,
    };
    let mut depth: *mut Depth = 0 as *mut Depth;
    let mut visualType: xVisualType = xVisualType {
        visualID: 0,
        class: 0,
        bitsPerRGB: 0,
        colormapEntries: 0,
        redMask: 0,
        greenMask: 0,
        blueMask: 0,
        pad: 0,
    };
    let mut visual: *mut Visual = 0 as *mut Visual;
    data = MapVirtualAddressData((*dataTransfer).address as isize)
        as&str;
    data = data.offset((*dataTransfer).offset as isize);
    match (*consoleChannel).openingState {
        1 => {
            setupPrefix.success = 1 as usize as CARD8;
            setupPrefix.lengthReason = 0 as usize as BYTE;
            setupPrefix.majorVersion = (*display).proto_major_version as CARD16;
            setupPrefix.minorVersion = (*display).proto_minor_version as CARD16;
            setupPrefix.length = 0 as usize as CARD16;
            memcpy(
                data as *mut libc::c_void,
                &mut setupPrefix as *mut xConnSetupPrefix as *const libc::c_void,
                ::std::mem::size_of::<xConnSetupPrefix>() as libc::c_ulong,
            );
            AdvanceOpeningState(consoleChannel);
        }
        2 => {
            setup.release = (*display).release as CARD32;
            setup.ridBase = (*display).resource_base as CARD32;
            setup.ridMask = (*display).resource_mask as CARD32;
            setup.motionBufferSize = (*display).motion_buffer as CARD32;
            setup.nbytesVendor = strlen((*display).vendor) as CARD16;
            setup.maxRequestSize = (*display).max_request_size as CARD16;
            setup.numRoots = (*display).nscreens as CARD8;
            setup.numFormats = (*display).nformats as CARD8;
            setup.imageByteOrder = (*display).byte_order as CARD8;
            setup.bitmapBitOrder = (*display).bitmap_bit_order as CARD8;
            setup.bitmapScanlineUnit = (*display).bitmap_unit as CARD8;
            setup.bitmapScanlinePad = (*display).bitmap_pad as CARD8;
            setup.minKeyCode = (*display).min_keycode as CARD8;
            setup.maxKeyCode = (*display).max_keycode as CARD8;
            memcpy(
                data as *mut libc::c_void,
                &mut setup as *mut xConnSetup as *const libc::c_void,
                ::std::mem::size_of::<xConnSetup>() as libc::c_ulong,
            );
            AdvanceOpeningState(consoleChannel);
        }
        3 => {
            memcpy(
                data as *mut libc::c_void,
                (*display).vendor as *const libc::c_void,
                strlen((*display).vendor),
            );
            AdvanceOpeningState(consoleChannel);
        }
        4 => {
            screenFormat = &mut *((*display).pixmap_format)
                .offset((*consoleChannel).nextPixmapFormat as isize)
                as *mut ScreenFormat;
            pixmapFormat.depth = (*screenFormat).depth as CARD8;
            pixmapFormat.bitsPerPixel = (*screenFormat).bits_per_pixel as CARD8;
            pixmapFormat.scanLinePad = (*screenFormat).scanline_pad as CARD8;
            memcpy(
                data as *mut libc::c_void,
                &mut pixmapFormat as *mut xPixmapFormat as *const libc::c_void,
                ::std::mem::size_of::<xPixmapFormat>() as libc::c_ulong,
            );
            AdvanceOpeningState(consoleChannel);
        }
        5 => {
            screen = &mut *((*display).screens)
                .offset((*consoleChannel).nextRoot as isize) as *mut Screen;
            windowRoot.windowId = (*screen).root as CARD32;
            windowRoot.defaultColormap = (*screen).cmap as CARD32;
            windowRoot.whitePixel = (*screen).white_pixel as CARD32;
            windowRoot.blackPixel = (*screen).black_pixel as CARD32;
            windowRoot.currentInputMask = (*screen).root_input_mask as CARD32;
            windowRoot.pixWidth = (*screen).width as CARD16;
            windowRoot.pixHeight = (*screen).height as CARD16;
            windowRoot.mmWidth = (*screen).mwidth as CARD16;
            windowRoot.mmHeight = (*screen).mheight as CARD16;
            windowRoot.minInstalledMaps = (*screen).min_maps as CARD16;
            windowRoot.maxInstalledMaps = (*screen).max_maps as CARD16;
            windowRoot.rootVisualID = (*(*screen).root_visual).visualid as CARD32;
            windowRoot.backingStore = (*screen).backing_store as CARD8;
            windowRoot.saveUnders = (*screen).save_unders as BOOL;
            windowRoot.rootDepth = (*screen).root_depth as CARD8;
            windowRoot.nDepths = (*screen).ndepths as CARD8;
            memcpy(
                data as *mut libc::c_void,
                &mut windowRoot as *mut xWindowRoot as *const libc::c_void,
                ::std::mem::size_of::<xWindowRoot>() as libc::c_ulong,
            );
            AdvanceOpeningState(consoleChannel);
        }
        6 => {
            screen = &mut *((*display).screens)
                .offset((*consoleChannel).nextRoot as isize) as *mut Screen;
            depth = &mut *((*screen).depths)
                .offset((*consoleChannel).nextRootDepth as isize) as *mut Depth;
            pDepth.depth = (*depth).depth as CARD8;
            pDepth.nVisuals = (*depth).nvisuals as CARD16;
            memcpy(
                data as *mut libc::c_void,
                &mut pDepth as *mut xDepth as *const libc::c_void,
                ::std::mem::size_of::<xDepth>() as libc::c_ulong,
            );
            AdvanceOpeningState(consoleChannel);
        }
        7 => {
            screen = &mut *((*display).screens)
                .offset((*consoleChannel).nextRoot as isize) as *mut Screen;
            depth = &mut *((*screen).depths)
                .offset((*consoleChannel).nextRootDepth as isize) as *mut Depth;
            visual = &mut *((*depth).visuals)
                .offset((*consoleChannel).nextRootDepthVisual as isize) as *mut Visual;
            visualType.visualID = (*visual).visualid as CARD32;
            visualType.class = (*visual).class as CARD8;
            visualType.bitsPerRGB = (*visual).bits_per_rgb as CARD8;
            visualType.colormapEntries = (*visual).map_entries as CARD16;
            visualType.redMask = (*visual).red_mask as CARD32;
            visualType.greenMask = (*visual).green_mask as CARD32;
            visualType.blueMask = (*visual).blue_mask as CARD32;
            memcpy(
                data as *mut libc::c_void,
                &mut visualType as *mut xVisualType as *const libc::c_void,
                ::std::mem::size_of::<xVisualType>() as libc::c_ulong,
            );
            AdvanceOpeningState(consoleChannel);
        }
        _ => {}
    }
    return 0;
}
 fn AdvanceOpeningState(mut pConsoleChannel: *mut EmbConsoleChannel) {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut display: *mut _XDisplay = (*consoleChannel).display as *mut _XDisplay;
    let mut screen: *mut Screen = 0 as *mut Screen;
    let mut depth: *mut Depth = 0 as *mut Depth;
    match (*consoleChannel).openingState {
        1 => {
            (*consoleChannel).openingState = OpeningStateHeader;
        }
        2 => {
            (*consoleChannel).openingState = OpeningStateVendor;
        }
        3 => {
            if (*display).nformats > 0 as usize {
                (*consoleChannel).openingState = OpeningStatePixmapFormat;
                (*consoleChannel).nextPixmapFormat = 0;
            } else if (*display).nscreens > 0 as usize {
                (*consoleChannel).openingState = OpeningStateRoot;
                (*consoleChannel).nextRoot = 0;
            } else {
                (*consoleChannel).openingState = OpeningStateNone;
            }
        }
        4 => {
            let ref mut fresh8 = (*consoleChannel).nextPixmapFormat;
            *fresh8 += 1;
            if (*consoleChannel).nextPixmapFormat >= (*display).nformats {
                if (*display).nscreens > 0 as usize {
                    (*consoleChannel).openingState = OpeningStateRoot;
                    (*consoleChannel).nextRoot = 0;
                } else {
                    (*consoleChannel).openingState = OpeningStateNone;
                }
            }
        }
        5 => {
            screen = &mut *((*display).screens)
                .offset((*consoleChannel).nextRoot as isize) as *mut Screen;
            if (*screen).ndepths > 0 as usize {
                (*consoleChannel).openingState = OpeningStateRootDepth;
                (*consoleChannel).nextRootDepth = 0;
            } else {
                let ref mut fresh9 = (*consoleChannel).nextRoot;
                *fresh9 += 1;
                if (*consoleChannel).nextRoot >= (*display).nscreens {
                    (*consoleChannel).openingState = OpeningStateNone;
                }
            }
        }
        6 => {
            screen = &mut *((*display).screens)
                .offset((*consoleChannel).nextRoot as isize) as *mut Screen;
            depth = &mut *((*screen).depths)
                .offset((*consoleChannel).nextRootDepth as isize) as *mut Depth;
            if (*depth).nvisuals > 0 as usize {
                (*consoleChannel)
                    .openingState = OpeningStateRootDepthVisual;
                (*consoleChannel).nextRootDepthVisual = 0;
            } else {
                let ref mut fresh10 = (*consoleChannel).nextRootDepth;
                *fresh10 += 1;
                if (*consoleChannel).nextRootDepth >= (*screen).ndepths {
                    let ref mut fresh11 = (*consoleChannel).nextRoot;
                    *fresh11 += 1;
                    if (*consoleChannel).nextRoot >= (*display).nscreens {
                        (*consoleChannel).openingState = OpeningStateNone;
                    }
                }
            }
        }
        7 => {
            screen = &mut *((*display).screens)
                .offset((*consoleChannel).nextRoot as isize) as *mut Screen;
            depth = &mut *((*screen).depths)
                .offset((*consoleChannel).nextRootDepth as isize) as *mut Depth;
            let ref mut fresh12 = (*consoleChannel).nextRootDepthVisual;
            *fresh12 += 1;
            if (*consoleChannel).nextRootDepthVisual >= (*depth).nvisuals {
                let ref mut fresh13 = (*consoleChannel).nextRootDepth;
                *fresh13 += 1;
                if (*consoleChannel).nextRootDepth >= (*screen).ndepths {
                    let ref mut fresh14 = (*consoleChannel).nextRoot;
                    *fresh14 += 1;
                    if (*consoleChannel).nextRoot >= (*display).nscreens {
                        (*consoleChannel).openingState = OpeningStateNone;
                    } else {
                        (*consoleChannel).openingState = OpeningStateRoot;
                    }
                } else {
                    (*consoleChannel)
                        .openingState = OpeningStateRootDepth;
                }
            }
        }
        _ => {}
    };
}
 fn ConsoleWrite(
    mut pConsoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) -> usize {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut command: *mut EmbConsoleBuffer = pCommand;
    let mut dataTransfer: *mut EmbConsoleDataTransfer = &mut *((*command).data)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut EmbWord
        as *mut EmbConsoleDataTransfer;
    let mut pollDisplay: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut data: &str =  "" ;
    let mut nBytes: ssize_t = 0;
    let mut actualBytes: ssize_t = 0;
    let mut result: usize = 0;
    data = MapVirtualAddressData((*dataTransfer).address as isize)
        as&str;
    data = data.offset((*dataTransfer).offset as isize);
    nBytes = (*dataTransfer).nBytes as ssize_t;
    result = 11;
    pollDisplay.fd = (*consoleChannel).fd;
    pollDisplay.events = 0x4 as usize as libc::c_short;
    while 11 as usize == result {
        pthread_testcancel();
        pollDisplay.revents = 0 as usize as libc::c_short;
        poll(&mut pollDisplay, 1 as usize as nfds_t, 1000);
        if pollDisplay.revents as usize & 0x4 as usize != 0 {
            actualBytes = write(
                (*consoleChannel).fd,
                data as *const libc::c_void,
                nBytes as size_t,
            );
            if actualBytes == nBytes {
                result = 0;
            } else {
                result = if actualBytes < 0 as usize as libc::c_long {
                    *__errno_location()
                } else {
                    11
                };
                nBytes
                    -= if actualBytes < 0 as usize as libc::c_long {
                        0 as usize as libc::c_long
                    } else {
                        actualBytes
                    };
                data = data
                    .offset(
                        (if actualBytes < 0 as usize as libc::c_long {
                            0 as usize as libc::c_long
                        } else {
                            actualBytes
                        }) as isize,
                    );
            }
        } else if pollDisplay.revents as usize & 0x20 as usize != 0 {
            result = 9;
        } else if pollDisplay.revents as usize & 0x10 as usize != 0 {
            result = 6;
        } else if pollDisplay.revents as usize & 0x8 as usize != 0 {
            result = 5;
        }
    }
    return result;
}
 fn ConsoleRead(
    mut pConsoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) -> usize {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut command: *mut EmbConsoleBuffer = pCommand;
    let mut dataTransfer: *mut EmbConsoleDataTransfer = &mut *((*command).data)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut EmbWord
        as *mut EmbConsoleDataTransfer;
    let mut pollDisplay: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut data: &str =  "" ;
    let mut nBytes: ssize_t = 0;
    let mut actualBytes: ssize_t = 0;
    let mut result: usize = 0;
    data = MapVirtualAddressData((*dataTransfer).address as isize)
        as&str;
    data = data.offset((*dataTransfer).offset as isize);
    nBytes = (*dataTransfer).nBytes as ssize_t;
    result = 11;
    pollDisplay.fd = (*consoleChannel).fd;
    pollDisplay.events = 0x1 as usize as libc::c_short;
    while 11 as usize == result {
        pthread_testcancel();
        pollDisplay.revents = 0 as usize as libc::c_short;
        poll(&mut pollDisplay, 1 as usize as nfds_t, 1000);
        if pollDisplay.revents as usize & 0x1 as usize != 0 {
            actualBytes = read(
                (*consoleChannel).fd,
                data as *mut libc::c_void,
                nBytes as size_t,
            );
            if actualBytes == nBytes {
                result = 0;
            } else if 0 as usize as libc::c_long == actualBytes
                && 11 as usize != *__errno_location()
            {
                result = 28;
            } else {
                result = if actualBytes < 0 as usize as libc::c_long {
                    *__errno_location()
                } else {
                    11
                };
                nBytes
                    -= if actualBytes < 0 as usize as libc::c_long {
                        0 as usize as libc::c_long
                    } else {
                        actualBytes
                    };
                data = data
                    .offset(
                        (if actualBytes < 0 as usize as libc::c_long {
                            0 as usize as libc::c_long
                        } else {
                            actualBytes
                        }) as isize,
                    );
            }
        } else if pollDisplay.revents as usize & 0x20 as usize != 0 {
            result = 9;
        } else if pollDisplay.revents as usize & 0x10 as usize != 0 {
            result = 6;
        } else if pollDisplay.revents as usize & 0x8 as usize != 0 {
            result = 5;
        }
    }
    return result;
}
#[no_mangle]
pub  fn ConsoleInputAvailableP() -> Boole {
    let mut consoleChannel: *mut EmbConsoleChannel = &mut *(EmbCommAreaPtr
        as *mut EmbWord)
        .offset((*EmbCommAreaPtr).consoleChannel as isize) as *mut EmbWord as PtrV
        as *mut EmbConsoleChannel;
    let mut pollDisplay: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    if ((*consoleChannel).display).is_null() {
        return 0 as usize as Boole
    } else {
        if (*consoleChannel).openingState != OpeningStateNone as usize {
            return 1 as usize as Boole;
        }
    }
    pollDisplay.fd = (*consoleChannel).fd;
    pollDisplay.events = 0x1 as usize as libc::c_short;
    pollDisplay.revents = 0 as usize as libc::c_short;
    poll(&mut pollDisplay, 1 as usize as nfds_t, 0);
    return (pollDisplay.revents as usize & 0x1 as usize != 0)
        as usize as Boole;
}
 fn ConsoleInputWait(
    mut pConsoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) -> usize {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut command: *mut EmbConsoleBuffer = pCommand;
    let mut inputWait: *mut EmbConsoleInputWait = &mut *((*command).data)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut EmbWord as *mut EmbConsoleInputWait;
    let mut pollDisplay: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut result: usize = 0;
    pollDisplay.fd = (*consoleChannel).fd;
    pollDisplay.events = 0x1 as usize as libc::c_short;
    pollDisplay.revents = 0 as usize as libc::c_short;
    result = poll(&mut pollDisplay, 1 as usize as nfds_t, (*inputWait).timeout);
    if 0 as usize == result {
        result = 0;
        (*inputWait).availableP = 0;
    } else if pollDisplay.revents as usize & 0x1 as usize != 0 {
        result = 0;
        (*inputWait).availableP = 1;
    } else if pollDisplay.revents as usize & 0x20 as usize != 0 {
        result = 9;
    } else if pollDisplay.revents as usize & 0x10 as usize != 0 {
        result = 6;
    } else if pollDisplay.revents as usize & 0x8 as usize != 0 {
        result = 5;
    }
    return result;
}
 fn CloseDisplay(mut consoleChannel: *mut EmbConsoleChannel) {
    DisableRunLights(consoleChannel);
    if !((*consoleChannel).display).is_null() {
        let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
            __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            }; 1],
            __pad: [0 as *mut libc::c_void; 4],
        };
        let mut __cancel_routine: Option::<
            fn(*mut libc::c_void) -> (),
        > = ::std::mem::transmute::<
            Option::<fn(*mut pthread_mutex_t) -> u32>,
            pthread_cleanuproutine_t,
        >(
            Some(
                pthread_mutex_unlock
                    as fn(*mut pthread_mutex_t) -> u32,
            ),
        );
        let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
            as *mut pthread_mutex_t as *mut libc::c_void;
        let mut __not_first_call: usize = __sigsetjmp(
            (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
                as *mut __jmp_buf_tag,
            0,
        );
        if __not_first_call as libc::c_long != 0 {
            __cancel_routine.expect("non-null function pointer")(__cancel_arg);
            __pthread_unwind_next(&mut __cancel_buf);
        }
        __pthread_register_cancel(&mut __cancel_buf);
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
            vpunt(
                 "" ,
                b"Unable to lock the Life Support XLock in thread %lx\0" as *const u8
                    as *const libc::c_char as&str,
                pthread_self(),
            );
        }
        XCloseDisplay((*consoleChannel).display as *mut Display);
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
            vpunt(
                 "" ,
                b"Unable to unlock the Life Support XLock in thread %lx\0" as *const u8
                    as *const libc::c_char as&str,
                pthread_self(),
            );
        }
        __pthread_unregister_cancel(&mut __cancel_buf);
        let ref mut fresh15 = (*consoleChannel).display;
        *fresh15 = 0 as *mut libc::c_void;
    }
}
 fn EnableRunLights(
    mut pConsoleChannel: *mut EmbConsoleChannel,
    mut pCommand: *mut EmbConsoleBuffer,
) {
    let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
    let mut command: *mut EmbConsoleBuffer = pCommand;
    let mut runLights: *mut EmbConsoleRunLights = &mut *((*command).data)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut EmbWord as *mut EmbConsoleRunLights;
    let mut displayName: [libc::c_char; 8192] = [0; 8192];
    let mut gcValues: XGCValues = XGCValues {
        function: 0,
        plane_mask: 0,
        foreground: 0,
        background: 0,
        line_width: 0,
        line_style: 0,
        cap_style: 0,
        join_style: 0,
        fill_style: 0,
        fill_rule: 0,
        arc_mode: 0,
        tile: 0,
        stipple: 0,
        ts_x_origin: 0,
        ts_y_origin: 0,
        font: 0,
        subwindow_mode: 0,
        graphics_exposures: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        dash_offset: 0,
        dashes: 0,
    };
    BuildXDisplayName(
        displayName.as_mut_ptr(),
        (*consoleChannel).hostName,
        (*consoleChannel).displayNumber,
        (*consoleChannel).screenNumber,
    );
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<fn(*mut pthread_mutex_t) -> u32>,
        pthread_cleanuproutine_t,
    >(
        Some(
            pthread_mutex_unlock
                as fn(*mut pthread_mutex_t) -> u32,
        ),
    );
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
        as *mut pthread_mutex_t as *mut libc::c_void;
    let mut __not_first_call: usize = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
             "" ,
            b"Unable to lock the Life Support XLock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    let ref mut fresh16 = (*consoleChannel).rlDisplay;
    *fresh16 = XOpenDisplay(displayName.as_mut_ptr()) as *mut libc::c_void;
    if !((*consoleChannel).rlDisplay).is_null() {
        let ref mut fresh17 = (*consoleChannel).rlGC;
        *fresh17 = malloc(::std::mem::size_of::<GC>() as libc::c_ulong);
        if !((*consoleChannel).rlGC).is_null() {
            memcpy(
                &mut (*consoleChannel).runLights as *mut EmbConsoleRunLights
                    as *mut libc::c_void,
                runLights as *const libc::c_void,
                ::std::mem::size_of::<EmbConsoleRunLights>() as libc::c_ulong,
            );
            gcValues
                .foreground = (*consoleChannel).runLights.lightForeground
                as libc::c_ulong;
            gcValues
                .background = (*consoleChannel).runLights.lightBackground
                as libc::c_ulong;
            gcValues
                .plane_mask = (*consoleChannel).runLights.lightPlaneMask
                as libc::c_ulong;
            let ref mut fresh18 = *((*consoleChannel).rlGC as *mut GC);
            *fresh18 = XCreateGC(
                (*consoleChannel).rlDisplay as *mut Display,
                (*consoleChannel).runLights.windowID as Drawable,
                ((1 as libc::c_long) << 2
                    | (1 as libc::c_long) << 3
                    | (1 as libc::c_long) << 1) as libc::c_ulong,
                &mut gcValues,
            );
        }
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
             "" ,
            b"Unable to unlock the Life Support XLock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    (*consoleChannel).lastRunLights = 0;
}
 fn DrawRunLights(mut argument: pthread_addr_t) {
    let mut consoleChannel: *mut EmbConsoleChannel = argument as *mut EmbConsoleChannel;
    let mut self_0: pthread_t = pthread_self();
    let mut drlSleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut changed: usize = 0;
    let mut i: usize = 0;
    let mut bit: usize = 0;
    let mut x: usize = 0;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<fn(pthread_t) -> u32>,
        pthread_cleanuproutine_t,
    >(Some(pthread_detach as fn(pthread_t) -> u32));
    let mut __cancel_arg: *mut libc::c_void = self_0 as *mut libc::c_void;
    let mut __not_first_call: usize = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
             "" ,
            b"Unable to lock the Life Support signal lock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
             "" ,
            b"Unable to unlock the Life Support signal lock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    drlSleep.tv_sec = 0 as usize as __time_t;
    drlSleep.tv_nsec = 10 as usize as libc::c_long * 10000000 as libc::c_long;
    loop {
        if !((*consoleChannel).rlDisplay).is_null() {
            let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
                __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                    __cancel_jmp_buf: [0; 8],
                    __mask_was_saved: 0,
                }; 1],
                __pad: [0 as *mut libc::c_void; 4],
            };
            let mut __cancel_routine_0: Option::<
                fn(*mut libc::c_void) -> (),
            > = ::std::mem::transmute::<
                Option::<fn(*mut pthread_mutex_t) -> u32>,
                pthread_cleanuproutine_t,
            >(
                Some(
                    pthread_mutex_unlock
                        as fn(*mut pthread_mutex_t) -> u32,
                ),
            );
            let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
                as *mut pthread_mutex_t as *mut libc::c_void;
            let mut __not_first_call_0: usize = __sigsetjmp(
                (__cancel_buf_0.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
                    as *mut __jmp_buf_tag,
                0,
            );
            if __not_first_call_0 as libc::c_long != 0 {
                __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
                __pthread_unwind_next(&mut __cancel_buf_0);
            }
            __pthread_register_cancel(&mut __cancel_buf_0);
            if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
                vpunt(
                     "" ,
                    b"Unable to lock the Life Support XLock in thread %lx\0" as *const u8
                        as *const libc::c_char as&str,
                    pthread_self(),
                );
            }
            changed = (*consoleChannel).lastRunLights ^ (*EmbCommAreaPtr).run_lights;
            (*consoleChannel).lastRunLights = (*EmbCommAreaPtr).run_lights;
            x = (*consoleChannel).runLights.firstLightX;
            i = 0;
            bit = 1;
            x = (*consoleChannel).runLights.firstLightX;
            while i < (*consoleChannel).runLights.nLights {
                if changed & bit != 0 {
                    if (*consoleChannel).lastRunLights & bit != 0 {
                        XFillRectangle(
                            (*consoleChannel).rlDisplay as *mut Display,
                            (*consoleChannel).runLights.windowID as Drawable,
                            *((*consoleChannel).rlGC as *mut GC),
                            x,
                            (*consoleChannel).runLights.firstLightY,
                            (*consoleChannel).runLights.lightWidth as libc::c_uint,
                            (*consoleChannel).runLights.lightHeight as libc::c_uint,
                        );
                    } else {
                        XClearArea(
                            (*consoleChannel).rlDisplay as *mut Display,
                            (*consoleChannel).runLights.windowID as Window,
                            x,
                            (*consoleChannel).runLights.firstLightY,
                            (*consoleChannel).runLights.lightWidth as libc::c_uint,
                            (*consoleChannel).runLights.lightHeight as libc::c_uint,
                            0,
                        );
                    }
                }
                i += 1;
                bit = bit << 1;
                x += (*consoleChannel).runLights.lightXSpacing;
            }
            XFlush((*consoleChannel).rlDisplay as *mut Display);
            if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
                vpunt(
                     "" ,
                    b"Unable to unlock the Life Support XLock in thread %lx\0"
                        as *const u8 as *const libc::c_char as&str,
                    pthread_self(),
                );
            }
            __pthread_unregister_cancel(&mut __cancel_buf_0);
        }
        if pthread_delay_np(&mut drlSleep) != 0 {
            vpunt(
                 "" ,
                b"Unable to sleep in thread %lx\0" as *const u8 as *const libc::c_char
                    as&str,
                self_0,
            );
        }
    };
}
 fn DisableRunLights(mut consoleChannel: *mut EmbConsoleChannel) {
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<fn(*mut pthread_mutex_t) -> u32>,
        pthread_cleanuproutine_t,
    >(
        Some(
            pthread_mutex_unlock
                as fn(*mut pthread_mutex_t) -> u32,
        ),
    );
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
        as *mut pthread_mutex_t as *mut libc::c_void;
    let mut __not_first_call: usize = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
             "" ,
            b"Unable to lock the Life Support XLock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    if !((*consoleChannel).rlGC).is_null() {
        free((*consoleChannel).rlGC);
        let ref mut fresh19 = (*consoleChannel).rlGC;
        *fresh19 = 0 as *mut libc::c_void;
    }
    if !((*consoleChannel).rlDisplay).is_null() {
        XCloseDisplay((*consoleChannel).rlDisplay as *mut Display);
        let ref mut fresh20 = (*consoleChannel).rlDisplay;
        *fresh20 = 0 as *mut libc::c_void;
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
             "" ,
            b"Unable to unlock the Life Support XLock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
}
#[no_mangle]
pub  fn ResetConsoleChannel(mut channel: *mut EmbChannel) {
    let mut consoleChannel: *mut EmbConsoleChannel = channel as *mut EmbConsoleChannel;
    ResetIncomingQueue((*consoleChannel).outputRequestQ);
    ResetOutgoingQueue((*consoleChannel).outputReplyQ);
    ResetIncomingQueue((*consoleChannel).inputRequestQ);
    ResetOutgoingQueue((*consoleChannel).inputReplyQ);
    CloseDisplay(consoleChannel);
}
#[no_mangle]
pub  fn TerminateConsoleChannel() {
    let mut exit_value: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut consoleChannel: *mut EmbConsoleChannel = 0 as *mut EmbConsoleChannel;
    if -(1) == (*EmbCommAreaPtr).consoleChannel {
        return
    } else {
        consoleChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
            .offset((*EmbCommAreaPtr).consoleChannel as isize) as *mut EmbWord as PtrV
            as *mut EmbConsoleChannel;
    }
    if (*consoleChannel).drawRunLightsSetup != 0 {
        pthread_cancel((*consoleChannel).drawRunLights);
        pthread_join((*consoleChannel).drawRunLights, &mut exit_value);
        (*consoleChannel).drawRunLightsSetup = 0 as usize as Boole;
    }
    CloseDisplay(consoleChannel);
}
