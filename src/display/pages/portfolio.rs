use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{display::{atoms::{slant_display::SlantDisplay, art_display::ArtDisplay}, organisms::{animated_banner::AnimatedBanner, page_header::PageHeader, page_footer::PageFooter}, theme::use_theme}, router::Route};


#[styled_component(Portfolio)]
pub fn portfolio() -> Html {
    let theme = use_theme();
    let style = css!(r#"
        .art-holder {
            display: flex;
            justify-content: space-evenly;
            align-items: center;
            flex-wrap: wrap;
        }

        @media screen and (max-width: 800px) {
        }
    "#);
    html! {
        <div class={style}>
            <PageHeader />
            <AnimatedBanner><h1>{"Artistic Portfolio"}</h1></AnimatedBanner>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Digital"}</h3>
                <div class="art-holder">
                    <ArtDisplay img="/images/personal-art/Digital-Fracture.png" title="Fracture" />
                    <ArtDisplay img="/images/personal-art/Digital-Magic-Aura.png" title="Node" />
                    <ArtDisplay img="/images/personal-art/Digital-Birb-Wizard.png" title="Bird Wizard"  />
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Acrylic"}</h3>
                <div class="art-holder">
                    <ArtDisplay img="/images/personal-art/Acrylic-abstract-air.png" title="Air" />
                    <ArtDisplay img="/images/personal-art/Acrylic-abstract-earth.png" title="Earth" />
                    <ArtDisplay img="/images/personal-art/Acrylic-abstract-fire.png" title="Fire"  />
                    <ArtDisplay img="/images/personal-art/Acrylic-abstract-water.png" title="Water"  />
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Watercolor"}</h3>
                <div class="art-holder">
                    <ArtDisplay img="/images/personal-art/Pandora.png" title="Pandora's Box"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-cave.jpg" title="Underground Lake"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-crystaltree.png" title="Crystal Tree"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-floating-islands.png" title="Floating Islands"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-mesa.jpg" title="Mesa Pylons"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-skull-valley.png" title="Skull Valley"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-bird.png" title="Bird"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-cherryblosom.png" title="Cherry Blossom"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-butterfly-blue.png" title="Blue Butterfly"  />
                    <ArtDisplay img="/images/personal-art/Watercolor-butterfly-orange.png" title="Orange Butterfly"  />
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Pencil & Pen"}</h3>
                <div class="art-holder">
                    <ArtDisplay img="/images/personal-art/Coloredpencil-donut-still-life.png" title="Donut Still-life" />
                    <ArtDisplay img="/images/personal-art/Coloredpencil-Silco.png" title="Silco (Arcane)" />
                    <ArtDisplay img="/images/personal-art/Coloredpencil-fox.png" title="Fox" />
                    <ArtDisplay img="/images/personal-art/Pen-cat-and-fish.png" title="Cat & Fish" />
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Other Pages"}</h3>
            </SlantDisplay>
            <PageFooter exclude={Route::Portfolio}/>
        </div>
    }
}