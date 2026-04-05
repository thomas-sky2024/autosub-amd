-------------------------------------
Translated Report (Full Report Below)
-------------------------------------
Process:             auto-sub [56186]
Path:                /Users/USER/Documents/*/auto-sub
Identifier:          auto-sub
Version:             0.1.0 (0.1.0)
Code Type:           ARM-64 (Native)
Role:                Foreground
Parent Process:      node [55947]
Coalition:           com.google.antigravity [559]
Responsible Process: Electron [49035]
User ID:             501

Date/Time:           2026-04-06 00:54:14.3590 +0700
Launch Time:         2026-04-06 00:45:32.4023 +0700
Hardware Model:      MacBookPro18,2
OS Version:          macOS 26.4 (25E246)
Release Type:        User

Crash Reporter Key:  E601F452-0C7D-49A2-E3D9-1DAD517AD943
Incident Identifier: D661A3A9-CE10-427E-9BDD-81A887BE6897

Time Awake Since Boot: 60000 seconds

System Integrity Protection: enabled

Triggered by Thread: 0  main, Dispatch Queue: com.apple.main-thread

Exception Type:    EXC_CRASH (SIGABRT)
Exception Codes:   0x0000000000000000, 0x0000000000000000

Termination Reason:  Namespace SIGNAL, Code 6, Abort trap: 6
Terminating Process: auto-sub [56186]


Application Specific Information:
abort() called


Thread 0 Crashed:: main Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib        	       0x1903e45e8 __pthread_kill + 8
1   libsystem_pthread.dylib       	       0x19041f8d8 pthread_kill + 296
2   libsystem_c.dylib             	       0x190326790 abort + 148
3   auto-sub                      	       0x103052bbc ggml_abort + 160
4   auto-sub                      	       0x1030d0d68 ggml_metal_rsets_free + 152
5   auto-sub                      	       0x1030d1664 ggml_metal_device_free + 24
6   auto-sub                      	       0x1030d2890 std::__1::unique_ptr<ggml_metal_device, ggml_metal_device_deleter>::~unique_ptr[abi:ne200100]() + 32
7   libsystem_c.dylib             	       0x1902d5e48 __cxa_finalize_ranges + 416
8   libsystem_c.dylib             	       0x1902d5c48 exit + 44
9   auto-sub                      	       0x1044878bc std::sys::pal::unix::os::exit::ha5ed4bc9d93d94d8 + 12
10  auto-sub                      	       0x10448e1c4 std::process::exit::h0dabe4f5f6de9a0c + 28
11  auto-sub                      	       0x102cc1be4 tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run::h76fd10526c123129 + 100
12  auto-sub                      	       0x102d29be4 tao::event_loop::EventLoop$LT$T$GT$::run::h1b842902c9f6e61d + 88
13  auto-sub                      	       0x102945f64 _$LT$tauri_runtime_wry..Wry$LT$T$GT$$u20$as$u20$tauri_runtime..Runtime$LT$T$GT$$GT$::run::h1f667a226663975d + 172
14  auto-sub                      	       0x102d0c324 tauri::app::App$LT$R$GT$::run::h1cbca2608fa4a004 + 640
15  auto-sub                      	       0x102d0c9a4 tauri::app::Builder$LT$R$GT$::run::he84bd9244481185c + 284
16  auto-sub                      	       0x1028cf174 auto_sub_lib::run::hf1313e0555180035 + 776
17  auto-sub                      	       0x10283d3c8 auto_sub::main::h797125012a65b44c + 12 (main.rs:5)
18  auto-sub                      	       0x10283d3b0 core::ops::function::FnOnce::call_once::h5e90b25f90b7d14b + 20 (function.rs:250)
19  auto-sub                      	       0x10283d288 std::sys::backtrace::__rust_begin_short_backtrace::hbe1a20f22cde41a0 + 24 (backtrace.rs:166)
20  auto-sub                      	       0x10283d310 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h5a16d6ad59ac689b + 28 (rt.rs:206)
21  auto-sub                      	       0x104482c6c std::rt::lang_start_internal::h11fc0900699d88c7 + 952
22  auto-sub                      	       0x10283d2e8 std::rt::lang_start::h4a962d0540ace8f4 + 84 (rt.rs:205)
23  auto-sub                      	       0x10283d3f4 main + 36
24  dyld                          	       0x190063da4 start + 6992

Thread 1:: com.apple.NSEventThread
0   libsystem_kernel.dylib        	       0x1903dbc34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x1903ee574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x1903e49c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x1903dbfc0 mach_msg + 24
4   CoreFoundation                	       0x1904dcd68 __CFRunLoopServiceMachPort + 160
5   CoreFoundation                	       0x1904db654 __CFRunLoopRun + 1188
6   CoreFoundation                	       0x1905adbe0 _CFRunLoopRunSpecificWithOptions + 532
7   AppKit                        	       0x194a28c64 _NSEventThread + 184
8   libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
9   libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 2:: WebCore: Scrolling
0   libsystem_kernel.dylib        	       0x1903dbc34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x1903ee574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x1903e49c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x1903dbfc0 mach_msg + 24
4   CoreFoundation                	       0x1904dcd68 __CFRunLoopServiceMachPort + 160
5   CoreFoundation                	       0x1904db654 __CFRunLoopRun + 1188
6   CoreFoundation                	       0x1905adbe0 _CFRunLoopRunSpecificWithOptions + 532
7   CoreFoundation                	       0x190551524 CFRunLoopRun + 64
8   JavaScriptCore                	       0x1b244f030 WTF::Detail::CallableWrapper<WTF::RunLoop::create(WTF::ASCIILiteral, WTF::ThreadType, WTF::Thread::QOS)::$_0, void>::call() + 240
9   JavaScriptCore                	       0x1b24968cc WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*) + 296
10  JavaScriptCore                	       0x1b2263088 WTF::wtfThreadEntryPoint(void*) + 16
11  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
12  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 3:: Log work queue
0   libsystem_kernel.dylib        	       0x1903dbbb0 semaphore_wait_trap + 8
1   WebKit                        	       0x1bb7e7ab0 IPC::StreamConnectionWorkQueue::startProcessingThread()::$_0::operator()() + 48
2   JavaScriptCore                	       0x1b24968cc WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*) + 296
3   JavaScriptCore                	       0x1b2263088 WTF::wtfThreadEntryPoint(void*) + 16
4   libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
5   libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 4:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 5:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903e1fc4 kevent + 8
1   auto-sub                      	       0x1040af764 mio::sys::unix::selector::Selector::select::hafab6b932786ff95 + 200
2   auto-sub                      	       0x1040ab7c4 mio::poll::Poll::poll::hbae0e0a074cec30d + 80
3   auto-sub                      	       0x10408abe8 tokio::runtime::io::driver::Driver::turn::h8955633b70a6deec + 200
4   auto-sub                      	       0x10408ab14 tokio::runtime::io::driver::Driver::park::h02b5ad2959686a4d + 80
5   auto-sub                      	       0x10409be98 tokio::runtime::signal::Driver::park::h1a2a4813cae6b2c0 + 36
6   auto-sub                      	       0x10407d16c tokio::runtime::process::Driver::park::h7225895c596a7a4c + 32
7   auto-sub                      	       0x10409ba3c tokio::runtime::driver::IoStack::park::h2b268599b72bfcf2 + 104
8   auto-sub                      	       0x1040662e0 tokio::runtime::time::Driver::park_internal::he14f614974152d7b + 424
9   auto-sub                      	       0x104066714 tokio::runtime::time::Driver::park::h5cc4a76dcd959357 + 40
10  auto-sub                      	       0x10409ad14 tokio::runtime::driver::TimeDriver::park::h02c9f591939bf24f + 92
11  auto-sub                      	       0x10409b810 tokio::runtime::driver::Driver::park::h271244f23ac59555 + 32
12  auto-sub                      	       0x1040941cc tokio::runtime::scheduler::multi_thread::park::Inner::park_driver::hd4e08710b10a9ece + 264
13  auto-sub                      	       0x104094c08 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 240
14  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
15  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
16  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
17  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
18  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
19  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
20  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
21  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
22  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
23  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
24  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
25  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
26  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
27  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
28  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
29  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
30  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
31  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
32  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
33  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
34  auto-sub                      	       0x10403703c __rust_try + 32
35  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
36  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
37  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
38  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
39  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
40  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
41  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
42  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
43  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
44  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
45  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
46  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
47  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
48  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
49  auto-sub                      	       0x104068160 __rust_try + 32
50  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
51  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
52  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
53  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
54  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 6:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 7:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 8:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 9:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 10:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 11:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 12:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 13:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   auto-sub                      	       0x1040b8110 _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x1040b6d70 parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x1040b5918 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x1040b87dc parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x104086378 parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x1040386dc tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1040944f0 tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x104094bcc tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x10409500c tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x10407facc tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x104080ac0 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1040805bc tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x10407d708 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x10407945c tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x10408bd78 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x10406fca4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x10406e478 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x10408bcbc tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x10407d62c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x10406a6a8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x10407d4d4 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x10407e694 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x104073da4 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x104034a14 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x10403443c tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10402ff8c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x104089c00 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x10409038c std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10403703c __rust_try + 32
31  auto-sub                      	       0x104036694 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x10402fc5c tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x104030798 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x104031448 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x104070b7c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x10407133c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1040986a4 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x104059f98 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x10405a1ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x10405b088 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1040558f0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1040601e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x104089f34 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1040908dc std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x104068160 __rust_try + 32
46  auto-sub                      	       0x10405fdf8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1040444b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x104488410 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 14::  Dispatch queue: com.apple.root.default-qos
0   libsystem_kernel.dylib        	       0x1903df308 __semwait_signal + 8
1   libsystem_c.dylib             	       0x1902bbcc0 nanosleep + 220
2   libsystem_c.dylib             	       0x1902bbbd8 usleep + 68
3   auto-sub                      	       0x1030d0c68 __ggml_metal_rsets_init_block_invoke + 92
4   libdispatch.dylib             	       0x190264a28 _dispatch_call_block_and_release + 32
5   libdispatch.dylib             	       0x19027e4b0 _dispatch_client_callout + 16
6   libdispatch.dylib             	       0x19029bd9c <deduplicated_symbol> + 76
7   libdispatch.dylib             	       0x190276adc _dispatch_root_queue_drain + 708
8   libdispatch.dylib             	       0x190277120 _dispatch_worker_thread2 + 184
9   libsystem_pthread.dylib       	       0x19041be84 _pthread_wqthread + 232
10  libsystem_pthread.dylib       	       0x19041ac10 start_wqthread + 8

Thread 15::  Dispatch queue: com.apple.root.user-interactive-qos
0   libsystem_kernel.dylib        	       0x1903dbc34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x1903ee574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x1903e49c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x1903dbfc0 mach_msg + 24
4   CoreFoundation                	       0x1904dcd68 __CFRunLoopServiceMachPort + 160
5   CoreFoundation                	       0x1904db654 __CFRunLoopRun + 1188
6   CoreFoundation                	       0x1905adbe0 _CFRunLoopRunSpecificWithOptions + 532
7   Foundation                    	       0x191d24b44 -[NSRunLoop(NSRunLoop) runMode:beforeDate:] + 212
8   AppKit                        	       0x1950d8f00 -[NSAnimation _runBlocking] + 412
9   libdispatch.dylib             	       0x190264a28 _dispatch_call_block_and_release + 32
10  libdispatch.dylib             	       0x19027e4b0 _dispatch_client_callout + 16
11  libdispatch.dylib             	       0x19029bd6c <deduplicated_symbol> + 28
12  libdispatch.dylib             	       0x190276adc _dispatch_root_queue_drain + 708
13  libdispatch.dylib             	       0x190277120 _dispatch_worker_thread2 + 184
14  libsystem_pthread.dylib       	       0x19041be84 _pthread_wqthread + 232
15  libsystem_pthread.dylib       	       0x19041ac10 start_wqthread + 8

Thread 16:: JavaScriptCore libpas scavenger
0   libsystem_kernel.dylib        	       0x1903df50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x190420128 _pthread_cond_wait + 980
2   JavaScriptCore                	       0x1b3b0ac38 scavenger_thread_main + 1416
3   libsystem_pthread.dylib       	       0x19041fc58 _pthread_start + 136
4   libsystem_pthread.dylib       	       0x19041ac1c thread_start + 8

Thread 17:

Thread 18:

Thread 19:

Thread 20:

Thread 21:


Thread 0 crashed with ARM Thread State (64-bit):
    x0: 0x0000000000000000   x1: 0x0000000000000000   x2: 0x0000000000000000   x3: 0x0000000000000000
    x4: 0x00000001903298b7   x5: 0x000000016d5ba260   x6: 0x0000000000000032   x7: 0x0000000000000000
    x8: 0x961f699acbb646ad   x9: 0x961f699b3794de6d  x10: 0x0000000000000002  x11: 0x00000000fffffffd
   x12: 0x0000000000000000  x13: 0x0000000000000000  x14: 0x0000000000000000  x15: 0x0000000000000000
   x16: 0x0000000000000148  x17: 0x00000001fd861f20  x18: 0x0000000000000000  x19: 0x0000000000000006
   x20: 0x0000000000000103  x21: 0x00000001fc2299a0  x22: 0x0000000cbe051e00  x23: 0x0000000000000002
   x24: 0x0000000000000008  x25: 0x00000001fc233000  x26: 0x0000000000000015  x27: 0x0000000cbe051e10
   x28: 0x0000000000000014   fp: 0x000000016d5bab50   lr: 0x000000019041f8d8
    sp: 0x000000016d5bab30   pc: 0x00000001903e45e8 cpsr: 0x40001000
   far: 0x0000000000000000  esr: 0x56000080 (Syscall)

Binary Images:
       0x10283c000 -        0x1052ebfff auto-sub (*) <ed2b4517-4a36-30f8-9212-48e276c94ee5> /Users/USER/Documents/*/auto-sub
       0x1073bc000 -        0x1073c7fff libobjc-trampolines.dylib (*) <a4dd56f1-375a-3540-844b-5e397f0b78b3> /usr/lib/libobjc-trampolines.dylib
       0x120da0000 -        0x12152bfff com.apple.AGXMetalG13X (350.38) <95c14223-fa99-3e3d-9570-8e7862d86a54> /System/Library/Extensions/AGXMetalG13X.bundle/Contents/MacOS/AGXMetalG13X
       0x11fb40000 -        0x11fba3fff com.apple.AppleMetalOpenGLRenderer (1.0) <0a5a2e2b-9899-3606-af8c-84850db23fea> /System/Library/Extensions/AppleMetalOpenGLRenderer.bundle/Contents/MacOS/AppleMetalOpenGLRenderer
       0x1903db000 -        0x19041828f libsystem_kernel.dylib (*) <51565b39-f595-3e96-a217-fef29815057a> /usr/lib/system/libsystem_kernel.dylib
       0x190419000 -        0x190425b3b libsystem_pthread.dylib (*) <e7a73008-0c09-31e3-9dd9-0c61652f0e85> /usr/lib/system/libsystem_pthread.dylib
       0x1902ae000 -        0x19032eef7 libsystem_c.dylib (*) <66ebd321-6899-3863-ba24-5cfc3076a0cb> /usr/lib/system/libsystem_c.dylib
       0x190044000 -        0x1900e9ec7 dyld (*) <9f682dcf-340c-3bfa-bcdd-dd702f30313e> /usr/lib/dyld
               0x0 - 0xffffffffffffffff ??? (*) <00000000-0000-0000-0000-000000000000> ???
       0x19045f000 -        0x1909bcc5f com.apple.CoreFoundation (6.9) <04941709-2330-3bf8-9213-6d33964db448> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
       0x1948ce000 -        0x195ff0abf com.apple.AppKit (6.9) <59e23bd5-d01e-305a-b96f-a5790356049a> /System/Library/Frameworks/AppKit.framework/Versions/C/AppKit
       0x1b225d000 -        0x1b3d05ddf com.apple.JavaScriptCore (21624) <cae1f78c-542e-30f7-8e08-fdca4b880e04> /System/Library/Frameworks/JavaScriptCore.framework/Versions/A/JavaScriptCore
       0x1ba5d6000 -        0x1bbbaf3df com.apple.WebKit (21624) <e7e04c4f-689c-3e21-bd4a-c7591fd3d5ca> /System/Library/Frameworks/WebKit.framework/Versions/A/WebKit
       0x190263000 -        0x1902aa23f libdispatch.dylib (*) <e17aa23f-db2a-3302-b14c-f6b08c540fcf> /usr/lib/system/libdispatch.dylib
       0x191ccb000 -        0x192cad1df com.apple.Foundation (6.9) <8e9a5c62-7e95-3047-81e7-735ae1aee5f8> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation

External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by all processes on this machine:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0

-----------
Full Report
-----------

{"app_name":"auto-sub","timestamp":"2026-04-06 00:54:18.00 +0700","app_version":"0.1.0","slice_uuid":"ed2b4517-4a36-30f8-9212-48e276c94ee5","build_version":"0.1.0","platform":1,"share_with_app_devs":0,"is_first_party":1,"bug_type":"309","os_version":"macOS 26.4 (25E246)","roots_installed":0,"incident_id":"D661A3A9-CE10-427E-9BDD-81A887BE6897","name":"auto-sub"}
{
  "uptime" : 60000,
  "procRole" : "Foreground",
  "version" : 2,
  "userID" : 501,
  "deployVersion" : 210,
  "modelCode" : "MacBookPro18,2",
  "coalitionID" : 559,
  "osVersion" : {
    "train" : "macOS 26.4",
    "build" : "25E246",
    "releaseType" : "User"
  },
  "captureTime" : "2026-04-06 00:54:14.3590 +0700",
  "codeSigningMonitor" : 2,
  "incident" : "D661A3A9-CE10-427E-9BDD-81A887BE6897",
  "pid" : 56186,
  "translated" : false,
  "cpuType" : "ARM-64",
  "procLaunch" : "2026-04-06 00:45:32.4023 +0700",
  "procStartAbsTime" : 1437440687154,
  "procExitAbsTime" : 1449966572794,
  "procName" : "auto-sub",
  "procPath" : "\/Users\/USER\/Documents\/*\/auto-sub",
  "bundleInfo" : {"CFBundleVersion":"0.1.0","CFBundleShortVersionString":"0.1.0"},
  "parentProc" : "node",
  "parentPid" : 55947,
  "coalitionName" : "com.google.antigravity",
  "crashReporterKey" : "E601F452-0C7D-49A2-E3D9-1DAD517AD943",
  "appleIntelligenceStatus" : {"state":"available"},
  "developerMode" : 1,
  "responsiblePid" : 49035,
  "responsibleProc" : "Electron",
  "codeSigningID" : "auto_sub-21e863f83ad9e4ae",
  "codeSigningTeamID" : "",
  "codeSigningFlags" : 570556929,
  "codeSigningValidationCategory" : 10,
  "codeSigningTrustLevel" : 4294967295,
  "codeSigningAuxiliaryInfo" : 0,
  "instructionByteStream" : {"beforePC":"fyMD1f17v6n9AwCRCuD\/l78DAJH9e8Go\/w9f1sADX9YQKYDSARAA1A==","atPC":"AwEAVH8jA9X9e7+p\/QMAkf\/f\/5e\/AwCR\/XvBqP8PX9bAA1\/WcAqA0g=="},
  "bootSessionUUID" : "4B4774C4-B0B7-4EF7-BBC7-45320FBE6398",
  "sip" : "enabled",
  "exception" : {"codes":"0x0000000000000000, 0x0000000000000000","rawCodes":[0,0],"type":"EXC_CRASH","signal":"SIGABRT"},
  "termination" : {"flags":0,"code":6,"namespace":"SIGNAL","indicator":"Abort trap: 6","byProc":"auto-sub","byPid":56186},
  "asi" : {"libsystem_c.dylib":["abort() called"]},
  "extMods" : {"caller":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"system":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"targeted":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"warnings":0},
  "faultingThread" : 0,
  "threads" : [{"threadState":{"x":[{"value":0},{"value":0},{"value":0},{"value":0},{"value":6714202295,"symbolLocation":0,"symbol":"__vfprintf.xdigs_lower"},{"value":6129689184},{"value":50},{"value":0},{"value":10817480943530821293},{"value":10817480945340571245},{"value":2},{"value":4294967293},{"value":0},{"value":0},{"value":0},{"value":0},{"value":328},{"value":8548392736},{"value":0},{"value":6},{"value":259},{"value":8525093280,"symbolLocation":224,"symbol":"_main_thread"},{"value":54727613952},{"value":2},{"value":8},{"value":8525131776,"symbolLocation":3288,"symbol":"usual_extra"},{"value":21},{"value":54727613968},{"value":20}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715209944},"cpsr":{"value":1073745920},"fp":{"value":6129691472},"sp":{"value":6129691440},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714967528,"matchesCrashFrame":1},"far":{"value":0}},"id":1926116,"triggered":true,"name":"main","queue":"com.apple.main-thread","frames":[{"imageOffset":38376,"symbol":"__pthread_kill","symbolLocation":8,"imageIndex":4},{"imageOffset":26840,"symbol":"pthread_kill","symbolLocation":296,"imageIndex":5},{"imageOffset":493456,"symbol":"abort","symbolLocation":148,"imageIndex":6},{"imageOffset":8481724,"symbol":"ggml_abort","symbolLocation":160,"imageIndex":0},{"imageOffset":8998248,"symbol":"ggml_metal_rsets_free","symbolLocation":152,"imageIndex":0},{"imageOffset":9000548,"symbol":"ggml_metal_device_free","symbolLocation":24,"imageIndex":0},{"imageOffset":9005200,"symbol":"std::__1::unique_ptr<ggml_metal_device, ggml_metal_device_deleter>::~unique_ptr[abi:ne200100]()","symbolLocation":32,"imageIndex":0},{"imageOffset":163400,"symbol":"__cxa_finalize_ranges","symbolLocation":416,"imageIndex":6},{"imageOffset":162888,"symbol":"exit","symbolLocation":44,"imageIndex":6},{"imageOffset":29669564,"symbol":"std::sys::pal::unix::os::exit::ha5ed4bc9d93d94d8","symbolLocation":12,"imageIndex":0},{"imageOffset":29696452,"symbol":"std::process::exit::h0dabe4f5f6de9a0c","symbolLocation":28,"imageIndex":0},{"imageOffset":4742116,"symbol":"tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run::h76fd10526c123129","symbolLocation":100,"imageIndex":0},{"imageOffset":5168100,"symbol":"tao::event_loop::EventLoop$LT$T$GT$::run::h1b842902c9f6e61d","symbolLocation":88,"imageIndex":0},{"imageOffset":1089380,"symbol":"_$LT$tauri_runtime_wry..Wry$LT$T$GT$$u20$as$u20$tauri_runtime..Runtime$LT$T$GT$$GT$::run::h1f667a226663975d","symbolLocation":172,"imageIndex":0},{"imageOffset":5047076,"symbol":"tauri::app::App$LT$R$GT$::run::h1cbca2608fa4a004","symbolLocation":640,"imageIndex":0},{"imageOffset":5048740,"symbol":"tauri::app::Builder$LT$R$GT$::run::he84bd9244481185c","symbolLocation":284,"imageIndex":0},{"imageOffset":602484,"symbol":"auto_sub_lib::run::hf1313e0555180035","symbolLocation":776,"imageIndex":0},{"imageOffset":5064,"sourceLine":5,"sourceFile":"main.rs","symbol":"auto_sub::main::h797125012a65b44c","imageIndex":0,"symbolLocation":12},{"imageOffset":5040,"sourceLine":250,"sourceFile":"function.rs","symbol":"core::ops::function::FnOnce::call_once::h5e90b25f90b7d14b","imageIndex":0,"symbolLocation":20},{"imageOffset":4744,"sourceLine":166,"sourceFile":"backtrace.rs","symbol":"std::sys::backtrace::__rust_begin_short_backtrace::hbe1a20f22cde41a0","imageIndex":0,"symbolLocation":24},{"imageOffset":4880,"sourceLine":206,"sourceFile":"rt.rs","symbol":"std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h5a16d6ad59ac689b","imageIndex":0,"symbolLocation":28},{"imageOffset":29650028,"symbol":"std::rt::lang_start_internal::h11fc0900699d88c7","symbolLocation":952,"imageIndex":0},{"imageOffset":4840,"sourceLine":205,"sourceFile":"rt.rs","symbol":"std::rt::lang_start::h4a962d0540ace8f4","imageIndex":0,"symbolLocation":84},{"imageOffset":5108,"symbol":"main","symbolLocation":36,"imageIndex":0},{"imageOffset":130468,"symbol":"start","symbolLocation":6992,"imageIndex":7}]},{"id":1926199,"name":"com.apple.NSEventThread","threadState":{"x":[{"value":268451845},{"value":21592279046},{"value":8589934592},{"value":134153303490560},{"value":0},{"value":134153303490560},{"value":2},{"value":4294967295},{"value":0},{"value":17179869184},{"value":0},{"value":2},{"value":0},{"value":0},{"value":31235},{"value":0},{"value":18446744073709551569},{"value":8548394544},{"value":0},{"value":4294967295},{"value":2},{"value":134153303490560},{"value":0},{"value":134153303490560},{"value":21592279046},{"value":6131998856},{"value":8589934592},{"value":18446744073709550527},{"value":4412409862}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715008372},"cpsr":{"value":4096},"fp":{"value":6131998704},"sp":{"value":6131998624},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714932276},"far":{"value":0}},"frames":[{"imageOffset":3124,"symbol":"mach_msg2_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":79220,"symbol":"mach_msg2_internal","symbolLocation":76,"imageIndex":4},{"imageOffset":39360,"symbol":"mach_msg_overwrite","symbolLocation":480,"imageIndex":4},{"imageOffset":4032,"symbol":"mach_msg","symbolLocation":24,"imageIndex":4},{"imageOffset":515432,"symbol":"__CFRunLoopServiceMachPort","symbolLocation":160,"imageIndex":9},{"imageOffset":509524,"symbol":"__CFRunLoopRun","symbolLocation":1188,"imageIndex":9},{"imageOffset":1371104,"symbol":"_CFRunLoopRunSpecificWithOptions","symbolLocation":532,"imageIndex":9},{"imageOffset":1420388,"symbol":"_NSEventThread","symbolLocation":184,"imageIndex":10},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926246,"name":"WebCore: Scrolling","threadState":{"x":[{"value":268451845},{"value":21592279046},{"value":8589934592},{"value":218815698829312},{"value":0},{"value":218815698829312},{"value":2},{"value":4294967295},{"value":0},{"value":17179869184},{"value":0},{"value":2},{"value":0},{"value":0},{"value":50947},{"value":50577534889472},{"value":18446744073709551569},{"value":8548394544},{"value":0},{"value":4294967295},{"value":2},{"value":218815698829312},{"value":0},{"value":218815698829312},{"value":21592279046},{"value":6134292424},{"value":8589934592},{"value":18446744073709550527},{"value":4412409862}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715008372},"cpsr":{"value":4096},"fp":{"value":6134292272},"sp":{"value":6134292192},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714932276},"far":{"value":0}},"frames":[{"imageOffset":3124,"symbol":"mach_msg2_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":79220,"symbol":"mach_msg2_internal","symbolLocation":76,"imageIndex":4},{"imageOffset":39360,"symbol":"mach_msg_overwrite","symbolLocation":480,"imageIndex":4},{"imageOffset":4032,"symbol":"mach_msg","symbolLocation":24,"imageIndex":4},{"imageOffset":515432,"symbol":"__CFRunLoopServiceMachPort","symbolLocation":160,"imageIndex":9},{"imageOffset":509524,"symbol":"__CFRunLoopRun","symbolLocation":1188,"imageIndex":9},{"imageOffset":1371104,"symbol":"_CFRunLoopRunSpecificWithOptions","symbolLocation":532,"imageIndex":9},{"imageOffset":992548,"symbol":"CFRunLoopRun","symbolLocation":64,"imageIndex":9},{"imageOffset":2039856,"symbol":"WTF::Detail::CallableWrapper<WTF::RunLoop::create(WTF::ASCIILiteral, WTF::ThreadType, WTF::Thread::QOS)::$_0, void>::call()","symbolLocation":240,"imageIndex":11},{"imageOffset":2332876,"symbol":"WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*)","symbolLocation":296,"imageIndex":11},{"imageOffset":24712,"symbol":"WTF::wtfThreadEntryPoint(void*)","symbolLocation":16,"imageIndex":11},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926325,"name":"Log work queue","threadState":{"x":[{"value":14},{"value":7440593928,"symbolLocation":0,"symbol":"non-virtual thunk to IPC::StreamServerConnection::deref() const"},{"value":0},{"value":6713443720,"symbolLocation":0,"symbol":"_dispatch_lane_push"},{"value":4},{"value":1},{"value":0},{"value":6140031200},{"value":0},{"value":46},{"value":47},{"value":42},{"value":7237707913161080831},{"value":39},{"value":7237707363405266943},{"value":7237707913161080831},{"value":18446744073709551580},{"value":8548397040},{"value":0},{"value":4999922560},{"value":4999922600},{"value":6140030976},{"value":0},{"value":0},{"value":4999922688},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":7440595632},"cpsr":{"value":2147487744},"fp":{"value":6140030800},"sp":{"value":6140030768},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714932144},"far":{"value":0}},"frames":[{"imageOffset":2992,"symbol":"semaphore_wait_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":18946736,"symbol":"IPC::StreamConnectionWorkQueue::startProcessingThread()::$_0::operator()()","symbolLocation":48,"imageIndex":12},{"imageOffset":2332876,"symbol":"WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*)","symbolLocation":296,"imageIndex":11},{"imageOffset":24712,"symbol":"WTF::wtfThreadEntryPoint(void*)","symbolLocation":16,"imageIndex":11},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926387,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":0},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6142168088},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54714906008},{"value":54714906072},{"value":6142177504},{"value":0},{"value":0},{"value":0},{"value":1},{"value":256},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6142168208},"sp":{"value":6142168064},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926388,"name":"tokio-rt-worker","threadState":{"x":[{"value":4},{"value":0},{"value":0},{"value":54708994048},{"value":1024},{"value":0},{"value":0},{"value":6144317344},{"value":1024},{"value":54708994048},{"value":1},{"value":54714450296},{"value":6144316856},{"value":54716161040},{"value":18446744073709491720},{"value":174},{"value":363},{"value":8548394384},{"value":0},{"value":4843634688},{"value":54695409648},{"value":4383491432,"symbolLocation":70888,"symbol":"tao::platform_impl::platform::util::cursor::invisible_cursor::CURSOR_BYTES::h065bd60fe84f49ce"},{"value":54716260480},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":4362794852},"cpsr":{"value":1610616832},"fp":{"value":6144315280},"sp":{"value":6144315120},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714957764},"far":{"value":0}},"frames":[{"imageOffset":28612,"symbol":"kevent","symbolLocation":8,"imageIndex":4},{"imageOffset":25638756,"symbol":"mio::sys::unix::selector::Selector::select::hafab6b932786ff95","symbolLocation":200,"imageIndex":0},{"imageOffset":25622468,"symbol":"mio::poll::Poll::poll::hbae0e0a074cec30d","symbolLocation":80,"imageIndex":0},{"imageOffset":25488360,"symbol":"tokio::runtime::io::driver::Driver::turn::h8955633b70a6deec","symbolLocation":200,"imageIndex":0},{"imageOffset":25488148,"symbol":"tokio::runtime::io::driver::Driver::park::h02b5ad2959686a4d","symbolLocation":80,"imageIndex":0},{"imageOffset":25558680,"symbol":"tokio::runtime::signal::Driver::park::h1a2a4813cae6b2c0","symbolLocation":36,"imageIndex":0},{"imageOffset":25432428,"symbol":"tokio::runtime::process::Driver::park::h7225895c596a7a4c","symbolLocation":32,"imageIndex":0},{"imageOffset":25557564,"symbol":"tokio::runtime::driver::IoStack::park::h2b268599b72bfcf2","symbolLocation":104,"imageIndex":0},{"imageOffset":25338592,"symbol":"tokio::runtime::time::Driver::park_internal::he14f614974152d7b","symbolLocation":424,"imageIndex":0},{"imageOffset":25339668,"symbol":"tokio::runtime::time::Driver::park::h5cc4a76dcd959357","symbolLocation":40,"imageIndex":0},{"imageOffset":25554196,"symbol":"tokio::runtime::driver::TimeDriver::park::h02c9f591939bf24f","symbolLocation":92,"imageIndex":0},{"imageOffset":25557008,"symbol":"tokio::runtime::driver::Driver::park::h271244f23ac59555","symbolLocation":32,"imageIndex":0},{"imageOffset":25526732,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_driver::hd4e08710b10a9ece","symbolLocation":264,"imageIndex":0},{"imageOffset":25529352,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":240,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926389,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":0},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6146460696},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54714907800},{"value":54714907864},{"value":6146470112},{"value":0},{"value":0},{"value":0},{"value":1},{"value":256},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6146460816},"sp":{"value":6146460672},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926390,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":256},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6148607000},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54720038040},{"value":54720038104},{"value":6148616416},{"value":0},{"value":0},{"value":256},{"value":257},{"value":512},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6148607120},"sp":{"value":6148606976},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926391,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":0},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6150753304},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54720041624},{"value":54720041688},{"value":6150762720},{"value":0},{"value":0},{"value":0},{"value":1},{"value":256},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6150753424},"sp":{"value":6150753280},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926392,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":0},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6152899608},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54720043416},{"value":54720043480},{"value":6152909024},{"value":0},{"value":0},{"value":0},{"value":1},{"value":256},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6152899728},"sp":{"value":6152899584},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926393,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":25600},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6155045912},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54720045208},{"value":54720045272},{"value":6155055328},{"value":0},{"value":0},{"value":25600},{"value":25601},{"value":25856},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6155046032},"sp":{"value":6155045888},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926394,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":0},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6157192216},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54720047000},{"value":54720047064},{"value":6157201632},{"value":0},{"value":0},{"value":0},{"value":1},{"value":256},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6157192336},"sp":{"value":6157192192},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926395,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":27392},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6159338520},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":54720048792},{"value":54720048856},{"value":6159347936},{"value":0},{"value":0},{"value":27392},{"value":27393},{"value":27648},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6159338640},"sp":{"value":6159338496},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1926396,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":51968},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6161484824},{"value":0},{"value":256},{"value":1099511628034},{"value":1099511628034},{"value":256},{"value":0},{"value":1099511628032},{"value":305},{"value":8548392664},{"value":0},{"value":54720050584},{"value":54720050648},{"value":6161494240},{"value":0},{"value":0},{"value":51968},{"value":51969},{"value":52224},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6161484944},"sp":{"value":6161484800},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25674000,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25668976,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25663768,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25675740,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25469816,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25151196,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25527536,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25529292,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25530380,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25443020,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25447104,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25445820,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25433864,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25416796,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25492856,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25377956,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25371768,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25492668,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25433644,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25355944,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25433300,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25437844,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25394596,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25135636,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25134140,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25116556,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25484288,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25510796,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25145404,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25142932,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25115740,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25118616,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25121864,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25381756,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25383740,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25544356,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25288600,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25289132,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25292936,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25270512,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25313768,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25485108,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25512156,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25346400,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25312760,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25199796,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29672464,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1927443,"threadState":{"x":[{"value":60},{"value":0},{"value":1},{"value":1},{"value":0},{"value":500000000},{"value":0},{"value":0},{"value":8525134872,"symbolLocation":0,"symbol":"clock_sem"},{"value":3},{"value":17},{"value":2},{"value":1},{"value":54717837760},{"value":72057602563215273,"symbolLocation":72057594037927937,"symbol":"OBJC_CLASS_$_NSLock"},{"value":8525287336,"symbolLocation":0,"symbol":"OBJC_CLASS_$_NSLock"},{"value":334},{"value":8548392784},{"value":0},{"value":0},{"value":6130855536},{"value":4294967295},{"value":4521498624},{"value":284},{"value":6130856160},{"value":18446744073709551615},{"value":4293984255},{"value":0},{"value":4}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6713752768},"cpsr":{"value":2684358656},"fp":{"value":6130855520},"sp":{"value":6130855472},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946312},"far":{"value":0}},"queue":"com.apple.root.default-qos","frames":[{"imageOffset":17160,"symbol":"__semwait_signal","symbolLocation":8,"imageIndex":4},{"imageOffset":56512,"symbol":"nanosleep","symbolLocation":220,"imageIndex":6},{"imageOffset":56280,"symbol":"usleep","symbolLocation":68,"imageIndex":6},{"imageOffset":8997992,"symbol":"__ggml_metal_rsets_init_block_invoke","symbolLocation":92,"imageIndex":0},{"imageOffset":6696,"symbol":"_dispatch_call_block_and_release","symbolLocation":32,"imageIndex":13},{"imageOffset":111792,"symbol":"_dispatch_client_callout","symbolLocation":16,"imageIndex":13},{"imageOffset":232860,"symbol":"<deduplicated_symbol>","symbolLocation":76,"imageIndex":13},{"imageOffset":80604,"symbol":"_dispatch_root_queue_drain","symbolLocation":708,"imageIndex":13},{"imageOffset":82208,"symbol":"_dispatch_worker_thread2","symbolLocation":184,"imageIndex":13},{"imageOffset":11908,"symbol":"_pthread_wqthread","symbolLocation":232,"imageIndex":5},{"imageOffset":7184,"symbol":"start_wqthread","symbolLocation":8,"imageIndex":5}]},{"id":1932861,"threadState":{"x":[{"value":0},{"value":21592279046},{"value":8589934592},{"value":406333970972672},{"value":0},{"value":406333970972672},{"value":2},{"value":4294967295},{"value":0},{"value":17179869184},{"value":0},{"value":2},{"value":0},{"value":0},{"value":94607},{"value":0},{"value":18446744073709551569},{"value":8548394544},{"value":0},{"value":4294967295},{"value":2},{"value":406333970972672},{"value":0},{"value":406333970972672},{"value":21592279046},{"value":6130278136},{"value":8589934592},{"value":18446744073709550527},{"value":4412409862}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715008372},"cpsr":{"value":4096},"fp":{"value":6130277984},"sp":{"value":6130277904},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714932276},"far":{"value":0}},"queue":"com.apple.root.user-interactive-qos","frames":[{"imageOffset":3124,"symbol":"mach_msg2_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":79220,"symbol":"mach_msg2_internal","symbolLocation":76,"imageIndex":4},{"imageOffset":39360,"symbol":"mach_msg_overwrite","symbolLocation":480,"imageIndex":4},{"imageOffset":4032,"symbol":"mach_msg","symbolLocation":24,"imageIndex":4},{"imageOffset":515432,"symbol":"__CFRunLoopServiceMachPort","symbolLocation":160,"imageIndex":9},{"imageOffset":509524,"symbol":"__CFRunLoopRun","symbolLocation":1188,"imageIndex":9},{"imageOffset":1371104,"symbol":"_CFRunLoopRunSpecificWithOptions","symbolLocation":532,"imageIndex":9},{"imageOffset":367428,"symbol":"-[NSRunLoop(NSRunLoop) runMode:beforeDate:]","symbolLocation":212,"imageIndex":14},{"imageOffset":8433408,"symbol":"-[NSAnimation _runBlocking]","symbolLocation":412,"imageIndex":10},{"imageOffset":6696,"symbol":"_dispatch_call_block_and_release","symbolLocation":32,"imageIndex":13},{"imageOffset":111792,"symbol":"_dispatch_client_callout","symbolLocation":16,"imageIndex":13},{"imageOffset":232812,"symbol":"<deduplicated_symbol>","symbolLocation":28,"imageIndex":13},{"imageOffset":80604,"symbol":"_dispatch_root_queue_drain","symbolLocation":708,"imageIndex":13},{"imageOffset":82208,"symbol":"_dispatch_worker_thread2","symbolLocation":184,"imageIndex":13},{"imageOffset":11908,"symbol":"_pthread_wqthread","symbolLocation":232,"imageIndex":5},{"imageOffset":7184,"symbol":"start_wqthread","symbolLocation":8,"imageIndex":5}]},{"id":1933232,"name":"JavaScriptCore libpas scavenger","threadState":{"x":[{"value":316},{"value":0},{"value":3529728},{"value":0},{"value":0},{"value":160},{"value":0},{"value":4999928},{"value":6131429032},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8548392664},{"value":0},{"value":4856621120},{"value":4856621184},{"value":6131429600},{"value":4999928},{"value":0},{"value":3529728},{"value":3530241},{"value":3530496},{"value":0},{"value":8527765504,"symbolLocation":3520,"symbol":"bmalloc_common_primitive_heap_support"}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6715212072},"cpsr":{"value":1610616832},"fp":{"value":6131429152},"sp":{"value":6131429008},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6714946828},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25877560,"symbol":"scavenger_thread_main","symbolLocation":1416,"imageIndex":11},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":1934008,"frames":[],"threadState":{"x":[{"value":6133723136},{"value":134227},{"value":6133186560},{"value":0},{"value":409604},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6133723136},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6715190280},"far":{"value":0}}},{"id":1934040,"frames":[],"threadState":{"x":[{"value":6137016320},{"value":94819},{"value":6136479744},{"value":6137015168},{"value":5128198},{"value":1},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6137015152},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6715190280},"far":{"value":0}}},{"id":1934041,"frames":[],"threadState":{"x":[{"value":6137589760},{"value":39971},{"value":6137053184},{"value":0},{"value":409602},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6137589760},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6715190280},"far":{"value":0}}},{"id":1934760,"frames":[],"threadState":{"x":[{"value":6132576256},{"value":126899},{"value":6132039680},{"value":0},{"value":409604},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6132576256},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6715190280},"far":{"value":0}}},{"id":1934761,"frames":[],"threadState":{"x":[{"value":6133149696},{"value":0},{"value":6132613120},{"value":0},{"value":278532},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6133149696},"esr":{"value":0},"pc":{"value":6715190280},"far":{"value":0}}}],
  "usedImages" : [
  {
    "source" : "P",
    "arch" : "arm64",
    "base" : 4337156096,
    "size" : 44761088,
    "uuid" : "ed2b4517-4a36-30f8-9212-48e276c94ee5",
    "path" : "\/Users\/USER\/Documents\/*\/auto-sub",
    "name" : "auto-sub"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 4416323584,
    "size" : 49152,
    "uuid" : "a4dd56f1-375a-3540-844b-5e397f0b78b3",
    "path" : "\/usr\/lib\/libobjc-trampolines.dylib",
    "name" : "libobjc-trampolines.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 4846125056,
    "CFBundleShortVersionString" : "350.38",
    "CFBundleIdentifier" : "com.apple.AGXMetalG13X",
    "size" : 7913472,
    "uuid" : "95c14223-fa99-3e3d-9570-8e7862d86a54",
    "path" : "\/System\/Library\/Extensions\/AGXMetalG13X.bundle\/Contents\/MacOS\/AGXMetalG13X",
    "name" : "AGXMetalG13X",
    "CFBundleVersion" : "350.38"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 4826857472,
    "CFBundleShortVersionString" : "1.0",
    "CFBundleIdentifier" : "com.apple.AppleMetalOpenGLRenderer",
    "size" : 409600,
    "uuid" : "0a5a2e2b-9899-3606-af8c-84850db23fea",
    "path" : "\/System\/Library\/Extensions\/AppleMetalOpenGLRenderer.bundle\/Contents\/MacOS\/AppleMetalOpenGLRenderer",
    "name" : "AppleMetalOpenGLRenderer",
    "CFBundleVersion" : "1"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6714929152,
    "size" : 250512,
    "uuid" : "51565b39-f595-3e96-a217-fef29815057a",
    "path" : "\/usr\/lib\/system\/libsystem_kernel.dylib",
    "name" : "libsystem_kernel.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6715183104,
    "size" : 52028,
    "uuid" : "e7a73008-0c09-31e3-9dd9-0c61652f0e85",
    "path" : "\/usr\/lib\/system\/libsystem_pthread.dylib",
    "name" : "libsystem_pthread.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6713696256,
    "size" : 528120,
    "uuid" : "66ebd321-6899-3863-ba24-5cfc3076a0cb",
    "path" : "\/usr\/lib\/system\/libsystem_c.dylib",
    "name" : "libsystem_c.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6711164928,
    "size" : 679624,
    "uuid" : "9f682dcf-340c-3bfa-bcdd-dd702f30313e",
    "path" : "\/usr\/lib\/dyld",
    "name" : "dyld"
  },
  {
    "size" : 0,
    "source" : "A",
    "base" : 0,
    "uuid" : "00000000-0000-0000-0000-000000000000"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6715469824,
    "CFBundleShortVersionString" : "6.9",
    "CFBundleIdentifier" : "com.apple.CoreFoundation",
    "size" : 5626976,
    "uuid" : "04941709-2330-3bf8-9213-6d33964db448",
    "path" : "\/System\/Library\/Frameworks\/CoreFoundation.framework\/Versions\/A\/CoreFoundation",
    "name" : "CoreFoundation",
    "CFBundleVersion" : "4424.1.402"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6787227648,
    "CFBundleShortVersionString" : "6.9",
    "CFBundleIdentifier" : "com.apple.AppKit",
    "size" : 24259264,
    "uuid" : "59e23bd5-d01e-305a-b96f-a5790356049a",
    "path" : "\/System\/Library\/Frameworks\/AppKit.framework\/Versions\/C\/AppKit",
    "name" : "AppKit",
    "CFBundleVersion" : "2685.50.120"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 7283789824,
    "CFBundleShortVersionString" : "21624",
    "CFBundleIdentifier" : "com.apple.JavaScriptCore",
    "size" : 27954656,
    "uuid" : "cae1f78c-542e-30f7-8e08-fdca4b880e04",
    "path" : "\/System\/Library\/Frameworks\/JavaScriptCore.framework\/Versions\/A\/JavaScriptCore",
    "name" : "JavaScriptCore",
    "CFBundleVersion" : "21624.1.16.11.4"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 7421648896,
    "CFBundleShortVersionString" : "21624",
    "CFBundleIdentifier" : "com.apple.WebKit",
    "size" : 22909920,
    "uuid" : "e7e04c4f-689c-3e21-bd4a-c7591fd3d5ca",
    "path" : "\/System\/Library\/Frameworks\/WebKit.framework\/Versions\/A\/WebKit",
    "name" : "WebKit",
    "CFBundleVersion" : "21624.1.16.11.4"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6713389056,
    "size" : 291392,
    "uuid" : "e17aa23f-db2a-3302-b14c-f6b08c540fcf",
    "path" : "\/usr\/lib\/system\/libdispatch.dylib",
    "name" : "libdispatch.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6741078016,
    "CFBundleShortVersionString" : "6.9",
    "CFBundleIdentifier" : "com.apple.Foundation",
    "size" : 16654816,
    "uuid" : "8e9a5c62-7e95-3047-81e7-735ae1aee5f8",
    "path" : "\/System\/Library\/Frameworks\/Foundation.framework\/Versions\/C\/Foundation",
    "name" : "Foundation",
    "CFBundleVersion" : "4424.1.402"
  }
],
  "sharedCache" : {
  "base" : 6710034432,
  "size" : 5978570752,
  "uuid" : "7d4906c9-9ca2-3f56-8242-3ec2c1e3245b"
},
  "legacyInfo" : {
  "threadTriggered" : {
    "name" : "main",
    "queue" : "com.apple.main-thread"
  }
},
  "logWritingSignature" : "0ca42597ee3f7d09ed1379fcd95ce961981b887b",
  "bug_type" : "309",
  "roots_installed" : 0,
  "trmStatus" : 1,
  "trialInfo" : {
  "rollouts" : [
    {
      "rolloutId" : "695fd05d8ca5554688521e5e",
      "factorPackIds" : [
        "695fd08781fcd20ded79c1d3",
        "695fd0d28ca5554688521e5f",
        "695fd09c8774dc09015a80e9",
        "695fd0b18774dc09015a80ea"
      ],
      "deploymentId" : 3
    },
    {
      "rolloutId" : "60186475825c62000ccf5450",
      "factorPackIds" : [

      ],
      "deploymentId" : 240000083
    }
  ],
  "experiments" : [

  ]
}
}

