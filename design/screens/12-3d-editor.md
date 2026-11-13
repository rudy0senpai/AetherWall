# Screen 12 — 3D Wallpaper Editor

## Layout

```
+------------------------------------------+
| Navigation | Search | [3D Wallpapers]     |
+------------------------------------------+
| Scene library      Center viewport       |
| [card] [card]    +---------------------+  |
| [card] [card]    |     3D viewport     |  |
| [card] [card]    |  (first-person)     |  |
|                  +---------------------+  |
+-------------------+-----------------------+
                    |[hierarchy/scene]|[props/inspector]|
                    |  [mesh] [mat]   |  [cam] [light]  |
                    |  transform,     |  animation      |
                    |  material,      |  timeline       |
                    |  rig            |  [play]         |
                    +------------------+----------------+
                                           [render quality]
```

## Panels

1. **Scene library:** list of imported 3D scenes; import .glTF/.glb.
2. **Center viewport:** first-person or orbiting view; render controls
   (quality, wireframe, shadows).
3. **Hierarchy/scene panel:** scene graph; select mesh, material, light,
   camera, animation.
4. **Inspector/properties panel:** transform (position/rotation/scale),
   material properties, camera settings, animation timeline.
5. **Render quality controls:** resolution, MSAA, shadows, refresh rate.

## States

- Loading, no scenes, import error, render error, fallback (none).
