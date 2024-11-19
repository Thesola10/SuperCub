char *my_path(void);

typedef enum {
    GET, POST, PUSH, DELETE
} http_method_t;

typedef struct {
    char *(*handler)();
    http_method_t method;
    char *path;
} uri_handler_t;

uri_handler_t handlers[] = {
    { &my_path, GET, "/" }
};

#include <stdio.h>
#include <string.h>

char *my_path(void)
{
    return "Hello!\n";
}

int main(int argc, char *argv[])
{
    printf("%s = %d", "2 + 15", 2 + 15);

    for (int i = 0; ; i++) {
        uri_handler_t hn = handlers[i];

        if (hn.handler == NULL)
            break;
        else if (hn.method == GET && !strcmp(hn.path, "/"))
            printf("%s\n", hn.handler());
    }
    return 0;
}
