use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::display::{atoms::{image_display::Image, slant_display::SlantDisplay}, pages::blog::BlogPost, post::data::BlogTemplate, theme::Theme};


pub fn page(ctx: &Context<BlogPost>, theme: &Theme) -> Html {
    let post = ctx.props().post;
    html! {
        <BlogTemplate title={post.title.clone()}>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3>{"Overview"}</h3>
                <hr/>
                <div class="blog-content">
                    <p>
                        {"In this project, I explore NPR (Non-Photorealistic Rendering) techniques with a custom post process shader aiming to create the effect of brushstrokes on a model in realtime. I aimed to create a shader which was able to create the appearance of a painted scene utilizing artistic techniques with shaders in realtime. For research, I used refrence papers "}
                        <a href="#ref-paint-by-numbers">
                            <em>
                                {"Paint By Numbers: Abstract Image Representations"}
                            </em>
                        </a> {" by Paul Haeberli and Amy and Bruce Gooch's "} 
                        <a href="https://users.cs.northwestern.edu/~ago820/SIG98/abstract.html">
                            <em>
                                {"A Non-Photorealistic Lighting Model for Automatic Technical Illustration"}
                            </em>
                        </a>{". I implemented the NPR shaders in Opengl using C++, GLFW, and GLEW, as well as the boilerplate provided by "}<a href="http://www.opengl-tutorial.org/download/">{" Opengl Tutorial"}</a>{"."}
                    </p>
                    <p>
                        {"A showcase of the result of my work is shown below. Beneath that, I have some documentation of my progress on the project and how I achieved my results. Finally, I also speculate on alternate avenues for achieving artisitc effects through shaders. Here is the "}<a href="https://github.com/jlkump/NPR-Painterly-Shader"><Icon icon_id={IconId::BootstrapGithub} style="vertical-align: middle; margin-right: 10px;" width="1em" height="1em"/>{"Github link"}</a>{" for the project."}
                    </p>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3>{"Video Showcase"}</h3>
                <hr/>
                <div style="display: flex; flex-direction: column; align-items: center; padding: 30px;">
                    <div class="media-display">
                        <iframe width="100%" height="500" src="https://www.youtube.com/embed/_AlnCiPDz30?si=wPz_pAeG36IL_nKE" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
                    </div>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3>{"Process"}</h3>
                <hr/>
                <div class="blog-content">
                    <h4>{"Setup"}</h4>
                    <p>
                        {"This being one of my first major projects with OpenGL rather than webGL, I began first by learning OpenGL. For this I used the online resource "}<a href="http://www.opengl-tutorial.org">{"opengl-tutorial.org"}</a>{", which I can highly recommend as it provides all the necessary basics you might need for getting started with any OpenGL project. Since there's a lot that goes into a build system (and since I didn't want to spend time debugging build problems), I used the CmakeLists and project structure provided by the tutorial website. In the end, this added a lot of bloat to my project structure, so for anyone begining with OpenGL, I recommend using "}<a href="https://github.com/tgalaj/OpenGLSampleCmake?tab=readme-ov-file">{"this github"}</a>{" for setting up the build system for your own projects."}
                    </p>
                    <hr/>
                    <h4>{"Approach"}</h4>
                    <p>
                        {"The tutorial's code provided all the necessary baseline code for rendering 3D objects with a camera. It also provided simple lighting with the Phong Illumination model in some shaders. So now, I had to decide on how to approach the problem of rendering brushstrokes in realtime for my shader. I had initially thought about drawing individual brushstrokes on a texture using samples from the scene rendered to texture, however, doing this would be difficult to do in parallel on the GPU, as each brush stroke would have to be layered on the same shared texture. In addition, this would also create a new brush texture each frame, sure to be extrememly expensive. "}
                    </p>
                    <p>
                        {"I tried a few approaches by creating the textures on the CPU, but ultimately they fell through, looking fairly bad with poor performance. I moved on to creating a modified illumination model for rendering. For this, I used the Gooch Illumination model to render objects in a more artistic way."}
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <Image src="/images/npr-shader/before-after-gooche-illumination.png" alt="NPR Brush Shader - Before and After Gooche Illumination" width="36rem"/>
                        </div>
                        <div style="flex: 50%;">
                            <p>
                                {"The Gooch Illumination model works, at a high level, by taking what would normally be in darkness and adding cool tones to the model, while adding warm tones to the illuminated side. This is what artists normally do in their work, and for my rendering, it resulting in a more pleasing appearance that is softer on the harsh shadows of the Phong lighting model."}
                            </p>
                        </div>
                    </div>
                    <p>
                        {"With the Illumination model much improved, I returned to the problem of rendering brush strokes. Originally, I wanted to update the brushstrokes on each frame to align with the form of  model, however, I couldn't see a way to do this without creating a new texture on each frame. So instead, I decided to create a static texture of brushstrokes and change the color of them as the rendered scene changes."}
                    </p>
                    <p>
                        {"Creating a static frame of brush strokes simplified the problem significantly. I would create a series of quads in screen space that each had a brush stroke texture. I created the quads on the CPU with a set of parametrizable variables, such as "}<code>{"BRUSH_SIZE"}</code>{", "}<code>{"BRUSH_SIZE_VARIANCE"}</code>{", "}<code>{"ANGLE"}</code>{", "}<code>{"ANGLE_VARIANCE"}</code>{", and "}<code>{"COLOR_VARIANCE"}</code>{". Brush size is pretty self explanatory, being the size of an individual brush stroke. Angle is the angle at which the brush strokes would be applied on the screen. Color variance is the range at which a random amount of color would be added to brush stroke. Variance for size and angle also added a random amount to the brush size and angle within a given range."}
                    </p>
                    <p>
                        {"To render the result, I simply had to render the scene I had previously to texture, then read from that texture in the brush stroke shaders. To make sure each brush stroke had fairly uniform color, I would sample the rendered texture only at the vertices of the brush stroke. The results are shown below."}
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <Image src="/images/npr-shader/first-iteration-small-strokes.png" alt="NPR Brush Shader - First Iterations with small brush strokes" width="36rem"/>
                        </div>
                        <div style="flex: 50%;">
                            <Image src="/images/npr-shader/first-iteration-large-strokes.png" alt="NPR Brush Shader - First Iterations with large brush strokes" width="36rem"/>
                        </div>
                    </div>
                    <p>
                        {"As you can see, this resulted in some strange shadow-like artifacts on the final render. This effect was particularly apparent with larger brush sizes. I found that the problem came from some brushstrokes in the lower right having a majority of the brush stroke quad with the background color value, but one or two values of the object itself. This resulted in a blended color between the two for a brush stroke. A somewhat poor solution to this problem, I simply discarded all fragments that had a color close to the background color."}
                    </p>
                    <p>
                        {"After fixing that, the shader really came together. Playing with the parameters to the brush strokes a bit, I came to some good looking renders:"}
                    </p>
                    <h5 style="text-align: center;">{"Large Standard Brush Strokes"}</h5>
                    <Image src="/images/npr-shader/brush-strokes-large.png" alt="NPR Brush Shader - Large Standard Brush Strokes" width="56rem"/>
                    <h5 style="text-align: center;">{"Small Standard Brush Strokes"}</h5>
                    <Image src="/images/npr-shader/brush-strokes-small.png" alt="NPR Brush Shader - Small Standard Brush Strokes" width="56rem"/>
                    <h5 style="text-align: center;">{"Small Splatter Brush Strokes"}</h5>
                    <Image src="/images/npr-shader/brush-strokes-splatter.png" alt="NPR Brush Shader - Small Splatter Brush Stroke" width="56rem"/>
                    <p>
                        {"Also some renders that were more just fun to look at and play around with:"}
                    </p>
                    <h5 style="text-align: center;">{"High Color Variance"}</h5>
                    <Image src="/images/npr-shader/high-variance.png" alt="NPR Brush Shader - High Color Variance" width="56rem"/>
                    <h5 style="text-align: center;">{"Enormous Size and High Color Variance"}</h5>
                    <Image src="/images/npr-shader/big-random-strokes.png" alt="NPR Brush Shader - Enormous Size and High Color Variance" width="56rem"/>

                    <p>
                        {"Lastly, I tried to implement having the brush strokes align with the form of the model. To do this, I tried to align each brush stroke with the tangents and bi-normals of the model in screen space. However, my implementation had some problems with it, resulting in some strange effects you can see below."}
                    </p>
                    <Image src="/images/npr-shader/brush-strokes-with-form.png" alt="NPR Brush Shader - Attempt at Brush Strokes with Form" width="56rem"/>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3>{"References"}</h3>
            </SlantDisplay>
            <div class="references">
                <ol>
                    <li>
                        <a id="ref-paint-by-numbers" href="https://doi.org/10.1145/97880.97902">{"Paul Haeberli. 1990. "}<em>{"Paint by numbers: abstract image representations"}</em>{". SIGGRAPH Comput. Graph. 24, 4 (Aug. 1990), 207-214."}</a>
                    </li>
                    <br/>
                    <li>
                        <a id="ref-npr-shading" href="https://users.cs.northwestern.edu/~ago820/SIG98/abstract.html">{"Amy Gooch, Bruce Gooch. "}<em>{"A Non-Photorealistic Lighting Model For Automatic Technical Illustration"}</em>{". SIGGRAPH Aug. 1998."}</a>
                    </li>
                </ol>
            </div>
        </BlogTemplate>
    }
}