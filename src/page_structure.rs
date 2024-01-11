use yew::prelude::*;
use yew_bootstrap::{
    component::*,
    util::Dimension,
};

#[derive(Properties, PartialEq)]
pub struct PabeStructureProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(PageStructure)]
pub fn page_structure(props: &PabeStructureProps) -> Html {
    let brand = BrandType::BrandImage {
        image_url: "static/logo.svg".into(),
        alt: "Logo".into(),
        dimension: Some(Dimension {
            width: "60vw".into(),
            height: "20%".into(),
        }),
    };

    html! {
    <>
        <NavBar nav_id={"test-nav"} class="navbar-expand-lg navbar-primary bg-primary" brand={brand} >
            <NavItem text="link 1" />
            <NavDropdown text="several items">
                <NavDropdownItem text="hello 1" />
                <NavDropdownItem text="hello 2" />
            </NavDropdown>
        </NavBar>
        <Container>
            {props.children.clone()}
        </Container>
        <footer class="bg-primary py-5 mt-5">
            {"Contacto"}
        </footer>
    </>
    }
}
