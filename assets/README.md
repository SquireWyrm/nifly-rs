# Assets

These NIF files were made to test nifly-rs.

- `Bones.nif` - Skyrim LE version file that has three bones that deform a skinned mesh.
- `Cone.nif` - a cone with unneeded and unsorted nodes.
- `Cube.nif` - a simple cube.
- `Icosphere.nif` - an icosphere nested under transformed nodes.
- `Stencil_MW.nif` - nifly's `TestNifFile_Static_MW.nif` resized to a 160×96 quad, with an empty `NiTexturingProperty` on every shape and an `NiStencilProperty` added to the first shape (enabled, function 4, reference 7, mask 0xFF, actions 1/2/3, draw mode 2).
- `Strips_MW.nif` - nifly's `TestNifFile_Static_MW.nif` resized to a 160×96 quad, with an empty `NiTexturingProperty` on every shape, both shapes converted to `NiTriStrips` (one 4-point strip each), plus a third, `Shared`, sharing the second's data.
- `SubIndex_SE.nif` - nifly's `TestNifFile_Static_SE.nif` with both shapes retyped to `BSSubIndexTriShape`, each with one segment (flags 1, index 0, 1 triangle).
- `Suzanne.nif` - a standard model with collider based on Blender's Suzanne.
