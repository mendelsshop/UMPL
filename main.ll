; ModuleID = 'main.bd587cf2a5a1bf91-cgu.0'
source_filename = "main.bd587cf2a5a1bf91-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"[closure@std::panicking::begin_panic<&str>::{closure#0}]" = type { { ptr, i64 }, ptr }
%"[closure@main.rs:10:9: 10:21]" = type { %t, %t }
%t = type { { ptr, i64 }, i32, [1 x i32] }
%"core::alloc::layout::LayoutError" = type {}
%"alloc::rc::RcBox<str>" = type { i64, i64, [0 x i8] }
%"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>" = type { i64, [2 x i64] }
%"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>::Ok" = type { [1 x i64], { ptr, i64 } }
%"alloc::rc::RcBox<[u8]>" = type { i64, i64, [0 x i8] }
%"core::ptr::metadata::PtrRepr<str>" = type { [2 x i64] }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%"alloc::alloc::Global" = type {}
%"[closure@main.rs:9:14: 9:25]" = type {}

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hdc1bebf6feaf1ee2E", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2fc2660f257f72a7E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcb523927de7c3b6cE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcb523927de7c3b6cE" }>, align 8
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr }> <{ ptr @"_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic..PanicPayload$LT$$RF$str$GT$$GT$17h4045312c374dbf02E", [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h451641809f4520c9E", ptr @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17h1f27caaa03aef3a8E" }>, align 8
@alloc_b0710386619a750dcc11646581875175 = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/8ede3aae28fe6e4d52b38157d7bfe0d3bceef225/library/core/src/alloc/layout.rs" }>, align 1
@alloc_451c02edcc1064a5c2d8a8c409aa417e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b0710386619a750dcc11646581875175, [16 x i8] c"P\00\00\00\00\00\00\00\BF\01\00\00)\00\00\00" }>, align 8
@str.2 = internal constant [25 x i8] c"attempt to divide by zero"
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.3 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr53drop_in_place$LT$core..alloc..layout..LayoutError$GT$17h4b5a07927901b802E", [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hb2f579bc0a3b7db3E" }>, align 8
@alloc_a27c0b87fdcfa3c42e227237e0d9eb03 = private unnamed_addr constant <{ [71 x i8] }> <{ [71 x i8] c"/rustc/8ede3aae28fe6e4d52b38157d7bfe0d3bceef225/library/alloc/src/rc.rs" }>, align 1
@alloc_9cb44305eed26d545f11abe0d9c1d0e8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a27c0b87fdcfa3c42e227237e0d9eb03, [16 x i8] c"G\00\00\00\00\00\00\00\C2\05\00\00)\00\00\00" }>, align 8
@alloc_38a9d1c1fccd92e612dd2762da060982 = private unnamed_addr constant <{}> zeroinitializer, align 1
@__rust_no_alloc_shim_is_unstable = external global i8
@vtable.4 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h5ed3373aba7857d0E", [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h380d24678bdc996dE" }>, align 8
@alloc_e16ab3422f24440ac4aefa70ee9428b2 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c"bb" }>, align 1
@alloc_62d2e2f2b7ce4560336617a7e46651ee = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c"aa" }>, align 1
@alloc_4693327ca9c5449cec9b739948ccbb5e = private unnamed_addr constant <{ [7 x i8] }> <{ [7 x i8] c"main.rs" }>, align 1
@alloc_cd4a1854d14f2c6ff367ecf9e4824f01 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4693327ca9c5449cec9b739948ccbb5e, [16 x i8] c"\07\00\00\00\00\00\00\00\0E\00\00\00\16\00\00\00" }>, align 8

; <T as core::any::Any>::type_id
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h380d24678bdc996dE"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  store i64 -4493808902380553279, ptr %0, align 8
  %_2 = load i64, ptr %0, align 8, !noundef !3
  store i64 %_2, ptr %1, align 8
  %2 = load i64, ptr %1, align 8, !noundef !3
  ret i64 %2
}

; std::sys_common::backtrace::__rust_end_short_backtrace
; Function Attrs: noinline noreturn nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h211c8971168b69e6E(ptr %f) unnamed_addr #1 {
start:
; call std::panicking::begin_panic::{{closure}}
  call void @"_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17ha9e96c7968a8f588E"(ptr %f) #17
  call void asm sideeffect "", "~{memory}"(), !srcloc !4
  call void @llvm.trap()
  unreachable
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h09efaa456381a7c3E(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h0c9e225d9fe487aeE(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !4
  ret void
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17hb0a30f0bcf58091dE(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 {
start:
  %_8 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17hd66bf6b7da144005E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %1 = load i64, ptr %_5, align 8, !noundef !3
  ret i64 %1
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcb523927de7c3b6cE"(ptr align 8 %_1) unnamed_addr #3 {
start:
  %self = alloca i8, align 1
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h09efaa456381a7c3E(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb0ee916be6c8ee5dE"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1, !noundef !3
  %1 = zext i8 %_6 to i32
  ret i32 %1
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
define internal void @_ZN3std9panicking11begin_panic17h2059a05e82eb06bfE(ptr align 1 %msg.0, i64 %msg.1, ptr align 8 %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %2 = alloca ptr, align 8
  %_3 = alloca %"[closure@std::panicking::begin_panic<&str>::{closure#0}]", align 8
  store ptr %0, ptr %2, align 8
  %loc = load ptr, ptr %2, align 8, !nonnull !3, !align !5, !noundef !3
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_3, i32 0, i32 0
  store ptr %msg.0, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_3, i32 0, i32 1
  store i64 %msg.1, ptr %4, align 8
  %5 = getelementptr inbounds %"[closure@std::panicking::begin_panic<&str>::{closure#0}]", ptr %_3, i32 0, i32 1
  store ptr %loc, ptr %5, align 8
; invoke std::sys_common::backtrace::__rust_end_short_backtrace
  invoke void @_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h211c8971168b69e6E(ptr %_3) #17
          to label %unreachable unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br i1 false, label %bb2, label %bb1

cleanup:                                          ; preds = %start
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  %9 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %7, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %8, ptr %10, align 8
  br label %bb3

unreachable:                                      ; preds = %start
  unreachable

bb1:                                              ; preds = %bb2, %bb3
  %11 = load ptr, ptr %1, align 8, !noundef !3
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %13 = load i32, ptr %12, align 8, !noundef !3
  %14 = insertvalue { ptr, i32 } poison, ptr %11, 0
  %15 = insertvalue { ptr, i32 } %14, i32 %13, 1
  resume { ptr, i32 } %15

bb2:                                              ; preds = %bb3
  br label %bb1
}

; std::panicking::begin_panic::{{closure}}
; Function Attrs: inlinehint noreturn nonlazybind uwtable
define internal void @"_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17ha9e96c7968a8f588E"(ptr %_1) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %_8 = alloca { ptr, i64 }, align 8
  %_4 = alloca { ptr, i64 }, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_1, i32 0, i32 0
  %inner.0 = load ptr, ptr %1, align 8, !nonnull !3, !align !6, !noundef !3
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_1, i32 0, i32 1
  %inner.1 = load i64, ptr %2, align 8, !noundef !3
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 0
  store ptr %inner.0, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 1
  store i64 %inner.1, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 0
  %6 = load ptr, ptr %5, align 8, !align !6, !noundef !3
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 1
  %8 = load i64, ptr %7, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_4, i32 0, i32 0
  store ptr %6, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_4, i32 0, i32 1
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds %"[closure@std::panicking::begin_panic<&str>::{closure#0}]", ptr %_1, i32 0, i32 1
  %_7 = load ptr, ptr %11, align 8, !nonnull !3, !align !5, !noundef !3
; invoke std::panicking::rust_panic_with_hook
  invoke void @_ZN3std9panicking20rust_panic_with_hook17h82ebcd5d5ed2fad4E(ptr align 1 %_4, ptr align 8 @vtable.1, ptr align 8 null, ptr align 8 %_7, i1 zeroext true) #17
          to label %unreachable unwind label %cleanup

bb1:                                              ; preds = %cleanup
  %12 = load ptr, ptr %0, align 8, !noundef !3
  %13 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %14 = load i32, ptr %13, align 8, !noundef !3
  %15 = insertvalue { ptr, i32 } poison, ptr %12, 0
  %16 = insertvalue { ptr, i32 } %15, i32 %14, 1
  resume { ptr, i32 } %16

cleanup:                                          ; preds = %start
  %17 = landingpad { ptr, i32 }
          cleanup
  %18 = extractvalue { ptr, i32 } %17, 0
  %19 = extractvalue { ptr, i32 } %17, 1
  %20 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %18, ptr %20, align 8
  %21 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %19, ptr %21, align 8
  br label %bb1

unreachable:                                      ; preds = %start
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2fc2660f257f72a7E"(ptr %_1) unnamed_addr #3 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17hd5ae3e0d2f980975E(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h0c9e225d9fe487aeE(ptr %_1) unnamed_addr #3 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17hd5ae3e0d2f980975E(ptr %0) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcb523927de7c3b6cE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %3 = load ptr, ptr %1, align 8, !noundef !3
  %4 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %5 = load i32, ptr %4, align 8, !noundef !3
  %6 = insertvalue { ptr, i32 } poison, ptr %3, 0
  %7 = insertvalue { ptr, i32 } %6, i32 %5, 1
  resume { ptr, i32 } %7

cleanup:                                          ; preds = %start
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  %11 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %9, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %10, ptr %12, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %2
}

; core::ptr::drop_in_place<&str>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h5ed3373aba7857d0E"(ptr align 8 %_1) unnamed_addr #3 {
start:
  ret void
}

; core::ptr::drop_in_place<main::t>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call core::ptr::drop_in_place<alloc::rc::Rc<str>>
  call void @"_ZN4core3ptr45drop_in_place$LT$alloc..rc..Rc$LT$str$GT$$GT$17h33ff7388c14d2ab9E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::rc::Rc<str>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr45drop_in_place$LT$alloc..rc..Rc$LT$str$GT$$GT$17h33ff7388c14d2ab9E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
  call void @"_ZN64_$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he710579c148bcdaaE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<core::alloc::layout::LayoutError>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$core..alloc..layout..LayoutError$GT$17h4b5a07927901b802E"(ptr align 1 %_1) unnamed_addr #3 {
start:
  ret void
}

; core::ptr::drop_in_place<dyn core::any::Any+core::marker::Send>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr66drop_in_place$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$17h0fc04eaf289ad2d6E"(ptr align 1 %_1.0, ptr align 8 %_1.1) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds ptr, ptr %_1.1, i64 0
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  call void %1(ptr align 1 %_1.0)
  ret void
}

; core::ptr::drop_in_place<std::panicking::begin_panic::PanicPayload<&str>>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic..PanicPayload$LT$$RF$str$GT$$GT$17h4045312c374dbf02E"(ptr align 8 %_1) unnamed_addr #3 {
start:
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hdc1bebf6feaf1ee2E"(ptr align 8 %_1) unnamed_addr #3 {
start:
  ret void
}

; core::ptr::drop_in_place<main::main::{{closure}}::{{closure}}>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr89drop_in_place$LT$main..main..$u7b$$u7b$closure$u7d$$u7d$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hff896d77741f45deE"(ptr align 8 %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
; invoke core::ptr::drop_in_place<main::t>
  invoke void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %1 = getelementptr inbounds %"[closure@main.rs:10:9: 10:21]", ptr %_1, i32 0, i32 1
; invoke core::ptr::drop_in_place<main::t>
  invoke void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %1) #18
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = extractvalue { ptr, i32 } %2, 0
  %4 = extractvalue { ptr, i32 } %2, 1
  %5 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %3, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %4, ptr %6, align 8
  br label %bb3

bb4:                                              ; preds = %start
  %7 = getelementptr inbounds %"[closure@main.rs:10:9: 10:21]", ptr %_1, i32 0, i32 1
; call core::ptr::drop_in_place<main::t>
  call void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %7)
  ret void

terminate:                                        ; preds = %bb3
  %8 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hc3ef110419ba8f94E() #19
  unreachable

bb1:                                              ; preds = %bb3
  %11 = load ptr, ptr %0, align 8, !noundef !3
  %12 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %13 = load i32, ptr %12, align 8, !noundef !3
  %14 = insertvalue { ptr, i32 } poison, ptr %11, 0
  %15 = insertvalue { ptr, i32 } %14, i32 %13, 1
  resume { ptr, i32 } %15
}

; core::ptr::drop_in_place<alloc::boxed::Box<dyn core::any::Any+core::marker::Send>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr91drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$$GT$17h9e5aa66c425eb603E"(ptr align 8 %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %1 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 0
  %_4.0 = load ptr, ptr %1, align 8, !noundef !3
  %2 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %_4.1 = load ptr, ptr %2, align 8, !nonnull !3, !align !5, !noundef !3
  %3 = getelementptr inbounds ptr, ptr %_4.1, i64 0
  %4 = load ptr, ptr %3, align 8, !invariant.load !3, !nonnull !3
  invoke void %4(ptr align 1 %_4.0)
          to label %bb3 unwind label %cleanup

bb4:                                              ; preds = %cleanup
  %5 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 0
  %6 = load ptr, ptr %5, align 8, !nonnull !3, !noundef !3
  %7 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %8 = load ptr, ptr %7, align 8, !nonnull !3, !align !5, !noundef !3
; invoke alloc::alloc::box_free
  invoke void @_ZN5alloc5alloc8box_free17ha6c22e5d114b3032E(ptr %6, ptr align 8 %8) #18
          to label %bb2 unwind label %terminate

cleanup:                                          ; preds = %start
  %9 = landingpad { ptr, i32 }
          cleanup
  %10 = extractvalue { ptr, i32 } %9, 0
  %11 = extractvalue { ptr, i32 } %9, 1
  %12 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %10, ptr %12, align 8
  %13 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %11, ptr %13, align 8
  br label %bb4

bb3:                                              ; preds = %start
  %14 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 0
  %15 = load ptr, ptr %14, align 8, !nonnull !3, !noundef !3
  %16 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %17 = load ptr, ptr %16, align 8, !nonnull !3, !align !5, !noundef !3
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17ha6c22e5d114b3032E(ptr %15, ptr align 8 %17)
  ret void

terminate:                                        ; preds = %bb4
  %18 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %19 = extractvalue { ptr, i32 } %18, 0
  %20 = extractvalue { ptr, i32 } %18, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hc3ef110419ba8f94E() #19
  unreachable

bb2:                                              ; preds = %bb4
  %21 = load ptr, ptr %0, align 8, !noundef !3
  %22 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %23 = load i32, ptr %22, align 8, !noundef !3
  %24 = insertvalue { ptr, i32 } poison, ptr %21, 0
  %25 = insertvalue { ptr, i32 } %24, i32 %23, 1
  resume { ptr, i32 } %25
}

; core::alloc::layout::Layout::array::inner
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h3cefc2b12bef8d2dE(i64 %element_size, i64 %align, i64 %n) unnamed_addr #3 {
start:
  %_20 = alloca i64, align 8
  %_15 = alloca i64, align 8
  %_10 = alloca { i64, i64 }, align 8
  %_4 = alloca i8, align 1
  %0 = alloca { i64, i64 }, align 8
  %1 = icmp eq i64 %element_size, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %start
  store i8 0, ptr %_4, align 1
  br label %bb3

bb2:                                              ; preds = %start
  store i64 %align, ptr %_15, align 8
  %_16 = load i64, ptr %_15, align 8, !range !7, !noundef !3
  %_17 = icmp uge i64 %_16, 1
  %_18 = icmp ule i64 %_16, -9223372036854775808
  %_19 = and i1 %_17, %_18
  call void @llvm.assume(i1 %_19)
  %_13 = sub i64 %_16, 1
  %_7 = sub i64 9223372036854775807, %_13
  %_8 = icmp eq i64 %element_size, 0
  %2 = call i1 @llvm.expect.i1(i1 %_8, i1 false)
  br i1 %2, label %panic, label %bb4

bb4:                                              ; preds = %bb2
  %_6 = udiv i64 %_7, %element_size
  %_5 = icmp ugt i64 %n, %_6
  %3 = zext i1 %_5 to i8
  store i8 %3, ptr %_4, align 1
  br label %bb3

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17ha338a74a5d65bf6fE(ptr align 1 @str.2, i64 25, ptr align 8 @alloc_451c02edcc1064a5c2d8a8c409aa417e) #17
  unreachable

bb3:                                              ; preds = %bb1, %bb4
  %4 = load i8, ptr %_4, align 1, !range !8, !noundef !3
  %5 = trunc i8 %4 to i1
  br i1 %5, label %bb5, label %bb6

bb6:                                              ; preds = %bb3
  %array_size = mul i64 %element_size, %n
  store i64 %align, ptr %_20, align 8
  %_21 = load i64, ptr %_20, align 8, !range !7, !noundef !3
  %_22 = icmp uge i64 %_21, 1
  %_23 = icmp ule i64 %_21, -9223372036854775808
  %_24 = and i1 %_22, %_23
  call void @llvm.assume(i1 %_24)
  %6 = getelementptr inbounds { i64, i64 }, ptr %_10, i32 0, i32 1
  store i64 %array_size, ptr %6, align 8
  store i64 %_21, ptr %_10, align 8
  %7 = getelementptr inbounds { i64, i64 }, ptr %_10, i32 0, i32 0
  %8 = load i64, ptr %7, align 8, !range !7, !noundef !3
  %9 = getelementptr inbounds { i64, i64 }, ptr %_10, i32 0, i32 1
  %10 = load i64, ptr %9, align 8, !noundef !3
  %11 = getelementptr inbounds { i64, i64 }, ptr %0, i32 0, i32 0
  store i64 %8, ptr %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  br label %bb7

bb5:                                              ; preds = %bb3
  store i64 0, ptr %0, align 8
  br label %bb7

bb7:                                              ; preds = %bb6, %bb5
  %13 = getelementptr inbounds { i64, i64 }, ptr %0, i32 0, i32 0
  %14 = load i64, ptr %13, align 8, !range !9, !noundef !3
  %15 = getelementptr inbounds { i64, i64 }, ptr %0, i32 0, i32 1
  %16 = load i64, ptr %15, align 8
  %17 = insertvalue { i64, i64 } poison, i64 %14, 0
  %18 = insertvalue { i64, i64 } %17, i64 %16, 1
  ret { i64, i64 } %18
}

; core::result::Result<T,E>::unwrap
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h5006c9174c9c8aaaE"(i64 %0, i64 %1, ptr align 8 %2) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %3 = alloca { ptr, i32 }, align 8
  %e = alloca %"core::alloc::layout::LayoutError", align 1
  %self = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 0
  store i64 %0, ptr %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  store i64 %1, ptr %5, align 8
  %6 = load i64, ptr %self, align 8, !range !9, !noundef !3
  %7 = icmp eq i64 %6, 0
  %_2 = select i1 %7, i64 1, i64 0
  %8 = icmp eq i64 %_2, 0
  br i1 %8, label %bb3, label %bb1

bb3:                                              ; preds = %start
  %9 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 0
  %10 = load i64, ptr %9, align 8, !range !7, !noundef !3
  %11 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  %12 = load i64, ptr %11, align 8, !noundef !3
  %13 = insertvalue { i64, i64 } poison, i64 %10, 0
  %14 = insertvalue { i64, i64 } %13, i64 %12, 1
  ret { i64, i64 } %14

bb1:                                              ; preds = %start
; invoke core::result::unwrap_failed
  invoke void @_ZN4core6result13unwrap_failed17h100c4d67576990cfE(ptr align 1 @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 43, ptr align 1 %e, ptr align 8 @vtable.3, ptr align 8 %2) #17
          to label %unreachable unwind label %cleanup

bb2:                                              ; No predecessors!
  unreachable

bb4:                                              ; preds = %cleanup
  %15 = load ptr, ptr %3, align 8, !noundef !3
  %16 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 0, i32 1
  %17 = load i32, ptr %16, align 8, !noundef !3
  %18 = insertvalue { ptr, i32 } poison, ptr %15, 0
  %19 = insertvalue { ptr, i32 } %18, i32 %17, 1
  resume { ptr, i32 } %19

cleanup:                                          ; preds = %bb1
  %20 = landingpad { ptr, i32 }
          cleanup
  %21 = extractvalue { ptr, i32 } %20, 0
  %22 = extractvalue { ptr, i32 } %20, 1
  %23 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 0, i32 0
  store ptr %21, ptr %23, align 8
  %24 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 0, i32 1
  store i32 %22, ptr %24, align 8
  br label %bb4

unreachable:                                      ; preds = %bb1
  unreachable
}

; <T as core::convert::Into<U>>::into
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h34e96fca8ead8cc8E"(ptr align 1 %self.0, i64 %self.1) unnamed_addr #3 {
start:
; call <alloc::rc::Rc<str> as core::convert::From<&str>>::from
  %0 = call { ptr, i64 } @"_ZN79_$LT$alloc..rc..Rc$LT$str$GT$$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$4from17hcde5ee7babad2917E"(ptr align 1 %self.0, i64 %self.1)
  %1 = extractvalue { ptr, i64 } %0, 0
  %2 = extractvalue { ptr, i64 } %0, 1
  %3 = insertvalue { ptr, i64 } poison, ptr %1, 0
  %4 = insertvalue { ptr, i64 } %3, i64 %2, 1
  ret { ptr, i64 } %4
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hb0ee916be6c8ee5dE"() unnamed_addr #3 {
start:
  ret i8 0
}

; alloc::rc::RcInnerPtr::weak
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @_ZN5alloc2rc10RcInnerPtr4weak17h1dc7eafe6806f926E(ptr align 8 %self.0, i64 %self.1) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds %"alloc::rc::RcBox<str>", ptr %self.0, i32 0, i32 1
  %1 = load i64, ptr %0, align 8, !noundef !3
  ret i64 %1
}

; alloc::rc::RcInnerPtr::strong
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @_ZN5alloc2rc10RcInnerPtr6strong17h3731c4d23f47675eE(ptr align 8 %self.0, i64 %self.1) unnamed_addr #3 {
start:
  %0 = load i64, ptr %self.0, align 8, !noundef !3
  ret i64 %0
}

; alloc::rc::Rc<T>::allocate_for_layout
; Function Attrs: nonlazybind uwtable
define internal { ptr, i64 } @"_ZN5alloc2rc11Rc$LT$T$GT$19allocate_for_layout17h005f3fb698796fbdE"(i64 %value_layout.0, i64 %value_layout.1, ptr align 8 %mem_to_rcbox) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %_11 = alloca i8, align 1
  %_10 = alloca i8, align 1
  %op = alloca ptr, align 8
  %self = alloca %"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>", align 8
  %layout = alloca { i64, i64 }, align 8
  store i8 1, ptr %_11, align 1
  store i8 1, ptr %_10, align 1
; invoke alloc::rc::rcbox_layout_for_value_layout
  %1 = invoke { i64, i64 } @_ZN5alloc2rc29rcbox_layout_for_value_layout17hd45e0b4117ba8398E(i64 %value_layout.0, i64 %value_layout.1)
          to label %bb1 unwind label %cleanup

bb6:                                              ; preds = %cleanup
  %2 = load i8, ptr %_10, align 1, !range !8, !noundef !3
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb5, label %bb3

cleanup:                                          ; preds = %bb8, %bb1, %start
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = extractvalue { ptr, i32 } %4, 0
  %6 = extractvalue { ptr, i32 } %4, 1
  %7 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %5, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %6, ptr %8, align 8
  br label %bb6

bb1:                                              ; preds = %start
  store { i64, i64 } %1, ptr %layout, align 8
  store i8 0, ptr %_11, align 1
  store i8 0, ptr %_10, align 1
; invoke alloc::rc::Rc<T>::try_allocate_for_layout
  invoke void @"_ZN5alloc2rc11Rc$LT$T$GT$23try_allocate_for_layout17hc6b1a6c6dbe033b1E"(ptr sret(%"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>") %self, i64 %value_layout.0, i64 %value_layout.1, ptr align 8 %mem_to_rcbox)
          to label %bb2 unwind label %cleanup

bb2:                                              ; preds = %bb1
  store ptr %layout, ptr %op, align 8
  %_12 = load i64, ptr %self, align 8, !range !10, !noundef !3
  %9 = icmp eq i64 %_12, 0
  br i1 %9, label %bb10, label %bb8

bb10:                                             ; preds = %bb2
  %10 = getelementptr inbounds %"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>::Ok", ptr %self, i32 0, i32 1
  %11 = getelementptr inbounds { ptr, i64 }, ptr %10, i32 0, i32 0
  %12 = load ptr, ptr %11, align 8, !noundef !3
  %13 = getelementptr inbounds { ptr, i64 }, ptr %10, i32 0, i32 1
  %14 = load i64, ptr %13, align 8, !noundef !3
  %15 = insertvalue { ptr, i64 } poison, ptr %12, 0
  %16 = insertvalue { ptr, i64 } %15, i64 %14, 1
  ret { ptr, i64 } %16

bb8:                                              ; preds = %bb2
  %_15 = load ptr, ptr %op, align 8, !nonnull !3, !align !5, !noundef !3
  %17 = getelementptr inbounds { i64, i64 }, ptr %_15, i32 0, i32 0
  %_14.0 = load i64, ptr %17, align 8, !range !7, !noundef !3
  %18 = getelementptr inbounds { i64, i64 }, ptr %_15, i32 0, i32 1
  %_14.1 = load i64, ptr %18, align 8, !noundef !3
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h52397d1f34536addE(i64 %_14.0, i64 %_14.1) #17
          to label %unreachable unwind label %cleanup

bb9:                                              ; No predecessors!
  unreachable

unreachable:                                      ; preds = %bb8
  unreachable

bb3:                                              ; preds = %bb5, %bb6
  %19 = load i8, ptr %_11, align 1, !range !8, !noundef !3
  %20 = trunc i8 %19 to i1
  br i1 %20, label %bb7, label %bb4

bb5:                                              ; preds = %bb6
  br label %bb3

bb4:                                              ; preds = %bb7, %bb3
  %21 = load ptr, ptr %0, align 8, !noundef !3
  %22 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %23 = load i32, ptr %22, align 8, !noundef !3
  %24 = insertvalue { ptr, i32 } poison, ptr %21, 0
  %25 = insertvalue { ptr, i32 } %24, i32 %23, 1
  resume { ptr, i32 } %25

bb7:                                              ; preds = %bb3
  br label %bb4
}

; alloc::rc::Rc<T>::try_allocate_for_layout
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc2rc11Rc$LT$T$GT$23try_allocate_for_layout17hc6b1a6c6dbe033b1E"(ptr sret(%"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>") %0, i64 %value_layout.0, i64 %value_layout.1, ptr align 8 %mem_to_rcbox) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_29 = alloca i8, align 1
  %_28 = alloca i8, align 1
  %self1 = alloca ptr, align 8
  %_14 = alloca ptr, align 8
  %_8 = alloca { i64, i64 }, align 8
  %self = alloca { ptr, i64 }, align 8
  %_5 = alloca { ptr, i64 }, align 8
  store i8 1, ptr %_29, align 1
  store i8 1, ptr %_28, align 1
; invoke alloc::rc::rcbox_layout_for_value_layout
  %2 = invoke { i64, i64 } @_ZN5alloc2rc29rcbox_layout_for_value_layout17hd45e0b4117ba8398E(i64 %value_layout.0, i64 %value_layout.1)
          to label %bb1 unwind label %cleanup

bb13:                                             ; preds = %bb10, %bb11, %cleanup
  %3 = load i8, ptr %_28, align 1, !range !8, !noundef !3
  %4 = trunc i8 %3 to i1
  br i1 %4, label %bb12, label %bb15

cleanup:                                          ; preds = %bb1, %start
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
  %8 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %6, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %7, ptr %9, align 8
  br label %bb13

bb1:                                              ; preds = %start
  %layout.0 = extractvalue { i64, i64 } %2, 0
  %layout.1 = extractvalue { i64, i64 } %2, 1
  store i8 0, ptr %_29, align 1
  %10 = getelementptr inbounds { i64, i64 }, ptr %_8, i32 0, i32 0
  store i64 %layout.0, ptr %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, ptr %_8, i32 0, i32 1
  store i64 %layout.1, ptr %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, ptr %_8, i32 0, i32 0
  %13 = load i64, ptr %12, align 8, !range !7, !noundef !3
  %14 = getelementptr inbounds { i64, i64 }, ptr %_8, i32 0, i32 1
  %15 = load i64, ptr %14, align 8, !noundef !3
; invoke alloc::rc::Rc<[T]>::allocate_for_slice::{{closure}}
  %16 = invoke { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$18allocate_for_slice28_$u7b$$u7b$closure$u7d$$u7d$17h29618d10c1527503E"(i64 %13, i64 %15)
          to label %bb2 unwind label %cleanup

bb2:                                              ; preds = %bb1
  store { ptr, i64 } %16, ptr %self, align 8
  %17 = load ptr, ptr %self, align 8, !noundef !3
  %18 = ptrtoint ptr %17 to i64
  %19 = icmp eq i64 %18, 0
  %_30 = select i1 %19, i64 1, i64 0
  %20 = icmp eq i64 %_30, 0
  br i1 %20, label %bb17, label %bb16

bb17:                                             ; preds = %bb2
  %21 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %v.0 = load ptr, ptr %21, align 8, !nonnull !3, !noundef !3
  %22 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %v.1 = load i64, ptr %22, align 8, !noundef !3
  %23 = getelementptr inbounds { ptr, i64 }, ptr %_5, i32 0, i32 0
  store ptr %v.0, ptr %23, align 8
  %24 = getelementptr inbounds { ptr, i64 }, ptr %_5, i32 0, i32 1
  store i64 %v.1, ptr %24, align 8
  br label %bb3

bb16:                                             ; preds = %bb2
  store ptr null, ptr %_5, align 8
  br label %bb3

bb3:                                              ; preds = %bb17, %bb16
  %25 = load ptr, ptr %_5, align 8, !noundef !3
  %26 = ptrtoint ptr %25 to i64
  %27 = icmp eq i64 %26, 0
  %_10 = select i1 %27, i64 1, i64 0
  %28 = icmp eq i64 %_10, 0
  br i1 %28, label %bb4, label %bb6

bb4:                                              ; preds = %bb3
  %29 = getelementptr inbounds { ptr, i64 }, ptr %_5, i32 0, i32 0
  %ptr.0 = load ptr, ptr %29, align 8, !nonnull !3, !noundef !3
  %30 = getelementptr inbounds { ptr, i64 }, ptr %_5, i32 0, i32 1
  %ptr.1 = load i64, ptr %30, align 8, !noundef !3
  store i8 0, ptr %_28, align 1
  store ptr %ptr.0, ptr %self1, align 8
  %_38 = load ptr, ptr %self1, align 8, !noundef !3
  store ptr %_38, ptr %_14, align 8
  %31 = load ptr, ptr %_14, align 8, !noundef !3
; invoke alloc::rc::Rc<[T]>::allocate_for_slice::{{closure}}
  %32 = invoke { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$18allocate_for_slice28_$u7b$$u7b$closure$u7d$$u7d$17h3e636776719f6d68E"(ptr align 8 %mem_to_rcbox, ptr %31)
          to label %bb7 unwind label %cleanup2

bb6:                                              ; preds = %bb3
  store i64 1, ptr %0, align 8
  br label %bb8

bb5:                                              ; No predecessors!
  unreachable

bb8:                                              ; preds = %bb7, %bb6
  ret void

bb11:                                             ; preds = %cleanup2
  br i1 false, label %bb10, label %bb13

cleanup2:                                         ; preds = %bb4
  %33 = landingpad { ptr, i32 }
          cleanup
  %34 = extractvalue { ptr, i32 } %33, 0
  %35 = extractvalue { ptr, i32 } %33, 1
  %36 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %34, ptr %36, align 8
  %37 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %35, ptr %37, align 8
  br label %bb11

bb7:                                              ; preds = %bb4
  %inner.0 = extractvalue { ptr, i64 } %32, 0
  %inner.1 = extractvalue { ptr, i64 } %32, 1
  store i64 1, ptr %inner.0, align 8
  %_24 = getelementptr inbounds %"alloc::rc::RcBox<[u8]>", ptr %inner.0, i32 0, i32 1
  store i64 1, ptr %_24, align 8
  %38 = getelementptr inbounds %"core::result::Result<*mut alloc::rc::RcBox<[u8]>, core::alloc::AllocError>::Ok", ptr %0, i32 0, i32 1
  %39 = getelementptr inbounds { ptr, i64 }, ptr %38, i32 0, i32 0
  store ptr %inner.0, ptr %39, align 8
  %40 = getelementptr inbounds { ptr, i64 }, ptr %38, i32 0, i32 1
  store i64 %inner.1, ptr %40, align 8
  store i64 0, ptr %0, align 8
  br label %bb8

bb10:                                             ; preds = %bb11
  br label %bb13

bb15:                                             ; preds = %bb12, %bb13
  %41 = load i8, ptr %_29, align 1, !range !8, !noundef !3
  %42 = trunc i8 %41 to i1
  br i1 %42, label %bb14, label %bb9

bb12:                                             ; preds = %bb13
  br label %bb15

bb9:                                              ; preds = %bb14, %bb15
  %43 = load ptr, ptr %1, align 8, !noundef !3
  %44 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %45 = load i32, ptr %44, align 8, !noundef !3
  %46 = insertvalue { ptr, i32 } poison, ptr %43, 0
  %47 = insertvalue { ptr, i32 } %46, i32 %45, 1
  resume { ptr, i32 } %47

bb14:                                             ; preds = %bb15
  br label %bb9
}

; alloc::rc::Rc<T>::from_raw
; Function Attrs: nonlazybind uwtable
define internal { ptr, i64 } @"_ZN5alloc2rc11Rc$LT$T$GT$8from_raw17h8eb43c4f80a3c62bE"(ptr %ptr.0, i64 %ptr.1) unnamed_addr #0 {
start:
  %ptr = alloca { ptr, i64 }, align 8
  %_15 = alloca { ptr, i64 }, align 8
  %_14 = alloca %"core::ptr::metadata::PtrRepr<str>", align 8
  %_13 = alloca %"core::ptr::metadata::PtrRepr<str>", align 8
  %0 = alloca { ptr, i64 }, align 8
; call alloc::rc::data_offset
  %offset = call i64 @_ZN5alloc2rc11data_offset17h48be1f37ca2debc7E(ptr %ptr.0, i64 %ptr.1)
  %count = sub i64 0, %offset
  %self = getelementptr inbounds i8, ptr %ptr.0, i64 %count
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_13, i32 0, i32 0
  store ptr %ptr.0, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_13, i32 0, i32 1
  store i64 %ptr.1, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_13, i32 0, i32 1
  %metadata = load i64, ptr %3, align 8, !noundef !3
  store ptr %self, ptr %_15, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 1
  store i64 %metadata, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 0
  %6 = load ptr, ptr %5, align 8, !noundef !3
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 1
  %8 = load i64, ptr %7, align 8, !noundef !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_14, i32 0, i32 0
  store ptr %6, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_14, i32 0, i32 1
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %_14, i32 0, i32 0
  %_4.0 = load ptr, ptr %11, align 8, !noundef !3
  %12 = getelementptr inbounds { ptr, i64 }, ptr %_14, i32 0, i32 1
  %_4.1 = load i64, ptr %12, align 8, !noundef !3
  %13 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 0
  store ptr %_4.0, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 1
  store i64 %_4.1, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 0
  %16 = load ptr, ptr %15, align 8, !nonnull !3, !noundef !3
  %17 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 1
  %18 = load i64, ptr %17, align 8, !noundef !3
  %19 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %16, ptr %19, align 8
  %20 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %18, ptr %20, align 8
  %21 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  %22 = load ptr, ptr %21, align 8, !nonnull !3, !noundef !3
  %23 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  %24 = load i64, ptr %23, align 8, !noundef !3
  %25 = insertvalue { ptr, i64 } poison, ptr %22, 0
  %26 = insertvalue { ptr, i64 } %25, i64 %24, 1
  ret { ptr, i64 } %26
}

; alloc::rc::data_offset
; Function Attrs: nonlazybind uwtable
define internal i64 @_ZN5alloc2rc11data_offset17h48be1f37ca2debc7E(ptr %ptr.0, i64 %ptr.1) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %layout = alloca { i64, i64 }, align 8
  %1 = mul nsw i64 %ptr.1, 1
  store i64 1, ptr %0, align 8
  %align = load i64, ptr %0, align 8, !noundef !3
  %2 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 16, ptr %2, align 8
  store i64 8, ptr %layout, align 8
  %3 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %_4 = load i64, ptr %3, align 8, !noundef !3
  %4 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %len = load i64, ptr %4, align 8, !noundef !3
  %self = add i64 %len, %align
  %_13 = sub i64 %self, 1
  %_16 = sub i64 %align, 1
  %_15 = xor i64 %_16, -1
  %len_rounded_up = and i64 %_13, %_15
  %_6 = sub i64 %len_rounded_up, %len
  %5 = add i64 %_4, %_6
  ret i64 %5
}

; alloc::rc::Rc<[T]>::copy_from_slice
; Function Attrs: nonlazybind uwtable
define internal { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$15copy_from_slice17hda30a344d394731eE"(ptr align 1 %v.0, i64 %v.1) unnamed_addr #0 {
start:
  %ptr = alloca { ptr, i64 }, align 8
  %0 = alloca { ptr, i64 }, align 8
; call alloc::rc::Rc<[T]>::allocate_for_slice
  %1 = call { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$18allocate_for_slice17he8cb4dfa76fd867bE"(i64 %v.1)
  %ptr.0 = extractvalue { ptr, i64 } %1, 0
  %ptr.1 = extractvalue { ptr, i64 } %1, 1
  %_7.0 = getelementptr inbounds %"alloc::rc::RcBox<[u8]>", ptr %ptr.0, i32 0, i32 2
  %2 = mul i64 %v.1, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_7.0, ptr align 1 %v.0, i64 %2, i1 false)
  %3 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 0
  store ptr %ptr.0, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 1
  store i64 %ptr.1, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 0
  %6 = load ptr, ptr %5, align 8, !nonnull !3, !noundef !3
  %7 = getelementptr inbounds { ptr, i64 }, ptr %ptr, i32 0, i32 1
  %8 = load i64, ptr %7, align 8, !noundef !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %6, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  %12 = load ptr, ptr %11, align 8, !nonnull !3, !noundef !3
  %13 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  %14 = load i64, ptr %13, align 8, !noundef !3
  %15 = insertvalue { ptr, i64 } poison, ptr %12, 0
  %16 = insertvalue { ptr, i64 } %15, i64 %14, 1
  ret { ptr, i64 } %16
}

; alloc::rc::Rc<[T]>::allocate_for_slice
; Function Attrs: nonlazybind uwtable
define internal { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$18allocate_for_slice17he8cb4dfa76fd867bE"(i64 %0) unnamed_addr #0 {
start:
  %_5 = alloca ptr, align 8
  %len = alloca i64, align 8
  store i64 %0, ptr %len, align 8
  %n = load i64, ptr %len, align 8, !noundef !3
; call core::alloc::layout::Layout::array::inner
  %1 = call { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h3cefc2b12bef8d2dE(i64 1, i64 1, i64 %n)
  %_3.0 = extractvalue { i64, i64 } %1, 0
  %_3.1 = extractvalue { i64, i64 } %1, 1
; call core::result::Result<T,E>::unwrap
  %2 = call { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h5006c9174c9c8aaaE"(i64 %_3.0, i64 %_3.1, ptr align 8 @alloc_9cb44305eed26d545f11abe0d9c1d0e8)
  %_2.0 = extractvalue { i64, i64 } %2, 0
  %_2.1 = extractvalue { i64, i64 } %2, 1
  store ptr %len, ptr %_5, align 8
  %3 = load ptr, ptr %_5, align 8, !nonnull !3, !align !5, !noundef !3
; call alloc::rc::Rc<T>::allocate_for_layout
  %4 = call { ptr, i64 } @"_ZN5alloc2rc11Rc$LT$T$GT$19allocate_for_layout17h005f3fb698796fbdE"(i64 %_2.0, i64 %_2.1, ptr align 8 %3)
  %5 = extractvalue { ptr, i64 } %4, 0
  %6 = extractvalue { ptr, i64 } %4, 1
  %7 = insertvalue { ptr, i64 } poison, ptr %5, 0
  %8 = insertvalue { ptr, i64 } %7, i64 %6, 1
  ret { ptr, i64 } %8
}

; alloc::rc::Rc<[T]>::allocate_for_slice::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$18allocate_for_slice28_$u7b$$u7b$closure$u7d$$u7d$17h29618d10c1527503E"(i64 %layout.0, i64 %layout.1) unnamed_addr #3 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h718849990651c999E(ptr align 1 @alloc_38a9d1c1fccd92e612dd2762da060982, i64 %layout.0, i64 %layout.1, i1 zeroext false)
  %1 = extractvalue { ptr, i64 } %0, 0
  %2 = extractvalue { ptr, i64 } %0, 1
  %3 = insertvalue { ptr, i64 } poison, ptr %1, 0
  %4 = insertvalue { ptr, i64 } %3, i64 %2, 1
  ret { ptr, i64 } %4
}

; alloc::rc::Rc<[T]>::allocate_for_slice::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$18allocate_for_slice28_$u7b$$u7b$closure$u7d$$u7d$17h3e636776719f6d68E"(ptr align 8 %0, ptr %mem) unnamed_addr #3 {
start:
  %_9 = alloca { ptr, i64 }, align 8
  %_8 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
  %_6 = load ptr, ptr %_1, align 8, !nonnull !3, !align !5, !noundef !3
  %len = load i64, ptr %_6, align 8, !noundef !3
  store ptr %mem, ptr %_9, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_9, i32 0, i32 1
  store i64 %len, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_9, i32 0, i32 0
  %3 = load ptr, ptr %2, align 8, !noundef !3
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_9, i32 0, i32 1
  %5 = load i64, ptr %4, align 8, !noundef !3
  %6 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 0
  store ptr %3, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 1
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 0
  %_3.0 = load ptr, ptr %8, align 8, !noundef !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 1
  %_3.1 = load i64, ptr %9, align 8, !noundef !3
  %10 = insertvalue { ptr, i64 } poison, ptr %_3.0, 0
  %11 = insertvalue { ptr, i64 } %10, i64 %_3.1, 1
  ret { ptr, i64 } %11
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @_ZN5alloc5alloc15exchange_malloc17hc234fb776788c6c3E(i64 %size, i64 %align) unnamed_addr #3 {
start:
  %self = alloca ptr, align 8
  %_4 = alloca { ptr, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  %0 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %size, ptr %0, align 8
  store i64 %align, ptr %layout, align 8
  %1 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %2 = load i64, ptr %1, align 8, !range !7, !noundef !3
  %3 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %4 = load i64, ptr %3, align 8, !noundef !3
; call alloc::alloc::Global::alloc_impl
  %5 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h718849990651c999E(ptr align 1 @alloc_38a9d1c1fccd92e612dd2762da060982, i64 %2, i64 %4, i1 zeroext false)
  store { ptr, i64 } %5, ptr %_4, align 8
  %6 = load ptr, ptr %_4, align 8, !noundef !3
  %7 = ptrtoint ptr %6 to i64
  %8 = icmp eq i64 %7, 0
  %_5 = select i1 %8, i64 1, i64 0
  %9 = icmp eq i64 %_5, 0
  br i1 %9, label %bb3, label %bb1

bb3:                                              ; preds = %start
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_4, i32 0, i32 0
  %ptr.0 = load ptr, ptr %10, align 8, !nonnull !3, !noundef !3
  %11 = getelementptr inbounds { ptr, i64 }, ptr %_4, i32 0, i32 1
  %ptr.1 = load i64, ptr %11, align 8, !noundef !3
  store ptr %ptr.0, ptr %self, align 8
  %_18 = load ptr, ptr %self, align 8, !noundef !3
  ret ptr %_18

bb1:                                              ; preds = %start
  %12 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %13 = load i64, ptr %12, align 8, !range !7, !noundef !3
  %14 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %15 = load i64, ptr %14, align 8, !noundef !3
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h52397d1f34536addE(i64 %13, i64 %15) #17
  unreachable

bb2:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h718849990651c999E(ptr align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #3 {
start:
  %2 = alloca i8, align 1
  %_85 = alloca { ptr, i64 }, align 8
  %_84 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %_69 = alloca ptr, align 8
  %_68 = alloca ptr, align 8
  %_61 = alloca i64, align 8
  %_46 = alloca i64, align 8
  %_36 = alloca { ptr, i64 }, align 8
  %_35 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %_22 = alloca i64, align 8
  %_18 = alloca { ptr, i64 }, align 8
  %self4 = alloca ptr, align 8
  %self3 = alloca ptr, align 8
  %_12 = alloca ptr, align 8
  %layout2 = alloca { i64, i64 }, align 8
  %layout1 = alloca { i64, i64 }, align 8
  %raw_ptr = alloca ptr, align 8
  %data = alloca ptr, align 8
  %_6 = alloca { ptr, i64 }, align 8
  %3 = alloca { ptr, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  store i64 %0, ptr %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %1, ptr %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %size = load i64, ptr %6, align 8, !noundef !3
  %7 = icmp eq i64 %size, 0
  br i1 %7, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %self10 = load i64, ptr %layout, align 8, !range !7, !noundef !3
  store i64 %self10, ptr %_22, align 8
  %_23 = load i64, ptr %_22, align 8, !range !7, !noundef !3
  %_24 = icmp uge i64 %_23, 1
  %_25 = icmp ule i64 %_23, -9223372036854775808
  %_26 = and i1 %_24, %_25
  call void @llvm.assume(i1 %_26)
  %ptr11 = inttoptr i64 %_23 to ptr
  store ptr %ptr11, ptr %data, align 8
  %_33 = load ptr, ptr %data, align 8, !noundef !3
  store ptr %_33, ptr %_36, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %_36, i32 0, i32 1
  store i64 0, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_36, i32 0, i32 0
  %10 = load ptr, ptr %9, align 8, !noundef !3
  %11 = getelementptr inbounds { ptr, i64 }, ptr %_36, i32 0, i32 1
  %12 = load i64, ptr %11, align 8, !noundef !3
  %13 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 0
  store ptr %10, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 1
  store i64 %12, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 0
  %ptr.012 = load ptr, ptr %15, align 8, !noundef !3
  %16 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 1
  %ptr.113 = load i64, ptr %16, align 8, !noundef !3
  %17 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 0
  store ptr %ptr.012, ptr %17, align 8
  %18 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 1
  store i64 %ptr.113, ptr %18, align 8
  %19 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 0
  %20 = load ptr, ptr %19, align 8, !nonnull !3, !noundef !3
  %21 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 1
  %22 = load i64, ptr %21, align 8, !noundef !3
  %23 = getelementptr inbounds { ptr, i64 }, ptr %3, i32 0, i32 0
  store ptr %20, ptr %23, align 8
  %24 = getelementptr inbounds { ptr, i64 }, ptr %3, i32 0, i32 1
  store i64 %22, ptr %24, align 8
  br label %bb10

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb4:                                              ; preds = %bb1
  %25 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %26 = load i64, ptr %25, align 8, !range !7, !noundef !3
  %27 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %28 = load i64, ptr %27, align 8, !noundef !3
  %29 = getelementptr inbounds { i64, i64 }, ptr %layout2, i32 0, i32 0
  store i64 %26, ptr %29, align 8
  %30 = getelementptr inbounds { i64, i64 }, ptr %layout2, i32 0, i32 1
  store i64 %28, ptr %30, align 8
  %31 = load volatile i8, ptr @__rust_no_alloc_shim_is_unstable, align 1
  store i8 %31, ptr %2, align 1
  %_51 = load i8, ptr %2, align 1, !noundef !3
  %32 = getelementptr inbounds { i64, i64 }, ptr %layout2, i32 0, i32 1
  %_55 = load i64, ptr %32, align 8, !noundef !3
  %self6 = load i64, ptr %layout2, align 8, !range !7, !noundef !3
  store i64 %self6, ptr %_61, align 8
  %_62 = load i64, ptr %_61, align 8, !range !7, !noundef !3
  %_63 = icmp uge i64 %_62, 1
  %_64 = icmp ule i64 %_62, -9223372036854775808
  %_65 = and i1 %_63, %_64
  call void @llvm.assume(i1 %_65)
  %33 = call ptr @__rust_alloc(i64 %_55, i64 %_62) #20
  store ptr %33, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
  %34 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %35 = load i64, ptr %34, align 8, !range !7, !noundef !3
  %36 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %37 = load i64, ptr %36, align 8, !noundef !3
  %38 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 0
  store i64 %35, ptr %38, align 8
  %39 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  store i64 %37, ptr %39, align 8
  %40 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  %_41 = load i64, ptr %40, align 8, !noundef !3
  %self5 = load i64, ptr %layout1, align 8, !range !7, !noundef !3
  store i64 %self5, ptr %_46, align 8
  %_47 = load i64, ptr %_46, align 8, !range !7, !noundef !3
  %_48 = icmp uge i64 %_47, 1
  %_49 = icmp ule i64 %_47, -9223372036854775808
  %_50 = and i1 %_48, %_49
  call void @llvm.assume(i1 %_50)
  %41 = call ptr @__rust_alloc_zeroed(i64 %_41, i64 %_47) #20
  store ptr %41, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  %ptr = load ptr, ptr %raw_ptr, align 8, !noundef !3
  store ptr %ptr, ptr %_69, align 8
  %ptr7 = load ptr, ptr %_69, align 8, !noundef !3
  %_71 = ptrtoint ptr %ptr7 to i64
  %_67 = icmp eq i64 %_71, 0
  %_66 = xor i1 %_67, true
  br i1 %_66, label %bb14, label %bb15

bb15:                                             ; preds = %bb5
  store ptr null, ptr %self4, align 8
  br label %bb16

bb14:                                             ; preds = %bb5
  store ptr %ptr, ptr %_68, align 8
  %42 = load ptr, ptr %_68, align 8, !nonnull !3, !noundef !3
  store ptr %42, ptr %self4, align 8
  br label %bb16

bb16:                                             ; preds = %bb15, %bb14
  %43 = load ptr, ptr %self4, align 8, !noundef !3
  %44 = ptrtoint ptr %43 to i64
  %45 = icmp eq i64 %44, 0
  %_76 = select i1 %45, i64 0, i64 1
  %46 = icmp eq i64 %_76, 0
  br i1 %46, label %bb17, label %bb18

bb17:                                             ; preds = %bb16
  store ptr null, ptr %self3, align 8
  br label %bb19

bb18:                                             ; preds = %bb16
  %v = load ptr, ptr %self4, align 8, !nonnull !3, !noundef !3
  store ptr %v, ptr %self3, align 8
  br label %bb19

bb19:                                             ; preds = %bb17, %bb18
  %47 = load ptr, ptr %self3, align 8, !noundef !3
  %48 = ptrtoint ptr %47 to i64
  %49 = icmp eq i64 %48, 0
  %_78 = select i1 %49, i64 1, i64 0
  %50 = icmp eq i64 %_78, 0
  br i1 %50, label %bb21, label %bb20

bb21:                                             ; preds = %bb19
  %v8 = load ptr, ptr %self3, align 8, !nonnull !3, !noundef !3
  store ptr %v8, ptr %_12, align 8
  br label %bb6

bb20:                                             ; preds = %bb19
  store ptr null, ptr %_12, align 8
  br label %bb6

bb6:                                              ; preds = %bb21, %bb20
  %51 = load ptr, ptr %_12, align 8, !noundef !3
  %52 = ptrtoint ptr %51 to i64
  %53 = icmp eq i64 %52, 0
  %_16 = select i1 %53, i64 1, i64 0
  %54 = icmp eq i64 %_16, 0
  br i1 %54, label %bb7, label %bb9

bb7:                                              ; preds = %bb6
  %ptr9 = load ptr, ptr %_12, align 8, !nonnull !3, !noundef !3
  store ptr %ptr9, ptr %_85, align 8
  %55 = getelementptr inbounds { ptr, i64 }, ptr %_85, i32 0, i32 1
  store i64 %size, ptr %55, align 8
  %56 = getelementptr inbounds { ptr, i64 }, ptr %_85, i32 0, i32 0
  %57 = load ptr, ptr %56, align 8, !noundef !3
  %58 = getelementptr inbounds { ptr, i64 }, ptr %_85, i32 0, i32 1
  %59 = load i64, ptr %58, align 8, !noundef !3
  %60 = getelementptr inbounds { ptr, i64 }, ptr %_84, i32 0, i32 0
  store ptr %57, ptr %60, align 8
  %61 = getelementptr inbounds { ptr, i64 }, ptr %_84, i32 0, i32 1
  store i64 %59, ptr %61, align 8
  %62 = getelementptr inbounds { ptr, i64 }, ptr %_84, i32 0, i32 0
  %ptr.0 = load ptr, ptr %62, align 8, !noundef !3
  %63 = getelementptr inbounds { ptr, i64 }, ptr %_84, i32 0, i32 1
  %ptr.1 = load i64, ptr %63, align 8, !noundef !3
  %64 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 0
  store ptr %ptr.0, ptr %64, align 8
  %65 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 1
  store i64 %ptr.1, ptr %65, align 8
  %66 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 0
  %67 = load ptr, ptr %66, align 8, !nonnull !3, !noundef !3
  %68 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 1
  %69 = load i64, ptr %68, align 8, !noundef !3
  %70 = getelementptr inbounds { ptr, i64 }, ptr %3, i32 0, i32 0
  store ptr %67, ptr %70, align 8
  %71 = getelementptr inbounds { ptr, i64 }, ptr %3, i32 0, i32 1
  store i64 %69, ptr %71, align 8
  br label %bb10

bb9:                                              ; preds = %bb6
  store ptr null, ptr %3, align 8
  br label %bb10

bb8:                                              ; No predecessors!
  unreachable

bb10:                                             ; preds = %bb2, %bb7, %bb9
  %72 = getelementptr inbounds { ptr, i64 }, ptr %3, i32 0, i32 0
  %73 = load ptr, ptr %72, align 8, !noundef !3
  %74 = getelementptr inbounds { ptr, i64 }, ptr %3, i32 0, i32 1
  %75 = load i64, ptr %74, align 8
  %76 = insertvalue { ptr, i64 } poison, ptr %73, 0
  %77 = insertvalue { ptr, i64 } %76, i64 %75, 1
  ret { ptr, i64 } %77
}

; alloc::alloc::box_free
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN5alloc5alloc8box_free17ha6c22e5d114b3032E(ptr %0, ptr align 8 %1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %2 = alloca { ptr, i32 }, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %_38 = alloca ptr, align 8
  %self = alloca ptr, align 8
  %unique = alloca ptr, align 8
  %_15 = alloca ptr, align 8
  %layout = alloca { i64, i64 }, align 8
  %alloc = alloca %"alloc::alloc::Global", align 1
  %ptr = alloca { ptr, ptr }, align 8
  %5 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  store ptr %0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  store ptr %1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  %self.0 = load ptr, ptr %7, align 8, !nonnull !3, !noundef !3
  %8 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  %self.1 = load ptr, ptr %8, align 8, !nonnull !3, !align !5, !noundef !3
  %9 = getelementptr inbounds i64, ptr %self.1, i64 1
  %10 = load i64, ptr %9, align 8, !range !11, !invariant.load !3
  %11 = getelementptr inbounds i64, ptr %self.1, i64 2
  %12 = load i64, ptr %11, align 8, !range !12, !invariant.load !3
  store i64 %10, ptr %4, align 8
  %size = load i64, ptr %4, align 8, !noundef !3
  %13 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  %self.01 = load ptr, ptr %13, align 8, !nonnull !3, !noundef !3
  %14 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  %self.12 = load ptr, ptr %14, align 8, !nonnull !3, !align !5, !noundef !3
  %15 = getelementptr inbounds i64, ptr %self.12, i64 1
  %16 = load i64, ptr %15, align 8, !range !11, !invariant.load !3
  %17 = getelementptr inbounds i64, ptr %self.12, i64 2
  %18 = load i64, ptr %17, align 8, !range !12, !invariant.load !3
  store i64 %18, ptr %3, align 8
  %align = load i64, ptr %3, align 8, !noundef !3
  %19 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %size, ptr %19, align 8
  store i64 %align, ptr %layout, align 8
  %20 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  %self.03 = load ptr, ptr %20, align 8, !nonnull !3, !noundef !3
  %21 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  %self.14 = load ptr, ptr %21, align 8, !nonnull !3, !align !5, !noundef !3
  store ptr %self.03, ptr %self, align 8
  %_37 = load ptr, ptr %self, align 8, !noundef !3
  store ptr %_37, ptr %_38, align 8
  %22 = load ptr, ptr %_38, align 8, !nonnull !3, !noundef !3
  store ptr %22, ptr %unique, align 8
  %self5 = load ptr, ptr %unique, align 8, !nonnull !3, !noundef !3
  store ptr %self5, ptr %_15, align 8
  %23 = load ptr, ptr %_15, align 8, !nonnull !3, !noundef !3
  %24 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %25 = load i64, ptr %24, align 8, !range !7, !noundef !3
  %26 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %27 = load i64, ptr %26, align 8, !noundef !3
; invoke <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  invoke void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h8daa38771d2b5acfE"(ptr align 1 %alloc, ptr %23, i64 %25, i64 %27)
          to label %bb3 unwind label %cleanup

bb5:                                              ; preds = %cleanup
  %28 = load ptr, ptr %2, align 8, !noundef !3
  %29 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  %30 = load i32, ptr %29, align 8, !noundef !3
  %31 = insertvalue { ptr, i32 } poison, ptr %28, 0
  %32 = insertvalue { ptr, i32 } %31, i32 %30, 1
  resume { ptr, i32 } %32

cleanup:                                          ; preds = %start
  %33 = landingpad { ptr, i32 }
          cleanup
  %34 = extractvalue { ptr, i32 } %33, 0
  %35 = extractvalue { ptr, i32 } %33, 1
  %36 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 0
  store ptr %34, ptr %36, align 8
  %37 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  store i32 %35, ptr %37, align 8
  br label %bb5

bb3:                                              ; preds = %start
  ret void
}

; alloc::boxed::Box<T,A>::into_raw
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, ptr } @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$8into_raw17h3581f31d74b6dbe5E"(ptr align 1 %b.0, ptr align 8 %b.1) unnamed_addr #3 {
start:
  %pointer = alloca { ptr, ptr }, align 8
  %_19 = alloca { ptr, ptr }, align 8
  %_11 = alloca { ptr, ptr }, align 8
  %b = alloca { ptr, ptr }, align 8
  %_5 = alloca { ptr, ptr }, align 8
  %_2 = alloca { ptr, ptr }, align 8
  %0 = getelementptr inbounds { ptr, ptr }, ptr %b, i32 0, i32 0
  store ptr %b.0, ptr %0, align 8
  %1 = getelementptr inbounds { ptr, ptr }, ptr %b, i32 0, i32 1
  store ptr %b.1, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, ptr }, ptr %b, i32 0, i32 0
  %b.01 = load ptr, ptr %2, align 8, !nonnull !3, !align !6, !noundef !3
  %3 = getelementptr inbounds { ptr, ptr }, ptr %b, i32 0, i32 1
  %b.12 = load ptr, ptr %3, align 8, !nonnull !3, !align !5, !noundef !3
  %4 = getelementptr inbounds { ptr, ptr }, ptr %_19, i32 0, i32 0
  store ptr %b.01, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, ptr }, ptr %_19, i32 0, i32 1
  store ptr %b.12, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, ptr }, ptr %_19, i32 0, i32 0
  %self.0 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3
  %7 = getelementptr inbounds { ptr, ptr }, ptr %_19, i32 0, i32 1
  %self.1 = load ptr, ptr %7, align 8, !nonnull !3, !align !5, !noundef !3
  %8 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 0
  store ptr %self.0, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 1
  store ptr %self.1, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 0
  %11 = load ptr, ptr %10, align 8, !nonnull !3, !noundef !3
  %12 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 1
  %13 = load ptr, ptr %12, align 8, !nonnull !3, !align !5, !noundef !3
  %14 = getelementptr inbounds { ptr, ptr }, ptr %_11, i32 0, i32 0
  store ptr %11, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, ptr }, ptr %_11, i32 0, i32 1
  store ptr %13, ptr %15, align 8
  %16 = getelementptr inbounds { ptr, ptr }, ptr %_11, i32 0, i32 0
  %17 = load ptr, ptr %16, align 8, !nonnull !3, !noundef !3
  %18 = getelementptr inbounds { ptr, ptr }, ptr %_11, i32 0, i32 1
  %19 = load ptr, ptr %18, align 8, !nonnull !3, !align !5, !noundef !3
  %20 = getelementptr inbounds { ptr, ptr }, ptr %_5, i32 0, i32 0
  store ptr %17, ptr %20, align 8
  %21 = getelementptr inbounds { ptr, ptr }, ptr %_5, i32 0, i32 1
  store ptr %19, ptr %21, align 8
  %22 = getelementptr inbounds { ptr, ptr }, ptr %_5, i32 0, i32 0
  %leaked.0 = load ptr, ptr %22, align 8, !nonnull !3, !noundef !3
  %23 = getelementptr inbounds { ptr, ptr }, ptr %_5, i32 0, i32 1
  %leaked.1 = load ptr, ptr %23, align 8, !nonnull !3, !align !5, !noundef !3
  %24 = getelementptr inbounds { ptr, ptr }, ptr %_2, i32 0, i32 0
  store ptr %leaked.0, ptr %24, align 8
  %25 = getelementptr inbounds { ptr, ptr }, ptr %_2, i32 0, i32 1
  store ptr %leaked.1, ptr %25, align 8
  %26 = getelementptr inbounds { ptr, ptr }, ptr %_2, i32 0, i32 0
  %27 = load ptr, ptr %26, align 8, !noundef !3
  %28 = getelementptr inbounds { ptr, ptr }, ptr %_2, i32 0, i32 1
  %29 = load ptr, ptr %28, align 8, !nonnull !3, !align !5, !noundef !3
  %30 = insertvalue { ptr, ptr } poison, ptr %27, 0
  %31 = insertvalue { ptr, ptr } %30, ptr %29, 1
  ret { ptr, ptr } %31
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h8daa38771d2b5acfE"(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1) unnamed_addr #3 {
start:
  %_14 = alloca i64, align 8
  %layout1 = alloca { i64, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  store i64 %0, ptr %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %1, ptr %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %_4 = load i64, ptr %4, align 8, !noundef !3
  %5 = icmp eq i64 %_4, 0
  br i1 %5, label %bb2, label %bb1

bb2:                                              ; preds = %start
  br label %bb3

bb1:                                              ; preds = %start
  %6 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %7 = load i64, ptr %6, align 8, !range !7, !noundef !3
  %8 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %9 = load i64, ptr %8, align 8, !noundef !3
  %10 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 0
  store i64 %7, ptr %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  store i64 %9, ptr %11, align 8
  %12 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  %_9 = load i64, ptr %12, align 8, !noundef !3
  %self2 = load i64, ptr %layout1, align 8, !range !7, !noundef !3
  store i64 %self2, ptr %_14, align 8
  %_15 = load i64, ptr %_14, align 8, !range !7, !noundef !3
  %_16 = icmp uge i64 %_15, 1
  %_17 = icmp ule i64 %_15, -9223372036854775808
  %_18 = and i1 %_16, %_17
  call void @llvm.assume(i1 %_18)
  call void @__rust_dealloc(ptr %ptr, i64 %_9, i64 %_15) #20
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  ret void
}

; <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN64_$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he710579c148bcdaaE"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  %_88 = alloca { i64, i64 }, align 8
  %_18 = alloca { i64, i64 }, align 8
  %_16 = alloca ptr, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.0 = load ptr, ptr %2, align 8, !nonnull !3, !noundef !3
  %3 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.1 = load i64, ptr %3, align 8, !noundef !3
; call alloc::rc::RcInnerPtr::strong
  %_28 = call i64 @_ZN5alloc2rc10RcInnerPtr6strong17h3731c4d23f47675eE(ptr align 8 %self.0, i64 %self.1)
  %val = sub i64 %_28, 1
  store i64 %val, ptr %self.0, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.01 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
  %5 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.12 = load i64, ptr %5, align 8, !noundef !3
  %_4 = load i64, ptr %self.01, align 8, !noundef !3
  %6 = icmp eq i64 %_4, 0
  br i1 %6, label %bb1, label %bb7

bb1:                                              ; preds = %start
  %7 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.03 = load ptr, ptr %7, align 8, !nonnull !3, !noundef !3
  %8 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.14 = load i64, ptr %8, align 8, !noundef !3
  %_9.0 = getelementptr inbounds %"alloc::rc::RcBox<str>", ptr %self.03, i32 0, i32 2
  %9 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.05 = load ptr, ptr %9, align 8, !nonnull !3, !noundef !3
  %10 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.16 = load i64, ptr %10, align 8, !noundef !3
  %self7 = getelementptr inbounds %"alloc::rc::RcBox<str>", ptr %self.05, i32 0, i32 1
; call alloc::rc::RcInnerPtr::weak
  %_57 = call i64 @_ZN5alloc2rc10RcInnerPtr4weak17h1dc7eafe6806f926E(ptr align 8 %self.05, i64 %self.16)
  %val8 = sub i64 %_57, 1
  store i64 %val8, ptr %self7, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.09 = load ptr, ptr %11, align 8, !nonnull !3, !noundef !3
  %12 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.110 = load i64, ptr %12, align 8, !noundef !3
  %self11 = getelementptr inbounds %"alloc::rc::RcBox<str>", ptr %self.09, i32 0, i32 1
  %_12 = load i64, ptr %self11, align 8, !noundef !3
  %13 = icmp eq i64 %_12, 0
  br i1 %13, label %bb3, label %bb5

bb7:                                              ; preds = %start
  br label %bb8

bb8:                                              ; preds = %bb6, %bb7
  ret void

bb3:                                              ; preds = %bb1
  %14 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.012 = load ptr, ptr %14, align 8, !nonnull !3, !noundef !3
  %15 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.113 = load i64, ptr %15, align 8, !noundef !3
  store ptr %self.012, ptr %_16, align 8
  %16 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %self.014 = load ptr, ptr %16, align 8, !nonnull !3, !noundef !3
  %17 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %self.115 = load i64, ptr %17, align 8, !noundef !3
  %18 = mul nsw i64 %self.115, 1
  %19 = add i64 16, %18
  %20 = add i64 %19, 7
  %21 = and i64 %20, -8
  store i64 %21, ptr %1, align 8
  %_89 = load i64, ptr %1, align 8, !noundef !3
  %22 = mul nsw i64 %self.115, 1
  %23 = add i64 16, %22
  %24 = add i64 %23, 7
  %25 = and i64 %24, -8
  store i64 8, ptr %0, align 8
  %_90 = load i64, ptr %0, align 8, !noundef !3
  store i64 %_89, ptr %_88, align 8
  %26 = getelementptr inbounds { i64, i64 }, ptr %_88, i32 0, i32 1
  store i64 %_90, ptr %26, align 8
  %size = load i64, ptr %_88, align 8, !noundef !3
  %27 = getelementptr inbounds { i64, i64 }, ptr %_88, i32 0, i32 1
  %align = load i64, ptr %27, align 8, !noundef !3
  %28 = getelementptr inbounds { i64, i64 }, ptr %_18, i32 0, i32 1
  store i64 %size, ptr %28, align 8
  store i64 %align, ptr %_18, align 8
  %29 = load ptr, ptr %_16, align 8, !nonnull !3, !noundef !3
  %30 = getelementptr inbounds { i64, i64 }, ptr %_18, i32 0, i32 0
  %31 = load i64, ptr %30, align 8, !range !7, !noundef !3
  %32 = getelementptr inbounds { i64, i64 }, ptr %_18, i32 0, i32 1
  %33 = load i64, ptr %32, align 8, !noundef !3
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h8daa38771d2b5acfE"(ptr align 1 @alloc_38a9d1c1fccd92e612dd2762da060982, ptr %29, i64 %31, i64 %33)
  br label %bb6

bb5:                                              ; preds = %bb1
  br label %bb6

bb6:                                              ; preds = %bb3, %bb5
  br label %bb8
}

; <alloc::rc::Rc<str> as core::convert::From<&str>>::from
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN79_$LT$alloc..rc..Rc$LT$str$GT$$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$4from17hcde5ee7babad2917E"(ptr align 1 %v.0, i64 %v.1) unnamed_addr #3 {
start:
  %this = alloca { ptr, i64 }, align 8
; call alloc::rc::Rc<[T]>::copy_from_slice
  %0 = call { ptr, i64 } @"_ZN5alloc2rc21Rc$LT$$u5b$T$u5d$$GT$15copy_from_slice17hda30a344d394731eE"(ptr align 1 %v.0, i64 %v.1)
  %rc.0 = extractvalue { ptr, i64 } %0, 0
  %rc.1 = extractvalue { ptr, i64 } %0, 1
  %1 = getelementptr inbounds { ptr, i64 }, ptr %this, i32 0, i32 0
  store ptr %rc.0, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %this, i32 0, i32 1
  store i64 %rc.1, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %this, i32 0, i32 0
  %self.0 = load ptr, ptr %3, align 8, !nonnull !3, !noundef !3
  %4 = getelementptr inbounds { ptr, i64 }, ptr %this, i32 0, i32 1
  %self.1 = load i64, ptr %4, align 8, !noundef !3
  %_11.0 = getelementptr inbounds %"alloc::rc::RcBox<[u8]>", ptr %self.0, i32 0, i32 2
; call alloc::rc::Rc<T>::from_raw
  %5 = call { ptr, i64 } @"_ZN5alloc2rc11Rc$LT$T$GT$8from_raw17h8eb43c4f80a3c62bE"(ptr %_11.0, i64 %self.1)
  %6 = extractvalue { ptr, i64 } %5, 0
  %7 = extractvalue { ptr, i64 } %5, 1
  %8 = insertvalue { ptr, i64 } poison, ptr %6, 0
  %9 = insertvalue { ptr, i64 } %8, i64 %7, 1
  ret { ptr, i64 } %9
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: nonlazybind uwtable
define internal { ptr, ptr } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17h1f27caaa03aef3a8E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = load ptr, ptr %self, align 8, !noundef !3
  %1 = ptrtoint ptr %0 to i64
  %2 = icmp eq i64 %1, 0
  %_2 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_2, 0
  br i1 %3, label %bb1, label %bb3

bb1:                                              ; preds = %start
; call std::process::abort
  call void @_ZN3std7process5abort17h6c0b0d8d16512c6cE() #17
  unreachable

bb3:                                              ; preds = %start
  %4 = insertvalue { ptr, ptr } poison, ptr %self, 0
  %5 = insertvalue { ptr, ptr } %4, ptr @vtable.4, 1
  ret { ptr, ptr } %5

bb2:                                              ; No predecessors!
  unreachable
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::take_box
; Function Attrs: nonlazybind uwtable
define internal { ptr, ptr } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h451641809f4520c9E"(ptr align 8 %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %src = alloca { ptr, i64 }, align 8
  %result = alloca { ptr, i64 }, align 8
  %data = alloca { ptr, ptr }, align 8
  store ptr null, ptr %src, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %2 = load ptr, ptr %1, align 8, !align !6, !noundef !3
  %3 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %4 = load i64, ptr %3, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 0
  store ptr %2, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 1
  store i64 %4, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %src, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !6, !noundef !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %src, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = load ptr, ptr %result, align 8, !noundef !3
  %14 = ptrtoint ptr %13 to i64
  %15 = icmp eq i64 %14, 0
  %_5 = select i1 %15, i64 0, i64 1
  %16 = icmp eq i64 %_5, 0
  br i1 %16, label %bb1, label %bb3

bb1:                                              ; preds = %start
; call std::process::abort
  call void @_ZN3std7process5abort17h6c0b0d8d16512c6cE() #17
  unreachable

bb3:                                              ; preds = %start
  %17 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 0
  %a.0 = load ptr, ptr %17, align 8, !nonnull !3, !align !6, !noundef !3
  %18 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 1
  %a.1 = load i64, ptr %18, align 8, !noundef !3
; invoke alloc::alloc::exchange_malloc
  %_21 = invoke ptr @_ZN5alloc5alloc15exchange_malloc17hc234fb776788c6c3E(i64 16, i64 8)
          to label %bb8 unwind label %cleanup

bb2:                                              ; No predecessors!
  unreachable

bb9:                                              ; preds = %cleanup
  br label %bb6

cleanup:                                          ; preds = %bb3
  %19 = landingpad { ptr, i32 }
          cleanup
  %20 = extractvalue { ptr, i32 } %19, 0
  %21 = extractvalue { ptr, i32 } %19, 1
  %22 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %20, ptr %22, align 8
  %23 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %21, ptr %23, align 8
  br label %bb9

bb8:                                              ; preds = %bb3
  %24 = getelementptr inbounds { ptr, i64 }, ptr %_21, i32 0, i32 0
  store ptr %a.0, ptr %24, align 8
  %25 = getelementptr inbounds { ptr, i64 }, ptr %_21, i32 0, i32 1
  store i64 %a.1, ptr %25, align 8
  %26 = getelementptr inbounds { ptr, ptr }, ptr %data, i32 0, i32 0
  store ptr %_21, ptr %26, align 8
  %27 = getelementptr inbounds { ptr, ptr }, ptr %data, i32 0, i32 1
  store ptr @vtable.4, ptr %27, align 8
  %28 = getelementptr inbounds { ptr, ptr }, ptr %data, i32 0, i32 0
  %_12.0 = load ptr, ptr %28, align 8, !nonnull !3, !align !6, !noundef !3
  %29 = getelementptr inbounds { ptr, ptr }, ptr %data, i32 0, i32 1
  %_12.1 = load ptr, ptr %29, align 8, !nonnull !3, !align !5, !noundef !3
; invoke alloc::boxed::Box<T,A>::into_raw
  %30 = invoke { ptr, ptr } @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$8into_raw17h3581f31d74b6dbe5E"(ptr align 1 %_12.0, ptr align 8 %_12.1)
          to label %bb4 unwind label %cleanup1

bb6:                                              ; preds = %bb7, %bb5, %bb9
  %31 = load ptr, ptr %0, align 8, !noundef !3
  %32 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %33 = load i32, ptr %32, align 8, !noundef !3
  %34 = insertvalue { ptr, i32 } poison, ptr %31, 0
  %35 = insertvalue { ptr, i32 } %34, i32 %33, 1
  resume { ptr, i32 } %35

bb5:                                              ; preds = %cleanup1
  br i1 false, label %bb7, label %bb6

cleanup1:                                         ; preds = %bb8
  %36 = landingpad { ptr, i32 }
          cleanup
  %37 = extractvalue { ptr, i32 } %36, 0
  %38 = extractvalue { ptr, i32 } %36, 1
  %39 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %37, ptr %39, align 8
  %40 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %38, ptr %40, align 8
  br label %bb5

bb4:                                              ; preds = %bb8
  %_11.0 = extractvalue { ptr, ptr } %30, 0
  %_11.1 = extractvalue { ptr, ptr } %30, 1
  %41 = insertvalue { ptr, ptr } poison, ptr %_11.0, 0
  %42 = insertvalue { ptr, ptr } %41, ptr %_11.1, 1
  ret { ptr, ptr } %42

bb7:                                              ; preds = %bb5
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn core::any::Any+core::marker::Send>>
  invoke void @"_ZN4core3ptr91drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$$GT$17h9e5aa66c425eb603E"(ptr align 8 %data) #18
          to label %bb6 unwind label %terminate

terminate:                                        ; preds = %bb7
  %43 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %44 = extractvalue { ptr, i32 } %43, 0
  %45 = extractvalue { ptr, i32 } %43, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hc3ef110419ba8f94E() #19
  unreachable
}

; main::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4main4main17h21dad3c11a6d2da9E() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %_14 = alloca i8, align 1
  %_13 = alloca i8, align 1
  %_12 = alloca %"[closure@main.rs:10:9: 10:21]", align 8
  %_11 = alloca %t, align 8
  %_8 = alloca %t, align 8
  %_5 = alloca %t, align 8
  %_4 = alloca { %t, %t }, align 8
  %v = alloca %"[closure@main.rs:10:9: 10:21]", align 8
  %_1 = alloca %"[closure@main.rs:9:14: 9:25]", align 1
  store i8 0, ptr %_14, align 1
; call <T as core::convert::Into<U>>::into
  %1 = call { ptr, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h34e96fca8ead8cc8E"(ptr align 1 @alloc_e16ab3422f24440ac4aefa70ee9428b2, i64 2)
  %_6.0 = extractvalue { ptr, i64 } %1, 0
  %_6.1 = extractvalue { ptr, i64 } %1, 1
  store i8 1, ptr %_14, align 1
  %2 = getelementptr inbounds %t, ptr %_5, i32 0, i32 1
  store i32 1, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_5, i32 0, i32 0
  store ptr %_6.0, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_5, i32 0, i32 1
  store i64 %_6.1, ptr %4, align 8
; invoke <T as core::convert::Into<U>>::into
  %5 = invoke { ptr, i64 } @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h34e96fca8ead8cc8E"(ptr align 1 @alloc_62d2e2f2b7ce4560336617a7e46651ee, i64 2)
          to label %bb2 unwind label %cleanup

bb8:                                              ; preds = %cleanup
  %6 = load i8, ptr %_14, align 1, !range !8, !noundef !3
  %7 = trunc i8 %6 to i1
  br i1 %7, label %bb7, label %bb6

cleanup:                                          ; preds = %bb2, %start
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  %11 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %9, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %10, ptr %12, align 8
  br label %bb8

bb2:                                              ; preds = %start
  %_9.0 = extractvalue { ptr, i64 } %5, 0
  %_9.1 = extractvalue { ptr, i64 } %5, 1
  %13 = getelementptr inbounds %t, ptr %_8, i32 0, i32 1
  store i32 2, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 0
  store ptr %_9.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %_8, i32 0, i32 1
  store i64 %_9.1, ptr %15, align 8
  store i8 0, ptr %_14, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_4, ptr align 8 %_5, i64 24, i1 false)
  %16 = getelementptr inbounds { %t, %t }, ptr %_4, i32 0, i32 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %16, ptr align 8 %_8, i64 24, i1 false)
  %17 = getelementptr inbounds { %t, %t }, ptr %_4, i32 0, i32 1
; invoke main::main::{{closure}}
  invoke void @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$17hf86d0e10a1a43718E"(ptr sret(%"[closure@main.rs:10:9: 10:21]") %v, ptr align 1 %_1, ptr %_4, ptr %17)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %bb2
  store i8 0, ptr %_14, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_12, ptr align 8 %v, i64 48, i1 false)
  store i8 1, ptr %_13, align 1
  %18 = load i8, ptr %_13, align 1, !noundef !3
; call main::main::{{closure}}::{{closure}}
  call void @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfc824f07791930a0E"(ptr sret(%t) %_11, ptr %_12, i8 %18)
; call core::ptr::drop_in_place<main::t>
  call void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %_11)
  ret void

bb6:                                              ; preds = %bb7, %bb8
  %19 = load ptr, ptr %0, align 8, !noundef !3
  %20 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %21 = load i32, ptr %20, align 8, !noundef !3
  %22 = insertvalue { ptr, i32 } poison, ptr %19, 0
  %23 = insertvalue { ptr, i32 } %22, i32 %21, 1
  resume { ptr, i32 } %23

bb7:                                              ; preds = %bb8
; invoke core::ptr::drop_in_place<main::t>
  invoke void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %_5) #18
          to label %bb6 unwind label %terminate

terminate:                                        ; preds = %bb7
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %25 = extractvalue { ptr, i32 } %24, 0
  %26 = extractvalue { ptr, i32 } %24, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hc3ef110419ba8f94E() #19
  unreachable
}

; main::main::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$17hf86d0e10a1a43718E"(ptr sret(%"[closure@main.rs:10:9: 10:21]") %0, ptr align 1 %_1, ptr %x, ptr %y) unnamed_addr #3 {
start:
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %x, i64 24, i1 false)
  %1 = getelementptr inbounds %"[closure@main.rs:10:9: 10:21]", ptr %0, i32 0, i32 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %1, ptr align 8 %y, i64 24, i1 false)
  ret void
}

; main::main::{{closure}}::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17hfc824f07791930a0E"(ptr sret(%t) %0, ptr %_1, i8 %z) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_5 = alloca i8, align 1
  %_4 = alloca i8, align 1
  store i8 0, ptr %_5, align 1
  store i8 0, ptr %_4, align 1
  store i8 1, ptr %_4, align 1
  store i8 1, ptr %_5, align 1
  switch i8 %z, label %bb1 [
    i8 0, label %bb2
    i8 1, label %bb3
  ]

bb1:                                              ; preds = %start
; invoke std::panicking::begin_panic
  invoke void @_ZN3std9panicking11begin_panic17h2059a05e82eb06bfE(ptr align 1 @alloc_38a9d1c1fccd92e612dd2762da060982, i64 0, ptr align 8 @alloc_cd4a1854d14f2c6ff367ecf9e4824f01) #17
          to label %unreachable unwind label %cleanup

bb2:                                              ; preds = %start
  store i8 0, ptr %_5, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %_1, i64 24, i1 false)
  br label %bb12

bb3:                                              ; preds = %start
  store i8 0, ptr %_4, align 1
  %2 = getelementptr inbounds %"[closure@main.rs:10:9: 10:21]", ptr %_1, i32 0, i32 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %2, i64 24, i1 false)
  br label %bb12

bb4:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<main::main::{{closure}}::{{closure}}>
  invoke void @"_ZN4core3ptr89drop_in_place$LT$main..main..$u7b$$u7b$closure$u7d$$u7d$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hff896d77741f45deE"(ptr align 8 %_1) #18
          to label %bb5 unwind label %terminate

cleanup:                                          ; preds = %bb1
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = extractvalue { ptr, i32 } %3, 0
  %5 = extractvalue { ptr, i32 } %3, 1
  %6 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %5, ptr %7, align 8
  br label %bb4

unreachable:                                      ; preds = %bb1
  unreachable

terminate:                                        ; preds = %bb8, %bb4
  %8 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hc3ef110419ba8f94E() #19
  unreachable

bb5:                                              ; preds = %bb8, %bb7, %bb4
  %11 = load ptr, ptr %1, align 8, !noundef !3
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %13 = load i32, ptr %12, align 8, !noundef !3
  %14 = insertvalue { ptr, i32 } poison, ptr %11, 0
  %15 = insertvalue { ptr, i32 } %14, i32 %13, 1
  resume { ptr, i32 } %15

bb12:                                             ; preds = %bb2, %bb3
  %16 = load i8, ptr %_5, align 1, !range !8, !noundef !3
  %17 = trunc i8 %16 to i1
  br i1 %17, label %bb11, label %bb9

bb9:                                              ; preds = %bb11, %bb12
  %18 = load i8, ptr %_4, align 1, !range !8, !noundef !3
  %19 = trunc i8 %18 to i1
  br i1 %19, label %bb10, label %bb6

bb11:                                             ; preds = %bb12
; invoke core::ptr::drop_in_place<main::t>
  invoke void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %_1)
          to label %bb9 unwind label %cleanup1

bb7:                                              ; preds = %cleanup1
  %20 = load i8, ptr %_4, align 1, !range !8, !noundef !3
  %21 = trunc i8 %20 to i1
  br i1 %21, label %bb8, label %bb5

cleanup1:                                         ; preds = %bb11
  %22 = landingpad { ptr, i32 }
          cleanup
  %23 = extractvalue { ptr, i32 } %22, 0
  %24 = extractvalue { ptr, i32 } %22, 1
  %25 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %23, ptr %25, align 8
  %26 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %24, ptr %26, align 8
  br label %bb7

bb8:                                              ; preds = %bb7
  %27 = getelementptr inbounds %"[closure@main.rs:10:9: 10:21]", ptr %_1, i32 0, i32 1
; invoke core::ptr::drop_in_place<main::t>
  invoke void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %27) #18
          to label %bb5 unwind label %terminate

bb6:                                              ; preds = %bb10, %bb9
  ret void

bb10:                                             ; preds = %bb9
  %28 = getelementptr inbounds %"[closure@main.rs:10:9: 10:21]", ptr %_1, i32 0, i32 1
; call core::ptr::drop_in_place<main::t>
  call void @"_ZN4core3ptr28drop_in_place$LT$main..t$GT$17h036e1b6cedf7d375E"(ptr align 8 %28)
  br label %bb6
}

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #6

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17hd66bf6b7da144005E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #0

; std::panicking::rust_panic_with_hook
; Function Attrs: noreturn nonlazybind uwtable
declare void @_ZN3std9panicking20rust_panic_with_hook17h82ebcd5d5ed2fad4E(ptr align 1, ptr align 8, ptr align 8, ptr align 8, i1 zeroext) unnamed_addr #7

; core::panicking::panic_cannot_unwind
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking19panic_cannot_unwind17hc3ef110419ba8f94E() unnamed_addr #8

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.assume(i1 noundef) #9

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #10

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17ha338a74a5d65bf6fE(ptr align 1, i64, ptr align 8) unnamed_addr #4

; <core::alloc::layout::LayoutError as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hb2f579bc0a3b7db3E"(ptr align 1, ptr align 8) unnamed_addr #0

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core6result13unwrap_failed17h100c4d67576990cfE(ptr align 1, i64, ptr align 1, ptr align 8, ptr align 8) unnamed_addr #4

; alloc::rc::rcbox_layout_for_value_layout
; Function Attrs: nonlazybind uwtable
declare { i64, i64 } @_ZN5alloc2rc29rcbox_layout_for_value_layout17hd45e0b4117ba8398E(i64, i64) unnamed_addr #0

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h52397d1f34536addE(i64, i64) unnamed_addr #11

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #12

; Function Attrs: nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc_zeroed(i64, i64 allocalign) unnamed_addr #13

; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc(i64, i64 allocalign) unnamed_addr #14

; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #15

; std::process::abort
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN3std7process5abort17h6c0b0d8d16512c6cE() unnamed_addr #11

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #16 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hb0a30f0bcf58091dE(ptr @_ZN4main4main17h21dad3c11a6d2da9E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { inlinehint noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { cold noreturn nounwind }
attributes #7 = { noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #10 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #11 = { cold noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #13 = { nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #14 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #15 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #16 = { nonlazybind "target-cpu"="x86-64" }
attributes #17 = { noreturn }
attributes #18 = { noinline }
attributes #19 = { noinline noreturn nounwind }
attributes #20 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{}
!4 = !{i32 2705457}
!5 = !{i64 8}
!6 = !{i64 1}
!7 = !{i64 1, i64 -9223372036854775807}
!8 = !{i8 0, i8 2}
!9 = !{i64 0, i64 -9223372036854775807}
!10 = !{i64 0, i64 2}
!11 = !{i64 0, i64 -9223372036854775808}
!12 = !{i64 1, i64 0}
