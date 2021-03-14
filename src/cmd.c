#include <stdbool.h>
#include <string.h>

#include "buzz.h"

void buzz_result_json(char* tag, TagPattern* tag_pattern, bool match_only, bool newline) {
    BuzzResult result = process_tag(tag, tag_pattern);
    if (!match_only || result.match) {
        printf(
            "{\"tag\": \"%s\", \"pattern\": \"%s\", \"match\": %d, \"capture_start\": %d, \"capture_end\": %d}",
            tag,
            tag_pattern->pattern,
            result.match,
            result.capture_start,
            result.capture_end
        );
        if (newline) {
            printf("\n");
        }
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        return 1;
    }

    FILE* ifp = fopen(argv[1], "r");
    TagPatternList* patterns = load_patterns_from_file(ifp);
    fclose(ifp);

    TagPatternList* current_pattern = patterns;

    if (argc > 2) {
        int current_tag = 2;
        while (current_tag < argc) {
            current_pattern = patterns;
            while (current_pattern != NULL) {
                buzz_result_json(argv[current_tag], &current_pattern->tag_pattern, true, true);
                current_pattern = current_pattern->next;
            }
            current_tag++;
        }
    } else {
        int MAX_TAG_SIZE = 512;
        char buffer[MAX_TAG_SIZE];
        while(fgets(buffer, MAX_TAG_SIZE - 1, stdin)) {
            buffer[strcspn(buffer, "\r\n")] = '\0';
            current_pattern = patterns;
            while (current_pattern != NULL) {
                buzz_result_json(buffer, &current_pattern->tag_pattern, true, true);
                current_pattern = current_pattern->next;
            }
        }
    }

    destroy(patterns);
};
