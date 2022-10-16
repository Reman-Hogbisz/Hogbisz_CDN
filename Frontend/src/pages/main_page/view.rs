use super::model::FileUploadData;
use super::util::FileUploadError;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Event, File, HtmlInputElement};
use yew::prelude::*;

pub enum MainMsg {
    UploadFile,
    UploadFileResult(Result<FileUploadData, FileUploadError>),
    ForceUpdate,
    SetFile(File),
}

pub struct Main {
    internal_error: bool,
    error_string: Option<String>,
    file: Option<File>,
    success_url: Option<String>,
    uploading: bool,
}

impl Component for Main {
    type Message = MainMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            internal_error: false,
            error_string: None,
            file: None,
            success_url: None,
            uploading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainMsg::SetFile(file) => {
                self.file = Some(file);
                self.error_string = None;
                self.internal_error = false;
                self.success_url = None;
                true
            }
            MainMsg::UploadFile => {
                if self.uploading {
                    return false;
                }
                self.success_url = None;
                {
                    let file = match &self.file {
                        Some(file) => file.clone(),
                        None => {
                            self.internal_error = true;
                            self.error_string = Some("No file selected".to_string());
                            return true;
                        }
                    };
                    ctx.link().send_future(async move {
                        MainMsg::UploadFileResult(super::util::upload_file(file).await)
                    });
                }
                self.uploading = true;
                true
            }
            MainMsg::ForceUpdate => true,
            MainMsg::UploadFileResult(result) => {
                self.uploading = false;
                match result {
                    Ok(response) => {
                        let base_url = match web_sys::window() {
                            Some(window) => match window.location().origin() {
                                Ok(origin) => origin,
                                Err(_) => String::default(),
                            },
                            None => String::default(),
                        };
                        self.error_string = None;
                        self.internal_error = false;
                        self.success_url = Some(format!(
                            "{}/api/uploads/{}/{}",
                            base_url, response.secret, response.name
                        ));
                        true
                    }
                    Err(e) => {
                        let error_string = match e {
                            FileUploadError::InternalError => "Internal Error".to_string(),
                            FileUploadError::DeserializeError => {
                                "Failed to parse response.".to_string()
                            }
                            FileUploadError::ResponseError(message) => message,
                        };
                        self.error_string = Some(error_string);
                        self.internal_error = true;
                        true
                    }
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // TODO : Implement file drop
        let file_drop_start_callback =
            Callback::from(|event: DragEvent| match event.data_transfer() {
                Some(data_transfer) => {
                    gloo::console::log!(data_transfer.types());
                    match data_transfer.clear_data() {
                        Ok(_) => (),
                        Err(e) => {
                            gloo::console::error!(format!(
                                "Failed to clear data transfer with error \"{:?}\"",
                                e
                            ));
                        }
                    }
                }
                None => {
                    gloo::console::error!("Failed to get data transfer in file drag enter");
                }
            });

        let file_drop_callback = ctx.link().callback(|event: DragEvent| {
            event.prevent_default();
            match event.data_transfer() {
                Some(data_transfer) => match data_transfer.files() {
                    Some(files) if files.length() == 1 => MainMsg::SetFile(files.item(0).unwrap()),
                    Some(_) => {
                        gloo::console::error!("User attempted to upload more than 1 file!");
                        MainMsg::ForceUpdate
                    }
                    None => {
                        gloo::console::error!("Failed to get files");
                        MainMsg::ForceUpdate
                    }
                },
                None => {
                    gloo::console::error!("Failed to get data transfer in file drop callback");
                    MainMsg::ForceUpdate
                }
            }
        });

        let file_upload_callback = ctx.link().callback(|event: Event| {
            event.prevent_default();
            match event.target() {
                Some(target) => match target.dyn_into::<HtmlInputElement>() {
                    Ok(input) => match input.files() {
                        Some(files) if files.length() == 1 => {
                            MainMsg::SetFile(files.item(0).unwrap())
                        }
                        Some(_) => {
                            gloo::console::error!("User attempted to upload more than 1 file!");
                            MainMsg::ForceUpdate
                        }
                        None => {
                            gloo::console::error!("Failed to get files");
                            MainMsg::ForceUpdate
                        }
                    },
                    Err(_) => {
                        gloo::console::error!("Failed to cast target to HtmlInputElement");
                        MainMsg::ForceUpdate
                    }
                },
                None => {
                    gloo::console::error!("Failed to get target");
                    MainMsg::ForceUpdate
                }
            }
        });

        html! {
            <div class="flex flex-col items-center justify-center w-full h-full">
                <div
                    class="flex flex-col items-center text-black bg-gray-400 md:w-2/5 h-1/2 rounded-xl justify-evenly"
                    ondrop={file_drop_callback}
                    ondragover={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}
                    ondragenter={file_drop_start_callback}
                >
                    <h1 class="text-2xl">{"Upload File"}</h1>
                    <input
                        type="file"
                        multiple={false}
                        onchange={file_upload_callback}
                    />
                    if let Some(file) = &self.file {
                        <div class="flex flex-col items-center">
                            <p>{"File Name: "}{file.name()}</p>
                            <p>{"File Size: "}{size::Size::from_bytes(file.size())}</p>
                            <p>{"File Type: "}{file.type_()}</p>
                        </div>
                    }
                    <button class="px-4 py-2 bg-green-200 rounded-md hover:bg-green-400 dark:bg-green-400 dark:hover:bg-green-600 hover:active:bg-green-800" onclick={ctx.link().callback(|_| {MainMsg::UploadFile})}>
                        {"Upload"}
                    </button>
                    if self.uploading {
                        <crate::components::pacman_spinner::PacmanSpinner />
                    }
                    if self.internal_error {
                        <div class="text-center">
                            <h1 class="text-xl text-red-700">{ "Error" }</h1>
                            if let Some(error_string) = &self.error_string {
                                if !error_string.is_empty() {
                                    <h1 class="text-xl text-red-700">{ error_string }</h1>
                                }
                            }
                        </div>
                    }
                    if let Some(url) = &self.success_url {
                        <div class="text-center">
                            <h1 class="text-xl text-green-800">{ "Success" }</h1>
                            <a target="_blank" href={url.to_string()} class="text-green-800">{ url }</a>
                        </div>
                    }
                </div>
            </div>
        }
    }
}
