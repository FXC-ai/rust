use test_protoc_buf::create_my_struct;
use test_protoc_buf::items::MyStruct;


fn main()
{

    let tetet = create_my_struct(14);

    let unstructre = MyStruct{val : 98};
    println!("Le rust est une passion {} {}", tetet.val, env!("OUT_DIR"));
}