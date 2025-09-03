var canvas = document.getElementById("canvaselement");
if (canvas === null)
    throw new Error("Could not find canvas element");
var gl = canvas.getContext("webgl");
if (gl === null)
    throw new Error("Could not get WebGL context");
gl.viewport(0, 0, canvas.width, canvas.height);
gl.clearColor(0.5, 0.5, 0.5, 1.0);
gl.enable(gl.DEPTH_TEST);
gl.clear(gl.COLOR_BUFFER_BIT);
