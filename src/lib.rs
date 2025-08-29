mod book_bridge;
use crate::book_bridge::{load_binary, split_it};
use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
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
pub extern "system" fn Java_com_devakash_bookbridge_pdfProcess_PdfGlobalStore_loadPdfiumBinary(
    mut _env: JNIEnv,
    _class: JObject,  path:JString
){

    let rust_string: String = _env
        .get_string(&path)
        .expect("Couldn't get java string!")
        .into();
    println!("{}",rust_string);
    load_binary(&rust_string);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_devakash_bookbridge_pdfProcess_PdfGlobalStore_SplitPdf(
    mut _env: JNIEnv,
    _class: JObject, source_path:JString, save_path:JString
) -> jstring {
    let source_string: String = _env
        .get_string(&source_path)
        .expect("Couldn't get java string!")
        .into();
    let save_string: String = _env
        .get_string(&save_path)
        .expect("Couldn't get java string!")
        .into();


    let jvm = _env.get_java_vm().unwrap();

    let option = split_it(source_string, save_string, Box::new( move |progress: i32| {

        let mut env = jvm.attach_current_thread().unwrap();


        let class = env.find_class("com/devakash/bookbridge/pdfProcess/PdfGlobalStore").unwrap();
        let message = env.new_string("Running").unwrap();
        let arg_obj: JObject = message.into();

        env.call_static_method(
            class,
            "pdfCallbackToFlutter",
            "(ILjava/lang/String;)V",
            &[
                JValue::Int(progress),
                JValue::Object(&arg_obj), // pass owned JObject
            ],
        )
            .unwrap();
    }));


    if let None = option {
        let output = _env.new_string("Errror")
            .expect("Couldn't create java string");

        // Hand it back to the JVM

        return output.into_raw();
    }

    let output = _env.new_string(option.unwrap())
        .expect("Couldn't create java string");

    return output.into_raw();
}




