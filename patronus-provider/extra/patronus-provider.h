#include <patronus.h>

typedef struct PatronusProvider PatronusProvider;

struct PatronusProvider {
    char const* (*name)(void);
    AnnotationArray* (*check)(char const* text, void* data);
    void (*free_annotations)(AnnotationArray* );
    void (*free_provider)(PatronusProvider* );
    void* data;
};
