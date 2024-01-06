pub unsafe extern "C" fn InitializeSignalHandlers() {
    let mut i: u32 = 0;
    (*EmbCommAreaPtr).guest_to_host_signals = 0 as SignalMask;
    (*EmbCommAreaPtr).live_guest_to_host_signals = 0 as SignalMask;
    (*EmbCommAreaPtr).host_to_guest_signals = 0 as SignalMask;
    (*EmbCommAreaPtr).live_host_to_guest_signals = 0 as SignalMask;
    (*EmbCommAreaPtr).reawaken = 0 as SignalMask;
    (*EmbCommAreaPtr).useSignalLocks = false;
    i = 0;
    while i < 32 {
        (*EmbCommAreaPtr).signalHandler[i].handlerThreadSetup = false;
        (*EmbCommAreaPtr).signalHandler[i].signal = 0 as SignalMask;
        let ref mut fresh0 = (*EmbCommAreaPtr).signalHandler[i].handlerFunction;
        *fresh0 = None;
        let ref mut fresh1 = (*EmbCommAreaPtr).signalHandler[i].handlerArgument;
        *fresh1 = 0;
        i += 1;
    }
}

pub unsafe extern "C" fn InstallSignalHandler(
    mut signalHandler: ProcPtrV,
    mut signalArgument: PtrV,
    mut inputP: bool
) -> SignalNumber {
    let mut policy: u32 = 0;
    let mut priority: u32 = 0;
    let mut signal: SignalMask = 0;
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(
                b"Unable to lock the Life Support signal lock in thread %lx\0" as &str,
                pthread_self()
            );
        }
    }

    let mut i: u32 = 0;
    while i < 32 {
        signal = (1 << i) as SignalMask;
        if ((*EmbCommAreaPtr).live_guest_to_host_signals & signal) == 0 {
            let ref mut fresh2 = (*EmbCommAreaPtr).live_guest_to_host_signals;
            *fresh2 |= signal;
            (*EmbCommAreaPtr).signalHandler[i].signal = signal;
            let ref mut fresh3 = (*EmbCommAreaPtr).signalHandler[i].handlerFunction;
            *fresh3 = signalHandler;
            let ref mut fresh4 = (*EmbCommAreaPtr).signalHandler[i].handlerArgument;
            *fresh4 = signalArgument;
            if (*EmbCommAreaPtr).signalHandler[i].handlerThreadSetup == 0 {
                if
                    pthread_create(
                        &mut (*(*EmbCommAreaPtr).signalHandler
                            .as_mut_ptr()
                            .offset(i)).handlerThread,
                        if inputP != 0 {
                            &mut (*EmbCommAreaPtr).inputThreadAttrs
                        } else {
                            &mut (*EmbCommAreaPtr).outputThreadAttrs
                        },
                        ::std::mem::transmute::<
                            Option<unsafe extern "C" fn(pthread_addr_t) -> ()>,
                            pthread_startroutine_t
                        >(
                            Some(
                                SignalHandlerTopLevel as unsafe extern "C" fn(pthread_addr_t) -> ()
                            )
                        ),
                        &mut *(*EmbCommAreaPtr).signalHandler
                            .as_mut_ptr()
                            .offset(i) as *mut SignalHandler
                    ) != 0
                {
                    vpunt(
                        b"Unable to create thread to handle signal %d for %lx (%lx)\0" as &str,
                        i,
                        signalHandler,
                        signalArgument
                    );
                }
                (*EmbCommAreaPtr).signalHandler[i].handlerThreadSetup = true;
            }
            break;
        } else {
            i += 1;
        }
    }
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(
                b"Unable to unlock the Life Support signal lock in thread %lx\0" as &str,
                pthread_self()
            );
        }
    }
    if i < 32 {
        return i;
    } else {
        vpunt(b"Signal table overflow\0" as &str);
    }
    panic!("Reached end of non-void function without returning");
}

pub unsafe extern "C" fn SendInterruptToLifeSupport() {
    if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).signalSignal) != 0 {
        vpunt(b"Unable to send Life Support an interrupt from the VLM\0" as &str);
    }
}

pub unsafe extern "C" fn WaitForLifeSupport() {
    let mut delta: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut abstime: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut result: u32 = 0;
    delta.tv_sec = 5 as __time_t;
    delta.tv_nsec = 0 as __syscall_slong_t;
    if (*EmbCommAreaPtr).host_to_guest_signals != 0 && (((*processor).control >> 30) & 3) != 3 {
        SendInterruptToEmulator();
    } else {
        let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
            __cancel_jmp_buf: [
                __cancel_jmp_buf_tag {
                    __cancel_jmp_buf: [0; 8],
                    __mask_was_saved: 0,
                };
                1
            ],
            __pad: [0; 4],
        };
        let mut __cancel_routine: Option<
            unsafe extern "C" fn(*mut libc::c_void) -> ()
        > = ::std::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u64) -> u32>,
            pthread_cleanuproutine_t
        >(Some(pthread_mutex_unlock as unsafe extern "C" fn(*mut u64) -> u32));
        let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).wakeupLock as *mut u64;
        let mut __not_first_call: u32 = __sigsetjmp(
            __cancel_buf.__cancel_jmp_buf.as_mut_ptr() as *mut __jmp_buf_tag,
            0
        );
        if __not_first_call != 0 {
            __cancel_routine.expect("non-null function pointer")(__cancel_arg);
            __pthread_unwind_next(&mut __cancel_buf);
        }
        __pthread_register_cancel(&mut __cancel_buf);
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
            vpunt(b"Unable to lock the VLM wakeup lock in thread %lx\0" as &str, pthread_self());
        }
        if pthread_get_expiration_np(&mut delta, &mut abstime) != 0 {
            vpunt(b"Unable to get absolute time\0" as &str);
        }
        result = u64imedwait(
            &mut (*EmbCommAreaPtr).wakeupSignal,
            &mut (*EmbCommAreaPtr).wakeupLock,
            &mut abstime
        );
        if result != 0 {
            if !(result == 110 || result == 4) {
                vpunt(
                    b"Unable to wait for a VLM wakeup signal in thread %lx\0" as &str,
                    pthread_self()
                );
            }
        }
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
            vpunt(b"Unable to unlock the VLM wakeup lock in thread %lx\0" as &str, pthread_self());
        }
        __pthread_unregister_cancel(&mut __cancel_buf);
    }
}

pub unsafe extern "C" fn EmbSendSignal(mut signal: SignalNumber) {
    signal == 0;
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
        vpunt(b"Unable to lock the VLM wakeup lock in thread %lx\0" as &str, pthread_self());
    }
    if signal > -1 && signal < 32 {
        let ref mut fresh5 = (*EmbCommAreaPtr).host_to_guest_signals;
        *fresh5 |= 1 << signal;
        SendInterruptToEmulator();
    }
    if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).wakeupSignal) != 0 {
        vpunt(b"Unable to wakeup the VLM from Life Support\0" as &str);
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
        vpunt(b"Unable to unlock the VLM wakeup lock in thread %lx\0" as &str, pthread_self());
    }
}

pub unsafe extern "C" fn SignalLater(mut signal: SignalNumber) {
    let mut self_0: u64 = pthread_self();
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(b"Unable to lock the Life Support signal lock in thread %lx\0" as &str, self_0);
    }
    let ref mut fresh6 = (*EmbCommAreaPtr).reawaken;
    *fresh6 |= (1 << signal) as SignalMask;
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(b"Unable to unlock the Life Support signal lock in thread %lx\0" as &str, self_0);
    }
}
unsafe extern "C" fn NullSignalHandler(mut ignore: PtrV) {}

pub unsafe extern "C" fn RemoveSignalHandler(mut signal: SignalNumber) {
    let mut mask: SignalMask = (1 << signal) as SignalMask;
    if signal < 0 || signal >= 32 {
        return;
    }
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(
                b"Unable to lock the Life Support signal lock in thread %lx\0" as &str,
                pthread_self()
            );
        }
    }
    let ref mut fresh7 = (*EmbCommAreaPtr).live_guest_to_host_signals;
    *fresh7 &= !mask;
    let ref mut fresh8 = (*EmbCommAreaPtr).reawaken;
    *fresh8 &= !mask;
    let ref mut fresh9 = (*EmbCommAreaPtr).guest_to_host_signals;
    *fresh9 |= mask;
    if (*EmbCommAreaPtr).signalHandler[signal].handlerThreadSetup != 0 {
        let ref mut fresh10 = (*EmbCommAreaPtr).signalHandler[signal].handlerFunction;
        *fresh10 = Some(NullSignalHandler as unsafe extern "C" fn(PtrV) -> ());
        let ref mut fresh11 = (*EmbCommAreaPtr).signalHandler[signal].handlerArgument;
        *fresh11 = 0;
    }
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(
                b"Unable to unlock the Life Support signal lock in thread %lx\0" as &str,
                pthread_self()
            );
        }
    }
}

pub unsafe extern "C" fn TerminateSignalHandlers() {
    let mut exit_value: *mut libc::c_void = 0;

    let mut i: u32 = 0;
    while i < 32 {
        if (*EmbCommAreaPtr).signalHandler[i].handlerThreadSetup != 0 {
            pthread_cancel((*EmbCommAreaPtr).signalHandler[i].handlerThread);
            pthread_join((*EmbCommAreaPtr).signalHandler[i].handlerThread, &mut exit_value);
            (*EmbCommAreaPtr).signalHandler[i].handlerThreadSetup = false;
        }
        i += 1;
    }
}

unsafe extern "C" fn SignalHandlerTopLevel(mut argument: pthread_addr_t) {
    let mut signalHandler: *mut SignalHandler = argument as *mut SignalHandler;
    let mut self_0: u64 = (*signalHandler).handlerThread;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [
            __cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            };
            1
        ],
        __pad: [0; 4],
    };
    let mut __cancel_routine: Option<
        unsafe extern "C" fn(*mut libc::c_void) -> ()
    > = ::std::mem::transmute::<Option<unsafe extern "C" fn(u64) -> u32>, pthread_cleanuproutine_t>(
        Some(pthread_detach as unsafe extern "C" fn(u64) -> u32)
    );
    let mut __cancel_arg: *mut libc::c_void = self_0;
    let mut __not_first_call: u32 = __sigsetjmp(
        __cancel_buf.__cancel_jmp_buf.as_mut_ptr() as *mut __jmp_buf_tag,
        0
    );
    if __not_first_call != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [
            __cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            };
            1
        ],
        __pad: [0; 4],
    };
    let mut __cancel_routine_0: Option<
        unsafe extern "C" fn(*mut libc::c_void) -> ()
    > = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(*mut u64) -> u32>,
        pthread_cleanuproutine_t
    >(Some(pthread_mutex_unlock as unsafe extern "C" fn(*mut u64) -> u32));
    let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).signalLock as *mut u64;
    let mut __not_first_call_0: u32 = __sigsetjmp(
        __cancel_buf_0.__cancel_jmp_buf.as_mut_ptr() as *mut __jmp_buf_tag,
        0
    );
    if __not_first_call_0 != 0 {
        __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
        __pthread_unwind_next(&mut __cancel_buf_0);
    }
    __pthread_register_cancel(&mut __cancel_buf_0);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support signalLock in thread %lx\0" as &str,
            pthread_self()
        );
    }
    loop {
        if ((*EmbCommAreaPtr).guest_to_host_signals & (*signalHandler).signal) != 0 {
            let ref mut fresh12 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh12 &= !(*signalHandler).signal;
            u64estcancel();
            if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
                vpunt(
                    b"Unable to unlock the Life Support signal lock in thread %lx\0" as &str,
                    self_0
                );
            }
            Some((*signalHandler).handlerFunction.expect("non-null function pointer")).expect(
                "non-null function pointer"
            )((*signalHandler).handlerArgument);
            if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
                vpunt(
                    b"Unable to lock the Life Support signal lock in thread %lx\0" as &str,
                    self_0
                );
            }
        } else if
            pthread_cond_wait(
                &mut (*EmbCommAreaPtr).signalSignal,
                &mut (*EmbCommAreaPtr).signalLock
            ) != 0
        {
            vpunt(
                b"Unable to wait for the Life Support signal signal in thread %lx\0" as &str,
                self_0
            );
        }
    }
}
