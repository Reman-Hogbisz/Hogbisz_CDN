use crate::route::Route;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::util::TryLoginError;

pub enum LoginMsg {
    TryLogin,
    LoginResult(Result<(), TryLoginError>),
    LoginSuccess,
    LoginFailure(String),
}

pub struct Login {
    internal_error: bool,
    error_string: Option<String>,
    password_field: NodeRef,
}
impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            internal_error: false,
            error_string: None,
            password_field: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::TryLogin => {
                {
                    let password_field = self.password_field.clone();
                    ctx.link().send_future(async move {
                        let password_field = match password_field.cast::<HtmlInputElement>() {
                            Some(field) => field,
                            None => {
                                gloo::console::error!(
                                    "Failed to cast password field to HtmlInputElement"
                                );
                                return LoginMsg::LoginFailure("Internal Error".to_string());
                            }
                        };
                        let password = password_field.value();
                        LoginMsg::LoginResult(super::util::try_login(password).await)
                    });
                }
                true
            }
            LoginMsg::LoginResult(result) => {
                match result {
                    Ok(_) => {
                        self.error_string = None;
                        self.internal_error = false;
                        ctx.link().send_message(LoginMsg::LoginSuccess);
                    }
                    Err(e) => {
                        let error_string = match e {
                            TryLoginError::IncorrectPassword => "Incorrect password".to_string(),
                            TryLoginError::InternalError => "Internal error".to_string(),
                            TryLoginError::SerializeError => "Fatal error".to_string(),
                        };
                        ctx.link()
                            .send_message(LoginMsg::LoginFailure(error_string));
                    }
                }
                false
            }
            LoginMsg::LoginSuccess => {
                let link = ctx.link();

                let history = match link.history() {
                    Some(history) => history,
                    None => {
                        gloo::console::error!("Failed to get internal history");
                        self.internal_error = true;
                        self.error_string = Some("Internal error".to_string());
                        return false;
                    }
                };
                history.push(Route::Main);
                true
            }
            LoginMsg::LoginFailure(error) => {
                self.internal_error = true;
                self.error_string = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let try_login = link.callback(move |_| LoginMsg::TryLogin);

        html! {
            <div class="flex flex-col items-center justify-center h-screen w-100">
                <div class="flex flex-col px-16 py-10 text-center text-black bg-gray-400 rounded-md dark:bg-gray-200">
                    <h1 class="m-1 text-2xl">{ "Login" }</h1>
                    <div class="py-2">
                        <input ref={self.password_field.clone()} class="m-1" type="password" />
                    </div>
                    <button onclick={try_login}
                        class="px-4 py-2 rounded-md bg-allow-200 hover:bg-allow-400 dark:bg-allow-400 dark:hover:bg-allow-600 hover:active:bg-allow-800"
                    >
                        { "Submit" }
                    </button>
                    if self.internal_error {
                            <h1 class="m-1 text-xl text-danger-500">{ "Error" }</h1>
                    }
                    if let Some(error_string) = &self.error_string {
                        if !error_string.is_empty() {
                            <h1 class="m-1 text-xl text-danger-500">{ error_string }</h1>
                        }
                    }
                </div>
            </div>
        }
    }
}
