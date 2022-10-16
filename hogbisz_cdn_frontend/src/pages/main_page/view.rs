use yew::prelude::*;

pub struct Main;
impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Main Page"}</h1>
                <button onclick={ctx.link().callback(|_| ())}>{"Click Me"}</button>
            </div>
        }
    }
}
