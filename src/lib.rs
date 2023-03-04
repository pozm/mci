#![feature(new_uninit)]
mod mci;
use std::ffi::CString;

use jni::{object::JObject, jstring::JString};

#[cfg(any(target_os="macos",target_os="linux"))]
use ctor::*;
#[cfg(target_os="macos")]
#[ctor]
fn entry() {
    use std::time::Duration;
    let mut args = std::env::args();

    if args.next().unwrap().contains("java") && args.filter_map(|a| if a.contains("minecraft") && a.ends_with(".jar") {Some(a)} else {None}).count() > 0  {
        println!("pogging");
        std::thread::spawn(|| {
            std::thread::sleep(Duration::from_secs(2));

            main_thread_wrap();
        });
    }
}
#[cfg(not(target_os="macos"))]
#[poggers_derive::create_entry]
fn main_thread() -> Result<(), String> {
    use jni::jvalue::JValue;
    use mc_mappings::mappings::net::minecraft::client::MinecraftClient;


    let mut mci = mci::MCI::default();

    mci.load_jvm()?;

    mci.attach_current_thread()?;


    println!("we're chillin");
    {
        let jenv = mci.get_jenv();
        let jenv = jenv.write().unwrap();
        let ver = jenv.get_version();
        println!("version: {}", ver);
                
        let minecraft_client = jenv.find_class("eev").unwrap();
        // println!("mc = {:?}",minecraft_client.ptr);
        if let Ok(obj) = minecraft_client.call_static_object_method::<JObject>("G", "()Leev;",&vec![]) {
            let mcc = MinecraftClient::from(obj);
            println!("is 64 bit: {:?}", mcc.is64Bit());
            if let Ok(ver) = mcc.gameVersion() {
                println!("game version: {:?}",JString::from(ver));
            }
        }
        
        println!("q");

        

    }


    Ok(())
}


fn main_thread_wrap() {
    use std::panic;

    match panic::catch_unwind(||main_thread()) {
        Err(e) => {
            println!("`main` has panicked: {:#?}", e);
        }
        Ok(r) => match r {
            Err(e) => {
                eprint!("`main` failed with {:?}", e);
            }
            _ => {},
        },
    }

    println!("done");
}