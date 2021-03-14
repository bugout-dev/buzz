#include "buzz.h"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        return 1;
    }
    FILE* ifp = fopen(argv[1], "r");
    TagPatternList* patterns = load_patterns_from_file(ifp);
    TagPatternList* head = patterns;
    while (patterns != NULL) {
        print_tag_pattern(patterns->tag_pattern);
        patterns = patterns->next;
    }
    destroy(head);
    fclose(ifp);
};
