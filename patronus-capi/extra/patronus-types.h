typedef enum AnnotationKind {
    Spelling = 1,
    Grammar = 2,
    Style = 3,
    Typography = 4,
    Suggestion = 5,
} AnnotationKind;

typedef struct Annotation* AnnotationPtr;
typedef struct Suggestion* SuggestionPtr;

typedef struct AnnotationArray {
    AnnotationPtr data;
    size_t len;
    void* extra;
    void (*cleanup)(AnnotationPtr, size_t, void*);
} AnnotationArray;

typedef struct SuggestionArray {
    char const* const* data;
    size_t len;
    void* extra;
    void (*cleanup)(SuggestionPtr, size_t, void*);
} SuggestionArray;

typedef struct Annotation {
    uintptr_t offset;
    uintptr_t length;
    char const* message;
    AnnotationKind kind;
    SuggestionArray* suggestions;
} Annotation;

typedef struct Properties {
     char const* primary_language;
} Properties;
