use crate::{example, example::Example};

use patternfly_yew::*;
use yew::prelude::*;

pub struct BadgeExample {}

impl Component for BadgeExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example! {"Badge" =>
            <Badge>{"123"}</Badge>
        };

        let example2 = example! {"Badge (Read-only)" =>
            <Badge read=true>{"123"}</Badge>
        };

        html! {
            <>
                <Example title="Badge">
                    {example1}
                    {example2}
                </Example>
            </>
        }
    }
}