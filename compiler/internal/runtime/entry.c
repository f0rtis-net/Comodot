#include <stdio.h>

#if defined (__APPLE__) 
    #include <pthread.h>
#endif

extern int language_main();

int main(int argc, char** argv) {
    printf("there will be runtime...\n");
    
    return language_main();
}