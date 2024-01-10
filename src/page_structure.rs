use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PabeStructureProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(PageStructure)]
pub fn page_structure(props: &PabeStructureProps) -> Html {

    html! {
        props.children.clone()
    }
}