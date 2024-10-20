#include <stdio.h>
#include <stdlib.h>

extern int print(char* arg0) {
    printf("%s", arg0);
    return 0;
}

extern int println(char* arg0) {
    printf("%s\n", arg0);
    return 0;
}

extern int readInt() {
    int num;
    if (scanf("%d", &num) != 1) {
        return -1; 
    }
    return num;
}

extern int readFloat() {
    float num;
    if (scanf("%f", &num) != 1) {
        return -1.0; 
    }
    
    printf("%f", num);
    
    return num;
}

extern char* readLine() {
    char *line = (char *)malloc(100);
    char *linep = line;
    line = line + sizeof(int64_t);
    size_t lenmax = 100, len = lenmax;
    size_t size = 0;

    int c;

    if (line == NULL)
        return NULL;

    for (;;) {
        c = fgetc(stdin);
        if (c == EOF)
            break;

        if (--len == 0) {
            len = lenmax;
            char *linen = (char *) realloc(linep, lenmax *= 2);

            if (linen == NULL) {
                free(linep);
                return NULL;
            }
            line = linen + (line - linep);
            linep = linen;
        }

        size++;
        if ((*line++ = c) == '\n')
            break;
    }
    *line = '\0';
    size++;
    *((int64_t *) (linep)) = size;

    char *linen = (char *) realloc(linep, size + sizeof(int64_t));
    if (linen == NULL) {
        free(linep);
        return NULL;
    }
    line = linen + (line - linep);
    linep = linen;

    return linep + sizeof(int64_t);
}