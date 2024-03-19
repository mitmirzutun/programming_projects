#include "string.h"
#include "strings.h"
#include "stdio.h"
#include "stdlib.h"
#include "stdbool.h"

typedef struct FileArray {
    unsigned int length;
    unsigned int capacity;
    FILE **data;
} Files;

void add_file(Files* files, FILE *file) {
    if (files->length >= files->capacity) {
        files->capacity <<= 1;
        files->data = realloc(files->data, sizeof(FILE *) * files->capacity);
    }
    files->data[files->length] = file;
    files->length++;
}

int main(int argc, char **argv) {
    Files files;
    files.length = 0;
    files.capacity = 16;
    files.data = malloc(16 * sizeof(FILE *));
    bool show_ends = false;
    bool number_nonblank = false;
    bool number = false;
    bool show_tabs = false;
    bool show_nonprinting = false;
    bool squeeze_blank;
    if (argc > 1) {
        for (int index = 1; index < argc; index++) {
            if (strncmp(argv[index], "--help", 10) == 0) {
                printf("Usage: cat [OPTION]... [FILE]...\n");
                printf(" -A, --show-all           equivalent to -vET\n"
                       "  -b, --number-nonblank    number nonempty output lines, overrides -n\n"
                       "  -e                       equivalent to -vE\n"
                       "  -E, --show-ends          display $ at end of each line\n"
                       "  -n, --number             number all output lines\n"
                       "  -s, --squeeze-blank      suppress repeated empty output lines\n"
                       "  -t                       equivalent to -vT\n"
                       "  -T, --show-tabs          display TAB characters as ^I\n"
                       "  -u                       (ignored)\n"
                       "  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB\n"
                       "      --help     display this help and exit\n"
                       "      --version  output version information and exit");
                printf("\n");
                return 0;
            } else if (strncmp(argv[index], "--version", 10) == 0) {
                printf("cat version 1.0");
                return 0;
            } else if (strncmp(argv[index], "-E", 3) == 0 || strncmp(argv[index], "--show-ends", 12) == 0) {
                show_ends = true;
            } else if (strncmp(argv[index], "-T", 3) == 0 || strncmp(argv[index], "--show-tabs", 12) == 0) {
                show_tabs = true;
            } else if (strncmp(argv[index], "-v", 3) == 0 || strncmp(argv[index], "--show-nonprinting", 19) == 0) {
                show_nonprinting = true;
            } else if (strncmp(argv[index], "-t", 3) == 0) {
                show_nonprinting = true;
                show_tabs = true;
            } else if (strncmp(argv[index], "-e", 3) == 0) {
                show_nonprinting = true;
                show_ends = true;
            } else if (strncmp(argv[index], "-A", 3) == 0 || strncmp(argv[index], "--show-all", 11) == 0) {
                show_nonprinting = true;
                show_ends = true;
                show_tabs = true;
            } else if (strncmp(argv[index], "-b", 3) == 0 || strncmp(argv[index], "--number-nonblank", 18) == 0) {
                number_nonblank = true;
            } else if (strncmp(argv[index], "-n", 3) == 0 || strncmp(argv[number], "--number", 9) == 0) {
                number = true;
            } else if (strncmp(argv[index], "-s", 3) == 0 || strncmp(argv[index], "--squeeze-blanks", 17) == 0) {
                squeeze_blank = true;
            } else if (strncmp(argv[index], "-", 3) == 0) {
                add_file(&files, stdin);
            } else {
                FILE *file = fopen(argv[index], "r");
                if (file == NULL) {
                    printf("Could not open file %s\n", argv[index]);
                } else {
                    add_file(&files, file);
                }
            }
        }
    }
    for (unsigned int index=0;index<files.length;index++){
        printf("Hello World!");
        char* p;
        fscanf(files.data[index],"%s",p);
        printf("%s\n",p);
        fclose(files.data[index]);
    }
    free(files.data);
}