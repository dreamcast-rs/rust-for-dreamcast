#include <stdio.h>

void rust_msg();

void print_message(char *chars, size_t len) {
    printf("%.*s", len, chars);
}

int main(int argc, char *argv[]) {
    rust_msg();
    return 0;
}
