#include <knu/klibc.h>
int main(int argc, char **argv) {
    printf("--- KNU OS Software Verification ---\n");
    printf("[1] TCC compiler check: OK\n");
    void* ptr = malloc(1024);
    if (ptr) {
        printf("[2] Dynamic memory (malloc): OK\n");
        free(ptr);
    }
    printf("[3] System Exit: OK\n");
    return 0;
}
