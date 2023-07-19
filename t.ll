; ModuleID = 't.08e0c1ca-cgu.0'
source_filename = "t.08e0c1ca-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"[closure@std::panicking::begin_panic<&str>::{closure#0}]" = type { { ptr, i64 }, ptr }
%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%UntaggedObject = type { [1 x i64] }
%Object = type { %UntaggedObject, i8, [7 x i8] }

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he23965e7801c1d08E", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h86b5ce23244e71daE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h4ea69a1c6eac2b1dE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h4ea69a1c6eac2b1dE" }>, align 8
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr }> <{ ptr @"_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic..PanicPayload$LT$$RF$str$GT$$GT$17hfb07ff0bcac3c10cE", [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7dfd6a7c49716ebaE", ptr @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17h2101c9803d2670e5E" }>, align 8
@alloc_91c7fa63c3cfeaa3c795652d5cf060e4 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_560206a49c61adca6f3f0639a12632eb = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_91c7fa63c3cfeaa3c795652d5cf060e4, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc_0f3d7beb2672f296d76a42c95890bef9 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/fmt/mod.rs" }>, align 1
@alloc_ea676e06474b3ad20dc2b78cc1c22fa8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0f3d7beb2672f296d76a42c95890bef9, [16 x i8] c"K\00\00\00\00\00\00\00\9E\01\00\00\0D\00\00\00" }>, align 8
@alloc_584d991c255ecc0b9a8eb25813e7c09e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0f3d7beb2672f296d76a42c95890bef9, [16 x i8] c"K\00\00\00\00\00\00\00\91\01\00\00\0D\00\00\00" }>, align 8
@alloc_513570631223a12912d85da2bec3b15a = private unnamed_addr constant <{}> zeroinitializer, align 8
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17ha30f0890de9cc96fE", [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hd09181e3380c92dbE" }>, align 8
@alloc_476509aa7047bd805d630c6546685797 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"explicit panic" }>, align 1
@alloc_a23d2f9c17e55df684aec3b7ce3eeea0 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"t.rs" }>, align 1
@alloc_31800e654abeed5428f47fd4d2679671 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a23d2f9c17e55df684aec3b7ce3eeea0, [16 x i8] c"\04\00\00\00\00\00\00\00\06\00\00\00\09\00\00\00" }>, align 8
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_4ed11f17954caf7854f80f02ae34907e = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_513570631223a12912d85da2bec3b15a, [8 x i8] zeroinitializer, ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; <T as core::any::Any>::type_id
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hd09181e3380c92dbE"(ptr align 8 %self) unnamed_addr #0 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17hf52addaece41506fE()
  ret i64 %0
}

; std::sys_common::backtrace::__rust_end_short_backtrace
; Function Attrs: noinline noreturn nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h5b010c37eb366514E(ptr %f) unnamed_addr #1 {
start:
; call std::panicking::begin_panic::{{closure}}
  call void @"_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17ha14395df02b73354E"(ptr %f) #14
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  call void @llvm.trap()
  unreachable
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he542954900c0fa20E(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h5ef5d5c4a6076cccE(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17h31a02dad753185d6E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 {
start:
  %_8 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h76f3e81e6b8f13f9E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %1 = load i64, ptr %_5, align 8, !noundef !4
  ret i64 %1
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h4ea69a1c6eac2b1dE"(ptr align 8 %_1) unnamed_addr #3 {
start:
  %self = alloca i8, align 1
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he542954900c0fa20E(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7a56d671c4f0c86aE"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1, !noundef !4
  %1 = zext i8 %_6 to i32
  ret i32 %1
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
define internal void @_ZN3std9panicking11begin_panic17h614b819013ba8a73E(ptr align 1 %msg.0, i64 %msg.1, ptr align 8 %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %2 = alloca ptr, align 8
  %_3 = alloca %"[closure@std::panicking::begin_panic<&str>::{closure#0}]", align 8
  store ptr %0, ptr %2, align 8
  %loc = load ptr, ptr %2, align 8, !nonnull !4, !align !5, !noundef !4
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_3, i32 0, i32 0
  store ptr %msg.0, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_3, i32 0, i32 1
  store i64 %msg.1, ptr %4, align 8
  %5 = getelementptr inbounds %"[closure@std::panicking::begin_panic<&str>::{closure#0}]", ptr %_3, i32 0, i32 1
  store ptr %loc, ptr %5, align 8
; invoke std::sys_common::backtrace::__rust_end_short_backtrace
  invoke void @_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h5b010c37eb366514E(ptr %_3) #14
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
  %11 = load ptr, ptr %1, align 8, !noundef !4
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %13 = load i32, ptr %12, align 8, !noundef !4
  %14 = insertvalue { ptr, i32 } poison, ptr %11, 0
  %15 = insertvalue { ptr, i32 } %14, i32 %13, 1
  resume { ptr, i32 } %15

bb2:                                              ; preds = %bb3
  br label %bb1
}

; std::panicking::begin_panic::PanicPayload<A>::new
; Function Attrs: nonlazybind uwtable
define internal { ptr, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17he7ef9595a88186ddE"(ptr align 1 %inner.0, i64 %inner.1) unnamed_addr #0 {
start:
  %_2 = alloca { ptr, i64 }, align 8
  %0 = alloca { ptr, i64 }, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  store ptr %inner.0, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  store i64 %inner.1, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  %4 = load ptr, ptr %3, align 8, !align !6, !noundef !4
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %6 = load i64, ptr %5, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %4, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %6, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  %10 = load ptr, ptr %9, align 8, !align !6, !noundef !4
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  %12 = load i64, ptr %11, align 8
  %13 = insertvalue { ptr, i64 } poison, ptr %10, 0
  %14 = insertvalue { ptr, i64 } %13, i64 %12, 1
  ret { ptr, i64 } %14
}

; std::panicking::begin_panic::{{closure}}
; Function Attrs: inlinehint noreturn nonlazybind uwtable
define internal void @"_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17ha14395df02b73354E"(ptr %_1) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %_5 = alloca { ptr, i64 }, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_1, i32 0, i32 0
  %_6.0 = load ptr, ptr %1, align 8, !nonnull !4, !align !6, !noundef !4
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_1, i32 0, i32 1
  %_6.1 = load i64, ptr %2, align 8, !noundef !4
; call std::panicking::begin_panic::PanicPayload<A>::new
  %3 = call { ptr, i64 } @"_ZN3std9panicking11begin_panic21PanicPayload$LT$A$GT$3new17he7ef9595a88186ddE"(ptr align 1 %_6.0, i64 %_6.1)
  store { ptr, i64 } %3, ptr %_5, align 8
  %4 = getelementptr inbounds %"[closure@std::panicking::begin_panic<&str>::{closure#0}]", ptr %_1, i32 0, i32 1
  %_8 = load ptr, ptr %4, align 8, !nonnull !4, !align !5, !noundef !4
; invoke std::panicking::rust_panic_with_hook
  invoke void @_ZN3std9panicking20rust_panic_with_hook17hafdc493a79370062E(ptr align 1 %_5, ptr align 8 @vtable.1, ptr align 8 null, ptr align 8 %_8, i1 zeroext true) #14
          to label %unreachable unwind label %cleanup

bb2:                                              ; preds = %cleanup
  %5 = load ptr, ptr %0, align 8, !noundef !4
  %6 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %7 = load i32, ptr %6, align 8, !noundef !4
  %8 = insertvalue { ptr, i32 } poison, ptr %5, 0
  %9 = insertvalue { ptr, i32 } %8, i32 %7, 1
  resume { ptr, i32 } %9

cleanup:                                          ; preds = %start
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = extractvalue { ptr, i32 } %10, 0
  %12 = extractvalue { ptr, i32 } %10, 1
  %13 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %11, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %12, ptr %14, align 8
  br label %bb2

unreachable:                                      ; preds = %start
  unreachable
}

; core::any::TypeId::of
; Function Attrs: nonlazybind uwtable
define internal i64 @_ZN4core3any6TypeId2of17hf52addaece41506fE() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  store i64 -4493808902380553279, ptr %0, align 8
  %_1 = load i64, ptr %0, align 8, !noundef !4
  store i64 %_1, ptr %1, align 8
  %2 = load i64, ptr %1, align 8, !noundef !4
  ret i64 %2
}

; core::fmt::ArgumentV1::new_display
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, ptr } @_ZN4core3fmt10ArgumentV111new_display17ha5b0444275f00845E(ptr align 1 %x) unnamed_addr #3 {
start:
  %0 = alloca { ptr, ptr }, align 8
  store ptr %x, ptr %0, align 8
  %1 = getelementptr inbounds { ptr, ptr }, ptr %0, i32 0, i32 1
  store ptr @"_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h01d72e8cca36df82E", ptr %1, align 8
  %2 = getelementptr inbounds { ptr, ptr }, ptr %0, i32 0, i32 0
  %3 = load ptr, ptr %2, align 8, !nonnull !4, !align !6, !noundef !4
  %4 = getelementptr inbounds { ptr, ptr }, ptr %0, i32 0, i32 1
  %5 = load ptr, ptr %4, align 8, !nonnull !4, !noundef !4
  %6 = insertvalue { ptr, ptr } poison, ptr %3, 0
  %7 = insertvalue { ptr, ptr } %6, ptr %5, 1
  ret { ptr, ptr } %7
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h4f92ad1b46ca7b51E(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #3 {
start:
  %_14 = alloca { ptr, i64 }, align 8
  %_12 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = alloca i8, align 1
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_9 = add i64 %args.1, 1
  %_7 = icmp ugt i64 %pieces.1, %_9
  %1 = zext i1 %_7 to i8
  store i8 %1, ptr %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, ptr %_3, align 1, !range !7, !noundef !4
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  store ptr null, ptr %_14, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  store ptr %pieces.0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  store i64 %pieces.1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_14, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !5, !noundef !4
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_14, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %14 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 0
  store ptr %args.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 1
  store i64 %args.1, ptr %15, align 8
  ret void

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17h67a60ef659b2c0dcE(ptr sret(%"core::fmt::Arguments<'_>") %_12, ptr align 8 @alloc_560206a49c61adca6f3f0639a12632eb, i64 1)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h0f6ef0178afce4f2E(ptr %_12, ptr align 8 @alloc_ea676e06474b3ad20dc2b78cc1c22fa8) #14
  unreachable
}

; core::fmt::Arguments::new_const
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments9new_const17h67a60ef659b2c0dcE(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1) unnamed_addr #3 {
start:
  %_7 = alloca { ptr, i64 }, align 8
  %_5 = alloca %"core::fmt::Arguments<'_>", align 8
  %_2 = icmp ugt i64 %pieces.1, 1
  br i1 %_2, label %bb1, label %bb3

bb3:                                              ; preds = %start
  store ptr null, ptr %_7, align 8
  %1 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %2 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 0
  store ptr %pieces.0, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 1
  store i64 %pieces.1, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 0
  %5 = load ptr, ptr %4, align 8, !align !5, !noundef !4
  %6 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 1
  %7 = load i64, ptr %6, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %5, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %7, ptr %9, align 8
  %10 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %11 = getelementptr inbounds { ptr, i64 }, ptr %10, i32 0, i32 0
  store ptr @alloc_513570631223a12912d85da2bec3b15a, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %10, i32 0, i32 1
  store i64 0, ptr %12, align 8
  ret void

bb1:                                              ; preds = %start
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17h67a60ef659b2c0dcE(ptr sret(%"core::fmt::Arguments<'_>") %_5, ptr align 8 @alloc_560206a49c61adca6f3f0639a12632eb, i64 1)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h0f6ef0178afce4f2E(ptr %_5, ptr align 8 @alloc_584d991c255ecc0b9a8eb25813e7c09e) #14
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h86b5ce23244e71daE"(ptr %_1) unnamed_addr #3 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h2bcc9ee2890afcdbE(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h2bcc9ee2890afcdbE(ptr %0) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h4ea69a1c6eac2b1dE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %3 = load ptr, ptr %1, align 8, !noundef !4
  %4 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %5 = load i32, ptr %4, align 8, !noundef !4
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

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h5ef5d5c4a6076cccE(ptr %_1) unnamed_addr #3 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<&str>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17ha30f0890de9cc96fE"(ptr %_1) unnamed_addr #3 {
start:
  ret void
}

; core::ptr::drop_in_place<std::panicking::begin_panic::PanicPayload<&str>>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic..PanicPayload$LT$$RF$str$GT$$GT$17hfb07ff0bcac3c10cE"(ptr %_1) unnamed_addr #3 {
start:
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he23965e7801c1d08E"(ptr %_1) unnamed_addr #3 {
start:
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7a56d671c4f0c86aE"() unnamed_addr #3 {
start:
  ret i8 0
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @_ZN5alloc5alloc15exchange_malloc17hbd546e854c4d66e6E(i64 %size, i64 %align) unnamed_addr #3 {
start:
  %self = alloca ptr, align 8
  %_4 = alloca { ptr, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  store i64 %size, ptr %layout, align 8
  %0 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %align, ptr %0, align 8
  %1 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %2 = load i64, ptr %1, align 8, !noundef !4
  %3 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %4 = load i64, ptr %3, align 8, !range !8, !noundef !4
; call alloc::alloc::Global::alloc_impl
  %5 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17ha8fe0d8981b35df1E(ptr align 1 @alloc_513570631223a12912d85da2bec3b15a, i64 %2, i64 %4, i1 zeroext false)
  store { ptr, i64 } %5, ptr %_4, align 8
  %6 = load ptr, ptr %_4, align 8, !noundef !4
  %7 = ptrtoint ptr %6 to i64
  %8 = icmp eq i64 %7, 0
  %_5 = select i1 %8, i64 1, i64 0
  %9 = icmp eq i64 %_5, 0
  br i1 %9, label %bb3, label %bb1

bb3:                                              ; preds = %start
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_4, i32 0, i32 0
  %ptr.0 = load ptr, ptr %10, align 8, !nonnull !4, !noundef !4
  %11 = getelementptr inbounds { ptr, i64 }, ptr %_4, i32 0, i32 1
  %ptr.1 = load i64, ptr %11, align 8, !noundef !4
  store ptr %ptr.0, ptr %self, align 8
  %_18 = load ptr, ptr %self, align 8, !noundef !4
  ret ptr %_18

bb1:                                              ; preds = %start
  %12 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %13 = load i64, ptr %12, align 8, !noundef !4
  %14 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %15 = load i64, ptr %14, align 8, !range !8, !noundef !4
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h90b7f1836babe573E(i64 %13, i64 %15) #14
  unreachable

bb2:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17ha8fe0d8981b35df1E(ptr align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #3 {
start:
  %_77 = alloca { ptr, i64 }, align 8
  %_76 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %_61 = alloca ptr, align 8
  %_60 = alloca ptr, align 8
  %_54 = alloca i64, align 8
  %_45 = alloca i64, align 8
  %_35 = alloca { ptr, i64 }, align 8
  %_34 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
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
  %2 = alloca { ptr, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  store i64 %0, ptr %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %1, ptr %4, align 8
  %size = load i64, ptr %layout, align 8, !noundef !4
  %5 = icmp eq i64 %size, 0
  br i1 %5, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %6 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %self10 = load i64, ptr %6, align 8, !range !8, !noundef !4
  store i64 %self10, ptr %_22, align 8
  %_23 = load i64, ptr %_22, align 8, !range !8, !noundef !4
  %_24 = icmp uge i64 -9223372036854775808, %_23
  call void @llvm.assume(i1 %_24)
  %_25 = icmp ule i64 1, %_23
  call void @llvm.assume(i1 %_25)
  %ptr11 = inttoptr i64 %_23 to ptr
  store ptr %ptr11, ptr %data, align 8
  %_32 = load ptr, ptr %data, align 8, !noundef !4
  store ptr %_32, ptr %_35, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 1
  store i64 0, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 0
  %9 = load ptr, ptr %8, align 8, !noundef !4
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_35, i32 0, i32 1
  %11 = load i64, ptr %10, align 8, !noundef !4
  %12 = getelementptr inbounds { ptr, i64 }, ptr %_34, i32 0, i32 0
  store ptr %9, ptr %12, align 8
  %13 = getelementptr inbounds { ptr, i64 }, ptr %_34, i32 0, i32 1
  store i64 %11, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, i64 }, ptr %_34, i32 0, i32 0
  %ptr.012 = load ptr, ptr %14, align 8, !noundef !4
  %15 = getelementptr inbounds { ptr, i64 }, ptr %_34, i32 0, i32 1
  %ptr.113 = load i64, ptr %15, align 8, !noundef !4
  %16 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 0
  store ptr %ptr.012, ptr %16, align 8
  %17 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 1
  store i64 %ptr.113, ptr %17, align 8
  %18 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 0
  %19 = load ptr, ptr %18, align 8, !nonnull !4, !noundef !4
  %20 = getelementptr inbounds { ptr, i64 }, ptr %_6, i32 0, i32 1
  %21 = load i64, ptr %20, align 8, !noundef !4
  %22 = getelementptr inbounds { ptr, i64 }, ptr %2, i32 0, i32 0
  store ptr %19, ptr %22, align 8
  %23 = getelementptr inbounds { ptr, i64 }, ptr %2, i32 0, i32 1
  store i64 %21, ptr %23, align 8
  br label %bb10

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb4:                                              ; preds = %bb1
  %24 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %25 = load i64, ptr %24, align 8, !noundef !4
  %26 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %27 = load i64, ptr %26, align 8, !range !8, !noundef !4
  %28 = getelementptr inbounds { i64, i64 }, ptr %layout2, i32 0, i32 0
  store i64 %25, ptr %28, align 8
  %29 = getelementptr inbounds { i64, i64 }, ptr %layout2, i32 0, i32 1
  store i64 %27, ptr %29, align 8
  %_49 = load i64, ptr %layout2, align 8, !noundef !4
  %30 = getelementptr inbounds { i64, i64 }, ptr %layout2, i32 0, i32 1
  %self6 = load i64, ptr %30, align 8, !range !8, !noundef !4
  store i64 %self6, ptr %_54, align 8
  %_55 = load i64, ptr %_54, align 8, !range !8, !noundef !4
  %_56 = icmp uge i64 -9223372036854775808, %_55
  call void @llvm.assume(i1 %_56)
  %_57 = icmp ule i64 1, %_55
  call void @llvm.assume(i1 %_57)
  %31 = call ptr @__rust_alloc(i64 %_49, i64 %_55) #15
  store ptr %31, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
  %32 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %33 = load i64, ptr %32, align 8, !noundef !4
  %34 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %35 = load i64, ptr %34, align 8, !range !8, !noundef !4
  %36 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 0
  store i64 %33, ptr %36, align 8
  %37 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  store i64 %35, ptr %37, align 8
  %_40 = load i64, ptr %layout1, align 8, !noundef !4
  %38 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  %self5 = load i64, ptr %38, align 8, !range !8, !noundef !4
  store i64 %self5, ptr %_45, align 8
  %_46 = load i64, ptr %_45, align 8, !range !8, !noundef !4
  %_47 = icmp uge i64 -9223372036854775808, %_46
  call void @llvm.assume(i1 %_47)
  %_48 = icmp ule i64 1, %_46
  call void @llvm.assume(i1 %_48)
  %39 = call ptr @__rust_alloc_zeroed(i64 %_40, i64 %_46) #15
  store ptr %39, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  %ptr = load ptr, ptr %raw_ptr, align 8, !noundef !4
  store ptr %ptr, ptr %_61, align 8
  %ptr7 = load ptr, ptr %_61, align 8, !noundef !4
  %_63 = ptrtoint ptr %ptr7 to i64
  %_59 = icmp eq i64 %_63, 0
  %_58 = xor i1 %_59, true
  br i1 %_58, label %bb13, label %bb14

bb14:                                             ; preds = %bb5
  store ptr null, ptr %self4, align 8
  br label %bb15

bb13:                                             ; preds = %bb5
  store ptr %ptr, ptr %_60, align 8
  %40 = load ptr, ptr %_60, align 8, !nonnull !4, !noundef !4
  store ptr %40, ptr %self4, align 8
  br label %bb15

bb15:                                             ; preds = %bb14, %bb13
  %41 = load ptr, ptr %self4, align 8, !noundef !4
  %42 = ptrtoint ptr %41 to i64
  %43 = icmp eq i64 %42, 0
  %_68 = select i1 %43, i64 0, i64 1
  %44 = icmp eq i64 %_68, 0
  br i1 %44, label %bb16, label %bb17

bb16:                                             ; preds = %bb15
  store ptr null, ptr %self3, align 8
  br label %bb18

bb17:                                             ; preds = %bb15
  %v = load ptr, ptr %self4, align 8, !nonnull !4, !noundef !4
  store ptr %v, ptr %self3, align 8
  br label %bb18

bb18:                                             ; preds = %bb16, %bb17
  %45 = load ptr, ptr %self3, align 8, !noundef !4
  %46 = ptrtoint ptr %45 to i64
  %47 = icmp eq i64 %46, 0
  %_70 = select i1 %47, i64 1, i64 0
  %48 = icmp eq i64 %_70, 0
  br i1 %48, label %bb20, label %bb19

bb20:                                             ; preds = %bb18
  %v8 = load ptr, ptr %self3, align 8, !nonnull !4, !noundef !4
  store ptr %v8, ptr %_12, align 8
  br label %bb6

bb19:                                             ; preds = %bb18
  store ptr null, ptr %_12, align 8
  br label %bb6

bb6:                                              ; preds = %bb20, %bb19
  %49 = load ptr, ptr %_12, align 8, !noundef !4
  %50 = ptrtoint ptr %49 to i64
  %51 = icmp eq i64 %50, 0
  %_16 = select i1 %51, i64 1, i64 0
  %52 = icmp eq i64 %_16, 0
  br i1 %52, label %bb7, label %bb9

bb7:                                              ; preds = %bb6
  %ptr9 = load ptr, ptr %_12, align 8, !nonnull !4, !noundef !4
  store ptr %ptr9, ptr %_77, align 8
  %53 = getelementptr inbounds { ptr, i64 }, ptr %_77, i32 0, i32 1
  store i64 %size, ptr %53, align 8
  %54 = getelementptr inbounds { ptr, i64 }, ptr %_77, i32 0, i32 0
  %55 = load ptr, ptr %54, align 8, !noundef !4
  %56 = getelementptr inbounds { ptr, i64 }, ptr %_77, i32 0, i32 1
  %57 = load i64, ptr %56, align 8, !noundef !4
  %58 = getelementptr inbounds { ptr, i64 }, ptr %_76, i32 0, i32 0
  store ptr %55, ptr %58, align 8
  %59 = getelementptr inbounds { ptr, i64 }, ptr %_76, i32 0, i32 1
  store i64 %57, ptr %59, align 8
  %60 = getelementptr inbounds { ptr, i64 }, ptr %_76, i32 0, i32 0
  %ptr.0 = load ptr, ptr %60, align 8, !noundef !4
  %61 = getelementptr inbounds { ptr, i64 }, ptr %_76, i32 0, i32 1
  %ptr.1 = load i64, ptr %61, align 8, !noundef !4
  %62 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 0
  store ptr %ptr.0, ptr %62, align 8
  %63 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 1
  store i64 %ptr.1, ptr %63, align 8
  %64 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 0
  %65 = load ptr, ptr %64, align 8, !nonnull !4, !noundef !4
  %66 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 1
  %67 = load i64, ptr %66, align 8, !noundef !4
  %68 = getelementptr inbounds { ptr, i64 }, ptr %2, i32 0, i32 0
  store ptr %65, ptr %68, align 8
  %69 = getelementptr inbounds { ptr, i64 }, ptr %2, i32 0, i32 1
  store i64 %67, ptr %69, align 8
  br label %bb10

bb9:                                              ; preds = %bb6
  store ptr null, ptr %2, align 8
  br label %bb10

bb8:                                              ; No predecessors!
  unreachable

bb10:                                             ; preds = %bb2, %bb7, %bb9
  %70 = getelementptr inbounds { ptr, i64 }, ptr %2, i32 0, i32 0
  %71 = load ptr, ptr %70, align 8, !noundef !4
  %72 = getelementptr inbounds { ptr, i64 }, ptr %2, i32 0, i32 1
  %73 = load i64, ptr %72, align 8
  %74 = insertvalue { ptr, i64 } poison, ptr %71, 0
  %75 = insertvalue { ptr, i64 } %74, i64 %73, 1
  ret { ptr, i64 } %75
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: nonlazybind uwtable
define internal { ptr, ptr } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17h2101c9803d2670e5E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = load ptr, ptr %self, align 8, !noundef !4
  %1 = ptrtoint ptr %0 to i64
  %2 = icmp eq i64 %1, 0
  %_2 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_2, 0
  br i1 %3, label %bb1, label %bb3

bb1:                                              ; preds = %start
; call std::process::abort
  call void @_ZN3std7process5abort17h96a864b26eb4d7f5E() #14
  unreachable

bb3:                                              ; preds = %start
  %4 = insertvalue { ptr, ptr } poison, ptr %self, 0
  %5 = insertvalue { ptr, ptr } %4, ptr @vtable.2, 1
  ret { ptr, ptr } %5

bb2:                                              ; No predecessors!
  unreachable
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::take_box
; Function Attrs: nonlazybind uwtable
define internal { ptr, ptr } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7dfd6a7c49716ebaE"(ptr align 8 %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %pointer = alloca { ptr, ptr }, align 8
  %_37 = alloca { ptr, ptr }, align 8
  %_30 = alloca { ptr, ptr }, align 8
  %_25 = alloca { ptr, ptr }, align 8
  %_23 = alloca { ptr, ptr }, align 8
  %src1 = alloca { ptr, i64 }, align 8
  %src = alloca { ptr, i64 }, align 8
  %result = alloca { ptr, i64 }, align 8
  store ptr null, ptr %src, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %2 = load ptr, ptr %1, align 8, !align !6, !noundef !4
  %3 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %4 = load i64, ptr %3, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 0
  store ptr %2, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 1
  store i64 %4, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %src, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !6, !noundef !4
  %9 = getelementptr inbounds { ptr, i64 }, ptr %src, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %src1, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %src1, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %self, ptr align 8 %src1, i64 16, i1 false)
  %13 = load ptr, ptr %result, align 8, !noundef !4
  %14 = ptrtoint ptr %13 to i64
  %15 = icmp eq i64 %14, 0
  %_4 = select i1 %15, i64 0, i64 1
  %16 = icmp eq i64 %_4, 0
  br i1 %16, label %bb1, label %bb3

bb1:                                              ; preds = %start
; call std::process::abort
  call void @_ZN3std7process5abort17h96a864b26eb4d7f5E() #14
  unreachable

bb3:                                              ; preds = %start
  %17 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 0
  %a.0 = load ptr, ptr %17, align 8, !nonnull !4, !align !6, !noundef !4
  %18 = getelementptr inbounds { ptr, i64 }, ptr %result, i32 0, i32 1
  %a.1 = load i64, ptr %18, align 8, !noundef !4
; invoke alloc::alloc::exchange_malloc
  %_21 = invoke ptr @_ZN5alloc5alloc15exchange_malloc17hbd546e854c4d66e6E(i64 16, i64 8)
          to label %bb5 unwind label %cleanup

bb2:                                              ; No predecessors!
  unreachable

bb6:                                              ; preds = %cleanup
  %19 = load ptr, ptr %0, align 8, !noundef !4
  %20 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %21 = load i32, ptr %20, align 8, !noundef !4
  %22 = insertvalue { ptr, i32 } poison, ptr %19, 0
  %23 = insertvalue { ptr, i32 } %22, i32 %21, 1
  resume { ptr, i32 } %23

cleanup:                                          ; preds = %bb3
  %24 = landingpad { ptr, i32 }
          cleanup
  %25 = extractvalue { ptr, i32 } %24, 0
  %26 = extractvalue { ptr, i32 } %24, 1
  %27 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %25, ptr %27, align 8
  %28 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %26, ptr %28, align 8
  br label %bb6

bb5:                                              ; preds = %bb3
  %29 = getelementptr inbounds { ptr, i64 }, ptr %_21, i32 0, i32 0
  store ptr %a.0, ptr %29, align 8
  %30 = getelementptr inbounds { ptr, i64 }, ptr %_21, i32 0, i32 1
  store i64 %a.1, ptr %30, align 8
  %31 = getelementptr inbounds { ptr, ptr }, ptr %_37, i32 0, i32 0
  store ptr %_21, ptr %31, align 8
  %32 = getelementptr inbounds { ptr, ptr }, ptr %_37, i32 0, i32 1
  store ptr @vtable.2, ptr %32, align 8
  %33 = getelementptr inbounds { ptr, ptr }, ptr %_37, i32 0, i32 0
  %self.0 = load ptr, ptr %33, align 8, !nonnull !4, !noundef !4
  %34 = getelementptr inbounds { ptr, ptr }, ptr %_37, i32 0, i32 1
  %self.1 = load ptr, ptr %34, align 8, !nonnull !4, !align !5, !noundef !4
  %35 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 0
  store ptr %self.0, ptr %35, align 8
  %36 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 1
  store ptr %self.1, ptr %36, align 8
  %37 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 0
  %38 = load ptr, ptr %37, align 8, !nonnull !4, !noundef !4
  %39 = getelementptr inbounds { ptr, ptr }, ptr %pointer, i32 0, i32 1
  %40 = load ptr, ptr %39, align 8, !nonnull !4, !align !5, !noundef !4
  %41 = getelementptr inbounds { ptr, ptr }, ptr %_30, i32 0, i32 0
  store ptr %38, ptr %41, align 8
  %42 = getelementptr inbounds { ptr, ptr }, ptr %_30, i32 0, i32 1
  store ptr %40, ptr %42, align 8
  %43 = getelementptr inbounds { ptr, ptr }, ptr %_30, i32 0, i32 0
  %44 = load ptr, ptr %43, align 8, !nonnull !4, !noundef !4
  %45 = getelementptr inbounds { ptr, ptr }, ptr %_30, i32 0, i32 1
  %46 = load ptr, ptr %45, align 8, !nonnull !4, !align !5, !noundef !4
  %47 = getelementptr inbounds { ptr, ptr }, ptr %_25, i32 0, i32 0
  store ptr %44, ptr %47, align 8
  %48 = getelementptr inbounds { ptr, ptr }, ptr %_25, i32 0, i32 1
  store ptr %46, ptr %48, align 8
  %49 = getelementptr inbounds { ptr, ptr }, ptr %_25, i32 0, i32 0
  %leaked.0 = load ptr, ptr %49, align 8, !nonnull !4, !noundef !4
  %50 = getelementptr inbounds { ptr, ptr }, ptr %_25, i32 0, i32 1
  %leaked.1 = load ptr, ptr %50, align 8, !nonnull !4, !align !5, !noundef !4
  %51 = getelementptr inbounds { ptr, ptr }, ptr %_23, i32 0, i32 0
  store ptr %leaked.0, ptr %51, align 8
  %52 = getelementptr inbounds { ptr, ptr }, ptr %_23, i32 0, i32 1
  store ptr %leaked.1, ptr %52, align 8
  %53 = getelementptr inbounds { ptr, ptr }, ptr %_23, i32 0, i32 0
  %54 = load ptr, ptr %53, align 8, !noundef !4
  %55 = getelementptr inbounds { ptr, ptr }, ptr %_23, i32 0, i32 1
  %56 = load ptr, ptr %55, align 8, !nonnull !4, !align !5, !noundef !4
  %57 = insertvalue { ptr, ptr } poison, ptr %54, 0
  %58 = insertvalue { ptr, ptr } %57, ptr %56, 1
  ret { ptr, ptr } %58
}

; Function Attrs: noinline nonlazybind uwtable
define dso_local double @extract_num({ i64, i64 } %0) unnamed_addr #2 {
start:
  %_10 = alloca [1 x { ptr, ptr }], align 8
  %_6 = alloca %"core::fmt::Arguments<'_>", align 8
  %i = alloca %UntaggedObject, align 8
  %1 = alloca { i64, i64 }, align 8
  %object = alloca %Object, align 8
  store { i64, i64 } %0, ptr %1, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %object, ptr align 8 %1, i64 16, i1 false)
  %2 = getelementptr inbounds %Object, ptr %object, i32 0, i32 1
  %_2 = load i8, ptr %2, align 8, !noundef !4
  %3 = icmp eq i8 %_2, 1
  br i1 %3, label %bb2, label %bb1

bb2:                                              ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %i, ptr align 8 %object, i64 8, i1 false)
; call core::fmt::ArgumentV1::new_display
  %4 = call { ptr, ptr } @_ZN4core3fmt10ArgumentV111new_display17ha5b0444275f00845E(ptr align 1 %i)
  %_11.0 = extractvalue { ptr, ptr } %4, 0
  %_11.1 = extractvalue { ptr, ptr } %4, 1
  %5 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_10, i64 0, i64 0
  %6 = getelementptr inbounds { ptr, ptr }, ptr %5, i32 0, i32 0
  store ptr %_11.0, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, ptr }, ptr %5, i32 0, i32 1
  store ptr %_11.1, ptr %7, align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h4f92ad1b46ca7b51E(ptr sret(%"core::fmt::Arguments<'_>") %_6, ptr align 8 @alloc_4ed11f17954caf7854f80f02ae34907e, i64 2, ptr align 8 %_10, i64 1)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h1de311987873daa6E(ptr %_6)
  %8 = load double, ptr %i, align 8, !noundef !4
  ret double %8

bb1:                                              ; preds = %start
; call std::panicking::begin_panic
  call void @_ZN3std9panicking11begin_panic17h614b819013ba8a73E(ptr align 1 @alloc_476509aa7047bd805d630c6546685797, i64 14, ptr align 8 @alloc_31800e654abeed5428f47fd4d2679671) #14
  unreachable
}

; t::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN1t4main17h79e86cc15d585b87E() unnamed_addr #0 {
start:
  ret void
}

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #6

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h76f3e81e6b8f13f9E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #0

; std::panicking::rust_panic_with_hook
; Function Attrs: noreturn nonlazybind uwtable
declare void @_ZN3std9panicking20rust_panic_with_hook17hafdc493a79370062E(ptr align 1, ptr align 8, ptr align 8, ptr align 8, i1 zeroext) unnamed_addr #7

; <bool as core::fmt::Display>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h01d72e8cca36df82E"(ptr align 1, ptr align 8) unnamed_addr #0

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17h0f6ef0178afce4f2E(ptr, ptr align 8) unnamed_addr #4

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h90b7f1836babe573E(i64, i64) unnamed_addr #8

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.assume(i1 noundef) #9

; Function Attrs: nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc_zeroed(i64, i64 allocalign) unnamed_addr #10

; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc(i64, i64 allocalign) unnamed_addr #11

; std::process::abort
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN3std7process5abort17h96a864b26eb4d7f5E() unnamed_addr #8

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #12

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h1de311987873daa6E(ptr) unnamed_addr #0

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #13 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h31a02dad753185d6E(ptr @_ZN1t4main17h79e86cc15d585b87E, i64 %2, ptr %1, i8 0)
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
attributes #8 = { cold noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #10 = { nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #11 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #13 = { nonlazybind "target-cpu"="x86-64" }
attributes #14 = { noreturn }
attributes #15 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{i32 1924451}
!4 = !{}
!5 = !{i64 8}
!6 = !{i64 1}
!7 = !{i8 0, i8 2}
!8 = !{i64 1, i64 -9223372036854775807}
