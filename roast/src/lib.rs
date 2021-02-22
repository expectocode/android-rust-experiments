#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use tokio::runtime;
use tokio::time::sleep;
use std::time::Duration;

async fn nothing() {

}

#[no_mangle]
pub extern "system" fn Java_com_example_jetpackcomposehelloworld_MainActivityKt_sleep(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let runtime = runtime::Builder::new_current_thread().enable_time().build().unwrap();

    runtime.block_on(async {
        sleep(Duration::from_millis(1000)).await
    });
    // runtime.block_on(nothing());

    let output = env
        .new_string(format!("I did a sleep"))
        .expect("Couldn't create a Java string!");
    output.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_com_example_jetpackcomposehelloworld_MainActivityKt_hello(
    env: JNIEnv,
    _: JClass,
    input: JString,
) -> jstring {
    let output = env
        .new_string(format!("Whee this string is from Rust: {:?}", Some(5)))
        .expect("Couldn't create a Java string!");
    output.into_inner()
}
