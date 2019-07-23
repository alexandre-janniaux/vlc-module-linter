typedef int (*foo_cb)(int a, int b);

struct test_struct { int test; int truc; foo_cb *pf_foo; };

int foo(int a, int b) { return a + b; }

static const struct test_struct decl_var_struct = {
    .test = 42,
    .truc = 72,
    .pf_foo = foo,
};
