extern void* malloc(unsigned long size);
extern void free(void* ptr);
extern int printf(const char* fmt);

int main(int argc, char **argv) {
    printf("[+] C ABI Test Started\n");
    
    char* data = malloc(32);
    data[0] = 'K'; data[1] = 'N'; data[2] = 'U'; data[3] = ' ';
    data[4] = 'L'; data[5] = 'I'; data[6] = 'B'; data[7] = 'C';
    data[8] = '\n'; data[9] = 0;
    
    printf("[+] Malloc success! Data: ");
    printf(data);
    
    free(data);
    printf("[+] Free success! Exiting...\n");
    
    return 0;
}
