use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PacmanSpinnerProps {
    #[prop_or(true)]
    pub loading: bool,
    #[prop_or("#5dc596".to_string())]
    pub color: String,
    #[prop_or("25px".to_string())]
    pub size: String,
    #[prop_or("2px".to_string())]
    pub margin: String,
    #[prop_or("100%".to_string())]
    pub radius: String,
}
impl Default for PacmanSpinnerProps {
    fn default() -> Self {
        Self {
            loading: true,
            color: "#5dc596".to_string(),
            size: "25px".to_string(),
            margin: "2px".to_string(),
            radius: "100%".to_string(),
        }
    }
}

#[function_component(PacmanSpinner)]
pub fn pacman_spinner(props: &PacmanSpinnerProps) -> Html {
    let spinner_delay_2 = "animation-delay: 0.25s;".to_string();
    let spinner_delay_3 = "animation-delay: 0.50s;".to_string();
    let spinner_delay_4 = "animation-delay: 0.75s;".to_string();
    let spinner_delay_5 = "animation-delay: 1s;".to_string();

    let border_1 = format!("{} solid transparent", props.size);
    let border_2 = format!("{} solid {}", props.size, props.color);

    let spinner_style_1 = format!(
        "
            width: 0; 
            height: 0;
            border-top: {};
            border-right: {};
            border-bottom: {};
            border-left: {};
            border-radius: {};
        ",
        border_2, border_1, border_2, border_2, props.size
    );
    let spinner_style = format!(
        "background-color: {}; width: {}; height: {}; margin: {}; border-radius: {};",
        props.color, props.size, props.size, props.margin, props.radius
    );
    let animation_style = format!(
        "
            width: 10px;
            height: 10px;
            transform: translate(0, {}px);
            position: absolute;
            top: 25px;
            left: 100px;
            animation-name: v-pacmanStretchDelay;
            animation-duration: 1s;
            animation-iteration-count: infinite;
            animation-timing-function: linear;
            animation-fill-mode: both;
        ",
        props.size.parse::<f32>().unwrap_or(25.0f32) / -4.0f32
    );

    html! {
        <>
            if props.loading {
                <style>
                    {"
                        @-webkit-keyframes v-pacmanStretchDelay
                        {
                            75%
                            {
                                -webkit-opacity: 0.7;             
                                opacity: 0.7;
                            }
                            100%
                            {
                                -webkit-transform: translate(-100px, -6.25px);
                                        transform: translate(-100px, -6.25px);
                            }
                        }
                        @keyframes v-pacmanStretchDelay
                        {
                            75%
                            {
                                -webkit-opacity: 0.7;             
                                opacity: 0.7;
                            }
                            100%
                            {
                                -webkit-transform: translate(-100px, -6.25px);
                                        transform: translate(-100px, -6.25px);
                            }
                        }
                    "}
                </style>
                <div style="position: relative; font-size: 0; text-align: center;">
                    <div class="v-pacman v-pacman1" style={spinner_style_1}>
                    </div><div class="v-pacman v-pacman2" style={vec![spinner_style.as_str(),animation_style.as_str(),spinner_delay_2.as_str()].join("")}>
                    </div><div class="v-pacman v-pacman3" style={vec![spinner_style.as_str(),animation_style.as_str(),spinner_delay_3.as_str()].join("")}>
                    </div><div class="v-pacman v-pacman4" style={vec![spinner_style.as_str(),animation_style.as_str(),spinner_delay_4.as_str()].join("")}>
                    </div><div class="v-pacman v-pacman5" style={vec![spinner_style.as_str(),animation_style.as_str(),spinner_delay_5.as_str()].join("")}>
                    </div>
                </div>
            } else {
                <></>
            }
        </>
    }
}
