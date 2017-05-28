typedef struct Annotation Annotation;
typedef struct Suggestion Suggestion;
typedef struct AnnotationArray AnnotationArray;
typedef struct SuggestionArray SuggestionArray;
typedef struct Properties Properties;

typedef enum AnnotationKind {
    Spelling = 1,
    Grammar = 2,
    Style = 3,
    Typography = 4,
    Suggestion = 5,
} AnnotationKind;

struct AnnotationArray {
    Annotation* data;
    size_t len;
    void* extra;
    void (*cleanup)(Annotation*, size_t, void*);
};

struct SuggestionArray {
    char const* const* data;
    size_t len;
    void* extra;
    void (*cleanup)(Suggestion*, size_t, void*);
};

struct Annotation {
    uintptr_t offset;
    uintptr_t length;
    char const* message;
    AnnotationKind kind;
    SuggestionArray* suggestions;
};

struct Properties {
     char const* primary_language;
};
