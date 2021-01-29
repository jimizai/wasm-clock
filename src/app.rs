use yew::prelude::*;

pub struct App {
}

pub enum Hand {
    Hour,
    Minute,
    Second,
}

impl Hand {
    fn get_hand_class_name(&self) -> &'static str {
        match *self {
            Hand::Hour => "hour_hand",
            Hand::Minute => "minute_hand",
            Hand::Second => "second_hand",
        }
    }
}

pub enum Msg {}

impl Component for App {
    type Properties = ();
    type Message = Msg;


    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="clock-body">
                    { (1..13).map(|x| self.time_slice_view(x)).collect::<Html>() }
                    <div class="center-point" />
                </div>
            </div>
        }
    }
}

impl App {
    fn second_slice_view(&self, index: i32) -> Html {
        html! {
            <div class=format!("second-slice-item transform-center second-{}", index)></div>
        }
    }

    fn time_slice_view(&self, index: i32) -> Html {
        html! {
            <div class=format!("time-slice-item transform-center time-{}", index)>
                {(1..5).map(|x| self.second_slice_view(x)).collect::<Html>()}
            </div>
        }
    }
}