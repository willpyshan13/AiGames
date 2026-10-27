//! Opens the existing Yikm game page. Game code stays on the remote website.

#[cfg(not(target_os = "android"))]
const GAME_URL: &str = "https://www.yikm.net/play?id=4275";

#[cfg(target_os = "android")]
pub fn open() -> Result<(), String> {
    use jni::{
        jni_sig, jni_str,
        objects::{JClass, JObject, JValue},
    };

    let app = bevy::window::ANDROID_APP.get().ok_or("安卓窗口尚未就绪")?;
    // AndroidApp retains the VM and Activity for the duration of this call.
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr().cast()) };
    let result: jni::errors::Result<()> = vm.attach_current_thread(|env| {
        let activity_raw = app.activity_as_ptr().cast();
        let borrowed = unsafe { env.as_cast_raw::<JObject>(&activity_raw) }?;
        let activity = env.new_local_ref(&*borrowed)?;
        // Native Bevy threads have no app class-loader context. Load the helper
        // through the Activity instead of JNI FindClass on this thread.
        let loader = env
            .call_method(
                &activity,
                jni_str!("getClassLoader"),
                jni_sig!("()Ljava/lang/ClassLoader;"),
                &[],
            )?
            .l()?;
        let name = env.new_string("com.aigames.magnetisland.WebGame")?;
        let class = env
            .call_method(
                &loader,
                jni_str!("loadClass"),
                jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                &[JValue::Object(name.as_ref())],
            )?
            .l()?;
        let class = env.cast_local::<JClass>(class)?;
        env.call_static_method(
            &class,
            jni_str!("open"),
            jni_sig!("(Landroid/app/Activity;)V"),
            &[JValue::Object(&activity)],
        )?;
        Ok(())
    });
    result.map_err(|error| error.to_string())
}

#[cfg(not(target_os = "android"))]
pub fn open() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let mut command = std::process::Command::new("open");
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut cmd = std::process::Command::new("rundll32.exe");
        cmd.arg("url.dll,FileProtocolHandler");
        cmd
    };
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let mut command = std::process::Command::new("xdg-open");
    let status = command
        .arg(GAME_URL)
        .status()
        .map_err(|error| error.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("浏览器启动失败：{status}"))
    }
}
