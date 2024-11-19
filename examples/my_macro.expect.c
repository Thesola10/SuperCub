// Prepared by SuperCub.

#line 0 "(generated)"
char *my_path(void);

#line 4 "my_macro.h"
typedef enum {
    GET, POST, PUSH, DELETE
} http_method_t;

typedef struct {
    char *(*handler)();
    http_method_t method;
    char *path;
} uri_handler_t;

#line 59 "my_macro.h"
uri_handler_t handlers[] = {
    { &my_path, GET, "/" }
};

#line 12 "my_macro.cub"
#include <stdio.h>
#include <string.h>

#line 20 "my_macro.cub"
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
