; ModuleID = 'main.fd4edd9f-cgu.0'
source_filename = "main.fd4edd9f-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h756d60ec90e7c756E", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h979775d0aae1eef3E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he697d9003afb7fa4E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he697d9003afb7fa4E" }>, align 8
@alloc_4693327ca9c5449cec9b739948ccbb5e = private unnamed_addr constant <{ [7 x i8] }> <{ [7 x i8] c"main.rs" }>, align 1
@alloc_a509a95d4d7ba9e1b62e0e7c06cae8aa = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4693327ca9c5449cec9b739948ccbb5e, [16 x i8] c"\07\00\00\00\00\00\00\00\04\00\00\00\10\00\00\00" }>, align 8
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc_063acf5217603408362721fa974b1dda = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4693327ca9c5449cec9b739948ccbb5e, [16 x i8] c"\07\00\00\00\00\00\00\00\07\00\00\00\13\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb8fc7987ccf73b2cE(ptr %f) unnamed_addr #0 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h52abfc5a263c59c8E(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17hc1bc80fbbc2f1194E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 {
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
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he697d9003afb7fa4E"(ptr align 8 %_1) unnamed_addr #2 {
start:
  %self = alloca i8, align 1
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hb8fc7987ccf73b2cE(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h85ef3eb093c2bd65E"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1, !noundef !4
  %1 = zext i8 %_6 to i32
  ret i32 %1
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h979775d0aae1eef3E"(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h199bfbaf765eaab0E(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h199bfbaf765eaab0E(ptr %0) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he697d9003afb7fa4E"(ptr align 8 %_1)
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
define internal void @_ZN4core3ops8function6FnOnce9call_once17h52abfc5a263c59c8E(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h756d60ec90e7c756E"(ptr %_1) unnamed_addr #2 {
start:
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h85ef3eb093c2bd65E"() unnamed_addr #2 {
start:
  ret i8 0
}

; main::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4main4main17hac82363469e1a7a2E() unnamed_addr #1 {
start:
  %_11 = alloca i32, align 4
  %z = alloca { ptr, ptr }, align 8
  %u = alloca { ptr, ptr }, align 8
  %ooo = alloca i32, align 4
  %yzzz = alloca i32, align 4
  store i32 1, ptr %yzzz, align 4
  store i32 2, ptr %ooo, align 4
  store ptr %yzzz, ptr %u, align 8
  %0 = getelementptr inbounds { ptr, ptr }, ptr %u, i32 0, i32 1
  store ptr %ooo, ptr %0, align 8
  store ptr %yzzz, ptr %z, align 8
  %1 = getelementptr inbounds { ptr, ptr }, ptr %z, i32 0, i32 1
  store ptr %ooo, ptr %1, align 8
  store i32 1, ptr %_11, align 4
  %2 = load i32, ptr %_11, align 4, !noundef !4
; call main::main::{{closure}}
  %_9 = call i32 @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$17hdaac0a0826ac2d36E"(ptr align 8 %z, i32 %2)
; call main::main::{{closure}}
  %_12 = call i32 @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$17h5799ca94dcd5ba85E"(ptr align 8 %u)
  ret void
}

; main::main::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$17h5799ca94dcd5ba85E"(ptr align 8 %_1) unnamed_addr #2 {
start:
  %_5 = load ptr, ptr %_1, align 8, !nonnull !4, !align !5, !noundef !4
  %_2 = load i32, ptr %_5, align 4, !noundef !4
  %0 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %_6 = load ptr, ptr %0, align 8, !nonnull !4, !align !5, !noundef !4
  %_3 = load i32, ptr %_6, align 4, !noundef !4
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_2, i32 %_3)
  %_4.0 = extractvalue { i32, i1 } %1, 0
  %_4.1 = extractvalue { i32, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false)
  br i1 %2, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i32 %_4.0

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h0ead933cb8f56d66E(ptr align 1 @str.1, i64 28, ptr align 8 @alloc_a509a95d4d7ba9e1b62e0e7c06cae8aa) #7
  unreachable
}

; main::main::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4main4main28_$u7b$$u7b$closure$u7d$$u7d$17hdaac0a0826ac2d36E"(ptr align 8 %_1, i32 %a) unnamed_addr #2 {
start:
  %_6 = load ptr, ptr %_1, align 8, !nonnull !4, !align !5, !noundef !4
  %_3 = load i32, ptr %_6, align 4, !noundef !4
  %0 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %_7 = load ptr, ptr %0, align 8, !nonnull !4, !align !5, !noundef !4
  %_4 = load i32, ptr %_7, align 4, !noundef !4
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_3, i32 %_4)
  %_5.0 = extractvalue { i32, i1 } %1, 0
  %_5.1 = extractvalue { i32, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_5.1, i1 false)
  br i1 %2, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i32 %_5.0

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h0ead933cb8f56d66E(ptr align 1 @str.1, i64 28, ptr align 8 @alloc_063acf5217603408362721fa974b1dda) #7
  unreachable
}

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h76f3e81e6b8f13f9E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #4

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h0ead933cb8f56d66E(ptr align 1, i64, ptr align 8) unnamed_addr #5

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #6 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hc1bc80fbbc2f1194E(ptr @_ZN4main4main17hac82363469e1a7a2E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #5 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { nonlazybind "target-cpu"="x86-64" }
attributes #7 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{i32 1149121}
!4 = !{}
!5 = !{i64 4}
