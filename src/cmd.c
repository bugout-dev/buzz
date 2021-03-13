#include "buzz.h"

int main(int argc, char* argv[]) {
    for (int i = 1; i < argc; i++) {
        TagPattern tag_pattern = read_pattern(argv[i]);
        print_tag_pattern(tag_pattern);
    }
    return 0;
};
