[!Title] Inverse Kinematics
[!Brief] Exploring Inverse Kinematics using the FABRIK method. Made using GDExtension and C++.

# Inverse Kinematics
---

## Overview
The goal of this project was to implement an Inverse Kinematics system into Godot using GDExtension that allowed for various bone lengths, number of bones, and number of IK chains. I also had the goal of implementing rotational constraints for each bone joint, but found many difficulties with the implementation. Despite that, I will still cover the different approaches I used as well as the theoretical implementation.

So, first of all, what is Inverse Kinematics? Inverse Kinematics or IK is a way of animating a set of bones to have realistic placement and orientation given some target position. This is really useful for the animation of limbs like hands and feet since it is much easier to give limbs a target position rather than define the rotations of each joint in a chain of bones.

My implementation uses FABRIK, a method defined in the paper *FABRIK: A fast, iterative solver for the Inverse Kinematics problem*
by Andreas Aristidou & Joan Lasenby. Below is the showcase video and following that is a walkthrough of my process and implementation. Here is the [Github link](https://github.com/jlkump/ik-godot-extension) for the project.

## Video Showcase
<iframe width="100%" height="700" src="https://www.youtube.com/embed/Zg2xze1RmAc?si=fiqa94oSvYkWOczD" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## Process
### FABRIK Method
To begin, we need an understanding of FABRIK, the method I used for my implementation of IK. FABRIK stands for Forward And Backward Reaching Inverse Kinematics. It is actually a pretty simple method and the name gives us a big hint on how the method works. Namely, there is a forward-reaching and backward-reaching stage, both of which have similar steps.
<img src="/images/inverse-kinematics/fabrik-fig1-example.png" alt="Inverse Kinematics Figure Example" title="Image from <em>FABRIK: A fast, iterative solver for the Inverse Kinematics problem</em>, figure 1." />