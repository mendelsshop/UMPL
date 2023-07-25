; ModuleID = 'repl'
source_filename = "repl"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"


declare i32 @printf(i8*, ...)

define i32 @rain(i8* %blk )   {
entry:
  %cond = icmp eq i8* %blk, null
  br i1 %cond, label %cont, label %excep
cont:
  %c = call i32 @other()
  %d = add i32 %c, 1
  ret i32 %d

excep:

  indirectbr i8* %blk, [label %error]
  ; ret i32 7

error:                     
  ret i32 5
}

define i32 @other() {
other:
    %c = call i32 @rain(i8* null)
    ; %c = call i32 @rain(i8* blockaddress(@rain, %error))
    ret i32 %c
}

define i32 @main() {
entry:
  %r = call i32 @rain(i8* null)
  ret i32 %r
}   

