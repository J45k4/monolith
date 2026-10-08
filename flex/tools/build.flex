import "host.flex";
fn main(argc,argv) {
    h_environment(argc,argv);h_assert(argc==2,"Usage: build FLEX-COMPILER (from flex/)");let compiler=h_real(load64(argv+8));h_mkdir("build");
    h_compile(compiler,"tools/monolith.flex","build/monolith");h_compile(compiler,"tests/integration.flex","build/test-integration");h_compile(compiler,"tests/browser.flex","build/test-browser");
    let args=h_args(h_real("build/monolith"),"build","examples/todo/app.flex","--schema","examples/todo/schema.mono","--compiler");h_add(args,compiler);h_add(args,"-o");h_add(args,"build/todo");h_ok(args);
    h_print(1,"Built build/monolith, build/todo, build/test-integration, build/test-browser\n");return 0;
}
