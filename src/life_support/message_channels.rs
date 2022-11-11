pub fn InitializeMessageChannels(mut config: *mut VLMConfig) {
    let mut cp: EmbPtr = EmbCommAreaAlloc(::std::mem::size_of::<EmbMessageChannel>());
    let mut p: *mut EmbMessageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(cp)
        as *mut EmbWord as PtrV as *mut EmbMessageChannel;
    let mut cc: *mut EmbCommandChannel = 0 as *mut EmbCommandChannel;
    (*p).type_0 = EmbMessageChannelType;
    (*p).unit = 0;
    (*EmbCommAreaPtr).command_channel = cp;
    (*p).next = (*EmbCommAreaPtr).channel_table;
    (*EmbCommAreaPtr).channel_table = cp;
    (*p).subtype = EmbMessageChannelCommandSubtype;
    (*p).hostToGuestQueue = -(1);
    (*p).hostToGuestSupplyQueue = -(1);
    cc = malloc(::std::mem::size_of::<EmbCommandChannel>()) as *mut EmbCommandChannel;
    if cc.is_null() {
        vpunt(b"Couldn't allocate master command message channel\0" as &str);
    }
    (*p).subtypeData0 = (cc as u64 >> 32 & 0xffffffff) as UEmbWord;
    (*p).subtypeData1 = (cc as u64 & 0xffffffff) as UEmbWord;
    let ref mut fresh0 = (*cc).header.nextActiveChannel;
    *fresh0 = 0 as *mut EmbMessageChannel;
    let ref mut fresh1 = (*cc).header.commArea;
    *fresh1 = EmbCommAreaPtr;
    let ref mut fresh2 = (*cc).header.messageChannel;
    *fresh2 = p;
    (*p).guestToHostQueue = CreateQueue(5, ::std::mem::size_of::<EmbPtr>());
    let ref mut fresh3 = (*cc).guestToHostQueue;
    *fresh3 = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*p).guestToHostQueue) as *mut EmbWord
        as PtrV as *mut EmbQueue;
    (*(*cc).guestToHostQueue).signal = InstallSignalHandler(
        ::std::mem::transmute::<Option<fn(*mut EmbCommandChannel) -> ()>, ProcPtrV>(Some(
            ExecuteGuestCommands as fn(*mut EmbCommandChannel) -> (),
        )),
        cc as PtrV,
        false,
    );
    (*p).guestToHostReturnQueue = CreateQueue(5, ::std::mem::size_of::<EmbPtr>());
    let ref mut fresh4 = (*cc).guestToHostReturnQueue;
    *fresh4 = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*p).guestToHostReturnQueue)
        as *mut EmbWord as PtrV as *mut EmbQueue;
}

pub fn PollMessageChannels() {
    let mut messageChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let mut guestToHostQueue: *mut EmbQueue = 0 as *mut EmbQueue;
    if -(1) == (*EmbCommAreaPtr).command_channel {
        return;
    }
    messageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*EmbCommAreaPtr).command_channel) as *mut EmbWord as PtrV
        as *mut EmbMessageChannel;
    while !messageChannel.is_null() {
        subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
            | (*messageChannel).subtypeData1) as PtrV
            as *mut EmbMessageSubtypeData;
        guestToHostQueue = &mut *(EmbCommAreaPtr as *mut EmbWord)
            .offset((*messageChannel).guestToHostQueue) as *mut EmbWord
            as PtrV as *mut EmbQueue;
        if (*messageChannel).guestToHostImpulse != 0 && (*guestToHostQueue).signal != -(1) {
            let ref mut fresh5 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh5 |= ((1) << (*guestToHostQueue).signal);
        }
        messageChannel = (*subtypeData).header.nextActiveChannel;
    }
}
fn ThreadActiveMessageChannel(mut theChannel: *mut EmbMessageChannel) {
    let mut messageChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let ref mut fresh6 =
        (*((((*theChannel).subtypeData0 as u64) << 32 | (*theChannel).subtypeData1) as PtrV
            as *mut EmbMessageSubtypeData))
            .header
            .nextActiveChannel;
    *fresh6 = 0 as *mut EmbMessageChannel;
    messageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*EmbCommAreaPtr).command_channel) as *mut EmbWord as PtrV
        as *mut EmbMessageChannel;
    while !messageChannel.is_null() {
        subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
            | (*messageChannel).subtypeData1) as PtrV
            as *mut EmbMessageSubtypeData;
        if ((*subtypeData).header.nextActiveChannel).is_null() {
            let ref mut fresh7 = (*subtypeData).header.nextActiveChannel;
            *fresh7 = theChannel;
            return;
        }
        messageChannel = (*subtypeData).header.nextActiveChannel;
    }
}

pub fn UnthreadMessageChannel(mut theChannel: *mut EmbMessageChannel) {
    let mut messageChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let mut theSubtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    messageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*EmbCommAreaPtr).command_channel) as *mut EmbWord as PtrV
        as *mut EmbMessageChannel;
    while !messageChannel.is_null() {
        subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
            | (*messageChannel).subtypeData1) as PtrV
            as *mut EmbMessageSubtypeData;
        if theChannel == (*subtypeData).header.nextActiveChannel {
            theSubtypeData = (((*theChannel).subtypeData0 as u64) << 32
                | (*theChannel).subtypeData1) as PtrV
                as *mut EmbMessageSubtypeData;
            let ref mut fresh8 = (*subtypeData).header.nextActiveChannel;
            *fresh8 = (*theSubtypeData).header.nextActiveChannel;
            let ref mut fresh9 = (*theSubtypeData).header.nextActiveChannel;
            *fresh9 = 0 as *mut EmbMessageChannel;
            return;
        }
        messageChannel = (*subtypeData).header.nextActiveChannel;
    }
}
fn ExecuteGuestCommands(mut commandChannel: *mut EmbCommandChannel) {
    let mut commandQueue: *mut EmbQueue = (*commandChannel).guestToHostQueue;
    let mut resultsQueue: *mut EmbQueue = (*commandChannel).guestToHostReturnQueue;
    let mut commandPtr: EmbPtr = 0;
    let mut command: *mut EmbCommandBuffer = 0 as *mut EmbCommandBuffer;
    let mut startMBINCommand: *mut EmbCommandStartMBINBuffer = 0 as *mut EmbCommandStartMBINBuffer;
    let mut mbinChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut mbinSubChannel: *mut EmbMBINChannel = 0 as *mut EmbMBINChannel;
    while EmbQueueFilled(commandQueue) != 0 {
        if 0 == EmbQueueSpace(resultsQueue) {
            SignalLater((*commandQueue).signal);
            return;
        }
        commandPtr = EmbQueueTakeWord(commandQueue);
        if commandPtr != 0 {
            command = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(commandPtr) as *mut EmbWord
                as PtrV as *mut EmbCommandBuffer;
            match ((*command).header).opcode() {
                1 => {
                    startMBINCommand = command as *mut EmbCommandStartMBINBuffer;
                    mbinChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
                        .offset((*startMBINCommand).mbinChannel)
                        as *mut EmbWord as PtrV
                        as *mut EmbMessageChannel;
                    mbinSubChannel =
                        malloc(::std::mem::size_of::<EmbMBINChannel>()) as *mut EmbMBINChannel;
                    if mbinSubChannel.is_null() {
                        (*command).resultCode = 12;
                    } else {
                        let ref mut fresh10 = (*mbinSubChannel).header.commArea;
                        *fresh10 = EmbCommAreaPtr;
                        let ref mut fresh11 = (*mbinSubChannel).header.messageChannel;
                        *fresh11 = mbinChannel;
                        let ref mut fresh12 = (*mbinSubChannel).guestToHostQueue;
                        *fresh12 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).guestToHostQueue)
                            as *mut EmbWord as PtrV
                            as *mut EmbQueue;
                        let ref mut fresh13 = (*mbinSubChannel).guestToHostReturnQueue;
                        *fresh13 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).guestToHostReturnQueue)
                            as *mut EmbWord as PtrV
                            as *mut EmbQueue;
                        let ref mut fresh14 = (*mbinSubChannel).hostToGuestQueue;
                        *fresh14 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).hostToGuestQueue)
                            as *mut EmbWord as PtrV
                            as *mut EmbQueue;
                        let ref mut fresh15 = (*mbinSubChannel).hostToGuestSupplyQueue;
                        *fresh15 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).hostToGuestSupplyQueue)
                            as *mut EmbWord as PtrV
                            as *mut EmbQueue;
                        (*mbinChannel).subtypeData0 =
                            (mbinSubChannel as u64 >> 32 & 0xffffffff) as UEmbWord;
                        (*mbinChannel).subtypeData1 =
                            (mbinSubChannel as u64 & 0xffffffff) as UEmbWord;
                        ThreadActiveMessageChannel(mbinChannel);
                        activeMBINChannel = mbinSubChannel;
                        (*(*mbinSubChannel).guestToHostQueue).signal = InstallSignalHandler(
                            ::std::mem::transmute::<Option<fn(*mut EmbMBINChannel) -> ()>, ProcPtrV>(
                                Some(SendMBINBuffers as fn(*mut EmbMBINChannel) -> ()),
                            ),
                            mbinSubChannel as PtrV,
                            false,
                        );
                        (*command).resultCode = 0;
                    }
                }
                _ => {
                    (*command).resultCode = 22;
                }
            }
            EmbQueuePutWord(resultsQueue, commandPtr);
        }
    }
}

pub fn ResetMessageChannel(mut channel: *mut EmbChannel) {
    let mut messageChannel: *mut EmbMessageChannel = channel as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let mut allocatedByVLM: bool = 0;
    allocatedByVLM = ((channel as *mut EmbWord).offset_from(EmbCommAreaPtr as *mut EmbWord)
        as EmbPtr
        > (*EmbCommAreaPtr).host_buffer_start + (*EmbCommAreaPtr).host_buffer_size)
        as bool;
    (*messageChannel).guestToHostImpulse = EmbMessageImpulseNone;
    (*messageChannel).hostToGuestImpulse = EmbMessageImpulseNone;
    subtypeData = (((*messageChannel).subtypeData0 as u64) << 32 | (*messageChannel).subtypeData1)
        as PtrV as *mut EmbMessageSubtypeData;
    let ref mut fresh16 = (*subtypeData).header.nextActiveChannel;
    *fresh16 = 0 as *mut EmbMessageChannel;
    match (*messageChannel).subtype {
        2 => {
            ResetIncomingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*messageChannel).guestToHostQueue)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            ResetOutgoingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).guestToHostReturnQueue)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
        }
        3 => {
            ResetIncomingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*messageChannel).guestToHostQueue)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            ResetOutgoingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).guestToHostReturnQueue)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            ResetIncomingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).hostToGuestSupplyQueue)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            ResetOutgoingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*messageChannel).hostToGuestQueue)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            if allocatedByVLM != 0 && activeMBINChannel == subtypeData as *mut EmbMBINChannel {
                activeMBINChannel = 0 as *mut EmbMBINChannel;
            }
        }
        _ => {}
    }
    if allocatedByVLM != 0 && !subtypeData.is_null() {
        free(subtypeData);
        (*messageChannel).subtypeData0 = (0 as u64 >> 32 & 0xffffffff) as UEmbWord;
        (*messageChannel).subtypeData1 = (0 as u64 & 0xffffffff) as UEmbWord;
    }
}

pub fn TerminateMessageChannels() {}
