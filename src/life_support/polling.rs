fn VLMIsRunning(mut ep: *mut EmbCommArea) -> Boole {
    return ((*ep).spy_status == 0 && ((*ep).fep).status() != HaltedFEPStatus) as bool;
}
fn VLMIsRunningLisp(mut ep: *mut EmbCommArea) -> Boole {
    return (VLMIsRunning(ep) != 0 && ((*ep).fep).status() == IdleFEPStatus) as bool;
}
fn UpdateVLMStatus() {
    let mut ep: *mut EmbCommArea = EmbCommAreaPtr;
    match (*ep).guestStatus {
        0 | 1 => {
            (*ep).guestStatus = if VLMIsRunningLisp(ep) != 0 {
                RunningGuestStatus
            } else if VLMIsRunning(ep) != 0 {
                InitializedGuestStatus
            } else {
                (*ep).guestStatus
            };
        }
        2 | 3 => {
            (*ep).guestStatus = if VLMIsRunningLisp(ep) != 0 {
                RunningGuestStatus
            } else if VLMIsRunning(ep) != 0 {
                (*ep).guestStatus
            } else {
                InitializingGuestStatus
            };
        }
        4 | 5 => {
            (*ep).guestStatus = if VLMIsRunningLisp(ep) != 0 {
                RunningGuestStatus
            } else if VLMIsRunning(ep) != 0 {
                CrashedGuestStatus
            } else {
                InitializingGuestStatus
            };
        }
        -2 | -1 | _ => {}
    }
    UpdateColdLoadNames();
}
fn ResetCommArea(mut fullReset: bool) {
    let mut channel: *mut EmbChannel = 0 as *mut EmbChannel;
    let mut channelP: EmbPtr = 0;
    channelP = (*EmbCommAreaPtr).channel_table;
    while channelP != -(1) {
        channel = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(channelP) as *mut EmbWord as PtrV
            as *mut EmbChannel;
        match (*channel).type_0 {
            1 => {
                if fullReset != 0 {
                    ResetDiskChannel(channel);
                }
            }
            2 => {
                if fullReset != 0 {
                    ResetConsoleChannel(channel);
                }
            }
            3 => {
                if fullReset != 0 {
                    ResetNetworkChannel(channel);
                }
            }
            6 => {
                ResetColdLoadChannel(channel);
            }
            8 => {
                if fullReset != 0 {
                    ResetMessageChannel(channel);
                }
            }
            _ => {}
        }
        channelP = (*channel).next;
    }
    fullReset != 0;
}
fn ProcessResetRequest() {
    match (*EmbCommAreaPtr).reset_request {
        -6 => {}
        -5 => {}
        -4 => {}
        -1 => {
            ResetCommArea(false);
        }
        1 => {
            ResetCommArea(true);
            let ref mut fresh0 = (*EmbCommAreaPtr).resetRequestCount;
            *fresh0 += 1;
            (*EmbCommAreaPtr).restart_applications = 1;
        }
        _ => {}
    }
    (*EmbCommAreaPtr).reset_request = NoResetRequest;
}

pub fn IvoryLifePolling(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut pollingSleep: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    pollingSleep.tv_sec = 0 as __time_t;
    pollingSleep.tv_nsec = 0 as __syscall_slong_t;
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
    loop {
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
        let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).signalLock as *mut u64;
        let mut __not_first_call_0: u32 = __sigsetjmp(
            (__cancel_buf_0.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
            0,
        );
        if __not_first_call_0 != 0 {
            __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
            __pthread_unwind_next(&mut __cancel_buf_0);
        }
        __pthread_register_cancel(&mut __cancel_buf_0);
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(
                b"Unable to lock the Life Support signalLock in thread %lx\0" as &str,
                pthread_self(),
            );
        }
        (*EmbCommAreaPtr).pollTime += pollingSleep.tv_nsec;
        PollMessageChannels();
        if (*EmbCommAreaPtr).reset_request != NoResetRequest {
            ProcessResetRequest();
        } else if (*EmbCommAreaPtr).pollTime > 250000000 {
            (*EmbCommAreaPtr).pollTime = 0;
            let ref mut fresh1 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh1 |= (*EmbCommAreaPtr).live_guest_to_host_signals;
            if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).signalSignal) != 0 {
                vpunt(
                    b"Unable to send Life Support signal signal in thread %lx\0" as &str,
                    self_0,
                );
            }
        } else if (*EmbCommAreaPtr).reawaken != 0 {
            let ref mut fresh2 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh2 |= (*EmbCommAreaPtr).reawaken;
            (*EmbCommAreaPtr).reawaken = 0 as SignalMask;
            if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).signalSignal) != 0 {
                vpunt(
                    b"Unable to send Life Support signal signal in thread %lx\0" as &str,
                    self_0,
                );
            }
        }
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(
                b"Unable to unlock the Life Support signalLock in thread %lx\0" as &str,
                pthread_self(),
            );
        }
        __pthread_unregister_cancel(&mut __cancel_buf_0);
        if (*EmbCommAreaPtr).clock_interval > 0 {
            (*EmbCommAreaPtr).pollClockTime -= pollingSleep.tv_nsec;
            if (*EmbCommAreaPtr).pollClockTime <= 0 {
                EmbSendSignal((*EmbCommAreaPtr).clock_signal);
                (*EmbCommAreaPtr).pollClockTime = (1000 * (*EmbCommAreaPtr).clock_interval);
            }
            if (*EmbCommAreaPtr).pollClockTime > 250000000 {
                pollingSleep.tv_nsec = 250000000;
            } else {
                pollingSleep.tv_nsec = (*EmbCommAreaPtr).pollClockTime;
            }
        } else {
            pollingSleep.tv_nsec = 7500000;
        }
        UpdateVLMStatus();
        pollingSleep.tv_sec = 1 as __time_t;
        pollingSleep.tv_nsec = 0 as __syscall_slong_t;
        if pthread_delay_np(&mut pollingSleep) != 0 {
            vpunt(b"Unable to sleep in thread %lx\0" as &str, self_0);
        }
    }
}

pub fn IntervalTimerDriver(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut expirationTime: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut expirationInterval: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut result: u32 = 0;
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
    (*EmbCommAreaPtr).clockTime = -(1);
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
    let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).clockLock as *mut u64;
    let mut __not_first_call_0: u32 = __sigsetjmp(
        (__cancel_buf_0.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call_0 != 0 {
        __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
        __pthread_unwind_next(&mut __cancel_buf_0);
    }
    __pthread_register_cancel(&mut __cancel_buf_0);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).clockLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support clockLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    loop {
        if (*EmbCommAreaPtr).clockTime >= 0 {
            expirationInterval.tv_sec = 0 as __time_t;
            expirationInterval.tv_nsec = 1000 * (*EmbCommAreaPtr).clockTime;
            while expirationInterval.tv_nsec >= 1000000000 {
                expirationInterval.tv_sec += 1;
                expirationInterval.tv_nsec -= 1000000000;
            }
            if pthread_get_expiration_np(&mut expirationInterval, &mut expirationTime) < 0 {
                vpunt(b"Unable to compute interval timer expiration time\0" as &str);
            }
            result = u64imedwait(
                &mut (*EmbCommAreaPtr).clockSignal,
                &mut (*EmbCommAreaPtr).clockLock,
                &mut expirationTime,
            );
        } else {
            result = pthread_cond_wait(
                &mut (*EmbCommAreaPtr).clockSignal,
                &mut (*EmbCommAreaPtr).clockLock,
            );
        }
        if result == 110 {
            EmbSendSignal((*EmbCommAreaPtr).clock_signal);
            (*EmbCommAreaPtr).clockTime = -(1);
        }
    }
}

pub fn SetIntervalTimer(mut relativeTimeout: isize) {
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
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).clockLock as *mut u64;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).clockLock) != 0 {
        vpunt(
            b"Unable to lock the Life Support clockLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    (*EmbCommAreaPtr).clockTime = relativeTimeout;
    if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).clockSignal) < 0 {
        vpunt(
            b"Unable to send Life Support clock signal in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).clockLock) != 0 {
        vpunt(
            b"Unable to unlock the Life Support clockLock in thread %lx\0" as &str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    sched_yield();
}
