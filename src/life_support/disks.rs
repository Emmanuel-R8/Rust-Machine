pub fn AttachDiskChannel(mut pRequest: *mut AttachDiskChannelRequest) {
    let mut request: *mut AttachDiskChannelRequest = pRequest;
    let mut diskChannel: *mut EmbDiskChannel =
        &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*request).diskChannel) as *mut EmbWord
            as PtrV as *mut EmbDiskChannel;
    let mut diskState: *mut DiskChannelState = 0 as *mut DiskChannelState;
    let mut fileStatus: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut filenameHeader: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_0 { u: 0 },
        },
    };
    let mut filenameSize: size_t = 0;
    let mut filename: &str = "";
    let mut openFlags: u32 = 0;
    (*request).result = 0;
    (*request).errorMsg = -(1);
    printf(b"AttachDiskChannel\n\0");
    diskState = malloc(::std::mem::size_of::<DiskChannelState>()) as *mut DiskChannelState;
    if diskState.is_null() {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Couldn't allocate disk channel status structure\0" as &str,
        );
        (*request).result = 12;
        return;
    }
    (*diskChannel).hostState0 = (diskState as u64 >> 32 & 0xffffffff) as EmbWord;
    (*diskChannel).hostState1 = (diskState as u64 & 0xffffffff) as EmbWord;
    (*diskState).fd = -(1);
    let ref mut fresh0 = (*diskState).command_queue_ptr;
    *fresh0 = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*diskChannel).command_queue)
        as *mut EmbWord as PtrV as *mut EmbQueue;
    let ref mut fresh1 = (*diskState).status_queue_ptr;
    *fresh1 = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*diskChannel).status_queue)
        as *mut EmbWord as PtrV as *mut EmbQueue;
    (*diskState).error_pending = false;
    if 23 as *mut Tag
        != (Some(
            (Some(MapVirtualAddressTag as fn(isize) -> *mut Tag))
                .expect("non-null function pointer"),
        ))
        .expect("non-null function pointer")(
            (&mut (*request).filename as *mut EmbWord as *mut isize)
                .offset_from(MapVirtualAddressData(0)),
        )
    {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Disk partition filename is not a simple string\0" as &str,
        );
        (*request).result = 22;
        return;
    }
    VirtualMemoryRead((*request).filename, &mut filenameHeader);
    if 3 != LispObjTag(filenameHeader) & 0x3f {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Disk partition filename is not a simple string\0" as &str,
        );
        (*request).result = 22;
        return;
    }
    if (LispObjData(filenameHeader) & !(32767)) != 0x50000000 {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Disk partition filename is not a simple string\0" as &str,
        );
        (*request).result = 22;
        return;
    }
    filenameSize = (LispObjData(filenameHeader) & 32767);
    filename = malloc(filenameSize.wrapping_add(1)) as &str;
    if filename.is_null() {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Couldn't allocate space for local copy of disk partition filename\0" as &str,
        );
        (*request).result = 12;
        return;
    }
    memcpy(
        filename,
        MapVirtualAddressData(((*request).filename + 1)),
        filenameSize,
    );
    *filename.offset(filenameSize) = 0;
    if ((*diskChannel).flags).read_only() != 0 {
        openFlags = 0;
    } else {
        openFlags = 0o2;
    }
    if CreateIfNotFound == (*request).ifNotFoundAction {
        openFlags |= 0o100;
    }
    printf(b"AttachDiskChannel open '%s'\n\0", filename);
    (*diskState).fd = open(
        filename,
        openFlags,
        0o400 | 0o200 | 0o400 >> 3 | 0o200 >> 3 | 0o400 >> 3 >> 3 | 0o200 >> 3 >> 3,
    );
    if -(1) == (*diskState).fd {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Unable to open disk partition %s\0" as &str,
            filename,
        );
        (*request).result = *__errno_location();
        return;
    }
    if fstat((*diskState).fd, &mut fileStatus) != 0 {
        verror(
            b"AttachDiskChannel\0" as &str,
            b"Unable to determine size of disk partition %s\0" as &str,
            filename,
        );
        (*request).result = *__errno_location();
        close((*diskState).fd);
        return;
    }
    if (*request).minimumLength > 0 {
        if (*request).minimumLength > fileStatus.st_size {
            if ftruncate((*diskState).fd, (*request).minimumLength) != 0 {
                verror(
                    b"AttachDiskChannel\0" as &str,
                    b"Unable to set size of disk partition %s to %d bytes\0" as &str,
                    filename,
                    (*request).minimumLength,
                );
                (*request).result = *__errno_location();
                close((*diskState).fd);
                return;
            }
            fileStatus.st_size = (*request).minimumLength as __off_t;
        }
    }
    (*diskChannel).number_of_pages = (fileStatus.st_size / 8192) as EmbWord;
    (*diskChannel).next = (*EmbCommAreaPtr).channel_table;
    (*EmbCommAreaPtr).channel_table =
        (diskChannel as *mut EmbWord).offset_from(EmbCommAreaPtr as *mut EmbWord) as EmbPtr;
    (*(*diskState).command_queue_ptr).signal = InstallSignalHandler(
        ::std::mem::transmute::<Option<fn(*mut EmbDiskChannel) -> ()>, ProcPtrV>(Some(
            DiskLife as fn(*mut EmbDiskChannel) -> (),
        )),
        diskChannel as PtrV,
        false,
    );
}

pub fn GrowDiskPartition(mut pRequest: *mut GrowDiskPartitionRequest) {
    let mut request: *mut GrowDiskPartitionRequest = pRequest;
    let mut diskChannel: *mut EmbDiskChannel =
        &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*request).diskChannel) as *mut EmbWord
            as PtrV as *mut EmbDiskChannel;
    let mut diskState: *mut DiskChannelState = (((*diskChannel).hostState0 as u64) << 32
        | (*diskChannel).hostState1)
        as *mut DiskChannelState;
    let mut fileStatus: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    (*request).result = 0;
    (*request).errorMsg = -(1);
    if -(1) == (*diskState).fd {
        verror(
            b"GrowDiskPartition\0" as &str,
            b"There is no disk partition attached to channel #%d\0" as &str,
            (*diskChannel).unit,
        );
        (*request).result = 22;
        return;
    }
    if fstat((*diskState).fd, &mut fileStatus) != 0 {
        verror(
            b"GrowDiskPartition\0" as &str,
            b"Unable to determine size of disk partition attached to channel #%d\0" as &str,
            (*diskChannel).unit,
        );
        (*request).result = *__errno_location();
        return;
    }
    if (*request).newLength > fileStatus.st_size {
        if ftruncate((*diskState).fd, (*request).newLength) != 0 {
            verror(
                b"GrowDiskPartition\0" as &str,
                b"Unable to set size of disk partition attached to channel #%d to %d bytes\0"
                    as &str,
                (*diskChannel).unit,
                (*request).newLength,
            );
            (*request).result = *__errno_location();
            return;
        }
        fileStatus.st_size = (*request).newLength as __off_t;
    }
    (*diskChannel).number_of_pages = (fileStatus.st_size / 8192) as EmbWord;
}

pub fn DetachDiskChannel(mut diskChannelPtr: EmbPtr) {
    let mut diskChannel: *mut EmbDiskChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset(diskChannelPtr) as *mut EmbWord
        as PtrV as *mut EmbDiskChannel;
    let mut diskState: *mut DiskChannelState = (((*diskChannel).hostState0 as u64) << 32
        | (*diskChannel).hostState1)
        as *mut DiskChannelState;
    let mut channelPtr: EmbPtr = 0;
    let mut prevChannelPtr: EmbPtr = 0;
    RemoveSignalHandler((*(*diskState).command_queue_ptr).signal);
    (*(*diskState).command_queue_ptr).signal = -(1);
    if (*diskState).fd != -(1) {
        close((*diskState).fd);
        (*diskState).fd = -(1);
    }
    prevChannelPtr = -(1);
    channelPtr = (*EmbCommAreaPtr).channel_table;
    while channelPtr != -(1) {
        if diskChannelPtr == channelPtr {
            if -(1) == prevChannelPtr {
                (*EmbCommAreaPtr).channel_table = (*diskChannel).next;
            } else {
                (*(&mut *(EmbCommAreaPtr as *mut EmbWord).offset(prevChannelPtr) as *mut EmbWord
                    as PtrV as *mut EmbChannel))
                    .next = (*diskChannel).next;
            }
            break;
        } else {
            prevChannelPtr = channelPtr;
            channelPtr = (*(&mut *(EmbCommAreaPtr as *mut EmbWord).offset(channelPtr)
                as *mut EmbWord as PtrV as *mut EmbChannel))
                .next;
        }
    }
}
fn DiskLife(mut diskChannel: *mut EmbDiskChannel) {
    let mut diskState: *mut DiskChannelState = (((*diskChannel).hostState0 as u64) << 32
        | (*diskChannel).hostState1)
        as *mut DiskChannelState;
    let mut commandQueue: *mut EmbQueue = (*diskState).command_queue_ptr;
    let mut statusQueue: *mut EmbQueue = (*diskState).status_queue_ptr;
    let mut command: *mut EmbDiskQueueElement = 0 as *mut EmbDiskQueueElement;
    let mut commandPtr: EmbWord = 0;
    while EmbQueueFilled(commandQueue) != 0 {
        if (*EmbCommAreaPtr).inhibitDisk != 0 || 0 == EmbQueueSpace(statusQueue) {
            SignalLater((*commandQueue).signal);
            return;
        }
        commandPtr = EmbQueueTakeWord(commandQueue);
        if commandPtr != 0 {
            command = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(commandPtr) as *mut EmbWord
                as PtrV as *mut EmbDiskQueueElement;
            let mut current_block_28: u64;
            match ((*command).op).cmd() {
                2 => {
                    if ((*diskChannel).flags).read_only() != 0 {
                        (*command).status = LostStatus;
                        (*command).error_code = 30;
                        current_block_28 = 17184638872671510253;
                    } else {
                        current_block_28 = 12130054763581113524;
                    }
                }
                1 => {
                    current_block_28 = 12130054763581113524;
                }
                3 => {
                    (*diskState).error_pending = false;
                    (*command).status = WonStatus;
                    current_block_28 = 17184638872671510253;
                }
                4 => {
                    (*diskState).error_pending = false;
                    (*command).status = WonStatus;
                    current_block_28 = 17184638872671510253;
                }
                _ => {
                    (*command).status = LostStatus;
                    (*command).error_code = 6;
                    current_block_28 = 17184638872671510253;
                }
            }
            match current_block_28 {
                12130054763581113524 => {
                    if (*diskState).error_pending != 0 {
                        (*command).status = AbortStatus;
                    } else if -(1) == (*diskState).fd {
                        (*command).status = LostStatus;
                        (*command).error_code = 6;
                    } else {
                        (*command).error_code = DoDiskIO(diskChannel, diskState, command);
                        if (*command).error_code != 0 {
                            (*command).status = LostStatus;
                            (*diskState).error_pending = true;
                        } else {
                            (*command).status = WonStatus;
                        }
                    }
                }
                _ => {}
            }
            EmbQueuePutWord(statusQueue, commandPtr);
        }
    }
}
fn DoDiskIO(
    mut diskChannel: *mut EmbDiskChannel,
    mut diskState: *mut DiskChannelState,
    mut command: *mut EmbDiskQueueElement,
) -> u32 {
    let mut addressPair: *mut EmbAddressPair = 0 as *mut EmbAddressPair;
    let mut nBytes: ssize_t = 0;
    let mut actualBytes: ssize_t = 0;
    let mut startingOffset: u32 = 0;
    let mut nAddresses: u32 = 0;
    let mut nVectors: u32 = 0;
    let mut i: u32 = 0;

    if (*command).page < 0 || (*command).page + (*command).count > (*diskChannel).number_of_pages {
        return 22;
    }
    startingOffset = (*command).page * 8192;
    if -(1) == lseek((*diskState).fd, startingOffset, 0) {
        return *__errno_location();
    }
    nAddresses = (*command).n_addresses;
    addressPair = &mut *((*command).addresses).as_mut_ptr().offset(0) as *mut EmbAddressPair;
    while nAddresses > 0 {
        nVectors = if nAddresses > 32 { 32 } else { nAddresses };
        nBytes = 0 as ssize_t;

        i = 0;
        while i < nVectors {
            let ref mut fresh2 = (*diskState).iovs[i].iov_base;
            *fresh2 = &mut *(EmbCommAreaPtr as *mut EmbWord).offset((*addressPair).address)
                as *mut EmbWord as PtrV as u64;
            (*diskState).iovs[i].iov_len =
                ((*addressPair).n_words).wrapping_mul(::std::mem::size_of::<EmbWord>());
            nBytes = (nBytes).wrapping_add((*diskState).iovs[i].iov_len) as ssize_t as ssize_t;
            i += 1;
            addressPair = addressPair.offset(1);
            nAddresses -= 1;
        }
        match ((*command).op).cmd() {
            1 => {
                actualBytes = readv((*diskState).fd, ((*diskState).iovs).as_mut_ptr(), nVectors);
            }
            2 => {
                actualBytes = writev((*diskState).fd, ((*diskState).iovs).as_mut_ptr(), nVectors);
            }
            _ => return 22,
        }
        if -(1) == actualBytes {
            return *__errno_location();
        } else {
            if actualBytes != nBytes {
                return 4;
            }
        }
    }
    return 0;
}

pub fn ResetDiskChannel(mut channel: *mut EmbChannel) {
    let mut diskChannel: *mut EmbDiskChannel = channel as *mut EmbDiskChannel;
    let mut diskState: *mut DiskChannelState = (((*diskChannel).hostState0 as u64) << 32
        | (*diskChannel).hostState1)
        as *mut DiskChannelState;
    ResetIncomingQueue((*diskState).command_queue_ptr);
    ResetOutgoingQueue((*diskState).status_queue_ptr);
    (*diskState).error_pending = false;
    if (diskChannel as *mut EmbWord).offset_from(EmbCommAreaPtr as *mut EmbWord) as EmbPtr
        > (*EmbCommAreaPtr).host_buffer_start + (*EmbCommAreaPtr).host_buffer_size
    {
        if (*diskState).fd != -(1) {
            close((*diskState).fd);
            (*diskState).fd = -(1);
        }
    }
}

fn TerminateDiskChannel(mut diskChannel: *mut EmbDiskChannel) {
    let mut diskState: *mut DiskChannelState = (((*diskChannel).hostState0 as u64) << 32
        | (*diskChannel).hostState1)
        as *mut DiskChannelState;
    if (*diskState).fd != -(1) {
        close((*diskState).fd);
        (*diskState).fd = -(1);
    }
}

pub fn TerminateDiskChannels() {
    let mut diskChannel: *mut EmbDiskChannel = 0 as *mut EmbDiskChannel;
    let mut channel: EmbPtr = 0;
    channel = (*EmbCommAreaPtr).channel_table;
    while channel != -(1) {
        diskChannel = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(channel) as *mut EmbWord as PtrV
            as *mut EmbDiskChannel;
        if EmbDiskChannelType == (*diskChannel).type_0 {
            TerminateDiskChannel(diskChannel);
        }
        channel = (*diskChannel).next;
    }
}
