module main;
import std::io;

interface Object
{
    fn String as_str();
}

struct Person (Object)
{
    String first;
    String last;
    int age;
}

fn String Person.as_str(Person* self) @dynamic
{
    DString str;
    str.appendf("%s %s (age %d)", self.first, self.last, self.age);
    return str.copy_str();
}

fn void main()
{
    Person p = {
        .first = "John",
        .last = "Doe",
        .age = 52,
    };
    io::printn(p.as_str());
}
