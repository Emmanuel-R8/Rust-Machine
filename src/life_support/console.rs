// pub  fn InitializeConsoleChannel(mut config: *mut VLMConfig) {
//     let mut cp: EmbPtr = EmbCommAreaAlloc(
//         ::std::mem::size_of::<EmbConsoleChannel>(),
//     );
//     let mut p: *mut EmbConsoleChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
//         .offset(cp ) as *mut EmbWord as PtrV as *mut EmbConsoleChannel;
//     (*p).type_0 = EmbConsoleChannelType;
//     (*p).unit = 0;
//     (*p).next = (*EmbCommAreaPtr).channel_table;
//     (*EmbCommAreaPtr).channel_table = cp;
//     (*EmbCommAreaPtr).consoleChannel = cp;
//     (*p)
//         .outputRequestQueue = CreateQueue(
//         50,
//         ::std::mem::size_of::<EmbPtr>(),
//     );
//     let ref mut fresh0 = (*p).outputRequestQ;
//     *fresh0 = &mut *(EmbCommAreaPtr as *mut EmbWord)
//         .offset((*p).outputRequestQueue ) as *mut EmbWord as PtrV
//         as *mut EmbQueue;
//     (*(*p).outputRequestQ)
//         .signal = InstallSignalHandler(
//         ::std::mem::transmute::<
//             Option::<fn(*mut EmbConsoleChannel) -> ()>,
//             ProcPtrV,
//         >(Some(ConsoleOutput as fn(*mut EmbConsoleChannel) -> ())),
//         p as PtrV,
//         false,
//     );
//     (*p)
//         .outputReplyQueue = CreateQueue(
//         50,
//         ::std::mem::size_of::<EmbPtr>(),
//     );
//     let ref mut fresh1 = (*p).outputReplyQ;
//     *fresh1 = &mut *(EmbCommAreaPtr as *mut EmbWord)
//         .offset((*p).outputReplyQueue ) as *mut EmbWord as PtrV as *mut EmbQueue;
//     (*p)
//         .inputRequestQueue = CreateQueue(
//         50,
//         ::std::mem::size_of::<EmbPtr>(),
//     );
//     let ref mut fresh2 = (*p).inputRequestQ;
//     *fresh2 = &mut *(EmbCommAreaPtr as *mut EmbWord)
//         .offset((*p).inputRequestQueue ) as *mut EmbWord as PtrV
//         as *mut EmbQueue;
//     (*(*p).inputRequestQ)
//         .signal = InstallSignalHandler(
//         ::std::mem::transmute::<
//             Option::<fn(*mut EmbConsoleChannel) -> ()>,
//             ProcPtrV,
//         >(Some(ConsoleInput as fn(*mut EmbConsoleChannel) -> ())),
//         p as PtrV,
//         true,
//     );
//     (*p)
//         .inputReplyQueue = CreateQueue(
//         50,
//         ::std::mem::size_of::<EmbPtr>(),
//     );
//     let ref mut fresh3 = (*p).inputReplyQ;
//     *fresh3 = &mut *(EmbCommAreaPtr as *mut EmbWord)
//         .offset((*p).inputReplyQueue ) as *mut EmbWord as PtrV as *mut EmbQueue;
//     let ref mut fresh4 = (*p).hostName;
//     *fresh4 = (*config).generaXParams.xpHostName;
//     (*p)
//         .hostAddress = htonl((*config).generaXParams.xpHostAddress )
//         as EmbWord;
//     (*p).displayNumber = (*config).generaXParams.xpDisplay;
//     (*p).screenNumber = (*config).generaXParams.xpScreen;
//     (*p).initialState = (*config).generaXParams.xpInitialState;
//     (*p).geometry = MakeEmbString((*config).generaXParams.xpGeometry);
//     (*p).foregroundColor = MakeEmbString((*config).generaXParams.xpForegroundColor);
//     (*p).backgroundColor = MakeEmbString((*config).generaXParams.xpBackgroundColor);
//     (*p).borderColor = MakeEmbString((*config).generaXParams.xpBorderColor);
//     (*p).borderWidth = (*config).generaXParams.xpBorderWidth;
//     let ref mut fresh5 = (*p).display;
//     *fresh5 = 0 ;
//     (*p).openingState = OpeningStateNone;
//     let ref mut fresh6 = (*p).rlDisplay;
//     *fresh6 = 0 ;
//     if pthread_create(
//         &mut (*p).drawRunLights,
//         &mut (*EmbCommAreaPtr).pollThreadAttrs,
//         ::std::mem::transmute::<
//             Option::<fn(pthread_addr_t) -> ()>,
//             pthread_startroutine_t,
//         >(Some(DrawRunLights as fn(pthread_addr_t) -> ())),
//         p ,
//     ) != 0
//     {
//         vpunt(

//             b"Unable to create the console channel polling thread\0"
//                  as&str,
//         );
//     }
//     (*p).drawRunLightsSetup = true;
// }

// pub  fn DoConsoleIO(
//     mut consoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) {
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     match (*command).opcode {
//         1 => {
//             (*command).result = OpenDisplay(consoleChannel, command);
//         }
//         2 => {
//             CloseDisplay(consoleChannel);
//             (*command).result = 0;
//         }
//         3 => {
//             (*command).result = 0;
//         }
//         4 => {
//             if OpeningStatePrefix  == (*consoleChannel).openingState {
//                 (*command).result = 0;
//             } else {
//                 (*command).result = ConsoleWrite(consoleChannel, command);
//             }
//         }
//         5 => {
//             if (*consoleChannel).openingState != OpeningStateNone  {
//                 (*command).result = ProcessConnectionRequest(consoleChannel, command);
//             } else {
//                 (*command).result = ConsoleRead(consoleChannel, command);
//             }
//         }
//         6 => {
//             if (*consoleChannel).openingState != OpeningStateNone  {
//                 (*(&mut *((*command).data).as_mut_ptr().offset(0 )
//                     as *mut EmbWord as *mut EmbConsoleInputWait))
//                     .availableP = 1;
//                 (*command).result = 0;
//             } else {
//                 (*command).result = ConsoleInputWait(consoleChannel, command);
//             }
//         }
//         7 => {
//             EnableRunLights(consoleChannel, command);
//             (*command).result = 0;
//         }
//         8 => {
//             DisableRunLights(consoleChannel);
//             (*command).result = 0;
//         }
//         _ => {}
//     };
// }
//  fn ConsoleDriver(
//     mut consoleChannel: *mut EmbConsoleChannel,
//     mut pRequestQueue: *mut EmbQueue,
//     mut pReplyQueue: *mut EmbQueue,
// ) {
//     let mut requestQueue: *mut EmbQueue = pRequestQueue;
//     let mut replyQueue: *mut EmbQueue = pReplyQueue;
//     let mut command: *mut EmbConsoleBuffer = 0 as *mut EmbConsoleBuffer;
//     let mut commandPtr: EmbPtr = 0;
//     while EmbQueueFilled(requestQueue) != 0 {
//         if 0  == EmbQueueSpace(replyQueue) {
//             SignalLater((*requestQueue).signal);
//             return;
//         }
//         commandPtr = EmbQueueTakeWord(requestQueue);
//         if commandPtr != 0 {
//             command = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(commandPtr )
//                 as *mut EmbWord as PtrV as *mut EmbConsoleBuffer;
//             DoConsoleIO(consoleChannel, command);
//             EmbQueuePutWord(replyQueue, commandPtr);
//         }
//     }
// }
//  fn ConsoleOutput(mut consoleChannel: *mut EmbConsoleChannel) {
//     ConsoleDriver(
//         consoleChannel,
//         (*consoleChannel).outputRequestQ,
//         (*consoleChannel).outputReplyQ,
//     );
// }
//  fn ConsoleInput(mut consoleChannel: *mut EmbConsoleChannel) {
//     ConsoleDriver(
//         consoleChannel,
//         (*consoleChannel).inputRequestQ,
//         (*consoleChannel).inputReplyQ,
//     );
// }
//  fn OpenDisplay(
//     mut pConsoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) -> u32 {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     let mut openDisplay: *mut EmbConsoleOpenDisplay = &mut *((*command).data)
//         .as_mut_ptr()
//         .offset(0 ) as *mut EmbWord as *mut EmbConsoleOpenDisplay;
//     let mut displayName: [libc::c_char; 8192] = [0; 8192];
//     let mut result: u32 = 0;
//     if !((*consoleChannel).display).is_null() {
//         return 16;
//     }
//     BuildXDisplayName(
//         displayName.as_mut_ptr(),
//         (*consoleChannel).hostName,
//         (*consoleChannel).displayNumber,
//         (*consoleChannel).screenNumber,
//     );
//     let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
//         __cancel_jmp_buf: [__cancel_jmp_buf_tag {
//             __cancel_jmp_buf: [0; 8],
//             __mask_was_saved: 0,
//         }; 1],
//         __pad: [0 ; 4],
//     };
//     let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
//         Option::<fn(*mut u64) -> u32>,
//         pthread_cleanuproutine_t,
//     >(
//         Some(
//             pthread_mutex_unlock
//                 as fn(*mut u64) -> u32,
//         ),
//     );
//     let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
//         as *mut u64 ;
//     let mut __not_first_call: u32 = __sigsetjmp(
//         (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
//             as *mut __jmp_buf_tag,
//         0,
//     );
//     if __not_first_call  != 0 {
//         __cancel_routine.expect("non-null function pointer")(__cancel_arg);
//         __pthread_unwind_next(&mut __cancel_buf);
//     }
//     __pthread_register_cancel(&mut __cancel_buf);
//     if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//         vpunt(

//             b"Unable to lock the Life Support XLock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     let ref mut fresh7 = (*consoleChannel).display;
//     *fresh7 = XOpenDisplay(displayName.as_mut_ptr()) ;
//     if !((*consoleChannel).display).is_null() {
//         (*consoleChannel)
//             .fd = XConnectionNumber((*consoleChannel).display as *mut Display);
//         (*consoleChannel).openingState = OpeningStatePrefix;
//         (*openDisplay)
//             .lastRequestNumber = (*((*consoleChannel).display as *mut _XDisplay)).request
//             as EmbWord;
//         result = 0;
//     } else {
//         result = *__errno_location();
//         match result {
//             0 => {
//                 result = 111;
//             }
//             11 => {
//                 result = 6;
//             }
//             _ => {}
//         }
//     }
//     if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//         vpunt(

//             b"Unable to unlock the Life Support XLock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     __pthread_unregister_cancel(&mut __cancel_buf);
//     return result;
// }
//  fn ProcessConnectionRequest(
//     mut pConsoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) -> u32 {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     let mut dataTransfer: *mut EmbConsoleDataTransfer = &mut *((*command).data)
//         .as_mut_ptr()
//         .offset(0 ) as *mut EmbWord
//         as *mut EmbConsoleDataTransfer;
//     let mut display: *mut _XDisplay = (*consoleChannel).display as *mut _XDisplay;
//     let mut data: &str =  "" ;
//     let mut setupPrefix: xConnSetupPrefix = xConnSetupPrefix {
//         success: 0,
//         lengthReason: 0,
//         majorVersion: 0,
//         minorVersion: 0,
//         length: 0,
//     };
//     let mut setup: xConnSetup = xConnSetup {
//         release: 0,
//         ridBase: 0,
//         ridMask: 0,
//         motionBufferSize: 0,
//         nbytesVendor: 0,
//         maxRequestSize: 0,
//         numRoots: 0,
//         numFormats: 0,
//         imageByteOrder: 0,
//         bitmapBitOrder: 0,
//         bitmapScanlineUnit: 0,
//         bitmapScanlinePad: 0,
//         minKeyCode: 0,
//         maxKeyCode: 0,
//         pad2: 0,
//     };
//     let mut pixmapFormat: xPixmapFormat = xPixmapFormat {
//         depth: 0,
//         bitsPerPixel: 0,
//         scanLinePad: 0,
//         pad1: 0,
//         pad2: 0,
//     };
//     let mut screenFormat: *mut ScreenFormat = 0 as *mut ScreenFormat;
//     let mut windowRoot: xWindowRoot = xWindowRoot {
//         windowId: 0,
//         defaultColormap: 0,
//         whitePixel: 0,
//         blackPixel: 0,
//         currentInputMask: 0,
//         pixWidth: 0,
//         pixHeight: 0,
//         mmWidth: 0,
//         mmHeight: 0,
//         minInstalledMaps: 0,
//         maxInstalledMaps: 0,
//         rootVisualID: 0,
//         backingStore: 0,
//         saveUnders: 0,
//         rootDepth: 0,
//         nDepths: 0,
//     };
//     let mut screen: *mut Screen = 0 as *mut Screen;
//     let mut pDepth: xDepth = xDepth {
//         depth: 0,
//         pad1: 0,
//         nVisuals: 0,
//         pad2: 0,
//     };
//     let mut depth: *mut Depth = 0 as *mut Depth;
//     let mut visualType: xVisualType = xVisualType {
//         visualID: 0,
//         class: 0,
//         bitsPerRGB: 0,
//         colormapEntries: 0,
//         redMask: 0,
//         greenMask: 0,
//         blueMask: 0,
//         pad: 0,
//     };
//     let mut visual: *mut Visual = 0 as *mut Visual;
//     data = MapVirtualAddressData((*dataTransfer).address )
//         as&str;
//     data = data.offset((*dataTransfer).offset );
//     match (*consoleChannel).openingState {
//         1 => {
//             setupPrefix.success = 1  as CARD8;
//             setupPrefix.lengthReason = 0 as u8;
//             setupPrefix.majorVersion = (*display).proto_major_version as CARD16;
//             setupPrefix.minorVersion = (*display).proto_minor_version as CARD16;
//             setupPrefix.length = 0  as CARD16;
//             memcpy(
//                 data ,
//                 &mut setupPrefix as *mut xConnSetupPrefix ,
//                 ::std::mem::size_of::<xConnSetupPrefix>(),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         2 => {
//             setup.release = (*display).release as CARD32;
//             setup.ridBase = (*display).resource_base as CARD32;
//             setup.ridMask = (*display).resource_mask as CARD32;
//             setup.motionBufferSize = (*display).motion_buffer as CARD32;
//             setup.nbytesVendor = strlen((*display).vendor) as CARD16;
//             setup.maxRequestSize = (*display).max_request_size as CARD16;
//             setup.numRoots = (*display).nscreens as CARD8;
//             setup.numFormats = (*display).nformats as CARD8;
//             setup.imageByteOrder = (*display).byte_order as CARD8;
//             setup.bitmapBitOrder = (*display).bitmap_bit_order as CARD8;
//             setup.bitmapScanlineUnit = (*display).bitmap_unit as CARD8;
//             setup.bitmapScanlinePad = (*display).bitmap_pad as CARD8;
//             setup.minKeyCode = (*display).min_keycode as CARD8;
//             setup.maxKeyCode = (*display).max_keycode as CARD8;
//             memcpy(
//                 data ,
//                 &mut setup as *mut xConnSetup ,
//                 ::std::mem::size_of::<xConnSetup>(),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         3 => {
//             memcpy(
//                 data ,
//                 (*display).vendor ,
//                 strlen((*display).vendor),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         4 => {
//             screenFormat = &mut *((*display).pixmap_format)
//                 .offset((*consoleChannel).nextPixmapFormat )
//                 as *mut ScreenFormat;
//             pixmapFormat.depth = (*screenFormat).depth as CARD8;
//             pixmapFormat.bitsPerPixel = (*screenFormat).bits_per_pixel as CARD8;
//             pixmapFormat.scanLinePad = (*screenFormat).scanline_pad as CARD8;
//             memcpy(
//                 data ,
//                 &mut pixmapFormat as *mut xPixmapFormat ,
//                 ::std::mem::size_of::<xPixmapFormat>(),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         5 => {
//             screen = &mut *((*display).screens)
//                 .offset((*consoleChannel).nextRoot ) as *mut Screen;
//             windowRoot.windowId = (*screen).root as CARD32;
//             windowRoot.defaultColormap = (*screen).cmap as CARD32;
//             windowRoot.whitePixel = (*screen).white_pixel as CARD32;
//             windowRoot.blackPixel = (*screen).black_pixel as CARD32;
//             windowRoot.currentInputMask = (*screen).root_input_mask as CARD32;
//             windowRoot.pixWidth = (*screen).width as CARD16;
//             windowRoot.pixHeight = (*screen).height as CARD16;
//             windowRoot.mmWidth = (*screen).mwidth as CARD16;
//             windowRoot.mmHeight = (*screen).mheight as CARD16;
//             windowRoot.minInstalledMaps = (*screen).min_maps as CARD16;
//             windowRoot.maxInstalledMaps = (*screen).max_maps as CARD16;
//             windowRoot.rootVisualID = (*(*screen).root_visual).visualid as CARD32;
//             windowRoot.backingStore = (*screen).backing_store as CARD8;
//             windowRoot.saveUnders = (*screen).save_unders as BOOL;
//             windowRoot.rootDepth = (*screen).root_depth as CARD8;
//             windowRoot.nDepths = (*screen).ndepths as CARD8;
//             memcpy(
//                 data ,
//                 &mut windowRoot as *mut xWindowRoot ,
//                 ::std::mem::size_of::<xWindowRoot>(),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         6 => {
//             screen = &mut *((*display).screens)
//                 .offset((*consoleChannel).nextRoot ) as *mut Screen;
//             depth = &mut *((*screen).depths)
//                 .offset((*consoleChannel).nextRootDepth ) as *mut Depth;
//             pDepth.depth = (*depth).depth as CARD8;
//             pDepth.nVisuals = (*depth).nvisuals as CARD16;
//             memcpy(
//                 data ,
//                 &mut pDepth as *mut xDepth ,
//                 ::std::mem::size_of::<xDepth>(),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         7 => {
//             screen = &mut *((*display).screens)
//                 .offset((*consoleChannel).nextRoot ) as *mut Screen;
//             depth = &mut *((*screen).depths)
//                 .offset((*consoleChannel).nextRootDepth ) as *mut Depth;
//             visual = &mut *((*depth).visuals)
//                 .offset((*consoleChannel).nextRootDepthVisual ) as *mut Visual;
//             visualType.visualID = (*visual).visualid as CARD32;
//             visualType.class = (*visual).class as CARD8;
//             visualType.bitsPerRGB = (*visual).bits_per_rgb as CARD8;
//             visualType.colormapEntries = (*visual).map_entries as CARD16;
//             visualType.redMask = (*visual).red_mask as CARD32;
//             visualType.greenMask = (*visual).green_mask as CARD32;
//             visualType.blueMask = (*visual).blue_mask as CARD32;
//             memcpy(
//                 data ,
//                 &mut visualType as *mut xVisualType ,
//                 ::std::mem::size_of::<xVisualType>(),
//             );
//             AdvanceOpeningState(consoleChannel);
//         }
//         _ => {}
//     }
//     return 0;
// }
//  fn AdvanceOpeningState(mut pConsoleChannel: *mut EmbConsoleChannel) {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut display: *mut _XDisplay = (*consoleChannel).display as *mut _XDisplay;
//     let mut screen: *mut Screen = 0 as *mut Screen;
//     let mut depth: *mut Depth = 0 as *mut Depth;
//     match (*consoleChannel).openingState {
//         1 => {
//             (*consoleChannel).openingState = OpeningStateHeader;
//         }
//         2 => {
//             (*consoleChannel).openingState = OpeningStateVendor;
//         }
//         3 => {
//             if (*display).nformats > 0  {
//                 (*consoleChannel).openingState = OpeningStatePixmapFormat;
//                 (*consoleChannel).nextPixmapFormat = 0;
//             } else if (*display).nscreens > 0  {
//                 (*consoleChannel).openingState = OpeningStateRoot;
//                 (*consoleChannel).nextRoot = 0;
//             } else {
//                 (*consoleChannel).openingState = OpeningStateNone;
//             }
//         }
//         4 => {
//             let ref mut fresh8 = (*consoleChannel).nextPixmapFormat;
//             *fresh8 += 1;
//             if (*consoleChannel).nextPixmapFormat >= (*display).nformats {
//                 if (*display).nscreens > 0  {
//                     (*consoleChannel).openingState = OpeningStateRoot;
//                     (*consoleChannel).nextRoot = 0;
//                 } else {
//                     (*consoleChannel).openingState = OpeningStateNone;
//                 }
//             }
//         }
//         5 => {
//             screen = &mut *((*display).screens)
//                 .offset((*consoleChannel).nextRoot ) as *mut Screen;
//             if (*screen).ndepths > 0  {
//                 (*consoleChannel).openingState = OpeningStateRootDepth;
//                 (*consoleChannel).nextRootDepth = 0;
//             } else {
//                 let ref mut fresh9 = (*consoleChannel).nextRoot;
//                 *fresh9 += 1;
//                 if (*consoleChannel).nextRoot >= (*display).nscreens {
//                     (*consoleChannel).openingState = OpeningStateNone;
//                 }
//             }
//         }
//         6 => {
//             screen = &mut *((*display).screens)
//                 .offset((*consoleChannel).nextRoot ) as *mut Screen;
//             depth = &mut *((*screen).depths)
//                 .offset((*consoleChannel).nextRootDepth ) as *mut Depth;
//             if (*depth).nvisuals > 0  {
//                 (*consoleChannel)
//                     .openingState = OpeningStateRootDepthVisual;
//                 (*consoleChannel).nextRootDepthVisual = 0;
//             } else {
//                 let ref mut fresh10 = (*consoleChannel).nextRootDepth;
//                 *fresh10 += 1;
//                 if (*consoleChannel).nextRootDepth >= (*screen).ndepths {
//                     let ref mut fresh11 = (*consoleChannel).nextRoot;
//                     *fresh11 += 1;
//                     if (*consoleChannel).nextRoot >= (*display).nscreens {
//                         (*consoleChannel).openingState = OpeningStateNone;
//                     }
//                 }
//             }
//         }
//         7 => {
//             screen = &mut *((*display).screens)
//                 .offset((*consoleChannel).nextRoot ) as *mut Screen;
//             depth = &mut *((*screen).depths)
//                 .offset((*consoleChannel).nextRootDepth ) as *mut Depth;
//             let ref mut fresh12 = (*consoleChannel).nextRootDepthVisual;
//             *fresh12 += 1;
//             if (*consoleChannel).nextRootDepthVisual >= (*depth).nvisuals {
//                 let ref mut fresh13 = (*consoleChannel).nextRootDepth;
//                 *fresh13 += 1;
//                 if (*consoleChannel).nextRootDepth >= (*screen).ndepths {
//                     let ref mut fresh14 = (*consoleChannel).nextRoot;
//                     *fresh14 += 1;
//                     if (*consoleChannel).nextRoot >= (*display).nscreens {
//                         (*consoleChannel).openingState = OpeningStateNone;
//                     } else {
//                         (*consoleChannel).openingState = OpeningStateRoot;
//                     }
//                 } else {
//                     (*consoleChannel)
//                         .openingState = OpeningStateRootDepth;
//                 }
//             }
//         }
//         _ => {}
//     };
// }
//  fn ConsoleWrite(
//     mut pConsoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) -> u32 {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     let mut dataTransfer: *mut EmbConsoleDataTransfer = &mut *((*command).data)
//         .as_mut_ptr()
//         .offset(0 ) as *mut EmbWord
//         as *mut EmbConsoleDataTransfer;
//     let mut pollDisplay: pollfd = pollfd {
//         fd: 0,
//         events: 0,
//         revents: 0,
//     };
//     let mut data: &str =  "" ;
//     let mut nBytes: ssize_t = 0;
//     let mut actualBytes: ssize_t = 0;
//     let mut result: u32 = 0;
//     data = MapVirtualAddressData((*dataTransfer).address )
//         as&str;
//     data = data.offset((*dataTransfer).offset );
//     nBytes = (*dataTransfer).nBytes as ssize_t;
//     result = 11;
//     pollDisplay.fd = (*consoleChannel).fd;
//     pollDisplay.events = 0x4  as libc::c_short;
//     while 11  == result {
//         u64estcancel();
//         pollDisplay.revents = 0  as libc::c_short;
//         poll(&mut pollDisplay, 1  as nfds_t, 1000);
//         if pollDisplay.revents  & 0x4  != 0 {
//             actualBytes = write(
//                 (*consoleChannel).fd,
//                 data ,
//                 nBytes,
//             );
//             if actualBytes == nBytes {
//                 result = 0;
//             } else {
//                 result = if actualBytes < 0   {
//                     *__errno_location()
//                 } else {
//                     11
//                 };
//                 nBytes
//                     -= if actualBytes < 0   {
//                         0
//                     } else {
//                         actualBytes
//                     };
//                 data = data
//                     .offset(
//                         (if actualBytes < 0   {
//                             0
//                         } else {
//                             actualBytes
//                         }) ,
//                     );
//             }
//         } else if pollDisplay.revents  & 0x20  != 0 {
//             result = 9;
//         } else if pollDisplay.revents  & 0x10  != 0 {
//             result = 6;
//         } else if pollDisplay.revents  & 0x8  != 0 {
//             result = 5;
//         }
//     }
//     return result;
// }
//  fn ConsoleRead(
//     mut pConsoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) -> u32 {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     let mut dataTransfer: *mut EmbConsoleDataTransfer = &mut *((*command).data)
//         .as_mut_ptr()
//         .offset(0 ) as *mut EmbWord
//         as *mut EmbConsoleDataTransfer;
//     let mut pollDisplay: pollfd = pollfd {
//         fd: 0,
//         events: 0,
//         revents: 0,
//     };
//     let mut data: &str =  "" ;
//     let mut nBytes: ssize_t = 0;
//     let mut actualBytes: ssize_t = 0;
//     let mut result: u32 = 0;
//     data = MapVirtualAddressData((*dataTransfer).address )
//         as&str;
//     data = data.offset((*dataTransfer).offset );
//     nBytes = (*dataTransfer).nBytes as ssize_t;
//     result = 11;
//     pollDisplay.fd = (*consoleChannel).fd;
//     pollDisplay.events = 0x1  as libc::c_short;
//     while 11  == result {
//         u64estcancel();
//         pollDisplay.revents = 0  as libc::c_short;
//         poll(&mut pollDisplay, 1  as nfds_t, 1000);
//         if pollDisplay.revents  & 0x1  != 0 {
//             actualBytes = read(
//                 (*consoleChannel).fd,
//                 data ,
//                 nBytes,
//             );
//             if actualBytes == nBytes {
//                 result = 0;
//             } else if 0   == actualBytes
//                 && 11  != *__errno_location()
//             {
//                 result = 28;
//             } else {
//                 result = if actualBytes < 0   {
//                     *__errno_location()
//                 } else {
//                     11
//                 };
//                 nBytes
//                     -= if actualBytes < 0   {
//                         0
//                     } else {
//                         actualBytes
//                     };
//                 data = data
//                     .offset(
//                         (if actualBytes < 0   {
//                             0
//                         } else {
//                             actualBytes
//                         }) ,
//                     );
//             }
//         } else if pollDisplay.revents  & 0x20  != 0 {
//             result = 9;
//         } else if pollDisplay.revents  & 0x10  != 0 {
//             result = 6;
//         } else if pollDisplay.revents  & 0x8  != 0 {
//             result = 5;
//         }
//     }
//     return result;
// }

// pub  fn ConsoleInputAvailableP() -> Boole {
//     let mut consoleChannel: *mut EmbConsoleChannel = &mut *(EmbCommAreaPtr
//         as *mut EmbWord)
//         .offset((*EmbCommAreaPtr).consoleChannel ) as *mut EmbWord as PtrV
//         as *mut EmbConsoleChannel;
//     let mut pollDisplay: pollfd = pollfd {
//         fd: 0,
//         events: 0,
//         revents: 0,
//     };
//     if ((*consoleChannel).display).is_null() {
//         return false
//     } else {
//         if (*consoleChannel).openingState != OpeningStateNone  {
//             return true;
//         }
//     }
//     pollDisplay.fd = (*consoleChannel).fd;
//     pollDisplay.events = 0x1  as libc::c_short;
//     pollDisplay.revents = 0  as libc::c_short;
//     poll(&mut pollDisplay, 1  as nfds_t, 0);
//     return (pollDisplay.revents  & 0x1  != 0)
//         as bool;
// }
//  fn ConsoleInputWait(
//     mut pConsoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) -> u32 {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     let mut inputWait: *mut EmbConsoleInputWait = &mut *((*command).data)
//         .as_mut_ptr()
//         .offset(0 ) as *mut EmbWord as *mut EmbConsoleInputWait;
//     let mut pollDisplay: pollfd = pollfd {
//         fd: 0,
//         events: 0,
//         revents: 0,
//     };
//     let mut result: u32 = 0;
//     pollDisplay.fd = (*consoleChannel).fd;
//     pollDisplay.events = 0x1  as libc::c_short;
//     pollDisplay.revents = 0  as libc::c_short;
//     result = poll(&mut pollDisplay, 1  as nfds_t, (*inputWait).timeout);
//     if 0  == result {
//         result = 0;
//         (*inputWait).availableP = 0;
//     } else if pollDisplay.revents  & 0x1  != 0 {
//         result = 0;
//         (*inputWait).availableP = 1;
//     } else if pollDisplay.revents  & 0x20  != 0 {
//         result = 9;
//     } else if pollDisplay.revents  & 0x10  != 0 {
//         result = 6;
//     } else if pollDisplay.revents  & 0x8  != 0 {
//         result = 5;
//     }
//     return result;
// }
//  fn CloseDisplay(mut consoleChannel: *mut EmbConsoleChannel) {
//     DisableRunLights(consoleChannel);
//     if !((*consoleChannel).display).is_null() {
//         let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
//             __cancel_jmp_buf: [__cancel_jmp_buf_tag {
//                 __cancel_jmp_buf: [0; 8],
//                 __mask_was_saved: 0,
//             }; 1],
//             __pad: [0 ; 4],
//         };
//         let mut __cancel_routine: Option::<
//             fn(*mut libc::c_void) -> (),
//         > = ::std::mem::transmute::<
//             Option::<fn(*mut u64) -> u32>,
//             pthread_cleanuproutine_t,
//         >(
//             Some(
//                 pthread_mutex_unlock
//                     as fn(*mut u64) -> u32,
//             ),
//         );
//         let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
//             as *mut u64 ;
//         let mut __not_first_call: u32 = __sigsetjmp(
//             (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
//                 as *mut __jmp_buf_tag,
//             0,
//         );
//         if __not_first_call  != 0 {
//             __cancel_routine.expect("non-null function pointer")(__cancel_arg);
//             __pthread_unwind_next(&mut __cancel_buf);
//         }
//         __pthread_register_cancel(&mut __cancel_buf);
//         if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//             vpunt(

//                 b"Unable to lock the Life Support XLock in thread %lx\0"
//                      as&str,
//                 pthread_self(),
//             );
//         }
//         XCloseDisplay((*consoleChannel).display as *mut Display);
//         if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//             vpunt(

//                 b"Unable to unlock the Life Support XLock in thread %lx\0"
//                      as&str,
//                 pthread_self(),
//             );
//         }
//         __pthread_unregister_cancel(&mut __cancel_buf);
//         let ref mut fresh15 = (*consoleChannel).display;
//         *fresh15 = 0 ;
//     }
// }
//  fn EnableRunLights(
//     mut pConsoleChannel: *mut EmbConsoleChannel,
//     mut pCommand: *mut EmbConsoleBuffer,
// ) {
//     let mut consoleChannel: *mut EmbConsoleChannel = pConsoleChannel;
//     let mut command: *mut EmbConsoleBuffer = pCommand;
//     let mut runLights: *mut EmbConsoleRunLights = &mut *((*command).data)
//         .as_mut_ptr()
//         .offset(0 ) as *mut EmbWord as *mut EmbConsoleRunLights;
//     let mut displayName: [libc::c_char; 8192] = [0; 8192];
//     let mut gcValues: XGCValues = XGCValues {
//         function: 0,
//         plane_mask: 0,
//         foreground: 0,
//         background: 0,
//         line_width: 0,
//         line_style: 0,
//         cap_style: 0,
//         join_style: 0,
//         fill_style: 0,
//         fill_rule: 0,
//         arc_mode: 0,
//         tile: 0,
//         stipple: 0,
//         ts_x_origin: 0,
//         ts_y_origin: 0,
//         font: 0,
//         subwindow_mode: 0,
//         graphics_exposures: 0,
//         clip_x_origin: 0,
//         clip_y_origin: 0,
//         clip_mask: 0,
//         dash_offset: 0,
//         dashes: 0,
//     };
//     BuildXDisplayName(
//         displayName.as_mut_ptr(),
//         (*consoleChannel).hostName,
//         (*consoleChannel).displayNumber,
//         (*consoleChannel).screenNumber,
//     );
//     let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
//         __cancel_jmp_buf: [__cancel_jmp_buf_tag {
//             __cancel_jmp_buf: [0; 8],
//             __mask_was_saved: 0,
//         }; 1],
//         __pad: [0 ; 4],
//     };
//     let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
//         Option::<fn(*mut u64) -> u32>,
//         pthread_cleanuproutine_t,
//     >(
//         Some(
//             pthread_mutex_unlock
//                 as fn(*mut u64) -> u32,
//         ),
//     );
//     let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
//         as *mut u64 ;
//     let mut __not_first_call: u32 = __sigsetjmp(
//         (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
//             as *mut __jmp_buf_tag,
//         0,
//     );
//     if __not_first_call  != 0 {
//         __cancel_routine.expect("non-null function pointer")(__cancel_arg);
//         __pthread_unwind_next(&mut __cancel_buf);
//     }
//     __pthread_register_cancel(&mut __cancel_buf);
//     if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//         vpunt(

//             b"Unable to lock the Life Support XLock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     let ref mut fresh16 = (*consoleChannel).rlDisplay;
//     *fresh16 = XOpenDisplay(displayName.as_mut_ptr()) ;
//     if !((*consoleChannel).rlDisplay).is_null() {
//         let ref mut fresh17 = (*consoleChannel).rlGC;
//         *fresh17 = malloc(::std::mem::size_of::<GC>());
//         if !((*consoleChannel).rlGC).is_null() {
//             memcpy(
//                 &mut (*consoleChannel).runLights as *mut EmbConsoleRunLights
//                     ,
//                 runLights ,
//                 ::std::mem::size_of::<EmbConsoleRunLights>(),
//             );
//             gcValues
//                 .foreground = (*consoleChannel).runLights.lightForeground
//               ;
//             gcValues
//                 .background = (*consoleChannel).runLights.lightBackground
//               ;
//             gcValues
//                 .plane_mask = (*consoleChannel).runLights.lightPlaneMask
//               ;
//             let ref mut fresh18 = *((*consoleChannel).rlGC as *mut GC);
//             *fresh18 = XCreateGC(
//                 (*consoleChannel).rlDisplay as *mut Display,
//                 (*consoleChannel).runLights.windowID as Drawable,
//                 ((1 ) << 2
//                     | (1 ) << 3
//                     | (1 ) << 1),
//                 &mut gcValues,
//             );
//         }
//     }
//     if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//         vpunt(

//             b"Unable to unlock the Life Support XLock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     __pthread_unregister_cancel(&mut __cancel_buf);
//     (*consoleChannel).lastRunLights = 0;
// }
//  fn DrawRunLights(mut argument: pthread_addr_t) {
//     let mut consoleChannel: *mut EmbConsoleChannel = argument as *mut EmbConsoleChannel;
//     let mut self_0: u64 = pthread_self();
//     let mut drlSleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
//     let mut changed: u32 = 0;
//     let mut i: u32 = 0;
//     let mut bit: u32 = 0;
//     let mut x: u32 = 0;
//     let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
//         __cancel_jmp_buf: [__cancel_jmp_buf_tag {
//             __cancel_jmp_buf: [0; 8],
//             __mask_was_saved: 0,
//         }; 1],
//         __pad: [0 ; 4],
//     };
//     let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
//         Option::<fn(u64) -> u32>,
//         pthread_cleanuproutine_t,
//     >(Some(pthread_detach as fn(u64) -> u32));
//     let mut __cancel_arg: *mut libc::c_void = self_0 ;
//     let mut __not_first_call: u32 = __sigsetjmp(
//         (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
//             as *mut __jmp_buf_tag,
//         0,
//     );
//     if __not_first_call  != 0 {
//         __cancel_routine.expect("non-null function pointer")(__cancel_arg);
//         __pthread_unwind_next(&mut __cancel_buf);
//     }
//     __pthread_register_cancel(&mut __cancel_buf);
//     if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
//         vpunt(

//             b"Unable to lock the Life Support signal lock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
//         vpunt(

//             b"Unable to unlock the Life Support signal lock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     drlSleep.tv_sec = 0  as __time_t;
//     drlSleep.tv_nsec = 10   * 10000000 ;
//     loop {
//         if !((*consoleChannel).rlDisplay).is_null() {
//             let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
//                 __cancel_jmp_buf: [__cancel_jmp_buf_tag {
//                     __cancel_jmp_buf: [0; 8],
//                     __mask_was_saved: 0,
//                 }; 1],
//                 __pad: [0 ; 4],
//             };
//             let mut __cancel_routine_0: Option::<
//                 fn(*mut libc::c_void) -> (),
//             > = ::std::mem::transmute::<
//                 Option::<fn(*mut u64) -> u32>,
//                 pthread_cleanuproutine_t,
//             >(
//                 Some(
//                     pthread_mutex_unlock
//                         as fn(*mut u64) -> u32,
//                 ),
//             );
//             let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
//                 as *mut u64 ;
//             let mut __not_first_call_0: u32 = __sigsetjmp(
//                 (__cancel_buf_0.__cancel_jmp_buf).as_mut_ptr()
//                     as *mut __jmp_buf_tag,
//                 0,
//             );
//             if __not_first_call_0  != 0 {
//                 __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
//                 __pthread_unwind_next(&mut __cancel_buf_0);
//             }
//             __pthread_register_cancel(&mut __cancel_buf_0);
//             if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//                 vpunt(

//                     b"Unable to lock the Life Support XLock in thread %lx\0"
//                          as&str,
//                     pthread_self(),
//                 );
//             }
//             changed = (*consoleChannel).lastRunLights ^ (*EmbCommAreaPtr).run_lights;
//             (*consoleChannel).lastRunLights = (*EmbCommAreaPtr).run_lights;
//             x = (*consoleChannel).runLights.firstLightX;
//             i = 0;
//             bit = 1;
//             x = (*consoleChannel).runLights.firstLightX;
//             while i < (*consoleChannel).runLights.nLights {
//                 if changed & bit != 0 {
//                     if (*consoleChannel).lastRunLights & bit != 0 {
//                         XFillRectangle(
//                             (*consoleChannel).rlDisplay as *mut Display,
//                             (*consoleChannel).runLights.windowID as Drawable,
//                             *((*consoleChannel).rlGC as *mut GC),
//                             x,
//                             (*consoleChannel).runLights.firstLightY,
//                             (*consoleChannel).runLights.lightWidth ,
//                             (*consoleChannel).runLights.lightHeight ,
//                         );
//                     } else {
//                         XClearArea(
//                             (*consoleChannel).rlDisplay as *mut Display,
//                             (*consoleChannel).runLights.windowID as Window,
//                             x,
//                             (*consoleChannel).runLights.firstLightY,
//                             (*consoleChannel).runLights.lightWidth ,
//                             (*consoleChannel).runLights.lightHeight ,
//                             0,
//                         );
//                     }
//                 }
//                 i += 1;
//                 bit = bit << 1;
//                 x += (*consoleChannel).runLights.lightXSpacing;
//             }
//             XFlush((*consoleChannel).rlDisplay as *mut Display);
//             if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//                 vpunt(

//                     b"Unable to unlock the Life Support XLock in thread %lx\0"
//                           as&str,
//                     pthread_self(),
//                 );
//             }
//             __pthread_unregister_cancel(&mut __cancel_buf_0);
//         }
//         if pthread_delay_np(&mut drlSleep) != 0 {
//             vpunt(

//                 b"Unable to sleep in thread %lx\0"
//                     as&str,
//                 self_0,
//             );
//         }
//     };
// }
//  fn DisableRunLights(mut consoleChannel: *mut EmbConsoleChannel) {
//     let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
//         __cancel_jmp_buf: [__cancel_jmp_buf_tag {
//             __cancel_jmp_buf: [0; 8],
//             __mask_was_saved: 0,
//         }; 1],
//         __pad: [0 ; 4],
//     };
//     let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
//         Option::<fn(*mut u64) -> u32>,
//         pthread_cleanuproutine_t,
//     >(
//         Some(
//             pthread_mutex_unlock
//                 as fn(*mut u64) -> u32,
//         ),
//     );
//     let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).XLock
//         as *mut u64 ;
//     let mut __not_first_call: u32 = __sigsetjmp(
//         (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
//             as *mut __jmp_buf_tag,
//         0,
//     );
//     if __not_first_call  != 0 {
//         __cancel_routine.expect("non-null function pointer")(__cancel_arg);
//         __pthread_unwind_next(&mut __cancel_buf);
//     }
//     __pthread_register_cancel(&mut __cancel_buf);
//     if pthread_mutex_lock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//         vpunt(

//             b"Unable to lock the Life Support XLock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     if !((*consoleChannel).rlGC).is_null() {
//         free((*consoleChannel).rlGC);
//         let ref mut fresh19 = (*consoleChannel).rlGC;
//         *fresh19 = 0 ;
//     }
//     if !((*consoleChannel).rlDisplay).is_null() {
//         XCloseDisplay((*consoleChannel).rlDisplay as *mut Display);
//         let ref mut fresh20 = (*consoleChannel).rlDisplay;
//         *fresh20 = 0 ;
//     }
//     if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).XLock) != 0 {
//         vpunt(

//             b"Unable to unlock the Life Support XLock in thread %lx\0"
//                  as&str,
//             pthread_self(),
//         );
//     }
//     __pthread_unregister_cancel(&mut __cancel_buf);
// }

// pub  fn ResetConsoleChannel(mut channel: *mut EmbChannel) {
//     let mut consoleChannel: *mut EmbConsoleChannel = channel as *mut EmbConsoleChannel;
//     ResetIncomingQueue((*consoleChannel).outputRequestQ);
//     ResetOutgoingQueue((*consoleChannel).outputReplyQ);
//     ResetIncomingQueue((*consoleChannel).inputRequestQ);
//     ResetOutgoingQueue((*consoleChannel).inputReplyQ);
//     CloseDisplay(consoleChannel);
// }

// pub  fn TerminateConsoleChannel() {
//     let mut exit_value: *mut libc::c_void = 0 ;
//     let mut consoleChannel: *mut EmbConsoleChannel = 0 as *mut EmbConsoleChannel;
//     if -(1) == (*EmbCommAreaPtr).consoleChannel {
//         return
//     } else {
//         consoleChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
//             .offset((*EmbCommAreaPtr).consoleChannel ) as *mut EmbWord as PtrV
//             as *mut EmbConsoleChannel;
//     }
//     if (*consoleChannel).drawRunLightsSetup != 0 {
//         pthread_cancel((*consoleChannel).drawRunLights);
//         pthread_join((*consoleChannel).drawRunLights, &mut exit_value);
//         (*consoleChannel).drawRunLightsSetup = false;
//     }
//     CloseDisplay(consoleChannel);
// }
