use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{display::{atoms::{code::CodeSnippet, image_display::Image, slant_display::SlantDisplay}, organisms::{animated_banner::AnimatedBanner, page_footer::PageFooter, page_header::PageHeader}, pages::blog::BlogPost, post::data::BlogTemplate, theme::Theme}, router::Route};


pub fn page(ctx: &Context<BlogPost>, theme: &Theme) -> Html {
    let post = ctx.props().post;
    html! {
        <BlogTemplate title={post.title.clone()}>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3>{"Overview"}</h3>
                <hr/>
                <div class="blog-content">
                    <p>
                        {"The goal of this project was to implement an Inverse Kinematics system into Godot using GDExtension that allowed for various bone lengths, number of bones, and number of IK chains. I also had the goal of implementing rotational constraints for each bone joint, but found many difficulties with the implementation. Despite that, I will still cover the different approaches I used as well as the theoretical implementation."}
                    </p>
                    <p>
                        {"So, first of all, what is Inverse Kinematics? Inverse Kinematics or IK is a way of animating a set of bones to have realistic placement and orientation given some target position. This is really useful for the animation of limbs like hands and feet since it is much easier to give limbs a target position rather than define the rotations of each joint in a chain of bones."}
                    </p>
                    <p>
                        {"My implementation uses FABRIK, a method defined in the paper "}<em>{"FABRIK: A fast, iterative solver for the Inverse Kinematics problem"}</em>{" by Andreas Aristidou & Joan Lasenby. Below is the showcase video and following that is a walkthrough of my process and implementation. Here is the "}<a href="https://github.com/jlkump/ik-godot-extension"><Icon icon_id={IconId::BootstrapGithub} style="vertical-align: middle; margin-right: 10px;" width="1em" height="1em"/>{"Github link"}</a>{" for the project."}
                    </p>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3>{"Video Showcase"}</h3>
                <hr/>
                <div style="display: flex; flex-direction: column; align-items: center; padding: 30px;">
                    <div class="media-display">
                        <iframe width="100%" height="100%" src="https://www.youtube.com/embed/Zg2xze1RmAc?si=fiqa94oSvYkWOczD" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
                    </div>
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3>{"Process"}</h3>
                <hr/>
                <div class="blog-content">
                    <h4>{"FABRIK Method"}</h4>
                    <div class="split">
                        <div style="flex: 50%;">
                            <p>
                                {"To begin, we need an understanding of FABRIK, the method I used for my implementation of IK. FABRIK stands for Forward And Backward Reaching Inverse Kinematics. It is actually a pretty simple method and the name gives us a big hint on how the method works. Namely, there is a forward-reaching and backward-reaching stage, both of which have similar steps."}
                            </p>
                            <p>
                                {"Firstly, we need a defined chain of bones and joints for a model. I did this with some simple models made both in Godot and Blender, but normally bones would be defined by an animation rigger or modeler. For the sake of explanation, we consider a chain of bones with only four joints."}
                            </p>
                            <blockquote>
                                {"\"Bones define how sections of a model will move relative to the changes in the bone orientation and positions, making moving model limbs easier since we don't have to move every joint individually.\" - "}<a href="#ref-fabrik" class="invert-quote">{"FABRIK"}</a>
                            </blockquote>
                            <p>
                                {"The forward-reaching stage is fairly straight-forward, with only two major steps: First moving the end effector (the end joint of the bone chain) to the target position; And second, adjusting previous joints, putting them back at the appropriate distance from the bone before it."}
                            </p>
                        </div>
                        <div style="flex: 50%;">
                            <Image src="/images/inverse-kinematics/fabrik-fig1-example.png" alt="FABRIK Reference Figure" href="#ref-fabrik" ref_text="Image from FABRIK: A fast, iterative solver for the Inverse Kinematics problem, figure 1." height="36rem"/>
                        </div>
                    </div>
                    <p>
                        {"The backward-reaching stage is just as simple, with the process just being performed in reverse. First, the first joint is placed back at the initial position before performing IK. Then, the joints are adjusted in the same way, conserving the distances between bone joints."}
                    </p>
                    <p>
                        {"Lastly, both stages are performed multiple times until the end effector is within some threshold of the target or an iteration maximum is reached. That's the general idea, however, more specifically, before we begin performing FABRIK, we first check to see if the target is even within range. If it is, then perform FABRIK, otherwise, we have the bones stretch towards the target (which doesn't require the iterative approach above)."}
                    </p>
                    <hr/>
                    <h4>{"Godot Implementation"}</h4>
                    <p>
                        {"Now, understanding the method, we can discuss my implementation in GDExtension. I initially planned to incorporate IK changes into Godot using the existing scene-tree hierarchy for transforms, but found that the result was fairly buggy. When I modified the transform data, it would not have the expected result. I think this was due to the scene-tree hierarchy enforcing transform updates to children nodes after the positions were already in the correct spot."}
                    </p>
                    <p>
                        {"To fix this, my system works outside of the Godot scene-tree, querying the position of some list of "}<code><a href="https://docs.godotengine.org/en/stable/classes/class_node3d.html">{"Node3D"}</a></code>{"s (representing the joints), performing FABRIK, then updating the global positions and orientations of the "}<code>{"Node3D"}</code>{"s when finished. Specifically, this was implemented using a class called "}<code>{"InverseKinematicChain"}</code>{", which derived from the Godot "}<code><a href="https://docs.godotengine.org/en/stable/classes/class_node.html">{"Node"}</a></code>{" class. The class can be configured with an input set of "}<code>{"Node3D"}</code>{"s which become the joint positions for the bones. The initial  distances between the nodes are the initial distances between the joints and are  kept constant."} 
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <p>
                                {"With this, ik works, but only assuming that the ik chain doesn't move. If the chain were attached to some larger model, the chain would ignore the scene-tree hierarchy of transforms. To fix this, I simply put a marker node at the location of the chain's starting position and update the first joint's position to be placed at this node before each ik calculation."}
                            </p>
                            <p>
                                {"With that fixed, unconstrained IK finally works completely with arbitrary size and model hierarchies. To add in some more realistic movement, I have an IK controller class, which has some IK chains that it animates based on the movement of the player, extrapolating the speed of the movement of the player to predict joint placement. Also, I added a resting position to which the legs will try to return to if the movement has been still for some amount of time."}
                            </p>
                            <p>
                                {"All these properties can be edited within the editor for ease of use, the appearance
                                of which is shown here."}
                            </p>
                        </div>
                        <div style="flex: 50%;">
                            <Image src="/images/inverse-kinematics/Ik-chain-editor.png" width="35rem" height="30rem"/>
                        </div>
                    </div>
                    <hr/>
                    <h4>{"Rotational Constraints"}</h4>
                    <p>
                        {"I will first walkthrough an explanation for rotational constraints as it is discussed in the paper. It is a bit more complicated than the unconstrained method, but luckily doesn't need to concern itself with jacobian matrices that typical rotational constraints for IK need to worry about."}
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <Image src="/images/inverse-kinematics/fabrik-fig5-constraints.png" href="#ref-fabrik" ref_text="Image from FABRIK: A fast, iterative solver for the Inverse Kinematics problem, figure 5." height="36rem"/>
                        </div>
                        <div style="flex: 50%;">
                        <p>
                            {"The basic idea is that there are rotational constraints on each joint which constrains itself in only two major axes (relative to the bone), the vertical and horizontal.There is also the axis that the bone can spin on, but that axis has no impact on the bone length and thus does not impact the IK solver."}
                        </p>
                        <p>
                            {"The figure to the left showcases how this is done. For the first bone, there are no constraints, so "}<code>{"p"}<sub>{"4"}</sub></code>{" and "}<code>{"p"}<sub>{"3"}</sub></code>{" are moved the same as before. However, now "}<code>{"p"}<sub>{"2"}</sub></code>{" needs to be checked for constraints, thus we draw a line "}<code>{"L"}</code>{" from the previous  bone direction ("}<code>{"p"}<sub>{"3"}</sub>{" - p"}<sub>{"4"}</sub></code>{") and position ("}<code>{"p"}<sub>{"4"}</sub></code>{"),  then center a cone on the line. This cone is made by the rotational constraints of "}<code>{"p"}<sub>{"2"}</sub></code>{", named "}<code>{"θ"}<sub>{"1"}</sub></code>{", "}<code>{"θ"}<sub>{"2"}</sub></code>{", "}<code>{"θ"}<sub>{"3"}</sub></code>{", and "}<code>{"θ"}<sub>{"4"}</sub></code>{", and the height of the cone is determined by the projection of "}<code>{"p"}<sub>{"2"}</sub></code>{" onto the line "}<code>{"L"}</code>{", named "}<code>{"O"}</code>{". The figure (figure 4 in the paper) shows how this cone is constructed visually."}
                        </p>
                        </div>
                    </div>
                    <div class="split">
                        <div style="flex: 50%;">
                            <p>
                                {"The cone acts to constrain the range of motion for the joint "}<code>{"p"}<sub>{"2"}</sub></code>{", so we first need to check if the joint is outside the range of motion. If it isn't, then there is no reason to perform rotational constraints. In the example, "}<code>{"p"}<sub>{"2"}</sub></code>{" lies outside the range of motion, so it is projected onto the cone, then treated as normal, being constrained to the right position based on the initial distance between itself and "}<code>{"p"}<sub>{"3"}</sub></code>{"."}
                            </p>
                        </div>
                        <div style="flex: 50%;">
                            <Image src="/images/inverse-kinematics/fabrik-fig4-range-of-motion.png" href="#ref-fabrik" ref_text="Image from FABRIK: A fast, iterative solver for the Inverse Kinematics problem, figure 4."/>
                        </div>
                    </div>
                    <p>
                        {"I haven't gone over how to determine if the point is within the range of motion or how to project the point onto the cone if it is. The basic idea is that the point is transformed to the space with "}<code>{"O"}</code>{" as the origin, then the equation of elipse that makes up the quadrant the point is within is solved for with the point's "}<code>{"x"}</code>{","}<code>{"y"}</code>{" values. If the value is "}<code>{"1"}</code>{", then the point is outside the range of motion."} 
                    </p>
                    <p>
                        {"For determining the projection onto the cone, the paper solves the eclipse equation and line equation (from the origin to the point). This is done by using the Newton-Raphson method reference from another paper (Sung Joon Ahn, Wolfgang Rauh, Hans-Jnrgen Warnecke, "}<em>{"Least-squares orthogonal distances fitting of circle, sphere, ellipse, hyperbola, and parabola"}</em>{", Pattern Recognition 34, 2001). The method seemed overkill to me, especially since FABRIK is already an approximation, so I came up with two alternatives."}
                    </p>
                    <div class="split">
                        <div style="flex: 50%;">
                            <Image src="/images/inverse-kinematics/approach-1-diagram.png" alt="Inverse Kinematics Drawn Diagram - Approach 1" width="32rem"/>
                        </div>
                        <div style="flex: 50%;">
                            <p>
                                {"The first alternative method was to pick points along the line, performing essentially a binary search to see if the point was inside or outside the eclipse, progressively getting closer to the edge of the eclipse."}
                            </p>
                            <p>
                                {"In the figure, the dark red and blue dots are the axis maximums, the light red and blue dots are the projection of the point onto the axes, and the green elispe is the range of motion."}
                            </p>
                        </div>
                    </div>
                    <div class="split">
                        <div style="flex: 50%;">
                            <p>
                                {"The second approach was to project the axis maximum onto the line from the origin to the target. This was far more approximate but should still work."}
                            </p>
                            <p>
                                {"In the figure to the right, again, the dark red and blue dots are the axis maximums, the light red and blue dots are the projection of the point onto the axes, and the green elispe is the range of motion."}
                            </p>
                        </div>
                        <div style="flex: 50%;">
                            <Image src="/images/inverse-kinematics/approach-2-diagram.png" alt="Inverse Kinematics Drawn Diagram - Approach 2" width="34rem"/>
                        </div>
                    </div>

                    <div class="split">
                        <div style="flex: 40%;">
                            <iframe width="100%" height="515" src="https://www.youtube.com/embed/oD5Y2-G0xqE?si=7QoPYW11kRMS9tjl" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
                        </div>
                        <div style="flex: 50%; padding-left: 20px;">
                            <Image src="/images/inverse-kinematics/ik-constraints-drawn-visualization.png" alt="" />
                            <p>
                                <code>{"z"}<sub>{"p"}</sub></code>{" and "}<code>{"x"}<sub>{"p"}</sub></code>{" are the projected points of point "}<code>{"p"}<sub>{"i"}</sub></code>{" onto the axes for the basis of constraints."}
                            </p>
                        </div>
                    </div>
                    <p>
                        {"With either approach, I still had problem with rotational constraints working properly. For debugging, I made a visualization that showed the axes of the constrained bones as well as the maximum points for the rotational constraints and the projection of the point onto the axes. Visually, everything seemed to be in order (I used rotational constraints of 45 degrees in each axis). However, the bone chain acted wildly whenever the constraints were enforced."}
                    </p>
                    <p>
                        {"While working on the project, I thought these problems were coming from solving for the projection onto the cone incorrectly and thus I spent a lot of time trying to fix this problem. However, every approach I took still came up with the same problem. After finishing the project, I believe the problem was actually with the spin of the bones, as the \"up\" direction of the basis for the bone joint constraints would change every frame (when constrained),  thus making the constrainted point position change rapidly. I think I will return to attempt IK in a later project, though I think I might try using a different framework than Godot or use Godot's built-in bones. This way, I might be able to get things to work as expected, as many of my problems in this project came from conflicts between what I expected to happen and how Godot wanted things to work."}
                    </p>
                </div>

            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3>{"References"}</h3>
            </SlantDisplay>
            <div class="references">
                <ul>
                    <li>
                        <a id="ref-fabrik" href="http://www.andreasaristidou.com/publications/papers/FABRIK.pdf">{"Andreas Aristidou, Joan Lasenby. "}<em>{"FABRIK: A fast, iterative solver for the Inverse Kinematics problem"}</em>{", 9 May 2011. Elseiver"}</a>
                    </li>
                </ul>
            </div>
        </BlogTemplate>
    }
}