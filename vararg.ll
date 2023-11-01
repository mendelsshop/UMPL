; ModuleID = 'vararg.c'
source_filename = "vararg.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct.callInfo = type { i32, ptr }
%struct.llNode = type { i32, ptr }
%struct.treeNode = type { i32, ptr, ptr }

@jmp = dso_local global ptr null, align 8
@.str = private unnamed_addr constant [14 x i8] c"arrity error\0A\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"%d, \00", align 1
@.str.2 = private unnamed_addr constant [21 x i8] c"you skipped printing\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main(i32 noundef %0, ptr noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca ptr, align 8
  %5 = alloca %struct.callInfo, align 8
  %6 = alloca ptr, align 8
  store i32 %0, ptr %3, align 4
  store ptr %1, ptr %4, align 8
  %7 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 0
  %8 = load i32, ptr %7, align 8
  %9 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 1
  %10 = load ptr, ptr %9, align 8
  %11 = call i32 @printff(i32 %8, ptr %10, ptr noundef null)
  %12 = load ptr, ptr @jmp, align 8
  %13 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 1
  store ptr %12, ptr %13, align 8
  %14 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 0
  %15 = load i32, ptr %14, align 8
  %16 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 1
  %17 = load ptr, ptr %16, align 8
  %18 = call i32 @printff(i32 %15, ptr %17, ptr noundef null)
  %19 = call noalias ptr @malloc(i64 noundef 24) #5
  store ptr %19, ptr %6, align 8
  %20 = load ptr, ptr %6, align 8
  %21 = getelementptr inbounds %struct.llNode, ptr %20, i32 0, i32 0
  store i32 1, ptr %21, align 8
  %22 = call noalias ptr @malloc(i64 noundef 24) #5
  %23 = load ptr, ptr %6, align 8
  %24 = getelementptr inbounds %struct.llNode, ptr %23, i32 0, i32 1
  store ptr %22, ptr %24, align 8
  %25 = load ptr, ptr %6, align 8
  %26 = getelementptr inbounds %struct.llNode, ptr %25, i32 0, i32 1
  %27 = load ptr, ptr %26, align 8
  %28 = getelementptr inbounds %struct.llNode, ptr %27, i32 0, i32 0
  store i32 2, ptr %28, align 8
  %29 = call noalias ptr @malloc(i64 noundef 24) #5
  %30 = load ptr, ptr %6, align 8
  %31 = getelementptr inbounds %struct.llNode, ptr %30, i32 0, i32 1
  %32 = load ptr, ptr %31, align 8
  %33 = getelementptr inbounds %struct.llNode, ptr %32, i32 0, i32 1
  store ptr %29, ptr %33, align 8
  %34 = load ptr, ptr %6, align 8
  %35 = getelementptr inbounds %struct.llNode, ptr %34, i32 0, i32 1
  %36 = load ptr, ptr %35, align 8
  %37 = getelementptr inbounds %struct.llNode, ptr %36, i32 0, i32 1
  %38 = load ptr, ptr %37, align 8
  %39 = getelementptr inbounds %struct.llNode, ptr %38, i32 0, i32 0
  store i32 3, ptr %39, align 8
  %40 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 0
  store i32 3, ptr %40, align 8
  %41 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 1
  store ptr null, ptr %41, align 8
  %42 = load ptr, ptr %6, align 8
  %43 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 0
  %44 = load i32, ptr %43, align 8
  %45 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 1
  %46 = load ptr, ptr %45, align 8
  %47 = call i32 @printff(i32 %44, ptr %46, ptr noundef %42)
  ret i32 0
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @printff(i32 %0, ptr %1, ptr noundef %2) #0 {
  %4 = alloca i32, align 4
  %5 = alloca %struct.callInfo, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i32, align 4
  %8 = alloca ptr, align 8
  %9 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 0
  store i32 %0, ptr %9, align 8
  %10 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 1
  store ptr %1, ptr %10, align 8
  store ptr %2, ptr %6, align 8
  %11 = load ptr, ptr @jmp, align 8
  %12 = icmp eq ptr %11, null
  br i1 %12, label %13, label %14

13:                                               ; preds = %3
  store ptr blockaddress(@printff, %42), ptr @jmp, align 8
  store i32 0, ptr %4, align 4
  br label %44

14:                                               ; preds = %3
  %15 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 1
  %16 = load ptr, ptr %15, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %18, label %21

18:                                               ; preds = %14
  %19 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 1
  %20 = load ptr, ptr %19, align 8
  br label %46

21:                                               ; preds = %14
  %22 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 0
  %23 = load i32, ptr %22, align 8
  %24 = icmp slt i32 %23, 1
  br i1 %24, label %25, label %27

25:                                               ; preds = %21
  %26 = call i32 (ptr, ...) @printf(ptr noundef @.str)
  call void @exit(i32 noundef 1) #6
  unreachable

27:                                               ; preds = %21
  %28 = load ptr, ptr %6, align 8
  %29 = getelementptr inbounds %struct.llNode, ptr %28, i32 0, i32 0
  %30 = load i32, ptr %29, align 8
  store i32 %30, ptr %7, align 4
  %31 = load ptr, ptr %6, align 8
  %32 = getelementptr inbounds %struct.llNode, ptr %31, i32 0, i32 1
  %33 = load ptr, ptr %32, align 8
  store ptr %33, ptr %6, align 8
  %34 = load i32, ptr %7, align 4
  %35 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, i32 noundef %34)
  %36 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 0
  %37 = load i32, ptr %36, align 8
  %38 = sub nsw i32 %37, 1
  %39 = load ptr, ptr %6, align 8
  %40 = call ptr @proccess_var_args(i32 noundef %38, ptr noundef %39)
  store ptr %40, ptr %8, align 8
  %41 = load ptr, ptr %8, align 8
  call void @print_tree_args(ptr noundef %41)
  store i32 0, ptr %4, align 4
  br label %44

42:                                               ; preds = %46
  %43 = call i32 (ptr, ...) @printf(ptr noundef @.str.2)
  store i32 1, ptr %4, align 4
  br label %44

44:                                               ; preds = %42, %27, %13
  %45 = load i32, ptr %4, align 4
  ret i32 %45

46:                                               ; preds = %18
  %47 = phi ptr [ %20, %18 ]
  indirectbr ptr %47, [label %42]
}

; Function Attrs: nounwind allocsize(0)
declare noalias ptr @malloc(i64 noundef) #1

declare i32 @printf(ptr noundef, ...) #2

; Function Attrs: noreturn nounwind
declare void @exit(i32 noundef) #3

; Function Attrs: noinline nounwind optnone uwtable
define dso_local ptr @proccess_var_args(i32 noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca i32, align 4
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  store i32 %0, ptr %4, align 4
  store ptr %1, ptr %5, align 8
  %7 = load i32, ptr %4, align 4
  %8 = icmp sle i32 %7, 0
  br i1 %8, label %9, label %10

9:                                                ; preds = %2
  store ptr null, ptr %3, align 8
  br label %40

10:                                               ; preds = %2
  %11 = load i32, ptr %4, align 4
  %12 = sdiv i32 %11, 2
  store i32 %12, ptr %4, align 4
  %13 = call noalias ptr @malloc(i64 noundef 24) #5
  store ptr %13, ptr %6, align 8
  %14 = load i32, ptr %4, align 4
  %15 = load ptr, ptr %5, align 8
  %16 = call ptr @proccess_var_args(i32 noundef %14, ptr noundef %15)
  %17 = load ptr, ptr %6, align 8
  %18 = getelementptr inbounds %struct.treeNode, ptr %17, i32 0, i32 1
  store ptr %16, ptr %18, align 8
  %19 = load ptr, ptr %5, align 8
  %20 = getelementptr inbounds %struct.llNode, ptr %19, i32 0, i32 0
  %21 = load i32, ptr %20, align 8
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.treeNode, ptr %22, i32 0, i32 0
  store i32 %21, ptr %23, align 8
  %24 = load ptr, ptr %5, align 8
  %25 = getelementptr inbounds %struct.llNode, ptr %24, i32 0, i32 1
  %26 = load ptr, ptr %25, align 8
  %27 = icmp eq ptr %26, null
  br i1 %27, label %28, label %29

28:                                               ; preds = %10
  store ptr null, ptr %3, align 8
  br label %40

29:                                               ; preds = %10
  %30 = load ptr, ptr %5, align 8
  %31 = load ptr, ptr %5, align 8
  %32 = getelementptr inbounds %struct.llNode, ptr %31, i32 0, i32 1
  %33 = load ptr, ptr %32, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %30, ptr align 8 %33, i64 16, i1 false)
  %34 = load i32, ptr %4, align 4
  %35 = load ptr, ptr %5, align 8
  %36 = call ptr @proccess_var_args(i32 noundef %34, ptr noundef %35)
  %37 = load ptr, ptr %6, align 8
  %38 = getelementptr inbounds %struct.treeNode, ptr %37, i32 0, i32 2
  store ptr %36, ptr %38, align 8
  %39 = load ptr, ptr %6, align 8
  store ptr %39, ptr %3, align 8
  br label %40

40:                                               ; preds = %29, %28, %9
  %41 = load ptr, ptr %3, align 8
  ret ptr %41
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @print_tree_args(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = icmp eq ptr %3, null
  br i1 %4, label %5, label %6

5:                                                ; preds = %1
  br label %17

6:                                                ; preds = %1
  %7 = load ptr, ptr %2, align 8
  %8 = getelementptr inbounds %struct.treeNode, ptr %7, i32 0, i32 1
  %9 = load ptr, ptr %8, align 8
  call void @print_tree_args(ptr noundef %9)
  %10 = load ptr, ptr %2, align 8
  %11 = getelementptr inbounds %struct.treeNode, ptr %10, i32 0, i32 0
  %12 = load i32, ptr %11, align 8
  %13 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, i32 noundef %12)
  %14 = load ptr, ptr %2, align 8
  %15 = getelementptr inbounds %struct.treeNode, ptr %14, i32 0, i32 2
  %16 = load ptr, ptr %15, align 8
  call void @print_tree_args(ptr noundef %16)
  br label %17

17:                                               ; preds = %6, %5
  ret void
}

; Function Attrs: argmemonly nocallback nofree nounwind willreturn
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #4

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind allocsize(0) "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { noreturn nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { argmemonly nocallback nofree nounwind willreturn }
attributes #5 = { nounwind allocsize(0) }
attributes #6 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"Debian clang version 15.0.6"}
