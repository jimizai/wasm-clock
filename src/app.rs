use js_sys::Date;
use log::*;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};

pub struct App {
    timer: Timer,
    interval_task: IntervalTask,
}

pub enum Hand {
    Hour,
    Minute,
    Second,
}

impl Hand {
    fn get_hand_class_name(&self) -> &'static str {
        match *self {
            Hand::Hour => "hour-hand",
            Hand::Minute => "minute-hand",
            Hand::Second => "second-hand",
        }
    }
}

pub enum Msg {
    Task,
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let interval_task =
            IntervalService::spawn(Duration::from_millis(1_000), link.callback(|_| Msg::Task));
        App {
            timer: Timer::new(),
            interval_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Task => self.timer.renew(),
        }
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
                    { self.hand_view(Hand::Hour) }
                    { self.hand_view(Hand::Minute) }
                    { self.hand_view(Hand::Second) }
                </div>
            </div>
        }
    }
}

fn pad_zero(num: i32) -> String {
    if num < 10 {
        format!("0{}", num)
    } else {
        num.to_string()
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
                <span>{pad_zero(index)}</span>
                {(1..5).map(|x| self.second_slice_view(x)).collect::<Html>()}
            </div>
        }
    }

    fn hand_view(&self, hand_type: Hand) -> Html {
        let deg = self.compute_deg(&hand_type);
        let class_name = hand_type.get_hand_class_name();
        let deg = if deg < 180 { 180 + deg } else { deg - 180 };
        let rotate_name = format!("rotate-{}", deg);
        html! {
            <div class=format!("{} {}", class_name, rotate_name)>
            </div>
        }
    }

    fn compute_deg(&self, hand_type: &Hand) -> u32 {
        let deg = match hand_type {
            Hand::Second => self.timer.seconds * 6,
            Hand::Minute => self.timer.minutes * 6,
            Hand::Hour => {
                self.timer.hours * 30 + (self.timer.minutes - self.timer.minutes % 6) / 12 * 6
            }
        };
        deg % 360
    }
}

struct Timer {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Timer {
    fn new() -> Self {
        let date = Date::new_0();
        Self {
            hours: date.get_hours(),
            minutes: date.get_minutes(),
            seconds: date.get_seconds(),
        }
    }

    fn renew(&mut self) {
        let date = Date::new_0();
        self.hours = date.get_hours();
        self.minutes = date.get_minutes();
        self.seconds = date.get_seconds();
    }
}
