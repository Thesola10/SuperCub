
#ifdef __SUPER_CUB__

typedef enum {
    GET, POST, PUSH, DELETE
} http_method_t;

typedef struct {
    char *(*handler)();
    http_method_t method;
    char *path;
} uri_handler_t;

/**
 * This macro defines an attribute, which applies to a top-level function.
 *
 * Given a function: T a(b, c) { d }                [fn]
 *  $0  = T
 *  $1  = a
 *  $2  = (b, c)
 *  $3  = { d }
 *  body!($3) = d
 *
 *
 * Given a variable: T a = b                        [var]
 *  $0  = T
 *  $1  = a
 *  $2  = b
 *
 *
 * Given a struct or enum: struct a { T b, T_ c }   [type]
 *
 *  $0  = struct
 *  $1  = a
 *  $2  = { T b, T_ c }
 *  body!($2) = T b, T_ c
 *
 * Or, with typedef: typedef struct a { b } c
 *
 *  $0  = typedef struct
 *  $1  = a
 *  $2  = { b }
 *  body!($2) = b
 *  $3  = c
 *
 *
 * The special variable $* will always yield the annotated
 * statement verbatim. Omitting it will cause the attribute
 * to "eat" the statement.
 */
macro_rules! path {
    fn ($method:expr, $path:expr) => {
        ${_handlers_stack += { &$1, $method, $path }, }

        $*
    }
}

uri_handler_t handlers[] = {
    $_handlers_stack
    { NULL, GET, NULL }
};

/**
 * This macro defines a callable with syntax print_test!(expr).
 */
macro_rules! print_test {
    ($ex:expr) => {
        printf("%s = %d", #$ex, $ex);
    }
}

#else
# error This library can only be used with SuperCub.
#endif //__SUPER_CUB__

// vim: ft=c.supercub
