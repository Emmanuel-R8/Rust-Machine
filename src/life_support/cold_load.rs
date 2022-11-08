#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

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
pub struct QData {
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
pub type u64 = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union u64 {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union u64 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union u64 {
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
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = &str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: u32,
    pub next: *mut _XExtData,
    pub free_private: Option<fn(*mut _XExtData) -> u32>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
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
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: u32,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: u32,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPoint {
    pub x: libc::c_short,
    pub y: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: u32,
    pub modifiermap: *mut KeyCode,
}
pub type Display = _XDisplay;
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
pub struct XClientMessageEvent {
    pub type_0: u32,
    pub serial: libc::c_ulong,
    pub send_event: u32,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: u32,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
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
pub struct XErrorEvent {
    pub type_0: u32,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
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
pub type XEvent = _XEvent;
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
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub width_inc: u32,
    pub height_inc: u32,
    pub min_aspect: C2RustUnnamed_1,
    pub max_aspect: C2RustUnnamed_1,
    pub base_width: u32,
    pub base_height: u32,
    pub win_gravity: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: u32,
    pub y: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: libc::c_long,
    pub input: u32,
    pub initial_state: u32,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: u32,
    pub icon_y: u32,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
pub type u8 = u8;
pub type u32 = u32;
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: &str,
    pub h_aliases: *mut &str,
    pub h_addrtype: u32,
    pub h_length: u32,
    pub h_addr_list: *mut &str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: u32,
    pub _IO_read_ptr: &str,
    pub _IO_read_end: &str,
    pub _IO_read_base: &str,
    pub _IO_write_base: &str,
    pub _IO_write_ptr: &str,
    pub _IO_write_end: &str,
    pub _IO_buf_base: &str,
    pub _IO_buf_end: &str,
    pub _IO_save_base: &str,
    pub _IO_backup_base: &str,
    pub _IO_save_end: &str,
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
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type pthread_addr_t = *mut libc::c_void;
pub type pthread_cleanuproutine_t = Option<fn(*mut libc::c_void) -> ()>;
pub type pthread_startroutine_t = Option<fn(*mut libc::c_void) -> *mut libc::c_void>;
pub type EmbWord = i32;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;
pub type SignalMask = UEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option<fn(PtrV) -> ()>;
pub type WindowInitialState = u32;
pub const Normal: WindowInitialState = 1;
pub const Unspecified: WindowInitialState = 0;
pub const Iconic: WindowInitialState = -1;
pub type Boole = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XParams {
    pub xpHostName: &str,
    pub xpHostAddress: libc::c_long,
    pub xpDisplay: u32,
    pub xpScreen: u32,
    pub xpInitialState: u32,
    pub xpGeometry: &str,
    pub xpForegroundColor: &str,
    pub xpBackgroundColor: &str,
    pub xpBorderColor: &str,
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
    pub outputFile: &str,
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
    pub worldSearchPath: &str,
    pub enableIDS: bool,
    pub virtualMemory: size_t,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: in_addr,
    pub interfaces: [NetworkInterface; 8],
    pub testFunction: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalHandler {
    pub handlerThread: u64,
    pub handlerThreadSetup: bool,
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
    pub generaVersion: C2RustUnnamed_5,
    pub osfVersion: C2RustUnnamed_4,
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
    pub fep: C2RustUnnamed_3,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_2,
    pub worldPathname: EmbPtr,
    pub unixLoginName: EmbPtr,
    pub unixUID: UEmbWord,
    pub unixGID: UEmbWord,
    pub pad0: EmbWord,
    pub pad1: [EmbWord; 15],
    pub guestStatus: EmbWord,
    pub pollThreadAttrs: u64,
    pub pollThreadAttrsSetup: bool,
    pub outputThreadAttrs: u64,
    pub outputThreadAttrsSetup: bool,
    pub inputThreadAttrs: u64,
    pub inputThreadAttrsSetup: bool,
    pub useSignalLocks: bool,
    pub signalHandler: [SignalHandler; 32],
    pub reawaken: SignalMask,
    pub signalLock: u64,
    pub signalLockSetup: bool,
    pub signalSignal: u64,
    pub signalSignalSetup: bool,
    pub pollingThread: u64,
    pub pollTime: libc::c_long,
    pub pollClockTime: libc::c_long,
    pub pollingThreadSetup: bool,
    pub clockThread: u64,
    pub clockTime: libc::c_long,
    pub clockLock: u64,
    pub clockLockSetup: bool,
    pub clockSignal: u64,
    pub clockSignalSetup: bool,
    pub clockThreadSetup: bool,
    pub resetRequestCount: EmbWord,
    pub restartApplicationsCount: EmbWord,
    pub inhibitDisk: bool,
    pub debugLevel: EmbWord,
    pub slaveTrigger: u64,
    pub XLock: u64,
    pub XLockSetup: bool,
    pub wakeupLock: u64,
    pub wakeupLockSetup: bool,
    pub wakeupSignal: u64,
    pub wakeupSignalSetup: bool,
}

pub type GuestStatus = u32;
pub const RunningGuestStatus: GuestStatus = 5;
pub const CrashedGuestStatus: GuestStatus = 4;
pub const StartedGuestStatus: GuestStatus = 3;
pub const InitializedGuestStatus: GuestStatus = 2;
pub const InitializingGuestStatus: GuestStatus = 1;
pub const UninitializedGuestStatus: GuestStatus = 0;
pub const BrokenGuestStatus: GuestStatus = -1;
pub const NonexistentGuestStatus: GuestStatus = -2;
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
pub struct EmbColdLoadChannel {
    pub type_0: EmbWord,
    pub unit: EmbWord,
    pub next: EmbPtr,
    pub keyboard_input_queue: EmbPtr,
    pub display_output_queue: EmbPtr,
    pub display_width: EmbWord,
    pub display_height: EmbWord,
    pub character_width: EmbWord,
    pub line_height: EmbWord,
    pub progress_note: C2RustUnnamed_6,
    pub coldLoadInput: u64,
    pub coldLoadInputSetup: bool,
    pub fd: u32,
    pub is_selected: bool,
    pub command_history_top: EmbWord,
    pub command_history_wrapped: EmbWord,
    pub command_history: [EmbWord; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub numerator: EmbWord,
    pub denominator: EmbWord,
    pub string_total_size: EmbWord,
    pub string_length: EmbWord,
    pub string: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub length: u32,
    pub chars: &str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coldmapentry {
    pub code: libc::c_short,
    pub keysym: KeySym,
}
pub type KeyboardType = libc::c_uint;
pub const Apple_Pro: KeyboardType = 3;
pub const DEC_PC: KeyboardType = 2;
pub const DEC_LK401: KeyboardType = 1;
pub const Unknown: KeyboardType = 0;
static mut fkmapDECLK: [libc::c_short; 46] = [
    0o204 as libc::c_short,
    0o204 as libc::c_short,
    0o237 as libc::c_short,
    0o237 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    0o236 as libc::c_short,
    0o236 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    0o225 as libc::c_short,
    0o225 as libc::c_short,
    0o226 as libc::c_short,
    0o226 as libc::c_short,
    0o227 as libc::c_short,
    0o227 as libc::c_short,
    0o206 as libc::c_short,
    0o206 as libc::c_short,
    0o224 as libc::c_short,
    0o224 as libc::c_short,
    0o202 as libc::c_short,
    0o202 as libc::c_short,
    0o201 as libc::c_short,
    0o201 as libc::c_short,
    0o222 as libc::c_short,
    0o222 as libc::c_short,
    0o221 as libc::c_short,
    0o221 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
];
static mut fkmapDECPC: [libc::c_short; 46] = [
    0o235 as libc::c_short,
    0o235 as libc::c_short,
    0o204 as libc::c_short,
    0o204 as libc::c_short,
    0o236 as libc::c_short,
    0o236 as libc::c_short,
    0o237 as libc::c_short,
    0o237 as libc::c_short,
    0o213 as libc::c_short,
    0o213 as libc::c_short,
    0o225 as libc::c_short,
    0o225 as libc::c_short,
    0o226 as libc::c_short,
    0o226 as libc::c_short,
    0o227 as libc::c_short,
    0o227 as libc::c_short,
    0o202 as libc::c_short,
    0o202 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    0o206 as libc::c_short,
    0o206 as libc::c_short,
    0o210 as libc::c_short,
    0o210 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    0o206 as libc::c_short,
    0o206 as libc::c_short,
    0o224 as libc::c_short,
    0o224 as libc::c_short,
    0o202 as libc::c_short,
    0o202 as libc::c_short,
    0o201 as libc::c_short,
    0o201 as libc::c_short,
    0o222 as libc::c_short,
    0o222 as libc::c_short,
    0o221 as libc::c_short,
    0o221 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
];
static mut coldmapDECPC: [coldmapentry; 23] = [
    {
        let mut init = coldmapentry {
            code: 0o207 as libc::c_short,
            keysym: 0xffff as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o206 as libc::c_short,
            keysym: 0xff63 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o210 as libc::c_short,
            keysym: 0xff08 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o210 as libc::c_short,
            keysym: 0x1000ff00 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o211 as libc::c_short,
            keysym: 0xff09 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o212 as libc::c_short,
            keysym: 0xff0a as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o212 as libc::c_short,
            keysym: 0xff53 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o215 as libc::c_short,
            keysym: 0xff0d as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o237 as libc::c_short,
            keysym: 0xff1b as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o240 as libc::c_short,
            keysym: 0xff68 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o240 as libc::c_short,
            keysym: 0xff50 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o212 as libc::c_short,
            keysym: 0xff53 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o235 as libc::c_short,
            keysym: 0xff60 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o232 as libc::c_short,
            keysym: 0xff56 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o224 as libc::c_short,
            keysym: 0xff67 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o224 as libc::c_short,
            keysym: 0xff8d as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o213 as libc::c_short,
            keysym: 0xff91 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o214 as libc::c_short,
            keysym: 0xff92 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o214 as libc::c_short,
            keysym: 0xff55 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o201 as libc::c_short,
            keysym: 0xffaf as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o222 as libc::c_short,
            keysym: 0xffaa as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o221 as libc::c_short,
            keysym: 0xffad as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: -(1) as libc::c_short,
            keysym: -(1) as KeySym,
        };
        init
    },
];
static mut fkmapApple: [libc::c_short; 46] = [
    0o235 as libc::c_short,
    0o235 as libc::c_short,
    0o204 as libc::c_short,
    0o204 as libc::c_short,
    0o236 as libc::c_short,
    0o236 as libc::c_short,
    0o213 as libc::c_short,
    0o213 as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
    -(1) as libc::c_short,
];
static mut coldmapApple: [coldmapentry; 16] = [
    {
        let mut init = coldmapentry {
            code: 0o202 as libc::c_short,
            keysym: 0 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o207 as libc::c_short,
            keysym: 0xffff as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o206 as libc::c_short,
            keysym: 0xff63 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o210 as libc::c_short,
            keysym: 0xff08 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o211 as libc::c_short,
            keysym: 0xff09 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o212 as libc::c_short,
            keysym: 0xff53 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o215 as libc::c_short,
            keysym: 0xff0d as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o237 as libc::c_short,
            keysym: 0xff1b as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o240 as libc::c_short,
            keysym: 0xff50 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o224 as libc::c_short,
            keysym: 0xff57 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o232 as libc::c_short,
            keysym: 0xff8d as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o214 as libc::c_short,
            keysym: 0xff55 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o201 as libc::c_short,
            keysym: 0xffbd as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o222 as libc::c_short,
            keysym: 0xffaf as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o221 as libc::c_short,
            keysym: 0xffaa as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: -(1) as libc::c_short,
            keysym: -(1) as KeySym,
        };
        init
    },
];
static mut coldmapDECLK: [coldmapentry; 17] = [
    {
        let mut init = coldmapentry {
            code: 0o207 as libc::c_short,
            keysym: 0xffff as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o206 as libc::c_short,
            keysym: 0xff6a as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o210 as libc::c_short,
            keysym: 0xff08 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o210 as libc::c_short,
            keysym: 0x1000ff00 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o211 as libc::c_short,
            keysym: 0xff09 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o212 as libc::c_short,
            keysym: 0xff0a as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o215 as libc::c_short,
            keysym: 0xff0d as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o237 as libc::c_short,
            keysym: 0xff1b as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o240 as libc::c_short,
            keysym: 0xff68 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o212 as libc::c_short,
            keysym: 0xff63 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o235 as libc::c_short,
            keysym: 0xff60 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o232 as libc::c_short,
            keysym: 0xff56 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o224 as libc::c_short,
            keysym: 0xff67 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o224 as libc::c_short,
            keysym: 0xff8d as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o213 as libc::c_short,
            keysym: 0xff91 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: 0o214 as libc::c_short,
            keysym: 0xff92 as KeySym,
        };
        init
    },
    {
        let mut init = coldmapentry {
            code: -(1) as libc::c_short,
            keysym: -(1) as KeySym,
        };
        init
    },
];
static mut GENERA_CPTFONT_bits: [u8; 2208] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5,
    128, 0, 0, 0, 16, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 240, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 24, 3, 0, 0, 0, 0, 0, 6, 0, 0, 0, 132, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 24, 0, 0, 0, 32, 244, 3, 0, 0, 0, 0, 0, 34, 0, 0, 0, 4, 133,
    196, 225, 51, 48, 8, 4, 0, 0, 0, 0, 0, 60, 4, 143, 7, 242, 227, 252, 60, 30, 0, 0, 0, 0, 120,
    0, 158, 143, 231, 243, 251, 121, 66, 31, 80, 40, 16, 10, 121, 62, 158, 143, 231, 19, 10, 133,
    66, 145, 31, 1, 128, 144, 0, 24, 128, 0, 0, 4, 224, 0, 2, 8, 72, 192, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 129, 128, 48, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 6, 140, 196, 4, 8, 49, 36, 0, 9, 0, 64, 2, 72, 0, 0, 0, 4, 128, 7, 0, 0, 0, 2, 17,
    1, 1, 0, 0, 32, 0, 0, 0, 32, 4, 2, 32, 0, 0, 16, 1, 65, 0, 0, 0, 4, 133, 164, 50, 74, 48, 8, 4,
    2, 0, 0, 0, 128, 66, 135, 80, 8, 19, 16, 128, 66, 33, 0, 0, 2, 16, 132, 60, 161, 80, 72, 20, 8,
    132, 66, 4, 80, 36, 48, 27, 133, 66, 161, 80, 136, 16, 10, 133, 66, 17, 16, 33, 128, 8, 1, 24,
    128, 0, 0, 4, 16, 1, 2, 8, 72, 0, 1, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 129, 128, 200, 160,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 70, 40, 243,
    243, 48, 36, 0, 9, 0, 32, 1, 48, 0, 0, 0, 4, 64, 8, 0, 0, 0, 2, 10, 129, 131, 224, 0, 64, 124,
    31, 71, 36, 4, 114, 64, 8, 8, 8, 129, 128, 248, 1, 0, 4, 197, 175, 48, 74, 32, 4, 136, 10, 1,
    0, 0, 192, 66, 132, 80, 136, 18, 8, 192, 66, 33, 0, 0, 1, 32, 132, 66, 161, 80, 72, 20, 8, 132,
    66, 4, 80, 34, 208, 42, 133, 66, 161, 80, 128, 16, 10, 133, 66, 17, 8, 97, 128, 0, 0, 8, 128,
    0, 0, 4, 16, 0, 2, 0, 64, 0, 1, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 129, 128, 0, 160, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0,
    144, 0, 0, 0, 0, 28, 4, 87, 136, 1, 240, 252, 4, 4, 66, 133, 80, 217, 64, 2, 160, 72, 36, 4,
    138, 252, 4, 144, 159, 66, 0, 1, 132, 0, 4, 128, 164, 0, 49, 16, 4, 8, 7, 1, 0, 0, 96, 98, 4,
    16, 72, 242, 9, 112, 66, 33, 134, 129, 240, 67, 64, 122, 161, 80, 64, 20, 8, 4, 66, 4, 80, 33,
    208, 74, 133, 66, 161, 80, 128, 16, 10, 133, 36, 10, 4, 193, 128, 0, 0, 16, 158, 143, 199, 231,
    121, 120, 62, 14, 76, 12, 177, 233, 120, 62, 190, 142, 239, 19, 10, 133, 66, 161, 31, 129, 128,
    0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 132, 200, 71, 242, 11, 72, 8, 10, 6, 225, 75, 38, 121, 2,
    160, 72, 228, 231, 87, 65, 126, 63, 68, 132, 128, 248, 73, 0, 4, 128, 196, 129, 48, 0, 4, 8,
    194, 7, 240, 3, 48, 82, 4, 8, 39, 2, 250, 112, 60, 62, 134, 65, 0, 128, 32, 74, 191, 79, 64,
    244, 249, 4, 126, 4, 208, 32, 16, 138, 133, 62, 161, 143, 135, 16, 10, 133, 24, 4, 15, 129,
    129, 0, 0, 0, 160, 80, 40, 20, 18, 132, 66, 8, 72, 2, 209, 26, 133, 66, 161, 81, 64, 16, 10,
    133, 36, 33, 136, 128, 0, 1, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 132, 72, 40, 4, 58, 72, 24, 17, 9,
    129, 248, 39, 69, 2, 160, 72, 36, 4, 38, 33, 4, 16, 130, 2, 65, 0, 48, 0, 4, 128, 132, 66, 72,
    1, 4, 8, 7, 1, 0, 0, 24, 74, 4, 6, 232, 7, 10, 17, 66, 32, 0, 128, 0, 64, 16, 74, 161, 80, 64,
    20, 8, 228, 66, 4, 80, 33, 16, 10, 133, 2, 161, 2, 136, 16, 146, 180, 36, 4, 2, 1, 131, 0, 0,
    0, 190, 80, 32, 244, 19, 132, 66, 8, 200, 1, 81, 10, 133, 66, 161, 128, 71, 16, 10, 133, 24,
    33, 15, 129, 128, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 72, 8, 0, 10, 72, 36, 145, 16, 129,
    72, 38, 69, 124, 159, 72, 68, 2, 86, 17, 8, 136, 31, 1, 34, 248, 1, 0, 4, 192, 143, 34, 136, 0,
    4, 136, 10, 1, 0, 0, 12, 70, 4, 65, 8, 2, 10, 25, 66, 32, 0, 0, 241, 35, 16, 122, 161, 80, 72,
    20, 8, 132, 66, 132, 80, 34, 16, 10, 133, 2, 169, 68, 136, 16, 146, 180, 66, 4, 1, 1, 134, 0,
    0, 0, 161, 80, 32, 20, 16, 132, 66, 8, 72, 2, 17, 10, 133, 66, 161, 0, 72, 16, 146, 148, 36,
    18, 2, 129, 128, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 72, 8, 0, 8, 72, 66, 145, 16, 225, 83,
    217, 68, 0, 128, 136, 67, 2, 138, 8, 0, 0, 1, 1, 0, 0, 0, 0, 0, 128, 164, 18, 139, 0, 8, 4, 2,
    192, 0, 96, 4, 66, 132, 64, 8, 18, 10, 9, 66, 16, 134, 1, 2, 16, 0, 2, 161, 80, 72, 20, 8, 132,
    66, 132, 80, 36, 16, 10, 133, 2, 145, 72, 136, 16, 146, 204, 66, 132, 0, 1, 132, 0, 0, 0, 161,
    80, 40, 20, 16, 248, 66, 8, 72, 4, 17, 10, 133, 66, 161, 64, 72, 20, 146, 180, 66, 12, 1, 129,
    128, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 215, 7, 0, 240, 72, 66, 14, 15, 1, 224, 0, 56, 0, 0,
    0, 128, 241, 115, 252, 0, 128, 0, 192, 227, 1, 0, 0, 4, 128, 196, 17, 115, 1, 8, 4, 0, 192, 0,
    96, 0, 60, 159, 159, 7, 226, 241, 8, 60, 14, 134, 1, 0, 0, 16, 60, 161, 143, 231, 243, 11, 120,
    66, 31, 79, 232, 23, 10, 121, 2, 174, 144, 135, 224, 97, 132, 66, 132, 31, 1, 128, 0, 0, 0,
    190, 143, 199, 231, 19, 128, 66, 8, 72, 8, 17, 10, 121, 62, 190, 128, 135, 227, 97, 72, 66,
    132, 31, 129, 128, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 16, 2, 0, 128, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 15, 240, 0, 0, 0, 0, 0, 0, 0, 0, 132, 0, 128, 8, 0, 0, 0, 0, 2, 32, 0, 0, 0, 0, 0,
    0, 2, 0, 134, 96, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 24, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 252, 0, 0, 0, 0, 0, 0, 120, 0, 0, 7, 0, 0, 0, 0, 2, 32, 0, 0, 0, 0, 0, 0,
    1, 0, 128, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 231, 7, 56, 0, 0, 0,
];
static mut GeneraIcon32_bits: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 252, 15, 0, 0, 172, 10, 0, 0, 84, 13, 0, 0, 172, 10, 0,
    0, 84, 13, 0, 0, 172, 10, 0, 0, 84, 253, 1, 0, 172, 158, 6, 0, 84, 75, 10, 0, 252, 37, 25, 0,
    0, 73, 18, 0, 128, 146, 36, 0, 132, 73, 50, 0, 134, 36, 41, 0, 142, 73, 50, 0, 141, 146, 36, 0,
    27, 73, 18, 128, 26, 37, 17, 128, 53, 74, 10, 64, 53, 156, 6, 192, 106, 240, 1, 160, 106, 0, 0,
    96, 213, 0, 0, 80, 213, 0, 0, 176, 170, 1, 0, 168, 170, 1, 0, 248, 255, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

pub static mut manage_run_lights: u32 = 0;

pub static mut run_lights_state: u32 = 0;
static mut cold_channel: *mut EmbColdLoadChannel =
    0 as *const EmbColdLoadChannel as *mut EmbColdLoadChannel;
static mut keyboard_queue: *mut EmbQueue = 0 as *const EmbQueue as *mut EmbQueue;
static mut display_queue: *mut EmbQueue = 0 as *const EmbQueue as *mut EmbQueue;
static mut display: *mut Display = 0 as *const Display as *mut Display;
static mut screen: *mut Screen = 0 as *const Screen as *mut Screen;
static mut visual: *mut Visual = 0 as *const Visual as *mut Visual;
static mut window: Window = 0;
static mut icon_window: Window = 0;
static mut root: Window = 0;
static mut colormap: Colormap = 0;
static mut gc: GC = 0 as *const _XGC as *mut _XGC;
static mut icon_gc: GC = 0 as *const _XGC as *mut _XGC;
static mut icon_gc_s: GC = 0 as *const _XGC as *mut _XGC;
static mut icon_gc_c: GC = 0 as *const _XGC as *mut _XGC;
static mut icon_gc_t: GC = 0 as *const _XGC as *mut _XGC;
static mut icon_bitmap: Pixmap = 0 as Pixmap;
static mut cptfont_bitmap: Pixmap = 0 as Pixmap;
static mut originalModmap: *mut XModifierKeymap =
    0 as *const XModifierKeymap as *mut XModifierKeymap;
static mut icon_width: u32 = 32;
static mut icon_height: u32 = 36;
static mut char_width: u32 = 0;
static mut char_height: u32 = 0;
static mut width: u32 = 0;
static mut height: u32 = 0;
static mut loff: u32 = 0;
static mut toff: u32 = 0;
static mut roff: u32 = 0;
static mut boff: u32 = 0;
static mut lmarg: u32 = 3;
static mut tmarg: u32 = 22;
static mut rmarg: u32 = 3;
static mut bmarg: u32 = 3;
static mut current_x: u32 = 0;
static mut current_y: u32 = 0;
static mut cursor_visible: u32 = 0;
static mut cursor_frozen: u32 = 0;
static mut cursor_state: u32 = 0;
static mut light_state: u32 = 0;
static mut visibility: u32 = 0;
static mut icon_visibility: u32 = 0;
static mut run_light_y: u32 = 0;
static mut run_light_first_x: u32 = 0;
static mut run_label_y: u32 = 0;
static mut progress_bar_first_x: u32 = 0;
static mut progress_bar_width: u32 = 0;
static mut run_label_width: u32 = 0;
static mut run_label_height: u32 = 0;
static mut progress_bar_numerator_state: u32 = 0;
static mut progress_bar_denominator_state: u32 = 0;
static mut progress_bar_length_state: u32 = 0;
static mut progress_label_length: u32 = 0;
static mut progress_label: &str = 0 as &str;
static mut meta_mask: u32 = 0;
static mut super_mask: u32 = 0;
static mut hyper_mask: u32 = 0;
static mut x_io_error: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut screen_array: *mut line = 0 as *const line as *mut line;
static mut keyboardType: KeyboardType = Unknown;
static mut skMap: *mut coldmapentry = 0 as *const coldmapentry as *mut coldmapentry;
static mut fkMap: *mut libc::c_short = 0 as *const libc::c_short as *mut libc::c_short;
static mut removeNumLockModifier: u32 = 0;
fn open_cold_load_display(mut params: *mut XParams, mut noWaiting: bool) -> u32 {
    open_display(params, noWaiting);
    if !display.is_null() {
        replay_command_history();
        return XConnectionNumber(display);
    } else {
        return -(1);
    };
}
fn manage_x_input(mut params: *mut XParams) -> u32 {
    while !display.is_null() && XPending(display) != 0 {
        handle_input();
    }
    if display.is_null() {
        close_display();
        open_display(params, false);
    }
    return if display.is_null() {
        -(1)
    } else {
        XConnectionNumber(display)
    };
}
fn manage_cold_load_output() {
    while EmbQueueFilled(display_queue) != 0 {
        handle_output();
    }
}
fn update_cold_load_blinkers() {
    show_cursor_internal(((*EmbCommAreaPtr).fep).cursor());
    show_lights(0);
    XFlush(display);
}
fn setup_x_io_error_handler() -> u32 {
    return _setjmp(x_io_error.as_mut_ptr());
}
fn stop_cold_x() {
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0; 4],
    };
    let mut __cancel_routine: Option<fn(*mut libc::c_void) -> ()> =
        ::std::mem::transmute::<Option<fn(*mut u64) -> u32>, pthread_cleanuproutine_t>(Some(
            pthread_mutex_unlock as fn(*mut u64) -> u32,
        ));
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock as *mut u64;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support XLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    close_display();
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
            b"Unable to unlock the Life Support XLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
}
fn open_display(mut params: *mut XParams, mut noWaiting: bool) {
    let mut wmhints: XWMHints = XWMHints {
        flags: 0,
        input: 0,
        initial_state: 0,
        icon_pixmap: 0,
        icon_window: 0,
        icon_x: 0,
        icon_y: 0,
        icon_mask: 0,
        window_group: 0,
    };
    let mut sizehints: XSizeHints = XSizeHints {
        flags: 0,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        min_width: 0,
        min_height: 0,
        max_width: 0,
        max_height: 0,
        width_inc: 0,
        height_inc: 0,
        min_aspect: C2RustUnnamed_1 { x: 0, y: 0 },
        max_aspect: C2RustUnnamed_1 { x: 0, y: 0 },
        base_width: 0,
        base_height: 0,
        win_gravity: 0,
    };
    let mut color: XColor = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    let mut attributes: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut gcv: XGCValues = XGCValues {
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
    let mut fontinfo: *mut XFontStruct = 0 as *mut XFontStruct;
    let mut display_name: [libc::c_char; 8192] = [0; 8192];
    let mut cp: &str = "";
    let mut screen_no: u32 = 0;
    let mut border_width: u32 = 0;
    let mut w_x: u32 = 0;
    let mut w_y: u32 = 0;
    let mut w_w: u32 = 0;
    let mut w_h: u32 = 0;
    let mut g_flags: u32 = 0;
    let mut openSleep: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    BuildXDisplayName(
        display_name.as_mut_ptr(),
        (*params).xpHostName,
        (*params).xpDisplay,
        (*params).xpScreen,
    );
    display = XOpenDisplay(display_name.as_mut_ptr());
    if display.is_null() {
        if noWaiting != 0 {
            return;
        } else {
            verror(b"Cold Load\0" as &str);
            vwarn(
                b"Cold Load\0" as &str,
                b"Waiting for X server... \0" as &str,
            );
            while display.is_null() {
                openSleep.tv_sec = 5 as __time_t;
                openSleep.tv_nsec = 0 as __syscall_slong_t;
                if pthread_delay_np(&mut openSleep) != 0 {
                    vpunt(b"Unable to sleep in thread %lx\0" as &str, pthread_self());
                }
                display = XOpenDisplay(display_name.as_mut_ptr());
            }
            fprintf(stderr, b"Done.\n\0");
        }
    }
    screen_no = XDefaultScreen(display);
    screen = XDefaultScreenOfDisplay(display);
    visual = XDefaultVisualOfScreen(screen);
    root = XRootWindowOfScreen(screen);
    colormap = XDefaultColormapOfScreen(screen);
    originalModmap = XGetModifierMapping(display);
    setup_modifier_mapping();
    fontinfo = XLoadQueryFont(display, b"genera-cptfont\0");
    if !fontinfo.is_null() {
        gcv.font = (*fontinfo).fid;
        XFreeFontInfo(0 as *mut &str, fontinfo, 0);
    } else {
        gcv.font = 0 as Font;
    }
    char_width = 8;
    char_height = 12;
    roff = rmarg - 0;
    toff = tmarg + 10;
    loff = lmarg + 0;
    boff = bmarg + 2;
    border_width = if (*params).xpBorderWidth < 0 {
        2
    } else {
        (*params).xpBorderWidth
    };
    if !((*params).xpGeometry).is_null() {
        g_flags = XGeometry(
            display,
            screen_no,
            (*params).xpGeometry,
            b"800x800+100+100\0",
            border_width,
            char_width,
            char_height,
            roff + loff,
            toff + boff,
            &mut w_x,
            &mut w_y,
            &mut w_w,
            &mut w_h,
        );
    } else {
        g_flags = 0;
        w_x = 100;
        w_y = 100;
        w_w = 800;
        w_h = 800;
    }
    if !((*params).xpForegroundColor).is_null()
        && XAllocNamedColor(
            display,
            colormap,
            (*params).xpForegroundColor,
            &mut color,
            &mut color,
        ) != 0
    {
        gcv.foreground = color.pixel;
    } else {
        gcv.foreground = XBlackPixelOfScreen(screen);
    }
    if !((*params).xpBackgroundColor).is_null()
        && XAllocNamedColor(
            display,
            colormap,
            (*params).xpBackgroundColor,
            &mut color,
            &mut color,
        ) != 0
    {
        gcv.background = color.pixel;
    } else {
        gcv.background = XWhitePixelOfScreen(screen);
    }
    if !((*params).xpBorderColor).is_null()
        && XAllocNamedColor(
            display,
            colormap,
            (*params).xpBorderColor,
            &mut color,
            &mut color,
        ) != 0
    {
        attributes.border_pixel = color.pixel;
    } else {
        attributes.border_pixel = XBlackPixelOfScreen(screen);
    }
    attributes.background_pixel = gcv.background;
    attributes.event_mask = (1) << 0 | (1) << 15 | (1) << 17 | (1) << 21 | (1) << 16;
    attributes.colormap = colormap;
    window = XCreateWindow(
        display,
        root,
        w_x,
        w_y,
        w_w,
        w_h,
        border_width,
        0,
        1,
        visual,
        ((1) << 1 | (1) << 3 | (1) << 11 | (1) << 13),
        &mut attributes,
    );
    icon_window = XCreateWindow(
        display,
        root,
        w_x,
        w_y,
        icon_width,
        icon_height,
        0,
        0,
        1,
        visual,
        ((1) << 1 | (1) << 11 | (1) << 13),
        &mut attributes,
    );
    gc = XCreateGC(
        display,
        window,
        ((1) << 2 | (1) << 3 | (if gcv.font != 0 { (1) << 14 } else { 0 })),
        &mut gcv,
    );
    icon_gc = XCreateGC(display, icon_window, ((1) << 2 | (1) << 3), &mut gcv);
    if gcv.font == 0 {
        cptfont_bitmap = XCreateBitmapFromData(
            display,
            root,
            GENERA_CPTFONT_bits.as_mut_ptr() as &str,
            1472,
            12,
        );
    }
    if XCellsOfScreen(screen) < 16 {
        icon_bitmap = XCreateBitmapFromData(
            display,
            icon_window,
            GeneraIcon32_bits.as_mut_ptr() as &str,
            32,
            32,
        );
        icon_gc_t = 0 as GC;
        icon_gc_c = icon_gc_t;
        icon_gc_s = icon_gc_c;
    } else {
        icon_bitmap = 0 as Pixmap;
        color.red = 0 as libc::c_ushort;
        color.green = 65535 as libc::c_ushort;
        color.blue = 0 as libc::c_ushort;
        if XAllocColor(display, colormap, &mut color) != 0 {
            gcv.foreground = color.pixel;
            icon_gc_s = XCreateGC(display, icon_window, ((1) << 2), &mut gcv);
        } else {
            icon_gc_s = icon_gc;
        }
        color.red = 65535 as libc::c_ushort;
        color.green = 0 as libc::c_ushort;
        color.blue = 0 as libc::c_ushort;
        if XAllocColor(display, colormap, &mut color) != 0 {
            gcv.foreground = color.pixel;
            icon_gc_c = XCreateGC(display, icon_window, ((1) << 2), &mut gcv);
        } else {
            icon_gc_c = icon_gc;
        }
        color.red = 65535 as libc::c_ushort;
        color.green = 0 as libc::c_ushort;
        color.blue = 65535 as libc::c_ushort;
        if XAllocColor(display, colormap, &mut color) != 0 {
            gcv.foreground = color.pixel;
            icon_gc_t = XCreateGC(display, icon_window, ((1) << 2), &mut gcv);
        } else {
            icon_gc_t = icon_gc;
        }
    }
    SetColdLoadNames();
    wmhints.flags = (1) << 0 | (1) << 1 | (1) << 3;
    wmhints.input = 1;
    wmhints.initial_state = if (*params).xpInitialState == Iconic {
        3
    } else {
        1
    };
    wmhints.icon_window = icon_window;
    XSetWMHints(display, window, &mut wmhints);
    sizehints.flags = (if g_flags & 0x1 != 0 {
        (1) << 0
    } else {
        (1) << 2
    }) | (if g_flags & 0x4 != 0 {
        (1) << 1
    } else {
        (1) << 3
    });
    sizehints.x = w_x;
    sizehints.y = w_y;
    sizehints.width = w_w;
    sizehints.height = w_h;
    XSetNormalHints(display, window, &mut sizehints);
    XMapWindow(display, window);
    XFlush(display);
    alloc_screen_array(w_w, w_h);
}
fn handle_input() {
    let mut event: XEvent = _XEvent { type_0: 0 };
    let mut keysym: KeySym = 0;
    let mut key: u32 = -(1);
    let mut bits: u32 = 0;
    let mut mapp: *mut coldmapentry = 0 as *mut coldmapentry;
    static mut first_keypress: u32 = 1;
    XNextEvent(display, &mut event);
    match event.type_0 {
        22 => {
            if event.xconfigure.window == window {
                alloc_screen_array(event.xconfigure.width, event.xconfigure.height);
            } else if event.xconfigure.window == icon_window {
                icon_width = event.xconfigure.width;
                icon_height = event.xconfigure.height;
            }
        }
        12 => {
            if event.xexpose.window == window {
                if event.xexpose.y < tmarg {
                    show_lights(1);
                }
                hide_cursor();
                redisplay_screen_array(
                    (event.xexpose.x - lmarg) / char_width,
                    (event.xexpose.y - tmarg) / char_height,
                    (event.xexpose.x - lmarg + event.xexpose.width - 1) / char_width + 1,
                    (event.xexpose.y - tmarg + event.xexpose.height - 1) / char_height + 1,
                );
                reset_light_state(1);
                show_lights(1);
            } else if event.xexpose.window == icon_window {
                show_icon();
            }
        }
        2 => {
            if first_keypress != 0 {
                first_keypress = 0;
                alarm(0);
            }
            keysym = XLookupKeysym(&mut event.xkey, 0);
            if !(keysym >= 0xffe1 && keysym <= 0xffee
                || keysym >= 0xfe01 && keysym <= 0xfe13
                || keysym == 0xff7e
                || keysym == 0xff7f
                || 0xff20 == keysym
                || 0xff94 == keysym)
            {
                if event.xkey.state & ((1) << 2) != 0 {
                    bits |= 1;
                }
                if event.xkey.state & meta_mask != 0 {
                    bits |= 2;
                }
                if event.xkey.state & super_mask != 0 {
                    bits |= 4;
                }
                if event.xkey.state & hyper_mask != 0 {
                    bits |= 8;
                }
                if 0x61 <= keysym && keysym <= 0x7a {
                    key = keysym.wrapping_sub(0x61).wrapping_add(65);
                    if if bits == 0 {
                        (event.xkey.state & ((1) << 0 | (1) << 1) == 0)
                    } else {
                        event.xkey.state & ((1) << 0)
                    } != 0
                    {
                        key = key + 32;
                    }
                } else if 0xffbe <= keysym && keysym <= 0xffd4 {
                    key =
                        *fkMap.offset((2).wrapping_mul(keysym.wrapping_sub(0xffbe)).wrapping_add(
                            (if event.xkey.state & ((1) << 0) != 0 {
                                1
                            } else {
                                0
                            }),
                        ));
                } else {
                    if event.xkey.state & ((1) << 0) != 0 {
                        if 0xff8d == keysym {
                            key = 0o215;
                        } else {
                            keysym = XLookupKeysym(&mut event.xkey, 1);
                        }
                    }
                    if 0x20 <= keysym && keysym <= 0x7e {
                        key = keysym;
                    } else if key == -(1) {
                        mapp = skMap;
                        while (*mapp).code != -(1) {
                            if keysym == (*mapp).keysym {
                                key = (*mapp).code;
                                break;
                            } else {
                                mapp = mapp.offset(1);
                            }
                        }
                    }
                }
                if key == -(1) {
                    XBell(display, 0);
                } else {
                    EmbQueuePutWord(
                        keyboard_queue,
                        ((0o200) << 24 | ((bits as UEmbWord) << 12) | key as UEmbWord) as EmbWord,
                    );
                    if key == 0o204 && bits & 9 == 9 {
                        (*EmbCommAreaPtr).stop_request = 1;
                    }
                }
            }
        }
        34 => {
            XRefreshKeyboardMapping(&mut event.xmapping);
            if event.xmapping.request == 0 {
                setup_modifier_mapping();
            }
        }
        15 => {
            if event.xvisibility.window == window {
                visibility = (event.xvisibility.state != 2);
            } else if event.xvisibility.window == icon_window {
                icon_visibility = (event.xvisibility.state != 2);
            }
        }
        9 => {
            cursor_frozen = 0;
            show_cursor_internal(((*EmbCommAreaPtr).fep).cursor());
        }
        10 => {
            show_cursor_internal(1);
            cursor_frozen = 1;
        }
        _ => {}
    };
}
fn alloc_screen_array(mut new_width_pixels: u32, mut new_height_pixels: u32) {
    let mut old_screen_array: *mut line = screen_array;
    let mut old_width: u32 = width;
    let mut old_height: u32 = height;
    let mut y: u32 = 0;
    let mut new_width: u32 = 0;
    let mut new_height: u32 = 0;
    let mut pixels_per_run_light: u32 = 0;
    new_width = (new_width_pixels - (roff + loff)) / char_width;
    new_height = (new_height_pixels - (toff + char_height + 3 + boff)) / char_height;
    if new_width == old_width && new_height == old_height {
        return;
    }
    screen_array = malloc((new_height).wrapping_mul(::std::mem::size_of::<line>())) as *mut line;
    while y < new_height {
        (*screen_array.offset(y)).length = 0;
        let ref mut fresh0 = (*screen_array.offset(y)).chars;
        *fresh0 = malloc(new_width) as &str;
        memset((*screen_array.offset(y)).chars, ' ' as i32, new_width);
        if y < old_height {
            (*screen_array.offset(y)).length = if (*old_screen_array.offset(y)).length < new_width {
                (*old_screen_array.offset(y)).length
            } else {
                new_width
            };
            memcpy(
                (*screen_array.offset(y)).chars,
                (*old_screen_array.offset(y)).chars,
                (*screen_array.offset(y)).length,
            );
        }
        y += 1;
    }
    if !old_screen_array.is_null() {
        y = 0;
        while y < old_height {
            free((*old_screen_array.offset(y)).chars);
            y += 1;
        }
        free(old_screen_array);
    }
    (*cold_channel).character_width = 1;
    (*cold_channel).line_height = 1;
    (*cold_channel).display_width = new_width;
    (*cold_channel).display_height = new_height;
    run_light_y = new_height_pixels - (3 - 1);
    run_label_y = new_height_pixels - 3;
    run_label_height = char_height;
    pixels_per_run_light = (new_width_pixels - (roff + loff)) / 32;
    run_light_first_x = pixels_per_run_light * 8 + loff;
    run_label_width = new_width_pixels - run_light_first_x - roff;
    progress_bar_first_x = pixels_per_run_light * 22 + loff;
    progress_bar_width = new_width_pixels - loff - progress_bar_first_x - roff;
    reset_light_state(1);
    EmbQueuePutWord(keyboard_queue, ((0o201) << 24) as EmbWord);
    width = new_width;
    height = new_height;
}
fn redisplay_line(mut y: u32, mut x: u32, mut xlim: u32) {
    if cptfont_bitmap == 0 {
        XDrawImageString(
            display,
            window,
            gc,
            x * char_width + loff,
            y * char_height + toff,
            &mut *((*screen_array.offset(y)).chars).offset(x),
            xlim - x,
        );
    } else {
        let mut cx: u32 = 0;
        let mut wx: u32 = 0;
        let mut wy: u32 = y * char_height + tmarg;
        cx = x;
        wx = x * char_width + lmarg;
        while cx < xlim {
            XCopyPlane(
                display,
                cptfont_bitmap,
                window,
                gc,
                (char_width - 1) * *((*screen_array.offset(y)).chars).offset(cx),
                0,
                (char_width - 1),
                char_height,
                wx,
                wy,
                1,
            );
            cx += 1;
            wx += char_width;
        }
    };
}
fn redisplay_screen_array(mut minx: u32, mut miny: u32, mut maxx: u32, mut maxy: u32) {
    let mut y: u32 = 0;
    let mut this_minx: u32 = if (0) < minx { minx } else { 0 };
    let mut this_miny: u32 = if (0) < miny { miny } else { 0 };
    let mut this_maxy: u32 = if height < maxy { height } else { maxy };
    y = this_miny;
    while y < this_maxy {
        let mut this_maxx: u32 = if (*screen_array.offset(y)).length < maxx {
            (*screen_array.offset(y)).length
        } else {
            maxx
        };
        if this_minx < this_maxx {
            redisplay_line(y, this_minx, this_maxx);
        }
        y += 1;
    }
}
fn show_cursor_internal(mut new_state: u32) {
    if visibility != 0 && cursor_frozen == 0 {
        if cursor_visible != 0 && cursor_state != new_state {
            hide_cursor();
        }
        if cursor_visible == 0 {
            cursor_state = ((*EmbCommAreaPtr).fep).cursor();
            if cursor_state != 0 {
                XFillRectangle(
                    display,
                    window,
                    gc,
                    current_x * char_width + lmarg,
                    current_y * char_height + tmarg,
                    (char_width - 1),
                    (char_height - 1),
                );
            }
            XDrawRectangle(
                display,
                window,
                gc,
                current_x * char_width + lmarg,
                current_y * char_height + tmarg,
                (char_width - 1),
                (char_height - 1),
            );
            cursor_visible = 1;
        }
    }
}
fn hide_cursor() {
    if cursor_visible != 0 {
        XClearArea(
            display,
            window,
            current_x * char_width + lmarg,
            current_y * char_height + tmarg,
            char_width,
            char_height,
            0,
        );
        redisplay_screen_array(current_x, current_y, current_x + 1, current_y + 1);
        cursor_visible = 0;
    }
}
fn show_icon() {
    let mut tri: [XPoint; 3] = [XPoint { x: 0, y: 0 }; 3];
    let mut xoff: u32 = if icon_width > 32 {
        (icon_width - 32) / 2
    } else {
        0
    };
    if icon_bitmap != 0 {
        XCopyPlane(
            display,
            icon_bitmap,
            icon_window,
            icon_gc,
            0,
            0,
            32,
            32,
            xoff,
            0,
            1,
        );
    } else {
        XFillRectangle(display, icon_window, icon_gc_s, xoff + 10, 3, 9, 9);
        XFillArc(
            display,
            icon_window,
            icon_gc_c,
            xoff + 15,
            9,
            14,
            14,
            0,
            360 * 64,
        );
        tri[0].x = (xoff + 3) as libc::c_short;
        tri[0].y = 29 as libc::c_short;
        tri[1].x = (xoff + 10) as libc::c_short;
        tri[1].y = 15 as libc::c_short;
        tri[2].x = (xoff + 17) as libc::c_short;
        tri[2].y = 29 as libc::c_short;
        XFillPolygon(display, icon_window, icon_gc_t, tri.as_mut_ptr(), 3, 2, 0);
    };
}
fn show_lights(mut force: u32) {
    let mut i: u32 = 0;
    let mut bit: u32 = 0;
    let mut changed: u32 = light_state ^ (*EmbCommAreaPtr).run_lights;
    let mut cls: *mut EmbColdLoadChannel = 0 as *mut EmbColdLoadChannel;
    let mut pb_length: u32 = 0;
    let mut pb_length_change: u32 = 0;
    light_state = (*EmbCommAreaPtr).run_lights;
    if visibility != 0 {
        if force != 0 || changed != 0 {
            i = run_light_first_x;
            bit = 1;
            while bit < 32 {
                if force != 0 || changed & bit != 0 {
                    if light_state & bit != 0 {
                        XFillRectangle(display, window, gc, i, run_light_y, 32, 1);
                    }
                } else {
                    XClearArea(display, window, i, run_light_y, 32, 1, 0);
                }
                i += 40;
                bit = bit << 1;
            }
        }
    }
    cls = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*EmbCommAreaPtr).cold_load_channel)
        as *mut EmbWord as PtrV as *mut EmbColdLoadChannel;
    if !cls.is_null() {
        if (*cls).progress_note.string_length == 0 {
            if !progress_label.is_null() {
                XClearArea(
                    display,
                    window,
                    run_light_first_x,
                    run_label_y - run_label_height + 1,
                    run_label_width,
                    run_label_height,
                    0,
                );
                free(progress_label);
                progress_label = "";
            }
            if progress_bar_length_state != 0 {
                XClearArea(
                    display,
                    window,
                    progress_bar_first_x,
                    run_light_y,
                    progress_bar_width,
                    1,
                    0,
                );
                progress_bar_length_state = 0;
                progress_bar_denominator_state = progress_bar_length_state;
                progress_bar_numerator_state = progress_bar_denominator_state;
            }
        } else {
            if progress_label.is_null() {
                XDrawString(
                    display,
                    window,
                    gc,
                    run_light_first_x + 3 * 40,
                    run_label_y,
                    b"Run\0",
                    3,
                );
                XDrawString(
                    display,
                    window,
                    gc,
                    run_light_first_x + 2 * 40,
                    run_label_y,
                    b"Disk\0",
                    4,
                );
                XDrawString(
                    display,
                    window,
                    gc,
                    run_light_first_x + 5 * 40,
                    run_label_y,
                    b"Net\0",
                    3,
                );
                progress_label = calloc(
                    (*cls).progress_note.string_total_size,
                    ::std::mem::size_of::<libc::c_char>(),
                ) as &str;
                progress_label_length = 0;
            }
            if progress_label_length != (*cls).progress_note.string_length
                || strcmp(progress_label, ((*cls).progress_note.string).as_mut_ptr()) != 0
            {
                progress_label_length = (*cls).progress_note.string_length;
                strncpy(
                    progress_label,
                    ((*cls).progress_note.string).as_mut_ptr(),
                    progress_label_length,
                );
                XClearArea(
                    display,
                    window,
                    progress_bar_first_x,
                    run_label_y - run_label_height + 1,
                    progress_bar_width,
                    run_label_height,
                    0,
                );
                XDrawString(
                    display,
                    window,
                    gc,
                    progress_bar_first_x,
                    run_label_y,
                    progress_label,
                    progress_label_length,
                );
            }
            if (*cls).progress_note.denominator > 0 {
                if progress_bar_numerator_state != (*cls).progress_note.numerator
                    || progress_bar_denominator_state != (*cls).progress_note.denominator
                {
                    progress_bar_numerator_state = (*cls).progress_note.numerator;
                    progress_bar_denominator_state = (*cls).progress_note.denominator;
                    pb_length = progress_bar_numerator_state * progress_bar_width
                        / progress_bar_denominator_state;
                    pb_length_change = pb_length - progress_bar_length_state;
                    if pb_length_change < 0 {
                        XClearArea(
                            display,
                            window,
                            progress_bar_first_x + pb_length,
                            run_light_y,
                            -pb_length_change,
                            1,
                            0,
                        );
                        progress_bar_length_state = pb_length;
                    } else if pb_length_change > 0 {
                        XFillRectangle(
                            display,
                            window,
                            gc,
                            progress_bar_first_x + progress_bar_length_state,
                            run_light_y,
                            pb_length_change,
                            1,
                        );
                        progress_bar_length_state = pb_length;
                    }
                }
            }
        }
    }
    if icon_visibility != 0 {
        if force != 0 || changed != 0 {
            i = 2;
            bit = 1;
            while bit < 32 {
                if force != 0 || changed & bit != 0 {
                    if light_state & bit != 0 {
                        XFillRectangle(display, icon_window, icon_gc, i, 32, 4, 4);
                    } else {
                        XClearArea(display, icon_window, i, 32, 4, 4, 0);
                    }
                }
                i += 6;
                bit = bit << 1;
            }
        }
    }
}
fn reset_light_state(mut screen_cleared_p: u32) {
    if screen_cleared_p == 1 {
        progress_bar_length_state = 0;
        progress_bar_denominator_state = progress_bar_length_state;
        progress_bar_numerator_state = progress_bar_denominator_state;
        light_state = 0;
    }
    if !progress_label.is_null() {
        free(progress_label);
        progress_label = "";
    }
}
fn replay_command_history() {
    let mut i: u32 = 0;
    let mut have_pos: u32 = 0;
    if (*cold_channel).command_history_wrapped != 0 {
        i = (*cold_channel).command_history_top + 1;
    } else {
        i = 0;
    }
    while i != (*cold_channel).command_history_top {
        if i == 1024 {
            i = 0;
        }
        if have_pos == 0 && ((*cold_channel).command_history[i] >> 24 & 0xff) == 0o1 {
            have_pos = 1;
        }
        if have_pos != 0 {
            handle_output_command((*cold_channel).command_history[i] as UEmbWord);
        }
        i += 1;
    }
    reset_light_state(0);
    show_lights(1);
}
fn handle_output() {
    let mut command: UEmbWord = 0;
    while EmbQueueFilled(display_queue) != 0 {
        hide_cursor();
        command = EmbQueueTakeWord(display_queue) as UEmbWord;
        let ref mut fresh1 = (*cold_channel).command_history_top;
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        (*cold_channel).command_history[fresh2] = command as EmbWord;
        if (*cold_channel).command_history_top == 1024 {
            (*cold_channel).command_history_top = 0;
            (*cold_channel).command_history_wrapped = 1;
        }
        handle_output_command(command);
    }
}
fn handle_output_command(mut command: UEmbWord) {
    let mut operator: u32 = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut c: libc::c_char = 0;
    let mut event: XEvent = _XEvent { type_0: 0 };
    operator = (command >> 24 & 0xff);
    let mut current_block_35: u64;
    match operator {
        0 | 5 => {
            if current_y < height && current_x < width {
                if (*screen_array.offset(current_y)).length <= current_x {
                    x = (*screen_array.offset(current_y)).length;
                    while x < current_x {
                        *((*screen_array.offset(current_y)).chars).offset(x) = ' ' as i32;
                        x += 1;
                    }
                    (*screen_array.offset(current_y)).length = current_x + 1;
                }
                c = (command & 0xff);
                *((*screen_array.offset(current_y)).chars).offset(current_x) = c;
                redisplay_line(current_y, current_x, current_x + 1);
            }
            current_x += 1;
            current_block_35 = 572715077006366937;
        }
        1 => {
            current_x = (command & 0xfff);
            current_y = (command >> 12 & 0xfff);
            current_block_35 = 572715077006366937;
        }
        3 => {
            y = current_y + 1;
            while y < height {
                (*screen_array.offset(y)).length = 0;
                y += 1;
            }
            XClearArea(
                display,
                window,
                lmarg,
                (current_y + 1) * char_height + tmarg,
                (width * char_width),
                ((height - (current_y + 1)) * char_height),
                0,
            );
            reset_light_state(1);
            current_block_35 = 4678190163169490533;
        }
        2 => {
            current_block_35 = 4678190163169490533;
        }
        10 => {
            XBell(display, 0);
            current_block_35 = 572715077006366937;
        }
        11 => {
            XMapRaised(display, window);
            XBell(display, 0);
            current_block_35 = 572715077006366937;
        }
        12 => {
            event.xclient.type_0 = 33;
            event.xclient.display = display;
            event.xclient.window = window;
            event.xclient.message_type = XInternAtom(display, b"WM_CHANGE_STATE\0", 0);
            event.xclient.format = 32;
            event.xclient.data.l[0] = 3;
            XSendEvent(display, root, 0, (1) << 20 | (1) << 19, &mut event);
            current_block_35 = 572715077006366937;
        }
        _ => {
            current_block_35 = 572715077006366937;
        }
    }
    match current_block_35 {
        4678190163169490533 => {
            if current_x < (*screen_array.offset(current_y)).length {
                (*screen_array.offset(current_y)).length = current_x;
                XClearArea(
                    display,
                    window,
                    current_x * char_width + lmarg,
                    current_y * char_height + tmarg,
                    ((width - current_x) * char_width),
                    char_height,
                    0,
                );
            }
        }
        _ => {}
    };
}
fn get_keyboard_modifier_codes(
    mut control_l_code: *mut KeyCode,
    mut control_r_code: *mut KeyCode,
    mut meta_l_code: *mut KeyCode,
    mut meta_r_code: *mut KeyCode,
    mut alt_l_code: *mut KeyCode,
    mut super_code: *mut KeyCode,
    mut hyper_code: *mut KeyCode,
) {
    let mut keycode1: KeyCode = 0;
    let mut keycode2: KeyCode = 0;
    *control_l_code = XKeysymToKeycode(display, 0xffe3 as KeySym);
    *control_r_code = XKeysymToKeycode(display, 0xffe4 as KeySym);
    *meta_l_code = XKeysymToKeycode(display, 0xffe7 as KeySym);
    *meta_r_code = XKeysymToKeycode(display, 0xffe8 as KeySym);
    *alt_l_code = XKeysymToKeycode(display, 0xffe9 as KeySym);
    keycode1 = XKeysymToKeycode(display, 0xfe20 as KeySym);
    keycode2 = XKeysymToKeycode(display, 0xc5 as KeySym);
    printf(b"keycode1 %d, keycode2 %d\n\0", keycode1, keycode2);
    if keycode1 != 0 || keycode2 != 0 {
        keyboardType = Apple_Pro;
        skMap = &mut coldmapApple as *mut [coldmapentry; 16] as *mut coldmapentry;
        fkMap = &mut fkmapApple as *mut [libc::c_short; 46] as *mut libc::c_short;
        if keycode1 != 0 {
            (*skMap).keysym = 0xff7f as KeySym;
            removeNumLockModifier = 1;
        } else {
            (*skMap).keysym = 0 as KeySym;
        }
        *super_code = XKeysymToKeycode(display, 0xff54 as KeySym);
        *hyper_code = XKeysymToKeycode(display, 0xff51 as KeySym);
    } else {
        keycode1 = XKeysymToKeycode(display, 0xff20 as KeySym);
        keycode2 = XKeysymToKeycode(display, 0x20 as KeySym);
        *super_code = (keycode1 + 4) as KeyCode;
        *hyper_code = keycode1;
        printf(b"dec keyboard\n\0");
        printf(b"keycode1 %d, keycode2 %d\n\0", keycode1, keycode2);
        if keycode1 == keycode2 {
            *hyper_code = 0 as KeyCode;
        }
        if *hyper_code == 0 {
            keyboardType = DEC_PC;
            skMap = &mut coldmapDECPC as *mut [coldmapentry; 23] as *mut coldmapentry;
            fkMap = &mut fkmapDECPC as *mut [libc::c_short; 46] as *mut libc::c_short;
            *super_code = XKeysymToKeycode(display, 0xff54 as KeySym);
            *hyper_code = XKeysymToKeycode(display, 0xff51 as KeySym);
        } else {
            keyboardType = DEC_LK401;
            skMap = &mut coldmapDECLK as *mut [coldmapentry; 17] as *mut coldmapentry;
            fkMap = &mut fkmapDECLK as *mut [libc::c_short; 46] as *mut libc::c_short;
        }
    }
    if *meta_l_code == 0 && *meta_r_code == 0 && *alt_l_code != 0 {
        *meta_l_code = *alt_l_code;
    }
    *control_r_code = *control_l_code;
    *super_code = XKeysymToKeycode(display, 0xffe4 as KeySym);
}
fn find_modifier(mut modmap: *mut XModifierKeymap, mut code: KeyCode) -> u32 {
    let mut modifier: u32 = 0;
    let mut i: u32 = 0;
    if code == 0 {
        return -(1);
    }
    modifier = 0;
    while modifier < 8 {
        i = 0;
        while i < (*modmap).max_keypermod {
            if *((*modmap).modifiermap).offset((i + modifier * (*modmap).max_keypermod)) == code {
                return modifier;
            }
            i += 1;
        }
        modifier += 1;
    }
    return -(1);
}
fn find_unused_modifier(mut modmapp: *mut *mut XModifierKeymap) -> u32 {
    let mut modifier: u32 = 0;
    let mut i: u32 = 0;
    let mut num_lock_code: KeyCode = 0;
    let mut current_block_2: u64;
    modifier = 0;
    while modifier < 8 {
        i = 0;
        loop {
            if !(i < (**modmapp).max_keypermod) {
                current_block_2 = 7815301370352969686;
                break;
            }
            if *((**modmapp).modifiermap).offset((i + modifier * (**modmapp).max_keypermod)) != 0 {
                current_block_2 = 16559507199688588974;
                break;
            }
            i += 1;
        }
        match current_block_2 {
            16559507199688588974 => {
                modifier += 1;
            }
            _ => return modifier,
        }
    }
    if removeNumLockModifier != 0 {
        num_lock_code = XKeysymToKeycode(display, 0xff7f as KeySym);
        modifier = 0;
        while modifier < 8 {
            i = 0;
            while i < (**modmapp).max_keypermod {
                if *((**modmapp).modifiermap).offset((i + modifier * (**modmapp).max_keypermod))
                    == num_lock_code
                {
                    *modmapp = XDeleteModifiermapEntry(*modmapp, num_lock_code, modifier);
                    return modifier;
                }
                i += 1;
            }
            modifier += 1;
        }
    }
    return -(1);
}
fn do_modifier(
    mut modmapp: *mut *mut XModifierKeymap,
    mut changedp: *mut u32,
    mut code1: KeyCode,
    mut code2: KeyCode,
    mut code3: KeyCode,
) -> u32 {
    let mut mod_0: u32 = -(1);
    mod_0 = find_modifier(*modmapp, code1);
    if mod_0 == -(1) {
        mod_0 = find_modifier(*modmapp, code2);
    }
    if mod_0 == -(1) {
        mod_0 = find_modifier(*modmapp, code3);
    }
    if mod_0 != -(1) {
        return (1) << mod_0;
    }
    if code1 == 0 && code2 == 0 && code3 == 0 {
        return 0;
    }
    mod_0 = find_unused_modifier(modmapp);
    if mod_0 == -(1) {
        return 0;
    }
    if code1 != 0 {
        *modmapp = XInsertModifiermapEntry(*modmapp, code1, mod_0);
        *changedp = 1;
    }
    if code2 != 0 {
        *modmapp = XInsertModifiermapEntry(*modmapp, code2, mod_0);
        *changedp = 1;
    }
    if code3 != 0 {
        *modmapp = XInsertModifiermapEntry(*modmapp, code3, mod_0);
        *changedp = 1;
    }
    return (1) << mod_0;
}
fn mask_to_modifier(mut mask: u32) -> u32 {
    let mut i: u32 = -(1);
    while mask != 0 {
        i += 1;
        mask >>= 1;
    }
    return i;
}
fn setup_modifier_mapping() {
    let mut modmap: *mut XModifierKeymap = 0 as *mut XModifierKeymap;
    let mut control_l_code: KeyCode = 0;
    let mut control_r_code: KeyCode = 0;
    let mut meta_l_code: KeyCode = 0;
    let mut meta_r_code: KeyCode = 0;
    let mut alt_l_code: KeyCode = 0;
    let mut super_code: KeyCode = 0;
    let mut hyper_code: KeyCode = 0;
    let mut changed: u32 = 0;
    get_keyboard_modifier_codes(
        &mut control_l_code,
        &mut control_r_code,
        &mut meta_l_code,
        &mut meta_r_code,
        &mut alt_l_code,
        &mut super_code,
        &mut hyper_code,
    );
    XGrabServer(display);
    modmap = XGetModifierMapping(display);
    do_modifier(
        &mut modmap,
        &mut changed,
        control_l_code,
        control_r_code,
        0 as KeyCode,
    );
    meta_mask = do_modifier(
        &mut modmap,
        &mut changed,
        meta_l_code,
        meta_r_code,
        0 as KeyCode,
    );
    if meta_mask == 0 {
        vwarn(
            b"Cold Load\0" as &str,
            b"Unable to allocate a modifier for the Meta key.\0" as &str,
        );
    }
    super_mask = do_modifier(
        &mut modmap,
        &mut changed,
        super_code,
        0 as KeyCode,
        0 as KeyCode,
    );
    if super_mask == 0 {
        vwarn(
            b"Cold Load\0" as &str,
            b"Unable to allocate a modifier for the Super key.\0" as &str,
        );
    }
    hyper_mask = do_modifier(
        &mut modmap,
        &mut changed,
        hyper_code,
        0 as KeyCode,
        0 as KeyCode,
    );
    if hyper_mask == 0 {
        vwarn(
            b"Cold Load\0" as &str,
            b"Unable to allocate a modifier for the Hyper key.\0" as &str,
        );
    } else if hyper_mask == super_mask {
        modmap = XDeleteModifiermapEntry(modmap, hyper_code, mask_to_modifier(super_mask));
        hyper_mask = do_modifier(
            &mut modmap,
            &mut changed,
            super_code,
            0 as KeyCode,
            0 as KeyCode,
        );
        if hyper_mask == 0 {
            vwarn(
                b"Cold Load\0" as &str,
                b"Unable to allocate a modifier for the Hyper key.\0" as &str,
            );
        } else {
            modmap = XDeleteModifiermapEntry(modmap, super_code, mask_to_modifier(hyper_mask));
        }
        changed = 1;
    }
    if changed != 0 {
        XSetModifierMapping(display, modmap);
    }
    XUngrabServer(display);
    XFreeModifiermap(modmap);
}
fn ColdLoadOutput(mut ignored: *mut libc::c_void) {
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0; 4],
    };
    let mut __cancel_routine: Option<fn(*mut libc::c_void) -> ()> =
        ::std::mem::transmute::<Option<fn(*mut u64) -> u32>, pthread_cleanuproutine_t>(Some(
            pthread_mutex_unlock as fn(*mut u64) -> u32,
        ));
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock as *mut u64;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support XLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    if (*cold_channel).fd > 0 {
        manage_cold_load_output();
        update_cold_load_blinkers();
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
            b"Unable to unlock the Life Support XLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
}
fn ColdLoadInput(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut config: *mut VLMConfig = argument as *mut VLMConfig;
    let mut xpoll: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0; 4],
    };
    let mut __cancel_routine: Option<fn(*mut libc::c_void) -> ()> =
        ::std::mem::transmute::<Option<fn(u64) -> u32>, pthread_cleanuproutine_t>(Some(
            pthread_detach as fn(u64) -> u32,
        ));
    let mut __cancel_arg: *mut libc::c_void = self_0;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support signal lock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
            b"Unable to unlock the Life Support signal lock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    if -(1) == (*cold_channel).fd {
        let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
            __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            }; 1],
            __pad: [0; 4],
        };
        let mut __cancel_routine_0: Option<fn(*mut libc::c_void) -> ()> =
            ::std::mem::transmute::<Option<fn(*mut u64) -> u32>, pthread_cleanuproutine_t>(Some(
                pthread_mutex_unlock as fn(*mut u64) -> u32,
            ));
        let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock as *mut u64;
        let mut __not_first_call_0: u32 = __sigsetjmp(
            (__cancel_buf_0.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
            0,
        );
        if __not_first_call_0 != 0 {
            __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
            __pthread_unwind_next(&mut __cancel_buf_0);
        }
        __pthread_register_cancel(&mut __cancel_buf_0);
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
            vpunt(
                b"Unable to lock the Life Support XLock in thread %lx\0" as &str,
                pthread_self(),
            );
        }
        (*cold_channel).fd = open_cold_load_display(&mut (*config).coldLoadXParams, false);
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
            vpunt(
                b"Unable to unlock the Life Support XLock in thread %lx\0" as &str,
                pthread_self(),
            );
        }
        __pthread_unregister_cancel(&mut __cancel_buf_0);
        setup_x_io_error_handler();
    }
    loop {
        u64estcancel();
        xpoll.fd = (*cold_channel).fd;
        xpoll.events = 0x1 as libc::c_short;
        poll(&mut xpoll, 1 as nfds_t, 1000);
        if xpoll.revents != 0 {
            let mut __cancel_buf_1: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
                __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                    __cancel_jmp_buf: [0; 8],
                    __mask_was_saved: 0,
                }; 1],
                __pad: [0; 4],
            };
            let mut __cancel_routine_1: Option<fn(*mut libc::c_void) -> ()> =
                ::std::mem::transmute::<Option<fn(*mut u64) -> u32>, pthread_cleanuproutine_t>(
                    Some(pthread_mutex_unlock as fn(*mut u64) -> u32),
                );
            let mut __cancel_arg_1: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock as *mut u64;
            let mut __not_first_call_1: u32 = __sigsetjmp(
                (__cancel_buf_1.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
                0,
            );
            if __not_first_call_1 != 0 {
                __cancel_routine_1.expect("non-null function pointer")(__cancel_arg_1);
                __pthread_unwind_next(&mut __cancel_buf_1);
            }
            __pthread_register_cancel(&mut __cancel_buf_1);
            if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
                vpunt(
                    b"Unable to lock the Life Support XLock in thread %lx\0" as &str,
                    pthread_self(),
                );
            }
            (*cold_channel).fd = manage_x_input(&mut (*config).coldLoadXParams);
            if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
                vpunt(
                    b"Unable to unlock the Life Support XLock in thread %lx\0" as &str,
                    pthread_self(),
                );
            }
            __pthread_unregister_cancel(&mut __cancel_buf_1);
        }
    }
}
static mut ColdLoadWindowName: &str = 0 as &str;
static mut ColdLoadIconName: &str = 0 as &str;
static mut DebuggerWindowName: &str = 0 as &str;
static mut DebuggerIconName: &str = 0 as &str;
static mut lastGuestStatus: GuestStatus = NonexistentGuestStatus;
fn concatenate_string(mut string1: &str, mut string2: &str) -> &str {
    let mut total_size: u32 = (strlen(string1))
        .wrapping_add(strlen(string2))
        .wrapping_add(1);
    let mut new_string: &str = malloc(total_size) as &str;
    if new_string.is_null() {
        vpunt(b"No room for concatenated string.\0" as &str);
    }
    strcpy(new_string, string1);
    return strcat(new_string, string2);
}
fn SetupColdLoadNameStrings(mut config: *mut VLMConfig) {
    let mut interface: *mut NetworkInterface = 0 as *mut NetworkInterface;
    let mut theHost: *mut hostent = 0 as *mut hostent;
    let mut theAddress: in_addr = in_addr { s_addr: 0 };
    let mut longHostName: &str = "";
    let mut shortHostName: &str = "";
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut pp: &str = "";
    let mut aName: &str = "";
    interface = &mut *((*config).interfaces).as_mut_ptr().offset(0) as *mut NetworkInterface;
    while (*interface).present == 0 {
        interface = interface.offset(1);
    }
    match (*interface).myProtocol {
        2048 => {
            theAddress.s_addr = htonl((*interface).myAddress.s_addr);
            theHost = gethostbyaddr(
                &mut theAddress.s_addr as *mut in_addr_t as &str,
                ::std::mem::size_of::<in_addr>() as __socklen_t,
                2,
            );
            if theHost.is_null() {
                sprintf(buffer.as_mut_ptr(), b"INTERNET|%s\0", inet_ntoa(theAddress));
                shortHostName = strdup(buffer.as_mut_ptr());
                longHostName = shortHostName;
            } else {
                longHostName = strdup((*theHost).h_name);
                pp = strchr(longHostName, '.' as i32);
                if !pp.is_null() {
                    *pp = 0;
                }
                shortHostName = longHostName;
                while !(*(*theHost).h_aliases).is_null() {
                    aName = strdup(*(*theHost).h_aliases);
                    pp = strchr(aName, '.' as i32);
                    if !pp.is_null() {
                        *pp = 0;
                    }
                    if strlen(aName) < strlen(shortHostName) {
                        shortHostName = aName;
                    }
                    let ref mut fresh3 = (*theHost).h_aliases;
                    *fresh3 = (*fresh3).offset(1);
                }
            }
        }
        2052 => {
            sprintf(
                buffer.as_mut_ptr(),
                b"CHAOS|%o\0",
                htonl((*interface).myAddress.s_addr),
            );
            shortHostName = strdup(buffer.as_mut_ptr());
            longHostName = shortHostName;
        }
        _ => {
            shortHostName = b"\0" as &str;
            longHostName = shortHostName;
        }
    }
    ColdLoadIconName = concatenate_string(shortHostName, b" Cold Load\0" as &str);
    ColdLoadWindowName = concatenate_string(longHostName, b" Cold Load Stream\0" as &str);
    DebuggerWindowName = concatenate_string(longHostName, b" VLM Debugger\0" as &str);
    DebuggerIconName = concatenate_string(shortHostName, b" Debugger\0" as &str);
}
fn SetColdLoadNames() {
    if !display.is_null() && window != 0 {
        if RunningGuestStatus == (*EmbCommAreaPtr).guestStatus {
            XStoreName(display, window, ColdLoadWindowName);
            XSetIconName(display, window, ColdLoadIconName);
        } else {
            XStoreName(display, window, DebuggerWindowName);
            XSetIconName(display, window, DebuggerIconName);
        }
    }
}
fn close_display() {
    if !display.is_null() {
        if !originalModmap.is_null() {
            XSetModifierMapping(display, originalModmap);
            XFreeModifiermap(originalModmap);
            originalModmap = 0 as *mut XModifierKeymap;
        }
        XCloseDisplay(display);
        display = 0 as *mut Display;
    }
}

pub fn UpdateColdLoadNames() {
    if (*EmbCommAreaPtr).guestStatus != lastGuestStatus {
        let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
            __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            }; 1],
            __pad: [0; 4],
        };
        let mut __cancel_routine: Option<fn(*mut libc::c_void) -> ()> =
            ::std::mem::transmute::<Option<fn(*mut u64) -> u32>, pthread_cleanuproutine_t>(Some(
                pthread_mutex_unlock as fn(*mut u64) -> u32,
            ));
        let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock as *mut u64;
        let mut __not_first_call: u32 = __sigsetjmp(
            (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
            0,
        );
        if __not_first_call != 0 {
            __cancel_routine.expect("non-null function pointer")(__cancel_arg);
            __pthread_unwind_next(&mut __cancel_buf);
        }
        __pthread_register_cancel(&mut __cancel_buf);
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
            vpunt(
                b"Unable to lock the Life Support XLock in thread %lx\0" as &str,
                pthread_self(),
            );
        }
        SetColdLoadNames();
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
            vpunt(
                b"Unable to unlock the Life Support XLock in thread %lx\0" as &str,
                pthread_self(),
            );
        }
        __pthread_unregister_cancel(&mut __cancel_buf);
        lastGuestStatus = (*EmbCommAreaPtr).guestStatus as GuestStatus;
    }
}

pub fn InitializeColdLoadChannel(mut config: *mut VLMConfig) {
    let mut cp: EmbPtr = EmbCommAreaAlloc(::std::mem::size_of::<EmbColdLoadChannel>());
    let mut p: *mut EmbColdLoadChannel = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(cp)
        as *mut EmbWord as PtrV as *mut EmbColdLoadChannel;
    (*p).type_0 = EmbColdLoadChannelType;
    (*p).unit = 0;
    (*p).next = (*EmbCommAreaPtr).channel_table;
    (*EmbCommAreaPtr).channel_table = cp;
    (*EmbCommAreaPtr).cold_load_channel = cp;
    cold_channel = p;
    (*p).keyboard_input_queue = CreateQueue(100, ::std::mem::size_of::<EmbPtr>());
    keyboard_queue = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*p).keyboard_input_queue)
        as *mut EmbWord as PtrV as *mut EmbQueue;
    (*p).display_output_queue = CreateQueue(50, ::std::mem::size_of::<EmbPtr>());
    display_queue = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*p).display_output_queue)
        as *mut EmbWord as PtrV as *mut EmbQueue;
    (*display_queue).signal = InstallSignalHandler(
        ::std::mem::transmute::<Option<fn(*mut libc::c_void) -> ()>, ProcPtrV>(Some(
            ColdLoadOutput as fn(*mut libc::c_void) -> (),
        )),
        0,
        false,
    );
    (*p).progress_note.string_total_size = 256;
    (*p).progress_note.string_length = 0;
    SetupColdLoadNameStrings(config);
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0; 4],
    };
    let mut __cancel_routine: Option<fn(*mut libc::c_void) -> ()> =
        ::std::mem::transmute::<Option<fn(*mut u64) -> u32>, pthread_cleanuproutine_t>(Some(
            pthread_mutex_unlock as fn(*mut u64) -> u32,
        ));
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock as *mut u64;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support XLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    (*p).fd = open_cold_load_display(&mut (*config).coldLoadXParams, true);
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
        vpunt(
            b"Unable to unlock the Life Support XLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    if -(1) == (*p).fd {
        verror(b"Cold Load\0" as &str);
        vwarn(
            b"Cold Load\0" as &str,
            b"Will wait for X server but cold load may not function properly.\0" as &str,
        );
    } else {
        setup_x_io_error_handler();
    }
    if pthread_create(
        &mut (*p).coldLoadInput,
        &mut (*EmbCommAreaPtr).inputThreadAttrs,
        ::std::mem::transmute::<Option<fn(pthread_addr_t) -> ()>, pthread_startroutine_t>(Some(
            ColdLoadInput as fn(pthread_addr_t) -> (),
        )),
        config as pthread_addr_t,
    ) != 0
    {
        vpunt(b"Unable to create the cold load window's input thread\0" as &str);
    }
    (*p).coldLoadInputSetup = true;
}

pub fn ResetColdLoadChannel(mut channel: *mut EmbChannel) {
    let mut coldLoadChannel: *mut EmbColdLoadChannel = channel as *mut EmbColdLoadChannel;
    ResetIncomingQueue(
        &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*coldLoadChannel).display_output_queue)
            as *mut EmbWord as PtrV as *mut EmbQueue,
    );
    ResetOutgoingQueue(
        &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*coldLoadChannel).keyboard_input_queue)
            as *mut EmbWord as PtrV as *mut EmbQueue,
    );
    (*coldLoadChannel).progress_note.string_length = 0;
    (*coldLoadChannel).is_selected = false;
    (*coldLoadChannel).command_history_top = 0;
    (*coldLoadChannel).command_history_wrapped = 0;
}

pub fn TerminateColdLoadChannel() {
    let mut exit_value: *mut libc::c_void = 0;
    stop_cold_x();
    if !cold_channel.is_null() && (*cold_channel).coldLoadInputSetup != 0 {
        pthread_cancel((*cold_channel).coldLoadInput);
        pthread_join((*cold_channel).coldLoadInput, &mut exit_value);
        (*cold_channel).coldLoadInputSetup = false;
    }
}
