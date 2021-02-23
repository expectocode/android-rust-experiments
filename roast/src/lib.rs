#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::time::Duration;

use jni::objects::{JClass, JObject};
use jni::sys::jstring;
use jni::JNIEnv;
use tokio::runtime;
use tokio::time::sleep;

use android_logger;
use grammers_client::{Client, Config};
use grammers_session::MemorySession;
use log::Level;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[no_mangle]
pub extern "system" fn Java_com_example_jetpackcomposehelloworld_MainActivityKt_loginit(
    _env: JNIEnv,
    _: JClass,
) {
    android_logger::init_once(android_logger::Config::default().with_min_level(Level::Trace));
    debug!("Initialized android logging");
}

#[no_mangle]
pub extern "system" fn Java_com_example_jetpackcomposehelloworld_MainActivityKt_connect(
    env: JNIEnv,
    _: JClass,
    setLabelText: JObject,
) -> jstring {
    let runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let msg = runtime.block_on(async {
        let api_id = 85381;
        let api_hash = "3e368f6bc7a6b30844b9e88cc940c151";
        let _phone = "PHONE";

        println!("Connecting to Telegram...");
        let client = Client::connect(Config {
            session: MemorySession::new(),
            api_id,
            api_hash: api_hash.into(),
            params: Default::default(),
        })
        .await;
        match client {
            Ok(_) => "it was ok".into(),
            Err(e) => format!("it was Err: {:?}", e),
        }
        // .expect("Make this function return result");
    });

    let msg = format!("{}, callback: {:?}", msg, setLabelText);

    // env.call_method(callback, "factCallback", "(I)V", &[res.into()])
    // .unwrap();

    let output = env
        .new_string(format!("I did a connect: {}", msg))
        .expect("Couldn't create a Java string!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_example_jetpackcomposehelloworld_MainActivityKt_sleep(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async { sleep(Duration::from_millis(1000)).await });
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
) -> jstring {
    let output = env
        .new_string(format!("Whee this string is from Rust: {:?}", Some(5)))
        .expect("Couldn't create a Java string!");
    output.into_inner()
}
