// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc executable.rs --extern rary=library.rlib && ./executable 
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}