use yew::prelude::*;
use yew_bootstrap::component::{Container, NavBar, NavItem, ContainerSize};

#[derive(Properties, PartialEq)]
pub struct PabeStructureProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(PageStructure)]
pub fn page_structure(props: &PabeStructureProps) -> Html {

    html! {
    <>
        <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-light bg-light">
            <NavItem text="link 1" />
        </NavBar>
        <Container>
            {props.children.clone()}
        </Container>
    </>
    }
}