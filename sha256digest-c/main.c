#include <stdio.h>

extern char* sha256_digest(char* str);

int main(){
    char *result = sha256_digest("Learn Rust Programming");
    printf("SHA256 digest of \"Learn Rust Programming\": %s", result);

    return 0;
}