# renderust

OpenGL-like renderer written from scratch in Rust.  
Based on this guide: [tinyrenderer](https://github.com/ssloy/tinyrenderer/wiki/Lesson-0:-getting-started). 

Triangle rasterization is done using [scanline method](https://en.wikipedia.org/wiki/Scanline_rendering). 

### Controls

    [1] — toggle normal map
    [2] — toggle specular light
    [3] — toggle glow map
    [4] — toggle self shadow
    [5] — toggle ambient occlusion
    [R] — toggle light spin

    [WASD / Space / Shift] — move model
    [LMB] — rotate model
    [Scroll] — zoom camera

    [Ctrl+S] — save image (to ./output.bmp)

## Demo

<img src="./demo/output_afro.png" width="512px" height="512px"><img/>

<details> <summary>Diablo</summary>

Without effects  
<img src="./demo/output_clear.png" width="512px" height="512px"><img/>  
With effects (normal maps, specular light, self shadowing, glow map, ambient occlusion)  
<img src="./demo/output_effects.png" width="512px" height="512px"><img/>  
Self shadows from two lights (which are directed top-down and back-forward)  
<img src="./demo/output_shadow.png" width="512px" height="512px"><img/></details>

<details> <summary>More pictures</summary>

<img src="./demo/mysterious_face.png" width="512px" height="512px"><img/> <img src="./demo/sideview.png" width="512px" height="512px"><img/></details>

<details> <summary>Video</summary>

<img src="./demo/effects.gif"><img/></details>