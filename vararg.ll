; ModuleID = 'vararg.c'
source_filename = "vararg.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct.callInfo = type { i32, ptr }
%struct.llNode = type { i32, ptr }
%struct.helper = type { ptr, ptr }
%struct.treeNode = type { i32, ptr, ptr }

@.str = private unnamed_addr constant [14 x i8] c"arrity error\0A\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"%d, \00", align 1
@jmp = dso_local global ptr null, align 8

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
  %12 = call noalias ptr @malloc(i64 noundef 24) #4
  store ptr %12, ptr %6, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = getelementptr inbounds %struct.llNode, ptr %13, i32 0, i32 0
  store i32 2, ptr %14, align 8
  %15 = call noalias ptr @malloc(i64 noundef 24) #4
  %16 = load ptr, ptr %6, align 8
  %17 = getelementptr inbounds %struct.llNode, ptr %16, i32 0, i32 1
  store ptr %15, ptr %17, align 8
  %18 = load ptr, ptr %6, align 8
  %19 = getelementptr inbounds %struct.llNode, ptr %18, i32 0, i32 1
  %20 = load ptr, ptr %19, align 8
  %21 = getelementptr inbounds %struct.llNode, ptr %20, i32 0, i32 0
  store i32 3, ptr %21, align 8
  %22 = call noalias ptr @malloc(i64 noundef 24) #4
  %23 = load ptr, ptr %6, align 8
  %24 = getelementptr inbounds %struct.llNode, ptr %23, i32 0, i32 1
  %25 = load ptr, ptr %24, align 8
  %26 = getelementptr inbounds %struct.llNode, ptr %25, i32 0, i32 1
  store ptr %22, ptr %26, align 8
  %27 = load ptr, ptr %6, align 8
  %28 = getelementptr inbounds %struct.llNode, ptr %27, i32 0, i32 1
  %29 = load ptr, ptr %28, align 8
  %30 = getelementptr inbounds %struct.llNode, ptr %29, i32 0, i32 1
  %31 = load ptr, ptr %30, align 8
  %32 = getelementptr inbounds %struct.llNode, ptr %31, i32 0, i32 0
  store i32 4, ptr %32, align 8
  %33 = getelementptr inbounds %struct.callInfo, ptr %5, i32 0, i32 0
  store i32 3, ptr %33, align 8
  %34 = load ptr, ptr %6, align 8
  %35 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 0
  %36 = load i32, ptr %35, align 8
  %37 = getelementptr inbounds { i32, ptr }, ptr %5, i32 0, i32 1
  %38 = load ptr, ptr %37, align 8
  %39 = call i32 @printff(i32 %36, ptr %38, ptr noundef %34)
  ret i32 0
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @printff(i32 %0, ptr %1, ptr noundef %2) #0 {
  %4 = alloca %struct.callInfo, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca %struct.helper, align 8
  %8 = getelementptr inbounds { i32, ptr }, ptr %4, i32 0, i32 0
  store i32 %0, ptr %8, align 8
  %9 = getelementptr inbounds { i32, ptr }, ptr %4, i32 0, i32 1
  store ptr %1, ptr %9, align 8
  store ptr %2, ptr %5, align 8
  %10 = getelementptr inbounds %struct.callInfo, ptr %4, i32 0, i32 0
  %11 = load i32, ptr %10, align 8
  %12 = icmp slt i32 %11, 0
  br i1 %12, label %13, label %15

13:                                               ; preds = %3
  %14 = call i32 (ptr, ...) @printf(ptr noundef @.str)
  call void @exit(i32 noundef 1) #5
  unreachable

15:                                               ; preds = %3
  %16 = getelementptr inbounds %struct.callInfo, ptr %4, i32 0, i32 0
  %17 = load i32, ptr %16, align 8
  %18 = load ptr, ptr %5, align 8
  %19 = call { ptr, ptr } @proccess_var_args(i32 noundef %17, ptr noundef %18)
  %20 = getelementptr inbounds { ptr, ptr }, ptr %7, i32 0, i32 0
  %21 = extractvalue { ptr, ptr } %19, 0
  store ptr %21, ptr %20, align 8
  %22 = getelementptr inbounds { ptr, ptr }, ptr %7, i32 0, i32 1
  %23 = extractvalue { ptr, ptr } %19, 1
  store ptr %23, ptr %22, align 8
  %24 = getelementptr inbounds %struct.helper, ptr %7, i32 0, i32 0
  %25 = load ptr, ptr %24, align 8
  store ptr %25, ptr %6, align 8
  %26 = load ptr, ptr %6, align 8
  call void @print_tree_args(ptr noundef %26)
  ret i32 0
}

; Function Attrs: nounwind allocsize(0)
declare noalias ptr @malloc(i64 noundef) #1

declare i32 @printf(ptr noundef, ...) #2

; Function Attrs: noreturn nounwind
declare void @exit(i32 noundef) #3

; Function Attrs: noinline nounwind optnone uwtable
define dso_local { ptr, ptr } @proccess_var_args(i32 noundef %0, ptr noundef %1) #0 {
  %3 = alloca %struct.helper, align 8
  %4 = alloca i32, align 4
  %5 = alloca ptr, align 8
  %6 = alloca i32, align 4
  %7 = alloca ptr, align 8
  %8 = alloca %struct.helper, align 8
  %9 = alloca %struct.helper, align 8
  store i32 %0, ptr %4, align 4
  store ptr %1, ptr %5, align 8
  %10 = load i32, ptr %4, align 4
  %11 = icmp sle i32 %10, 0
  br i1 %11, label %12, label %16

12:                                               ; preds = %2
  %13 = getelementptr inbounds %struct.helper, ptr %3, i32 0, i32 0
  store ptr null, ptr %13, align 8
  %14 = getelementptr inbounds %struct.helper, ptr %3, i32 0, i32 1
  %15 = load ptr, ptr %5, align 8
  store ptr %15, ptr %14, align 8
  br label %59

16:                                               ; preds = %2
  %17 = load i32, ptr %4, align 4
  %18 = sdiv i32 %17, 2
  store i32 %18, ptr %6, align 4
  %19 = call noalias ptr @malloc(i64 noundef 24) #4
  store ptr %19, ptr %7, align 8
  %20 = load i32, ptr %6, align 4
  %21 = load ptr, ptr %5, align 8
  %22 = call { ptr, ptr } @proccess_var_args(i32 noundef %20, ptr noundef %21)
  %23 = getelementptr inbounds { ptr, ptr }, ptr %8, i32 0, i32 0
  %24 = extractvalue { ptr, ptr } %22, 0
  store ptr %24, ptr %23, align 8
  %25 = getelementptr inbounds { ptr, ptr }, ptr %8, i32 0, i32 1
  %26 = extractvalue { ptr, ptr } %22, 1
  store ptr %26, ptr %25, align 8
  %27 = getelementptr inbounds %struct.helper, ptr %8, i32 0, i32 0
  %28 = load ptr, ptr %27, align 8
  %29 = load ptr, ptr %7, align 8
  %30 = getelementptr inbounds %struct.treeNode, ptr %29, i32 0, i32 1
  store ptr %28, ptr %30, align 8
  %31 = getelementptr inbounds %struct.helper, ptr %8, i32 0, i32 1
  %32 = load ptr, ptr %31, align 8
  %33 = getelementptr inbounds %struct.llNode, ptr %32, i32 0, i32 0
  %34 = load i32, ptr %33, align 8
  %35 = load ptr, ptr %7, align 8
  %36 = getelementptr inbounds %struct.treeNode, ptr %35, i32 0, i32 0
  store i32 %34, ptr %36, align 8
  %37 = load i32, ptr %4, align 4
  %38 = load i32, ptr %6, align 4
  %39 = sub nsw i32 %37, %38
  %40 = sub nsw i32 %39, 1
  %41 = getelementptr inbounds %struct.helper, ptr %8, i32 0, i32 1
  %42 = load ptr, ptr %41, align 8
  %43 = getelementptr inbounds %struct.llNode, ptr %42, i32 0, i32 1
  %44 = load ptr, ptr %43, align 8
  %45 = call { ptr, ptr } @proccess_var_args(i32 noundef %40, ptr noundef %44)
  %46 = getelementptr inbounds { ptr, ptr }, ptr %9, i32 0, i32 0
  %47 = extractvalue { ptr, ptr } %45, 0
  store ptr %47, ptr %46, align 8
  %48 = getelementptr inbounds { ptr, ptr }, ptr %9, i32 0, i32 1
  %49 = extractvalue { ptr, ptr } %45, 1
  store ptr %49, ptr %48, align 8
  %50 = getelementptr inbounds %struct.helper, ptr %9, i32 0, i32 0
  %51 = load ptr, ptr %50, align 8
  %52 = load ptr, ptr %7, align 8
  %53 = getelementptr inbounds %struct.treeNode, ptr %52, i32 0, i32 2
  store ptr %51, ptr %53, align 8
  %54 = getelementptr inbounds %struct.helper, ptr %3, i32 0, i32 0
  %55 = load ptr, ptr %7, align 8
  store ptr %55, ptr %54, align 8
  %56 = getelementptr inbounds %struct.helper, ptr %3, i32 0, i32 1
  %57 = getelementptr inbounds %struct.helper, ptr %9, i32 0, i32 1
  %58 = load ptr, ptr %57, align 8
  store ptr %58, ptr %56, align 8
  br label %59

59:                                               ; preds = %16, %12
  %60 = load { ptr, ptr }, ptr %3, align 8
  ret { ptr, ptr } %60
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

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind allocsize(0) "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { noreturn nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { nounwind allocsize(0) }
attributes #5 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"Debian clang version 15.0.6"}
