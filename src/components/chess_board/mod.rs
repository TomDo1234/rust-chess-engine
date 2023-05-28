use yew::{function_component, Properties, Html, html, classes};

use crate::chess_engine::{Piece};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub board: [Option<Piece>; 64],
}

#[function_component]
pub fn ChessBoard(props: &Props) -> Html {
    let Props { board } = props;
    html! {
        <div class="flex flex-col" >
            {
                (0..8).map(|rowindex| html! {
                    <div class={classes!("flex-1 flex".to_owned())} >
                        {
                            (0..8).map(|colindex| html! {
                                {2}
                            }).collect::<Html>()
                        }
                    </div>
                }).collect::<Html>()
            }
        </div>
    }
}
