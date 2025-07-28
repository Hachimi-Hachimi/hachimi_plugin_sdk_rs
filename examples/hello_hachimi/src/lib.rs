use hachimi_plugin_sdk::{api::{Hachimi, HachimiApi, LogLevel}, hachimi_plugin};

static mut API: Option<HachimiApi> = None;

static mut PUSH_DIALOG_ORIG: usize = 0;
type PushDialogFn = extern "C" fn();
unsafe extern "C" fn push_dialog_hook() {
    let orig_fn: PushDialogFn = std::mem::transmute(PUSH_DIALOG_ORIG);
    API.unwrap().log(LogLevel::Info, c"hello_hachimi", c"Title screen menu opened");
    orig_fn();
}

#[hachimi_plugin]
pub fn main(api: HachimiApi) {
    unsafe { API = Some(api); }
    api.log(LogLevel::Info, c"hello_hachimi", c"Hello Hachimi!");

    let hachimi = Hachimi::instance(&api);
    let interceptor = hachimi.interceptor();

    let image = api.il2cpp_get_assembly_image(c"umamusume.dll");
    let class = api.il2cpp_get_class(image, c"Gallop", c"DialogTitleMenu");
    let push_dialog = api.il2cpp_get_method_addr(class, c"PushDialog", 0);

    if let Some(trampoline) = interceptor.hook(push_dialog, push_dialog_hook as _) {
        unsafe { PUSH_DIALOG_ORIG = trampoline; }
    }
}