; ModuleID = 'repl'
source_filename = "repl"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

@goto = private global 

declare i32 @printf(i8*, ...)

define i32 @rain(i8* %blk, label %label )   {
entry:
  %cond = icmp eq i8* null, %blk
  br i1 %cond, label %cont, label %excep
cont:
  ret i32 0

excep:
  indirectbr i8* %blk, [label %error]
  ; ret i32 7

error:                     
  ret i32 -1
}

define i32 @other() {
other:
    ; %c = call i32 @rain(i8* null)
    %c = call i32 @rain(i8* blockaddress(@rain, %error), label %error)
    ret i32 %c
}

define i32 @main() {
entry:
  %r = call i32 @other()
  ret i32 %r
}   

