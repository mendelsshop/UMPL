; ModuleID = 'repl'
source_filename = "repl"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

%object = type { i8, i1, double, ptr, ptr, { ptr, ptr }, ptr }

@"error exit" = private unnamed_addr constant [16 x i8] c"not a function\0A\00", align 1
@str = private unnamed_addr constant [15 x i8] c"not a function\00", align 1
@"error exit.1" = private unnamed_addr constant [14 x i8] c"not a string\0A\00", align 1
@str.2 = private unnamed_addr constant [13 x i8] c"not a string\00", align 1
@"error exit.3" = private unnamed_addr constant [15 x i8] c"not a boolean\0A\00", align 1
@str.4 = private unnamed_addr constant [14 x i8] c"not a boolean\00", align 1
@"error exit.5" = private unnamed_addr constant [14 x i8] c"not a number\0A\00", align 1
@str.6 = private unnamed_addr constant [13 x i8] c"not a number\00", align 1
@"error exit.7" = private unnamed_addr constant [18 x i8] c"not a valid type\0A\00", align 1
@"boolean fmt specifier" = private unnamed_addr constant [3 x i8] c"%i\00", align 1
@"number fmt specifier" = private unnamed_addr constant [3 x i8] c"%f\00", align 1
@"string fmt specifier" = private unnamed_addr constant [3 x i8] c"%s\00", align 1
@str.8 = private unnamed_addr constant [17 x i8] c"not a valid type\00", align 1
@globalsb = global [0 x %object] []
@globalsb.9 = global [5 x %object] [%object zeroinitializer,%object zeroinitializer,%object zeroinitializer,%object zeroinitializer,%object zeroinitializer]

declare void @exit(i32)

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %extract-function = call { ptr, ptr } @extract_function(%object { i8 4, i1 false, double 0.000000e+00, ptr null, ptr null, { ptr, ptr } { ptr @lambda, ptr null }, ptr null })
  %"function load" = extractvalue { ptr, ptr } %extract-function, 0
  %"function env load" = extractvalue { ptr, ptr } %extract-function, 1
  %"application:call" = call %object (ptr, %object, ...) %"function load"(ptr %"function env load", %object { i8 1, i1 false, double 5.000000e+00, ptr null, ptr null, { ptr, ptr } zeroinitializer, ptr null }, %object { i8 1, i1 false, double 6.000000e+00, ptr null, ptr null, { ptr, ptr } zeroinitializer, ptr null })
  %extract-function3 = call { ptr, ptr } @extract_function(%object %"application:call")
  %"function load4" = extractvalue { ptr, ptr } %extract-function3, 0
  %"function env load5" = extractvalue { ptr, ptr } %extract-function3, 1
 %"application:call6" = call %object (ptr, %object, ...) %"function load4"(ptr %"function env load5", %object { i8 0, i1 true, double 0.000000e+00, ptr null, ptr null, { ptr, ptr } zeroinitializer, ptr null })
  %extract-function7 = call { ptr, ptr } @extract_function(%object { i8 4, i1 false, double 0.000000e+00, ptr null, ptr null, { ptr, ptr } { ptr @print, ptr null }, ptr null })
  %"function load8" = extractvalue { ptr, ptr } %extract-function7, 0
  %"function env load9" = extractvalue { ptr, ptr } %extract-function7, 1
  %"application:call10" = call %object (ptr, %object, ...) %"function load8"(ptr %"function env load9", %object %"application:call6")
  ret i32 0
}

define { ptr, ptr } @extract_function(%object %0) {
"extract-function:entry":
  %get_type = extractvalue %object %0, 0
  %"extract-function:cmp-type" = icmp eq i8 %get_type, 4
  br i1 %"extract-function:cmp-type", label %"extract-function:return", label %"extract-function:error"

"extract-function:return":                        ; preds = %"extract-function:entry"
  %"extract-function:return1" = extractvalue %object %0, 5
  ret { ptr, ptr } %"extract-function:return1"

"extract-function:error":                         ; preds = %"extract-function:entry"
  %puts = call i32 @puts(ptr nonnull @str)
  call void @exit(i32 1)
  unreachable
}

; Function Attrs: nofree nounwind
declare noundef i32 @puts(ptr nocapture noundef readonly) #0

define ptr @extract_string(%object %0) {
"extract-string:entry":
  %get_type = extractvalue %object %0, 0
  %"extract-string:cmp-type" = icmp eq i8 %get_type, 2
  br i1 %"extract-string:cmp-type", label %"extract-string:return", label %"extract-string:error"

"extract-string:return":                          ; preds = %"extract-string:entry"
  %"extract-string:return1" = extractvalue %object %0, 3
  ret ptr %"extract-string:return1"

"extract-string:error":                           ; preds = %"extract-string:entry"
  %puts = call i32 @puts(ptr nonnull @str.2)
  call void @exit(i32 1)
  unreachable
}

define i1 @extract_boolean(%object %0) {
"extract-boolean:entry":
  %get_type = extractvalue %object %0, 0
  %"extract-boolean:cmp-type" = icmp eq i8 %get_type, 0
  br i1 %"extract-boolean:cmp-type", label %"extract-boolean:return", label %"extract-boolean:error"

"extract-boolean:return":                         ; preds = %"extract-boolean:entry"
  %"extract-boolean:return1" = extractvalue %object %0, 1
  ret i1 %"extract-boolean:return1"

"extract-boolean:error":                          ; preds = %"extract-boolean:entry"
  %puts = call i32 @puts(ptr nonnull @str.4)
  call void @exit(i32 1)
  unreachable
}

define double @extract_number(%object %0) {
"extract-number:entry":
  %get_type = extractvalue %object %0, 0
  %"extract-number:cmp-type" = icmp eq i8 %get_type, 1
  br i1 %"extract-number:cmp-type", label %"extract-number:return", label %"extract-number:error"

"extract-number:return":                          ; preds = %"extract-number:entry"
  %"extract-number:return1" = extractvalue %object %0, 2
  ret double %"extract-number:return1"

"extract-number:error":                           ; preds = %"extract-number:entry"
  %puts = call i32 @puts(ptr nonnull @str.6)
  call void @exit(i32 1)
  unreachable
}

define %object @print(ptr %0, %object %1) {
entry:
  %get_type = extractvalue %object %1, 0
  switch i8 %get_type, label %error [
    i8 0, label %bool
    i8 1, label %number
    i8 2, label %string
  ]

bool:                                             ; preds = %entry
  %extract-bool = call i1 @extract_boolean(%object %1)
  %"print boolean" = call i32 (ptr, ...) @printf(ptr noundef nonnull @"boolean fmt specifier", i1 %extract-bool)
  br label %return

number:                                           ; preds = %entry
  %extract-number = call double @extract_number(%object %1)
  %"print number" = call i32 (ptr, ...) @printf(ptr noundef nonnull @"number fmt specifier", double %extract-number)
  br label %return

string:                                           ; preds = %entry
  %extract-string = call ptr @extract_string(%object %1)
  %"print string" = call i32 (ptr, ...) @printf(ptr noundef nonnull @"string fmt specifier", ptr %extract-string)
  br label %return

return:                                           ; preds = %string, %number, %bool
  ret %object %1

error:                                            ; preds = %entry
  %puts = call i32 @puts(ptr nonnull @str.8)
  call void @exit(i32 1)
  unreachable
}

define %object @lambda(ptr %0, %object %"0", %object %"1") {
entry:
  %"12" = alloca %object, align 8
  store %object %"1", ptr %"12", align 8
  %"01" = alloca %object, align 8
  store %object %"0", ptr %"01", align 8
  %"load env" = load [0 x %object], ptr @globalsb, align 8
  %"03" = load %object, ptr %"01", align 8
  %x = alloca %object, align 8
  store %object %"03", ptr %x, align 8
  %"14" = load %object, ptr %"12", align 8
  %y = alloca %object, align 8
  store %object %"14", ptr %y, align 8

  ; %"env create" = load [5 x %object], ptr @globalsb.9, align 8
  %"05" = load %object, ptr %"01", align 8
   store %object %"05", ptr getelementptr inbounds ([5 x %object] , ptr @globalsb.9, i32 0, i32 0)

  %"16" = load %object, ptr %"12", align 8
    store %object %"16", ptr getelementptr inbounds ([5 x %object] , ptr @globalsb.9, i32 0, i32 1)
  ; %"env insert7" = insertvalue [5 x %object] %"env create", %object %"16", 1
  ; %"28" = load %object, ptr %"2", align 8
  ;  store %object %"28", ptr getelementptr inbounds ([5 x %object] , ptr @globalsb.9, i32 0, i32 2)
  %x10 = load %object, ptr %x, align 8
   store %object %"x10", ptr getelementptr inbounds ([5 x %object] , ptr @globalsb.9, i32 0, i32 3)
  %y12 = load %object, ptr %y, align 8
  store %object %"y12", ptr getelementptr inbounds ([5 x %object] , ptr @globalsb.9, i32 0, i32 4)
  %"env_loaded" = load [5 x %object], ptr @globalsb.9, align 8


  %env_get = extractvalue  [5 x %object] %"env_loaded", 4
      ; %c = call %object @print(ptr null, %object %env_get)
  ;     %cc = call %object @print(ptr null, %object %"16")
  ret %object { i8 4, i1 false, double 0.000000e+00, ptr null, ptr null, { ptr, ptr } { ptr @lambda.10, ptr null }, ptr null }
}

define %object @lambda.10(ptr %0, %object %"0") {
entry:
  %"17" = alloca %object, align 8
  store %object %"0", ptr %"17", align 8
  %"06" = alloca %object, align 8
  store ptr %0, ptr %"06", align 8
  %"load env" = load [5 x %object], ptr @globalsb.9, align 8
  %"01" = alloca %object, align 8
  %"load captured" = extractvalue [5 x %object] %"load env", 0
  store %object %"load captured", ptr %"01", align 8
  %"1" = alloca %object, align 8
  %"load captured2" = extractvalue [5 x %object] %"load env", 1
  store %object %"load captured2", ptr %"1", align 8
  %"2" = alloca %object, align 8
  %"load captured3" = extractvalue [5 x %object] %"load env", 2
  store %object %"load captured3", ptr %"2", align 8
  %x = alloca %object, align 8
  %"load captured4" = extractvalue [5 x %object] %"load env", 3
  store %object %"load captured4", ptr %x, align 8
  %y = alloca %object, align 8
  %"load captured5" = extractvalue [5 x %object] %"load env", 4

  store %object %"load captured5", ptr %y, align 8
  %"08" = load %object, ptr %"17", align 8

  %extract-bool = call i1 @extract_boolean(%object %"08")
  %get_type = extractvalue %object %"08", 0
  %"if:cond:boolean?" = icmp ne i8 %get_type, 0
  %"if:cond:false?" =or i1 %extract-bool, %"if:cond:boolean?"
  br i1 %"if:cond:false?", label %then, label %else

then:               
  %y10 = load %object, ptr %y, align 8
  br label %ifcont                              ; preds = %entry


else:                                             ; preds = %entry
  %x9 = load %object, ptr %x, align 8
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  %"if:phi-cont" = phi %object [ %y10, %then ], [ %x9, %else ]
  ret %object %"if:phi-cont"
}

attributes #0 = { nofree nounwind }
