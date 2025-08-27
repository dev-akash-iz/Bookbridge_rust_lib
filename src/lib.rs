mod book_bridge;
use crate::book_bridge::{load_binary, split_it};
use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::{jint, jstring};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_devakash_bookbridge_pdfProcess_PdfGlobalStore_hello(
    _env: JNIEnv,
    _class: JObject,
) {
    println!("Hello from Rust JNI!");
}


#[unsafe(no_mangle)]
pub extern "system" fn Java_com_devakash_bookbridge_pdfProcess_PdfGlobalStore_getMessage(
    env: JNIEnv,
    _class: JObject,
) -> jstring {
    // Create a new Java string
    let output = env.new_string("Hello Java")
        .expect("Couldn't create java string");

    // Hand it back to the JVM
    output.into_raw()
}



#[unsafe(no_mangle)]
pub extern "system" fn Java_com_devakash_bookbridge_pdfProcess_PdfGlobalStore_page(
    _env: JNIEnv,
    _class: JObject,
)-> i32 {
    load_binary();


    let count = split_it("".to_string());

    23
}


