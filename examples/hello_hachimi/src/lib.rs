use hachimi_plugin_sdk::{api::{Hachimi, HachimiApi}, hachimi_plugin};
use log::info;

static mut API: Option<HachimiApi> = None;

static mut PUSH_DIALOG_ORIG: usize = 0;
type PushDialogFn = extern "C" fn();
unsafe extern "C" fn push_dialog_hook() {
    let orig_fn: PushDialogFn = std::mem::transmute(PUSH_DIALOG_ORIG);
    info!("Title screen menu opened");
    orig_fn();
}

#[hachimi_plugin]
pub fn main(api: HachimiApi) {
    unsafe { API = Some(api); }
    // Silently ignore log init errors
    _ = hachimi_plugin_sdk::log::init(api, log::Level::Info);

    info!("Hello Hachimi!");

    let hachimi = Hachimi::instance(&api);
    let interceptor = hachimi.interceptor();

    let image = api.il2cpp_get_assembly_image(c"umamusume.dll");
    if image.is_null() { return };

    let class = api.il2cpp_get_class(image, c"Gallop", c"DialogTitleMenu");
    if class.is_null() { return };

    let push_dialog = api.il2cpp_get_method_addr(class, c"PushDialog", 0);
    if push_dialog == 0 { return };

    if let Some(trampoline) = interceptor.hook(push_dialog, push_dialog_hook as _) {
        unsafe { PUSH_DIALOG_ORIG = trampoline; }
    }
}