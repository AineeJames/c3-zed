module main;
import std::io;

fn int add(int a, int b) {
    return a + b;
}

fn void main() {
    std::io::printfn("2 + 3 = %d", add(2, 3));
}
