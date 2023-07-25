#include <stdio.h>
#include <stdlib.h>
#include <setjmp.h>
void jmpfunction(jmp_buf env_buf);
jmp_buf env_buffer;
void other_function() {
    int val = setjmp( env_buffer );
     if( val != 0 ) {
      printf("Returned from a longjmp() with value = %s\n", val);
      exit(0);
   }
}
int main () {

    setjmp( env_buffer );
    int val = setjmp( env_buffer );
   if( val != 0 ) {
      printf("Returned from a longjmpssss() with value = %s\n", val);
   }
       other_function();
   printf("Jump function call\n");
   jmpfunction( env_buffer );
   
   return(0);
}

void jmpfunction(jmp_buf env_buf) {
   longjmp(env_buf, "tutorialspoint.com");
}