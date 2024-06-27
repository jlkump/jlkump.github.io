use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::display::{atoms::{code::CodeSnippet, image_display::Image, slant_display::SlantDisplay}, pages::blog::BlogPost, post::data::BlogTemplate, theme::Theme};


pub fn page(ctx: &Context<BlogPost>, theme: &Theme) -> Html {
    let post = ctx.props().post;
    html! {
        <BlogTemplate title={post.title.clone()}>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3>{"Overview"}</h3>
                <hr/>
                <div class="blog-content">
                    <p>
                        {"The goal of this project was to explore water simulations and water rendering using the OpenGL API and C++. For this, I gathered papers and online articles on the subject, primarily drawing from the GDC 2010 talk slides titled "}<a href="#ref-water-rendering"><em>{"Screen Space Fluid Rendering for Games"}</em></a>{" by Simon Green at NVIDIA. I also drew upon Ten Minute Physics' "}<a href="https://youtu.be/XmzBREkK8kY?si=8lUgJCES6hdRJI9I">{"YouTube video"}</a>{" for the physical simulation of particles, which I will cover in a different post."}
                    </p>
                    <p>
                        {"Everything is done essentially from scratch, save for an OpenGL CMake build template I used as a basis for building the project. The template comes from Tomasz Galaj's OpenGL CMake tutorial "}<a href="#ref-cmake"><em>{"How to setup OpenGL project with CMake"}</em></a>{" (the tutorial no longer seems to be available sadly, but the github template is still available)."}
                    </p>
                    <p>
                        {"Below is a video showcase of my progress throughout the development of the water renderer. After that, I have a walkthrough of my process for development. Here is the "}<a class="icon brands fa-github" href="https://github.com/jlkump/opengl-waterflow"><Icon icon_id={IconId::BootstrapGithub} style="vertical-align: middle; margin-right: 10px;" width="1em" height="1em"/>{"Github link"}</a>{" for the project."}
                    </p>
                </div>
            </SlantDisplay>
                <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3>{"Video Showcase"}</h3>
                <hr/>
                <div style="display: flex; flex-direction: column; align-items: center; padding: 50px; margin-bottom: 20px;">
                    <div class="media-display">
                    <iframe width="100%" height="500" src="https://www.youtube.com/embed/MIAm_noXHIc?si=7t0uoWxlvioESILt" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
                    </div>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3>{"Process"}</h3>
                <hr/>
                <div class="blog-content">
                    <p>
                        {"With any simulation, it is important to think about the representation for the object of simulation. Water in real life is a continous form that changes shape and can split off from the larger form in a spray. In any computer simulation, computing over continous regions is very expensive, involving integrations over some function of shape. In order to get a fast implementation, we can instead discretize over some region. Thus, for water simulation, there are two main approaches  called Eularian and Lagrangian based simulation, which are essentially just fancy names for grid-based and particle-based simulation respectively."}
                    </p>
                    <div class="split">
                        <div style="flex: 40%;">
                            <iframe width="100%" height="400" src="https://www.youtube.com/embed/C9DtT2yXxME?si=zkBnAVGGioiMIn-I" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
                        </div>
                        <div style="flex: 50%; margin-left: 20px;">
                            <p>
                                {"I started by creating a 2D visualizatoin of diffusion using a Eularian grid-based simulation, both to get familiar with water physics and to get familiar with OpenGL's compute shaders. You can see a short video demonstration to the left."}
                            </p>
                            <p>
                                {"Since this was used to mainly get familiar with compute shaders and setup the project with OpenGL (and also because I was eager to do a 3d-based rendering) I quickly moved on to the 3d rendering and simulation."}
                            </p>
                        </div>
                    </div>
                    <p>
                        {"A Eularian-based simulation is somewhat difficult to visualize in 3D, especially for a realistic rendering, so instead, using a Lagrangian particle-based representation makes things much easier. For the actual simulation, the representation uses a mix of Eularian and Lagrangian which makes the simulation more efficient and more accurate, while the rendering just considers the particle-based representation."}
                    </p>
                    <blockquote>
                        {"The technique used for simulation is called PIC/FLIP and is discussed in "}<a href="#ref-sand-as-fluid"><em>{"Animating Sand as a Fluid"}</em></a>{"."}
                    </blockquote>
                    <p>
                        {"A particle-based rendering technique is discussed by "}<a href="#ref-water-rendering">{"Simon Green's talk"}</a>{", which, at a high level, involves taking particles, rendered as sprites, and calculating their depth. This depth is rendered to texture and smoothed to create the appearance of a continous surface. Finally, the normals are extracted from the depth texture and form the normals of the water itself. Having extracted the normals, we can perform traditional rendering techniques for shading, reflection, and refraction."}
                    </p>
                    <p>
                        {"With that, in the following section I will cover my implementation of this high-level idea and any problems I encountered during my implementation. "}
                    </p>
                    <hr/>
                    <h4>{"Implementation in OpenGL"}</h4>
                    <p>
                        {"There are four major steps in rendering the water. The first step is to render the particles as sprites with their depth rendered to texture. The second step is to perform a blend on the rendered texture. The third step is to extract normals from the blended depth texture. Lastly, we render the water using the Phong-illumination model."}
                    </p>
                    <hr/>
                    <h4>{"Step 1 - Depth Texture"}</h4>
                    <div class="split">
                        <div style="flex: 50%;">
                            <Image src="/images/waterflow/Depth-fix.png" alt="" />
                        </div>
                        <div style="flex: 50%;">
                            <p>
                                {"In the first step, we need to render particle sprites at the positions of each particle. The positions are stored in a 2D texture for simulation on the GPU, so we take in the particle ids as UVs rather than positions in the rendering shader. I used GPU instancing for rendering the sprites since each particle renders the same 2D, screnn-space quad  just at different positions on the screen. Rendering the depth, we can see the result to the left with a rectangular section of water."}
                            </p>
                            <blockquote>
                                {"Instancing is a technique for rendering the same or very similar models many times without passing the same information across the bus to the GPU multiple times. A tutorial on how it is done can be found "}<a href="https://www.opengl-tutorial.org/intermediate-tutorials/billboards-particles/particles-instancing/">{"here"}</a>{"."}
                            </blockquote>
                        </div>
                    </div>
                    <hr/>
                    <h4>{"Step 2 - Blended Depth"}</h4>
                    <p>
                        {"The next step blends the depth texture, producing a new blended texture. For this, I used a Gaussian blur, although the talk recommends a Bilateral filter to perserve in the water holes in the resulting depth texture. I found that my bilateral filter did not blend the depth texture enough however, as shown to the right. The changes in color are just due to differences in how I rendered the normal vector."}
                    </p>
                    <div class="split">
                        <div style="flex: 33%;">
                            <h5 style="text-align: center;">{"The normals as they are without blending, extracted directly from depth."}</h5>
                            <Image src="/images/waterflow/waterflow-fixed-normals.png" alt="" />
                        </div>
                        <div style="flex: 33%;">
                            <h5 style="text-align: center;">{"The normals blended with a Bilateral filter, producing artifacts."}</h5>
                            <Image src="/images/waterflow/blend-radius-10.png" alt="" />
                        </div>
                        <div style="flex: 33%;">
                            <h5 style="text-align: center;">{"The normals blended with a Gaussian filter with minor artifacts at the edges."}</h5>
                            <Image src="/images/waterflow/waterflow-documentation-blended-normals.png" alt="" />
                        </div>
                    </div>
                    <hr/>
                    <h4>{"Step 3 - Normal Extraction"}</h4>
                    <p>
                        {"The next step extracts the normals from the depth texture, the result of which you can see in the pictures above. This step comes directly from Simon Green's talk and is the core of why it is an efficient screen-space rendering. Since we only consider the depth texture of a 2D rendering of water, we can quickly render an approximation of how the water should look if we extract the normals."}
                    </p>
                    <p>
                        {"The normals are extracted with the following steps:"}
                        <ul>
                            <li>
                                {"Calculating the view-space position (the slides call it eye-space) of the texel of water given the UV and depth. This is fairly straight-forward, simply taking the homogenous-space position from the UV and depth then multiplying by the inverse of the projection matrix. This is what it looks like in my shader:"}
                                <CodeSnippet>
{"vec3 GetViewPos(vec2 tex_coord) {
    vec4 hs_pos = vec4 (
        2.0 * tex_coord.x - 1.0,
        2.0 * tex_coord.y - 1.0,
        2.0 * texture(depth_tex, tex_coord).r - 1.0,
        1.0);
    vec4 vs_pos = (inv_proj * hs_pos);
    return vs_pos.xyz / vs_pos.w;
}"}
                                </CodeSnippet>
                            </li>
                            <li>
                                {"We then calculate the view-space position of nearby points in positive and negative x and y. We do this to get vectors that are tangent to the surface of the water based on the depth texture. It is an approximation, but allows us to get a fairly realistic normal by cross producting these two tangent vectors. I discarded any normals that were pointing directly in the Z axis in order to keep the background of the scene clear, however, a much better approach would have been to use a stencil buffer to render to only sections of the screen with water. The code I used is as follows:"}
                                <CodeSnippet>
{"vec3 GetNormal() {
    float depth = texture(depth_tex, uv).r;
    
    ivec2 texDimen = textureSize(depth_tex, 0);
    
    vec3 vs_pos = GetViewPos(uv);
    
    vec3 ddx = GetViewPos(uv + vec2(1, 0) / texDimen) - vs_pos;
    vec3 ddx2 = vs_pos - GetViewPos(uv + vec2(-1, 0) / texDimen);
    if (abs(ddx.z) > abs(ddx2.z)) {
        ddx = ddx2;
    }
    
    vec3 ddy = GetViewPos(uv + vec2(0, 1) / texDimen) - vs_pos;
    vec3 ddy2 = vs_pos - GetViewPos(uv + vec2(0, -1) / texDimen);
    if (abs(ddy.z) > abs(ddy2.z)) {
        ddy = ddy2;
    }
    vec3 norm = normalize(cross(ddx, ddy));
    if (norm.z + 1e-5 >= 1) discard;
    return norm;
}"}
                                </CodeSnippet>
                            </li>
                        </ul>
                    </p>
                    <hr/>
                    <h4>{"Step 4 - Rendering"}</h4>
                    <p>
                        {"With the normal extracted, the last step is simply rendering the water using some illumination model. I used Phong illumination with a cube-map to sample for rendering reflections and refractions. The results at various different steps are shown below."}
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <h5>{"Water rendered in full without blending the depth texture."}</h5>
                            <Image src="/images/waterflow/Reflective-spheres.png" alt="" />
                        </div>
                        <div style="flex: 50%;">
                            <h5>{"Water rendered blending with a small radius size for Gaussian."}</h5>
                            <Image src="/images/waterflow/Blended-reflections.png" alt="" />
                        </div>
                        <div style="flex: 50%;">
                            <h5>{"Water rendering blending with a large radius size (18 texels) for Gaussian."}</h5>
                            <Image src="/images/waterflow/Blending-depth-18-slow.png" alt="" />
                        </div>
                        <div style="flex: 50%;">
                            <h5>{"Water rendering on a cube of water with small particle size."}</h5>
                            <Image src="/images/waterflow/watercube.png" alt="" />
                        </div>
                    </div>

                    <hr/>
                    <h4>{"Cleanup"}</h4>
                    <p>
                        {"You'll notice in the pictures above, the water doesn't appear perfectly realistic. There are artifacts along the edges of the water and the particles are still fairly visible. There were two fixes I came up with for the artifact problem. The first was to simply render the depth texture at lower resolution. We can do this because accuracy isn't as much a concern as getting the water to look realistic. This ended up having a pretty significant improvement, which you can see going from the left picture to the right picture below."}
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <h5>{"Rendering of normals as it was before with Guassian filter and a high resolution depth texture."}</h5>
                            <Image src="/images/waterflow/smoothing-normals.png" alt="" />
                        </div>
                        <div style="flex: 50%;">
                            <h5>{"Rendering with a lower resolution depth texture and Guassian filter. Much cleaner :)"}</h5>
                            <Image src="/images/waterflow/smoothing-normals-improved.png" alt="" />
                        </div>
                    </div>

                    <div class="split">
                        <div style="flex: 50%;">
                            <p>
                                {"The second fix, which solved the artifacts at the edge of the cube came when I examined the pictures above. The problem clearly came from blending, but the Gaussian filter was being applied properly. The actual problem was it was blending the background color of the image with the normals of the water. This was a fairly simply fix, simply ignoring any normals from the background which shouldn't be considered anyways. With that, you can see a big improvement to the render."}
                            </p>
                            <p>
                                {"With that, we get a much nicer final render and all that's left is messing with the parameters for the illumination model to get a good looking effect. I messed with various appearances which you can see below."}
                            </p>
                        </div>
                        <div style="flex: 50%;">
                            <Image src="/images/waterflow/smoothing-normals-big-improvement.png" alt="" />
                        </div>
                    </div>
                    <div style="display: flex; justify-content: space-evenly; flex-wrap: wrap;">
                        <Image src="/images/waterflow/water-messing-with-color.png" />
                        <Image src="/images/waterflow/final-color-choice.png" />
                        <Image src="/images/waterflow/Final-water-cube.png" />
                        <Image src="/images/waterflow/final-smoothing-appearance.png" />
                        <Image src="/images/waterflow/reflection-refraction-alt.png" />
                        <Image src="/images/waterflow/refraction-reflection-combined.png" />
                    </div>
                    // For the Simulation Post
                    // <p>
                    //     I have later found out, thanks to my class in concurrency, a spin-lock
                    //     on the GPU is an awful idea (especially as the way I implemented it) due to
                    //     the structure of a GPU. The GPU is broken up into Thread-blocks, warps, 
                    //     and threads. Threads aren't exactly the same as they are on the CPU, since
                    //     the GPU has SIMD processors, typically 32 lanes wide. The threads
                    //     on a processor is called a warp and every thread in a warp executes in 
                    //     lock-step. With a spin-lock, this could easily prevent the warp from
                    //     progressing as a whole since a singular thread could hold the lock while
                    //     the rest in the warp need to wait. The threads that need to wait will prevent
                    //     the lock-holding thread from progressing. 
                    // </p>
                    // <p>
                    //     Luckily, in my implementation, the only problem was poor utilization and poor
                    //     performance since I acted at the scale of thread-blocks rather than on threads
                    //     individually. With this, I had only really one thread per warp perfoming work,
                    //     but it would prevent the lock-step problem. Without this, the screen would freeze
                    //     up as the GPU would just spin (luckily most modern GPUs will just exit the program if it
                    //     detects spining like this).
                    // </p>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3>{"References"}</h3>
            </SlantDisplay>
            <div class="references">
                <ol>
                    <li>
                        <a id="ref-sand-as-fluid" href="https://www.cs.ubc.ca/~rbridson/docs/zhu-siggraph05-sandfluid.pdf">{"Yongning Zhu, Robert Bridson. "}<em>{"Animating Sand as a Fluid"}</em>{", 01 July 2005."}</a>
                    </li>
                    <br/>
                    <li>
                        <a id="ref-water-rendering" href="https://developer.download.nvidia.com/presentations/2010/gdc/Direct3D_Effects.pdf">{"Simon Green. "}<em>{"GDC 2010 - Screen Space Fluid Rendering for Games"}</em>{", 9-13 March 2010."}</a>
                    </li>
                    <br/>
                    <li>
                        <a href="https://shot511.github.io/2018-05-29-how-to-setup-opengl-project-with-cmake/"><s>{"Tomasz Galaj. "}<em>{"How to setup OpenGL project with CMake"}</em></s></a><a id="ref-cmake" href="https://github.com/tgalaj/OpenGLSampleCmake?tab=readme-ov-file">{" CMake Opengl Boilerplate"}</a>
                    </li>
                </ol>
            </div>
        </BlogTemplate>
    }
}