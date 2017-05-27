#include <stdio.h>
#include <patronus/patronus.h>

int main() {
    char* text = "Tou manny misteaks woudl confuez an horz.";

    printf("Checking '%s'\n\n", text);

    Patronus* checker = patronus_create();
    PatronusProperties properties = {
        .primary_language = "en",
    };
    PatronusAnnotationArray * anns = patronus_check(checker, &properties, text);

    printf("Number of annotations: %lu\n\n", anns->len);

    for (uintptr_t i = 0; i < anns->len; ++i) {
        PatronusAnnotation ann = anns->data[i];
        printf("Offset: %lu\n", ann.offset);
        printf("Length: %lu\n", ann.length);
        printf("Message: %s\n", ann.message);
        PatronusSuggestionArray* suggs = ann.suggestions;
        if (suggs->len > 0) {
            printf("Suggestions:\n");
            for (uintptr_t s = 0; s < suggs->len; ++s) {
                printf(" - %s\n", suggs->data[s]);
            }
        }
        printf("\n");
    }

    patronus_free_annotations(anns);
    patronus_free(checker);
}
