#include "buzz.h"

int main(int argc, char* argv[]) {
    if (argc > 1) {
        FILE* ifp = fopen(argv[1], "r");
        int batch_size = 2;
        while (1) {
            TagPatternList patterns = read_patterns(ifp, batch_size);
            int items = 0;
            for (int i = 0; i < patterns.length; i++) {
                print_tag_pattern(patterns.items[i]);
                items++;
            }
            if (items < batch_size) {
                break;
            }
        }
        fclose(ifp);
    }
};
