#![cfg(target_os = "android")]

use robusta_jni::bridge;

#[bridge]
mod jni_exports {
use robusta_jni::jni::JNIEnv;
use robusta_jni::jni::sys::jbyteArray;
use tokio::runtime;
use tokio::sync::mpsc;

use grammers_client::{Client, Config};
use grammers_session::MemorySession;
use log::Level;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use once_cell::sync::OnceCell;

static CELL: OnceCell<mpsc::Sender<Vec<u8>>> = OnceCell::new();

    #[package(com.example.jetpackcomposehelloworld)]
    struct MainActivityKt;

    impl MainActivityKt {
pub extern "jni" fn init(text_from_kotlin: String) {
    android_logger::init_once(android_logger::Config::default().with_min_level(Level::Trace));
    debug!("Initialized android logging");
    debug!("Kotlin says {}", text_from_kotlin);
}

pub extern "jni" fn sendMsg(env: &JNIEnv, input: jbyteArray) {
    let input = env.convert_byte_array(input).unwrap();
    let tx = CELL.get().unwrap();
    tx.blocking_send(input).unwrap();
}

pub extern "jni" fn listen() -> String {
    let runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let msg = runtime.block_on(async {
        let (tx, mut rx) = mpsc::channel(100); /* TODO const*/
        CELL.get_or_init(|| tx);

        // tx.send(b"Got em!\n");

        while let Some(res) = rx.recv().await {
            warn!("rust got = {:?}", res);
            warn!("string version = {:?}", String::from_utf8_lossy(&res));
        }

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

    let msg = format!("{}", msg);

    // env.call_method(callback, "factCallback", "(I)V", &[res.into()])
    // .unwrap();

    format!("I did a connect: {}", msg)
}
    }
}
