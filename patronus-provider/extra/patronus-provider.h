#include <patronus.h>

typedef struct Provider {
	char const* (*name)(void);
	AnnotationArray* (*check)(char const* text, void* data);
	void (*free_annotations)(AnnotationArray* );
	void* data;
} Provider;
